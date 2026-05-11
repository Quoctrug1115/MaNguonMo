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
use std::collections::HashMap;
// Import các "Khuôn mẫu" dữ liệu từ thư mục models
use crate::models::product::{CreateProductRequest, Product, VariantReq};
use crate::models::user::Claims;


// cấu trúc nhận dữ liệu Lọc & Tìm kiếm từ Frontend
#[derive(Deserialize, Default)]
pub struct ProductFilter {
    pub search: Option<String>,
    pub category_id: Option<Uuid>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}


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


pub async fn create_product(
    claims: Claims,
    State(pool): State<PgPool>,
    Json(payload): Json<CreateProductRequest>,
) -> (StatusCode, Json<Value>) {

    // 1. MỞ GIAO DỊCH (TRANSACTION)
    let mut tx = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::error!("Lỗi mở transaction: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Lỗi kết nối cơ sở dữ liệu" })),
            );
        }
    };

    // 2. THÊM SẢN PHẨM GỐC VÀO BẢNG PRODUCTS
    // Chú ý: Đổi .fetch_one(&pool) thành .fetch_one(&mut *tx)
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
    .fetch_one(&mut *tx)
    .await;

    // Kiểm tra kết quả lưu sản phẩm
    let new_product = match insert_result {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Lỗi DB khi thêm sản phẩm gốc: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Không thể lưu sản phẩm vào kho" })),
            );
        }
    };

    // 3. THÊM BIẾN THỂ (VARIANTS) NẾU CÓ
    if let Some(variants) = payload.variants {
        for variant in variants {
            let variant_insert = sqlx::query!(
                r#"
                INSERT INTO product_variants (product_id, color_name, color_hex, stock, image_url)
                VALUES ($1, $2, $3, $4, $5)
                "#,
                new_product.id, // Lấy ID của sản phẩm vừa tạo ở bước 2
                variant.color_name,
                variant.color_hex,
                variant.stock,
                variant.image_url
            )
            .execute(&mut *tx)
            .await;

            // Nếu có bất kỳ màu nào lưu thất bại, trả về lỗi ngay lập tức
            // Lúc này tx sẽ bị drop và toàn bộ lệnh insert_product ở trên sẽ bị rollback
            if let Err(e) = variant_insert {
                tracing::error!("Lỗi DB khi thêm biến thể màu sắc: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Không thể lưu biến thể sản phẩm" })),
                );
            }
        }
    }

    // 4. LƯU CHÍNH THỨC (COMMIT TRANSACTION)
    if let Err(e) = tx.commit().await {
        tracing::error!("Lỗi commit transaction: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Lỗi hệ thống khi hoàn tất quá trình lưu trữ" })),
        );
    }

    // 5. TRẢ VỀ THÀNH CÔNG
    (
        StatusCode::CREATED,
        Json(json!({
            "message": "Thêm sản phẩm thành công!",
            "product": new_product,
            "added_by": claims.email
        })),
    )
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


pub async fn get_product_variants(
    State(pool): State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    
    // THÊM DẤU ? ĐỂ SQLX KHÔNG BỊ CRASH KHI GẶP NULL TỪ LEFT JOIN
    let rows = sqlx::query!(
        r#"
        SELECT 
            p.id::TEXT as "product_id!", 
            p.name as "product_name!", 
            p.price::FLOAT as "price!", 
            p.image_url as "main_image?",
            v.id::TEXT as "variant_id?",
            v.color_hex as "color_hex?",
            v.stock as "stock?",
            v.image_url as "variant_image?"
        FROM products p
        LEFT JOIN product_variants v ON p.id = v.product_id
        ORDER BY p.name ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        println!("Lỗi Database: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Lỗi DB: {}", e))
    })?;

    let mut product_map: HashMap<String, Value> = HashMap::new();

    for row in rows {
        let entry = product_map.entry(row.product_id.clone()).or_insert_with(|| {
            json!({
                "id": row.product_id,
                "product_name": row.product_name,
                "category": "Digital Product", // Đặt tạm để giống Figma
                "price": row.price,
                "main_image": row.main_image,
                "total_stock": 0,
                "variants": []
            })
        });

        // Xử lý an toàn với Option
        if let Some(color_hex) = row.color_hex {
            let stock = row.stock.unwrap_or(0);
            
            // Cộng dồn tổng tồn kho (Piece)
            let current_total = entry["total_stock"].as_i64().unwrap_or(0);
            entry["total_stock"] = json!(current_total + stock as i64);
            
            entry["variants"].as_array_mut().unwrap().push(json!({
                "variant_id": row.variant_id,
                "color_hex": color_hex,
                "stock": stock,
                "image_url": row.variant_image
            }));
        }
    }

    let mut data: Vec<Value> = product_map.into_values().collect();
    // Sắp xếp lại theo tên bảng chữ cái
    data.sort_by(|a, b| {
        let name_a = a["product_name"].as_str().unwrap_or("");
        let name_b = b["product_name"].as_str().unwrap_or("");
        name_a.cmp(name_b)
    });

    Ok(Json(json!({ "status": "success", "data": data })))
}