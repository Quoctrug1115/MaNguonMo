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
use crate::models::product::{CreateProductRequest, Product};
use crate::models::user::AdminClaims;


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
    AdminClaims(claims): AdminClaims, // Chỉ admin mới được phép thêm sản phẩm
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

    if let Some(specs) = payload.specifications {
        for spec in specs {
            let spec_insert = sqlx::query!(
                r#"
                INSERT INTO product_specifications (product_id, spec_name, spec_value)
                VALUES ($1, $2, $3)
                "#,
                new_product.id, // ID của sản phẩm vừa sinh ra
                spec.spec_name,
                spec.spec_value
            )
            .execute(&mut *tx)
            .await;

            if let Err(e) = spec_insert {
                tracing::error!("Lỗi DB khi thêm thông số kỹ thuật: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": "Không thể lưu thông số kỹ thuật" })),
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

pub async fn delete_product(
    AdminClaims(_claims): AdminClaims, // Bảo vệ bằng quyền Admin
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>, // Lấy ID sản phẩm từ trên URL xuống
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    
    // 1. Mở giao dịch
    let mut tx = pool.begin().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Lỗi DB: {}", e)})))
    })?;

    // 2. Xóa dữ liệu ở các bảng con (Biến thể & Thông số) TRƯỚC
    sqlx::query!("DELETE FROM product_specifications WHERE product_id = $1", id)
        .execute(&mut *tx).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Lỗi xóa thông số: {}", e)})))
        })?;

    sqlx::query!("DELETE FROM product_variants WHERE product_id = $1", id)
        .execute(&mut *tx).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Lỗi xóa biến thể: {}", e)})))
        })?;

    // 3. Cuối cùng, xóa Sản phẩm gốc ở bảng cha
    let result = sqlx::query!("DELETE FROM products WHERE id = $1", id)
        .execute(&mut *tx).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Lỗi xóa sản phẩm: {}", e)})))
        })?;

    // 4. Chốt giao dịch
    tx.commit().await.map_err(|_| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi hệ thống khi chốt lệnh xóa"})))
    })?;

    // Kiểm tra xem có thực sự xóa được dòng nào không (nhỡ ID không tồn tại)
    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND, 
            Json(json!({"error": "Không tìm thấy sản phẩm này trong kho!"}))
        ));
    }

    Ok(Json(json!({
        "status": "success",
        "message": "🗑️ Đã xóa sản phẩm và các dữ liệu liên quan thành công!"
    })))
}

// API: GET /api/admin/products/:id
pub async fn get_product_detail(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    
    // 1. Lấy thông tin cơ bản
    let product = sqlx::query!("SELECT * FROM products WHERE id = $1", id)
        .fetch_optional(&pool).await.map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    let product = match product {
        Some(p) => p,
        None => return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Không tìm thấy sản phẩm"})))),
    };

    // 2. Lấy danh sách Biến thể
    let variants = sqlx::query!(
        "SELECT id, color_name, color_hex, stock, image_url FROM product_variants WHERE product_id = $1", 
        id
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();
    // 3. Lấy danh sách Thông số
    let specs = sqlx::query!("SELECT spec_name, spec_value FROM product_specifications WHERE product_id = $1", id)
        .fetch_all(&pool).await.unwrap_or_default();

    // 4. Ghép tất cả lại thành 1 cục JSON trả về Frontend
    Ok(Json(json!({
        "status": "success",
        "data": {
            "product": {
                "id": product.id,
                "category_id": product.category_id,
                "name": product.name,
                "description": product.description,
                "price": product.price,
                "original_price": product.original_price,
                "discount_percent": product.discount_percent,
                "stock_quantity": product.stock_quantity,
                "image_url": product.image_url,
                "is_new": product.is_new
            },
            "variants": variants.into_iter().map(|v| json!({
                "id": v.id,
                "color_name": v.color_name,
                "color_hex": v.color_hex,
                "stock": v.stock,
                "image_url": v.image_url
            })).collect::<Vec<_>>(),
            "specifications": specs.into_iter().map(|s| json!({
                "spec_name": s.spec_name,
                "spec_value": s.spec_value
            })).collect::<Vec<_>>()
        }
    })))
}


// API: PUT /api/admin/products/:id
pub async fn update_product(
    AdminClaims(_claims): AdminClaims,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateProductRequest>, // Tái sử dụng Struct cũ
) -> (StatusCode, Json<Value>) {
    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi DB"}))),
    };

    // 1. Cập nhật thông tin cơ bản của Sản phẩm
    let update_res = sqlx::query!(
        r#"
        UPDATE products 
        SET category_id = $1, name = $2, description = $3, price = $4, original_price = $5, 
            discount_percent = $6, stock_quantity = $7, image_url = $8, is_new = $9
        WHERE id = $10
        "#,
        payload.category_id, payload.name, payload.description, payload.price, 
        payload.original_price, payload.discount_percent, payload.stock_quantity, 
        payload.image_url, payload.is_new, id
    )
    .execute(&mut *tx).await;

    if update_res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Không thể cập nhật sản phẩm"})));
    }

    // 2. XÓA SẠCH biến thể và thông số cũ của sản phẩm này
    let _ = sqlx::query!("DELETE FROM product_variants WHERE product_id = $1", id).execute(&mut *tx).await;
    let _ = sqlx::query!("DELETE FROM product_specifications WHERE product_id = $1", id).execute(&mut *tx).await;

    // 3. XÂY LẠI: Chèn toàn bộ Biến thể mới
    if let Some(variants) = payload.variants {
        for v in variants {
            let _ = sqlx::query!(
                "INSERT INTO product_variants (product_id, color_name, color_hex, stock, image_url) VALUES ($1, $2, $3, $4, $5)",
                id, v.color_name, v.color_hex, v.stock, v.image_url
            ).execute(&mut *tx).await;
        }
    }

    // 4. XÂY LẠI: Chèn toàn bộ Thông số mới
    if let Some(specs) = payload.specifications {
        for s in specs {
            let _ = sqlx::query!(
                "INSERT INTO product_specifications (product_id, spec_name, spec_value) VALUES ($1, $2, $3)",
                id, s.spec_name, s.spec_value
            ).execute(&mut *tx).await;
        }
    }

    if tx.commit().await.is_ok() {
        (StatusCode::OK, Json(json!({"message": "Cập nhật sản phẩm thành công!"})))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Lỗi khi chốt dữ liệu"})))
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
                "SELECT spec_name as spec_key, spec_value FROM product_specifications WHERE product_id = $1 ORDER BY created_at",
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