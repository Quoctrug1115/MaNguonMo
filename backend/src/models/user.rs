use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// NẠP THÊM VŨ KHÍ CHO BẢO VỆ (Thư viện dùng cho Extractor)
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};


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

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Claims {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Tìm Header có tên là "Authorization"
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());

        let auth_header = match auth_header {
            Some(header) => header,
            None => return Err((StatusCode::UNAUTHORIZED, "Thiếu thẻ thông hành (Token)".to_string())),
        };

        // 2. Kiểm tra xem có đúng định dạng "Bearer <token>" không
        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Sai định dạng Token".to_string()));
        }

        // 3. Cắt bỏ 7 ký tự đầu ("Bearer ") để lấy đúng cái chuỗi mã hóa
        let token = &auth_header[7..];

        // 4. Lấy chìa khóa bí mật từ file .env
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_default();

        // 5. Tiến hành giải mã và kiểm tra hạn sử dụng
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Token đã hết hạn hoặc bị làm giả!".to_string()))?;

        // 6. Nếu mọi thứ OK, trả về thông tin User (Claims)
        Ok(token_data.claims)
    }
}