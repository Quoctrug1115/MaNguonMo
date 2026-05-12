use axum::{extract::State, http::StatusCode, Json};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

// 1. Struct để hứng dữ liệu từ DB gửi về cho Vue
#[derive(Serialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String, // Giả sử bảng categories của bạn có cột tên là 'name'
}

// 2. Hàm xử lý API
pub async fn get_all_categories(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CategoryResponse>>, (StatusCode, String)> {
    
    // Truy vấn tất cả danh mục từ bảng categories
    let categories = sqlx::query_as!(
        CategoryResponse,
        "SELECT id, name FROM categories ORDER BY name ASC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(categories))
}