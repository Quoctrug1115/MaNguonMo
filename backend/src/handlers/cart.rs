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