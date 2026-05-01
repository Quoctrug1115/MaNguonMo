use axum::{extract::{State, Query}, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::models::product::Product;

// Struct hứng tham số trên URL (Ví dụ: ?page=2&limit=10)
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// API Lấy danh sách sản phẩm (Có phân trang)
pub async fn get_products(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>, // Bắt tham số từ URL
) -> (StatusCode, Json<Value>) {

    // 1. Cài đặt giá trị mặc định: Nếu Frontend không gửi, lấy trang 1, mỗi trang 12 sản phẩm
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(12);

    // Công thức tính số dòng cần bỏ qua (OFFSET)
    let offset = (page - 1) * limit;

    // 2. Đếm tổng số sản phẩm trong kho để tính tổng số trang
    let count_result = sqlx::query!("SELECT COUNT(*) as count FROM products")
        .fetch_one(&pool)
        .await;

    let total_items = count_result.unwrap().count.unwrap_or(0);
    // Tính tổng số trang (làm tròn lên)
    let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

    // 3. Lấy dữ liệu có LIMIT và OFFSET
    let products_result = sqlx::query_as!(
        Product,
        "SELECT id, category_id, name, description, price, original_price, discount_percent, stock_quantity, image_url, is_new, rating, reviews_count, created_at, updated_at
         FROM products
         ORDER BY created_at DESC
         LIMIT $1 OFFSET $2",
        limit,
        offset
    )
        .fetch_all(&pool)
        .await;

    match products_result {
        Ok(products) => (
            StatusCode::OK,
            Json(json!({
                "message": "Lấy danh sách sản phẩm thành công",
                "data": products,
                // Trả thêm thông tin phân trang cho Vue biết đường mà vẽ nút
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

// API Thêm sản phẩm mới (Được bảo vệ bởi Extractor Claims)
pub async fn create_product(
    claims: Claims, // <--- Bắt buộc phải có thẻ thông hành (Token) hợp lệ mới chạy vào đây
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductRequest>,
) -> (StatusCode, Json<Value>) {

    // Thực thi câu lệnh chèn vào CSDL và hứng lại kết quả vừa tạo (RETURNING)
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
            StatusCode::CREATED, // Code 201: Đã tạo thành công
            Json(json!({
                "message": "Thêm sản phẩm thành công!",
                "product": new_product,
                "added_by": claims.email // In ra email của người vừa thêm (để chứng minh Extractor hoạt động)
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