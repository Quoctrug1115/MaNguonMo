use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;
use axum::extract::Path;


#[derive(Deserialize)]
pub struct CheckoutRequest {
    pub user_id: Uuid,
    pub shipping_address: String,
    pub phone_number: String,
}

pub async fn checkout(
    State(pool): State<PgPool>,
    Json(payload): Json<CheckoutRequest>,
) -> (StatusCode, Json<Value>) {

    // --- BƯỚC 0: BẮT ĐẦU TRANSACTION (Tạo phiên giao dịch mới) ---
    // Mọi lỗi từ đây trở đi sẽ được thu hồi (rollback) lại CSDL
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Lỗi bắt đầu phiên giao dịch: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi hệ thống"})));
        }
    };

    // 1. LẤY GIỎ HÀNG (Chú ý: Dùng &mut *tx thay vì &pool)
    let cart_items_result = sqlx::query!(
        r#"
        SELECT c.product_id, c.quantity, COALESCE(p.price, 0)::float8 as "price!" 
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        "#,
        payload.user_id
    )
    .fetch_all(&mut *tx) // Thực thi trong transaction
    .await;

    let cart_items = match cart_items_result {
        Ok(items) => items,
        Err(e) => { tracing::error!("Lỗi lấy giỏ: {:?}", e); return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi hệ thống"}))); }
    };

    if cart_items.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Giỏ hàng đang trống!"})));
    }

    // 2. TÍNH TỔNG TIỀN
    let mut total_price: f64 = 0.0;
    for item in &cart_items {
        total_price += item.price * (item.quantity as f64);
    }

    // 3. TẠO ĐƠN HÀNG (Dùng &mut *tx)
    let order_insert_result = sqlx::query!(
        r#"
        INSERT INTO orders (user_id, total_price, shipping_address, phone_number, status)
        VALUES ($1, $2::float8, $3, $4, 'pending')
        RETURNING id
        "#,
        payload.user_id, total_price, payload.shipping_address, payload.phone_number
    )
    .fetch_one(&mut *tx) // Thực thi trong transaction
    .await;

    let order_record = match order_insert_result {
        Ok(record) => record,
        Err(e) => { tracing::error!("Lỗi tạo đơn: {:?}", e); return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Không thể tạo hóa đơn"}))); }
    };
    let order_id = order_record.id;

    // 4. LƯU CHI TIẾT ĐƠN HÀNG (Dùng &mut *tx)
    for item in cart_items {
        let item_insert_result = sqlx::query!(
            r#"
            INSERT INTO order_items (order_id, product_id, quantity, price_at_purchase)
            VALUES ($1, $2, $3, $4::float8)
            "#,
            order_id, item.product_id, item.quantity, item.price 
        )
        .execute(&mut *tx) // Thực thi trong transaction
        .await;

        // --- NẾU LỖI GIỮA ĐƯỜNG: Về nguyên tắc, Transaction sẽ tự hủy, nhưng tốt nhất là in lỗi ra ---
        if let Err(e) = item_insert_result {
            tracing::error!("Lỗi lưu chi tiết (Transaction will abort): {:?}", e);
            // Bạn có thể trả về lỗi ở đây, hoặc để cho Transaction tự động Rollback
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi lưu chi tiết sản phẩm"})));
        }
    }

    // 5. DỌN DẸP GIỎ HÀNG (Dùng &mut *tx)
    let delete_cart_result = sqlx::query!("DELETE FROM cart_items WHERE user_id = $1", payload.user_id)
        .execute(&mut *tx) // Thực thi trong transaction
        .await;

    if let Err(e) = delete_cart_result {
        tracing::error!("Lỗi xóa giỏ: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Không thể dọn dẹp giỏ hàng"})));
    }

    // --- BƯỚC 6: CHỐT LỆNH (COMMIT) ---
    // Chỉ khi code chạy tới tận đây, mọi dữ liệu ở trên mới thực sự được lưu vào Database!
    match tx.commit().await {
        Ok(_) => {
            (StatusCode::OK, Json(json!({ 
                "message": "Đặt hàng thành công!",
                "order_id": order_id
            })))
        },
        Err(e) => {
            tracing::error!("Lỗi chốt phiên giao dịch: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi chốt đơn hàng"})))
        }
    }
}

// API: Lấy danh sách đơn hàng của một người dùng
pub async fn get_user_orders(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> (StatusCode, Json<Value>) {

    // 1. Lấy tất cả các hóa đơn (orders) của user này, sắp xếp từ mới nhất đến cũ nhất
    let orders_result = sqlx::query!(
        r#"
        SELECT id, total_price::float8, status, shipping_address, phone_number, created_at 
        FROM orders 
        WHERE user_id = $1 
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await;

    let orders = match orders_result {
        Ok(o) => o,
        Err(e) => {
            tracing::error!("Lỗi lấy danh sách đơn hàng: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi hệ thống"})));
        }
    };

    // 2. Tạo một mảng trống để chứa toàn bộ dữ liệu trả về cho Vue
    let mut order_list = Vec::new();

    // 3. Lặp qua từng hóa đơn để nhặt "chi tiết các món đồ" bên trong nó
    for order in orders {
        let items_result = sqlx::query!(
            r#"
            SELECT oi.quantity, oi.price_at_purchase::float8 as price, p.name as product_name, p.image_url 
            FROM order_items oi
            JOIN products p ON oi.product_id = p.id
            WHERE oi.order_id = $1
            "#,
            order.id
        )
        .fetch_all(&pool)
        .await
        .unwrap_or_default(); // Nếu lỗi thì trả về mảng rỗng cho an toàn

        // Đóng gói từng món đồ vào định dạng JSON
        let items_json: Vec<serde_json::Value> = items_result.into_iter().map(|item| {
            json!({
                "product_name": item.product_name,
                "image_url": item.image_url,
                "quantity": item.quantity,
                "price": item.price
            })
        }).collect();

        // Gói hóa đơn + các món đồ vào chung 1 hộp
        order_list.push(json!({
            "id": order.id,
            "total_price": order.total_price,
            "status": order.status,
            "shipping_address": order.shipping_address,
            "phone_number": order.phone_number,
            "created_at": order.created_at,
            "items": items_json
        }));
    }

    // 4. Gửi cục hàng này về cho Frontend
    (StatusCode::OK, Json(json!({ "data": order_list })))
}