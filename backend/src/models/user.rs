use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};


// Struct này dùng để map với dữ liệu lấy từ Database
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub full_name: String,
    pub email: String,
    pub password_hash: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub role: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Struct này dùng để hứng dữ liệu JSON từ Frontend gửi lên khi Đăng ký
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

// Struct hứng dữ liệu JSON khi người dùng Đăng nhập
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Struct chứa thông tin sẽ được mã hóa vào trong JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (thường là User ID)
    pub email: String,
    pub role: String,
    pub exp: usize,  // Expiration time (Thời gian hết hạn)
}