use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,

    // Vì ON DELETE SET NULL và không có NOT NULL, nên phải dùng Option
    pub category_id: Option<Uuid>,

    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub original_price: Option<i64>,

    // Các trường DEFAULT không có NOT NULL trong SQLx vẫn được coi là có thể null (Option)
    pub discount_percent: Option<i32>,
    pub stock_quantity: i32, // Có NOT NULL
    pub image_url: Option<String>,
    pub is_new: Option<bool>,

    // Kiểu FLOAT trong Postgres map với f64 trong Rust
    pub rating: Option<f64>,
    pub reviews_count: Option<i32>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Struct hứng dữ liệu JSON khi thêm mới sản phẩm
#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub category_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub original_price: Option<i64>,
    pub discount_percent: Option<i32>,
    pub stock_quantity: i32,
    pub image_url: Option<String>,
    pub is_new: Option<bool>,
    pub rating: Option<f64>,
    pub reviews_count: Option<i32>,
    pub variants: Option<Vec<VariantReq>>,
}

#[derive(Debug, Deserialize)]
pub struct VariantReq {
    pub color_name: String,
    pub color_hex: String,
    pub stock: i32,
    pub image_url: Option<String>,
}