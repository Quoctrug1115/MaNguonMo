use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use axum::extract::Path;

// Import các "Khuôn mẫu" dữ liệu từ thư mục models
use crate::models::product::{CreateProductRequest, Product};
use crate::models::user::Claims; // Import "Anh bảo vệ"

// 1. Thêm trường category vào để hứng dữ liệu từ Frontend
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub category: Option<String>,
}

// ==============================================================
// API Lấy danh sách sản phẩm (Public - Có phân trang & Lọc Category)
// ==============================================================
pub async fn get_products(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
) -> (StatusCode, Json<Value>) {

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(12);
    let offset = (page - 1) * limit;

    // 1. FIX LỖI MOVE: Biến Option<String> thành Option<&str> để xài được nhiều lần
    let category_filter = query.category.as_deref();

    // 2. Đếm số lượng sản phẩm
    let count_result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM products
        -- FIX LỖI ÉP KIỂU: Thêm ::varchar vào đuôi $1
        WHERE $1::varchar IS NULL OR category_id = (SELECT id FROM categories WHERE slug = $1::varchar LIMIT 1)
        "#,
        category_filter // Dùng lần 1
    )
        .fetch_one(&pool)
        .await;

    let total_items = count_result.unwrap().count.unwrap_or(0);
    let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

    // 3. Lấy dữ liệu sản phẩm
    let products_result = sqlx::query_as!(
        Product,
        r#"
        SELECT id, category_id, name, description, price, original_price, discount_percent, stock_quantity, image_url, is_new, rating, reviews_count, created_at, updated_at
        FROM products
        -- FIX LỖI ÉP KIỂU: Thêm ::varchar vào đuôi $3
        WHERE $3::varchar IS NULL OR category_id = (SELECT id FROM categories WHERE slug = $3::varchar LIMIT 1)
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset,
        category_filter // Dùng lần 2 thoải mái không bị lỗi
    )
        .fetch_all(&pool)
        .await;

    match products_result {
        Ok(products) => (
            StatusCode::OK,
            Json(json!({
                "message": "Lấy danh sách sản phẩm thành công",
                "data": products,
                "pagination": {
                    "current_page": page,
                    "limit": limit,
                    "total_items": total_items,
                    "total_pages": total_pages
                }
            })),
        ),
        Err(e) => {
            tracing::error!("Lỗi DB: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Lỗi hệ thống" })))
        }
    }
}

// ==============================================================
// 2. API Thêm sản phẩm mới (Private - Bắt buộc có thẻ Claims)
// ==============================================================
pub async fn create_product(
    claims: Claims,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductRequest>,
) -> (StatusCode, Json<Value>) {

    let insert_result = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (
            category_id, name, description, price, original_price,
            discount_percent, stock_quantity, image_url, is_new,
            rating, reviews_count
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING id, category_id, name, description, price, original_price, discount_percent, stock_quantity, image_url, is_new, rating, reviews_count, created_at, updated_at
        "#,
        payload.category_id,
        payload.name,
        payload.description,
        payload.price,
        payload.original_price,
        payload.discount_percent,
        payload.stock_quantity,
        payload.image_url,
        payload.is_new,
        payload.rating,
        payload.reviews_count
    )
        .fetch_one(&pool)
        .await;

    match insert_result {
        Ok(new_product) => (
            StatusCode::CREATED,
            Json(json!({
                "message": "Thêm sản phẩm thành công!",
                "product": new_product,
                "added_by": claims.email
            })),
        ),
        Err(e) => {
            tracing::error!("Lỗi DB khi thêm sản phẩm: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Không thể lưu sản phẩm vào kho" })),
            )
        }
    }
}

pub async fn get_product_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>, // Lấy ID (chuẩn UUID) từ URL
) -> (StatusCode, Json<Value>) {

    let product_result = sqlx::query_as!(
        Product,
        "SELECT id, category_id, name, description, price, original_price, discount_percent, stock_quantity, image_url, is_new, rating, reviews_count, created_at, updated_at
         FROM products
         WHERE id = $1",
        id
    )
        .fetch_optional(&pool)
        .await;

    match product_result {
        Ok(Some(product)) => (
            StatusCode::OK,
            Json(json!({ "data": product })),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Sản phẩm không tồn tại" })),
        ),
        Err(e) => {
            tracing::error!("Lỗi DB: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Lỗi hệ thống" })))
        }
    }
}