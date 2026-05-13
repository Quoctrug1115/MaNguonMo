use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use tower_http::classify;
use uuid::Uuid;
use axum::extract::Path;
use crate::models::user::Claims;

#[derive(Deserialize)]
pub struct CheckoutRequest {
    pub shipping_address: String,
    pub phone_number: String,
}

pub async fn checkout(
    claims: Claims,
    State(pool): State<PgPool>,
    Json(payload): Json<CheckoutRequest>,
) -> (StatusCode, Json<Value>) {

    // --- MỚI THÊM: Bóc tách user_id từ Token ---
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Token không hợp lệ"}))),
    };

    // --- BƯỚC 0: BẮT ĐẦU TRANSACTION ---
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Lỗi bắt đầu phiên giao dịch: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi hệ thống"})));
        }
    };

    // 1. LẤY GIỎ HÀNG 
    let cart_items_result = sqlx::query!(
        r#"
        SELECT c.product_id, c.quantity, COALESCE(p.price, 0)::float8 as "price!" 
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        "#,
        user_id // SỬA: Dùng biến user_id từ Token
    )
    .fetch_all(&mut *tx) 
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

    // 3. TẠO ĐƠN HÀNG 
    let order_insert_result = sqlx::query!(
        r#"
        INSERT INTO orders (user_id, total_price, shipping_address, phone_number, status)
        VALUES ($1, $2::float8, $3, $4, 'pending')
        RETURNING id
        "#,
        user_id, total_price, payload.shipping_address, payload.phone_number // SỬA: Dùng biến user_id từ Token
    )
    .fetch_one(&mut *tx) 
    .await;

    let order_record = match order_insert_result {
        Ok(record) => record,
        Err(e) => { tracing::error!("Lỗi tạo đơn: {:?}", e); return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Không thể tạo hóa đơn"}))); }
    };
    let order_id = order_record.id;

    // 4. LƯU CHI TIẾT ĐƠN HÀNG 
    for item in cart_items {
        let item_insert_result = sqlx::query!(
            r#"
            INSERT INTO order_items (order_id, product_id, quantity, price_at_purchase)
            VALUES ($1, $2, $3, $4::float8)
            "#,
            order_id, item.product_id, item.quantity, item.price 
        )
        .execute(&mut *tx) 
        .await;

        if let Err(e) = item_insert_result {
            tracing::error!("Lỗi lưu chi tiết (Transaction will abort): {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi lưu chi tiết sản phẩm"})));
        }
    }

    // 5. DỌN DẸP GIỎ HÀNG 
    let delete_cart_result = sqlx::query!("DELETE FROM cart_items WHERE user_id = $1", user_id) // SỬA: Dùng biến user_id từ Token
        .execute(&mut *tx) 
        .await;

    if let Err(e) = delete_cart_result {
        tracing::error!("Lỗi xóa giỏ: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Không thể dọn dẹp giỏ hàng"})));
    }

    // --- BƯỚC 6: CHỐT LỆNH (COMMIT) ---
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
    claims: Claims,
    State(pool): State<PgPool>,
) -> (StatusCode, Json<Value>) {

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"error": "Token không hợp lệ"}))),
    };
    
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



#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn get_all_orders_admin(
    // Tùy vào cách cấu hình của bạn, hãy thêm AdminClaims vào đây để bảo mật nhé!
    // AdminClaims(_claims): AdminClaims, 
    State(pool): State<PgPool>,
) -> (StatusCode, Json<Value>) {

    // Lấy tất cả đơn hàng và JOIN với bảng users để lấy Tên & Email khách hàng
    let orders_result = sqlx::query!(
        r#"
        SELECT o.id, o.total_price::float8, o.status, o.shipping_address, o.phone_number, o.created_at,
               u.full_name, u.email
        FROM orders o
        JOIN users u ON o.user_id = u.id
        ORDER BY o.created_at DESC
        "#
    )
    .fetch_all(&pool)
    .await;

    let orders = match orders_result {
        Ok(o) => o,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Lỗi DB: {}", e)}))),
    };

    let mut order_list = Vec::new();

    for order in orders {
        // Nhặt chi tiết món đồ cho từng đơn
        let items = sqlx::query!(
            r#"
            SELECT oi.quantity, oi.price_at_purchase::float8 as price, p.name as product_name
            FROM order_items oi
            JOIN products p ON oi.product_id = p.id
            WHERE oi.order_id = $1
            "#,
            order.id
        ).fetch_all(&pool).await.unwrap_or_default();

        let items_json: Vec<Value> = items.into_iter().map(|item| {
            json!({
                "product_name": item.product_name,
                "quantity": item.quantity,
                "price": item.price
            })
        }).collect();

        order_list.push(json!({
            "id": order.id,
            "customer_name": order.full_name,
            "customer_email": order.email,
            "total_price": order.total_price,
            "status": order.status,
            "shipping_address": order.shipping_address,
            "phone_number": order.phone_number,
            "created_at": order.created_at,
            "items": items_json
        }));
    }

    (StatusCode::OK, Json(json!({ "data": order_list })))
}

// 2. Cập nhật trạng thái đơn hàng (Pending -> Shipping -> Completed)
pub async fn update_order_status(
    State(pool): State<PgPool>,
    Path(order_id): Path<Uuid>,
    Json(payload): Json<UpdateStatusRequest>,
) -> (StatusCode, Json<Value>) {
    
    let res = sqlx::query!(
        "UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2", 
        payload.status, order_id
    )
    .execute(&pool)
    .await;

    match res {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return (StatusCode::NOT_FOUND, Json(json!({"error": "Không tìm thấy đơn hàng"})));
            }
            (StatusCode::OK, Json(json!({"message": "✅ Đã cập nhật trạng thái đơn hàng!"})))
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Lỗi: {}", e)}))),
    }
}