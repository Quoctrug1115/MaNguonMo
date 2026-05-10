use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use axum::extract::Path;
use uuid::Uuid;

// Import các "Khuôn mẫu" dữ liệu từ thư mục models
use crate::models::product::{CreateProductRequest, Product};
use crate::models::user::Claims; // Import "Anh bảo vệ"

// Cấu trúc nhận dữ liệu phân trang từ Frontend
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub category: Option<String>,
}
// cấu trúc nhận dữ liệu Lọc & Tìm kiếm từ Frontend
#[derive(Deserialize, Default)]
pub struct ProductFilter {
    pub search: Option<String>,
    pub category_id: Option<Uuid>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}

// ==============================================================
// API Lấy danh sách sản phẩm (Public - Có phân trang & Lọc Category)
// ==============================================================
pub async fn get_products(
    State(pool): State<PgPool>,
    Query(filter): Query<ProductFilter>, // Nhận biến filter từ URL (?search=...&min_price=...)
) -> Result<Json<Value>, (StatusCode, String)> {

    // Xử lý từ khóa tìm kiếm (Nếu không gõ gì thì tìm chuỗi rỗng '%')
    let search_term = filter.search.unwrap_or_default();
    let search_pattern = format!("%{}%", search_term);

    // Dùng tuyệt chiêu COALESCE để bỏ qua điều kiện nếu Frontend không truyền vào
    let products_result = sqlx::query!(
        r#"
        SELECT id, name, description, price::float8, stock_quantity, category_id, image_url, created_at
        FROM products
        WHERE name ILIKE $1
          AND ($2::uuid IS NULL OR category_id = $2)
          AND ($3::float8 IS NULL OR price >= $3)
          AND ($4::float8 IS NULL OR price <= $4)
        ORDER BY created_at DESC
        "#,
        search_pattern,
        filter.category_id,
        filter.min_price,
        filter.max_price
    )
    .fetch_all(&pool)
    .await;

    let products = match products_result {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Lỗi tải sản phẩm: {:?}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Lỗi hệ thống".to_string()));
        }
    };

    // Đóng gói trả về JSON
    let items_json: Vec<serde_json::Value> = products.into_iter().map(|item| {
        json!({
            "id": item.id,
            "name": item.name,
            "description": item.description,
            "price": item.price,
            "stock_quantity": item.stock_quantity,
            "category_id": item.category_id,
            "image_url": item.image_url,
            "created_at": item.created_at
        })
    }).collect();

    Ok(Json(json!({ "data": items_json })))
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

#[derive(Serialize)]
pub struct ProductSpec {
    pub spec_key: String,
    pub spec_value: String,
}

#[derive(Serialize)]
pub struct ProductReview {
    pub id: uuid::Uuid,
    pub user_name: String,
    pub rating: i32,
    pub content: Option<String>,
    pub is_verified: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_product_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> (StatusCode, Json<Value>) {

    // 1. Lấy thông tin cơ bản của sản phẩm (Y hệt cũ)
    let product_result = sqlx::query_as!(
        Product,
        "SELECT id, category_id, name, description, price, original_price, discount_percent, stock_quantity, image_url, is_new, rating, reviews_count, created_at, updated_at
         FROM products WHERE id = $1",
        id
    )
        .fetch_optional(&pool)
        .await;

    match product_result {
        Ok(Some(product)) => {
            // 2. [LẤY DỮ LIỆU THẬT MỚI] Lấy danh sách Thông số kỹ thuật
            let specs_result = sqlx::query_as!(
                ProductSpec,
                "SELECT spec_key, spec_value FROM product_specifications WHERE product_id = $1 ORDER BY created_at",
                id
            )
                .fetch_all(&pool)
                .await;

            let specs = specs_result.unwrap_or_default(); // Nếu lỗi thì trả về mảng rỗng

            // 3. [LẤY DỮ LIỆU THẬT MỚI] Lấy danh sách Đánh giá
            let reviews_result = sqlx::query_as!(
                    ProductReview,
                    "SELECT id, user_name, rating, content, is_verified, created_at
                     FROM product_reviews
                     WHERE product_id = $1::uuid
                     ORDER BY created_at DESC",
                    id
                )
                .fetch_all(&pool)
                .await;

            let reviews = reviews_result.unwrap_or_default();

            // 4. Gom tất cả dữ liệu lại và trả về cho Frontend
            (
                StatusCode::OK,
                Json(json!({
                    "data": {
                        "product": product,
                        "specifications": specs,
                        "reviews": reviews
                    }
                })),
            )
        },
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