use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

// Cấu trúc dữ liệu Frontend sẽ gửi lên khi bấm "Thêm vào giỏ"
#[derive(Deserialize)]
pub struct AddToCartRequest {
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

// 1. API THÊM VÀO GIỎ HÀNG
pub async fn add_to_cart(
    State(pool): State<PgPool>,
    Json(payload): Json<AddToCartRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    
    // Tuyệt chiêu UPSERT của PostgreSQL: 
    // Nếu chưa có thì INSERT (thêm mới). Nếu có rồi thì UPDATE (cộng dồn số lượng).
    let result = sqlx::query!(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, product_id)
        DO UPDATE SET 
            quantity = cart_items.quantity + EXCLUDED.quantity,
            updated_at = NOW()
        "#,
        payload.user_id,
        payload.product_id,
        payload.quantity
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({ "message": "Đã thêm vào giỏ hàng thành công!" }))),
        Err(e) => {
            tracing::error!("Lỗi thêm giỏ hàng: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Lỗi khi lưu vào giỏ hàng".to_string()))
        }
    }
}

// 2. API LẤY DANH SÁCH GIỎ HÀNG (CỦA 1 USER)
pub async fn get_cart(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>, // Lấy user_id từ đường dẫn URL
) -> Result<Json<Value>, (StatusCode, String)> {
    
    // Phép thuật JOIN: Lấy số lượng từ bảng cart_items, kết hợp với tên, hình ảnh, giá từ bảng products
    let cart_items = sqlx::query!(
        r#"
        SELECT 
            c.id as cart_item_id,
            c.product_id,
            c.quantity,
            p.name as product_name,
            p.image_url,
            p.price
        FROM cart_items c
        JOIN products p ON c.product_id = p.id
        WHERE c.user_id = $1
        ORDER BY c.created_at DESC
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Lỗi lấy giỏ hàng: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Lỗi tải giỏ hàng".to_string())
    })?;

    let cart_json: Vec<serde_json::Value> = cart_items.into_iter().map(|item| {
        json!({
            "cart_item_id": item.cart_item_id,
            "product_id": item.product_id,
            "quantity": item.quantity,
            "product_name": item.product_name,
            "image_url": item.image_url,
            "price": item.price
        })
    }).collect();

    // 2. Trả về mảng cart_json đã được xử lý
    Ok(Json(json!({
        "message": "Thành công",
        "data": cart_json 
    })))
}

// 1. API CẬP NHẬT SỐ LƯỢNG (Dùng cho nút + và -)
pub async fn update_cart_quantity(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>, // ID của dòng trong bảng cart_items
    Json(payload): Json<serde_json::Value>, // Nhận { "quantity": new_val }
) -> Result<Json<Value>, (StatusCode, String)> {
    
    let new_quantity = payload["quantity"].as_i64().unwrap_or(1) as i32;

    sqlx::query!(
        "UPDATE cart_items SET quantity = $1, updated_at = NOW() WHERE id = $2",
        new_quantity,
        id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "message": "Cập nhật số lượng thành công" })))
}

// 2. API XÓA MỘT MÓN ĐỒ KHỎI GIỎ
pub async fn delete_cart_item(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, String)> {

    sqlx::query!("DELETE FROM cart_items WHERE id = $1", id)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "message": "Đã xóa sản phẩm khỏi giỏ hàng" })))
}