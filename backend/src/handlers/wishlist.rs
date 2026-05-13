use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::user::Claims;

#[derive(Deserialize)]
pub struct WishlistRequest {
    pub product_id: Uuid,
}

// 1. API: THÊM VÀO MỤC YÊU THÍCH (POST)
pub async fn add_to_wishlist(
    claims: Claims,
    State(pool): State<PgPool>,
    Json(payload): Json<WishlistRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Token lỗi"})))),
    };

    // Tuyệt chiêu ON CONFLICT DO NOTHING
    sqlx::query!(
        r#"
        INSERT INTO wishlist_items (user_id, product_id) 
        VALUES ($1, $2)
        ON CONFLICT (user_id, product_id) DO NOTHING
        "#,
        user_id,
        payload.product_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?; 

    Ok(Json(json!({ "message": "Đã thêm vào mục yêu thích" })))
}

// 2. API: LẤY DANH SÁCH MỤC YÊU THÍCH (Dùng Token)
pub async fn get_wishlist(
    claims: Claims, // Tự động lấy UserID từ Token
    State(pool): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    // Bóc tách ID an toàn
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Token không hợp lệ"})))),
    };

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
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Đóng gói dữ liệu
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
    claims: Claims,
    State(pool): State<PgPool>,
    Path((product_id)): Path<Uuid>, // Nhận 2 ID cùng lúc từ đường dẫn
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

    let user_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Token lỗi"})))),
        };

// Chạy lệnh xóa
    let result = sqlx::query!(
        "DELETE FROM wishlist_items WHERE user_id = $1 AND product_id = $2",
        user_id,
        product_id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({"message": "Đã xóa khỏi yêu thích"}))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))),
    }
}