use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct WishlistRequest {
    pub user_id: Uuid,
    pub product_id: Uuid,
}

// 1. API: THÊM VÀO MỤC YÊU THÍCH (POST)
pub async fn add_to_wishlist(
    State(pool): State<PgPool>,
    Json(payload): Json<WishlistRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    
    // Tuyệt chiêu ON CONFLICT DO NOTHING: Nếu đã thả tim rồi mà bấm thêm lần nữa thì DB tự động bỏ qua, không báo lỗi sập server!
    sqlx::query!(
        r#"
        INSERT INTO wishlist_items (user_id, product_id) 
        VALUES ($1, $2)
        ON CONFLICT (user_id, product_id) DO NOTHING
        "#,
        payload.user_id,
        payload.product_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "message": "Đã thêm vào mục yêu thích" })))
}

// 2. API: LẤY DANH SÁCH MỤC YÊU THÍCH (GET)
pub async fn get_wishlist(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, String)> {

    let wishlist_items = sqlx::query!(
        r#"
        SELECT w.product_id, p.name as product_name, p.price::float8, p.image_url 
        FROM wishlist_items w
        JOIN products p ON w.product_id = p.id
        WHERE w.user_id = $1
        ORDER BY w.created_at DESC
        "#,
        user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Đóng gói dữ liệu trả về cho Frontend
    let items_json: Vec<serde_json::Value> = wishlist_items.into_iter().map(|item| {
        json!({
            "product_id": item.product_id,
            "product_name": item.product_name,
            "price": item.price,
            "image_url": item.image_url
        })
    }).collect();

    Ok(Json(json!({ "data": items_json })))
}

// 3. API: BỎ THẢ TIM (DELETE)
pub async fn remove_from_wishlist(
    State(pool): State<PgPool>,
    Path((user_id, product_id)): Path<(Uuid, Uuid)>, // Nhận 2 ID cùng lúc từ đường dẫn
) -> Result<Json<Value>, (StatusCode, String)> {

    sqlx::query!(
        "DELETE FROM wishlist_items WHERE user_id = $1 AND product_id = $2",
        user_id,
        product_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(json!({ "message": "Đã bỏ yêu thích" })))
}