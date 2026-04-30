use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::env;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

use crate::models::user::{RegisterRequest, LoginRequest, Claims, User};

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<Value>) {
    // 1. Kiểm tra xem email đã tồn tại trong DB chưa
    let user_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
    )
        .bind(&payload.email)
        .fetch_one(&pool)
        .await
        .unwrap_or(false);

    if user_exists {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Email đã được sử dụng" })),
        );
    }

    // 2. Mã hóa mật khẩu với bcrypt
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Lỗi mã hóa mật khẩu" })),
            );
        }
    };

    // 3. Lưu user mới vào Database
    let insert_result = sqlx::query(
        "INSERT INTO users (full_name, email, password_hash) VALUES ($1, $2, $3)"
    )
        .bind(&payload.full_name)
        .bind(&payload.email)
        .bind(&password_hash)
        .execute(&pool)
        .await;

    match insert_result {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({ "message": "Đăng ký tài khoản thành công!" })),
        ),
        Err(e) => {
            tracing::error!("Lỗi khi insert user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Lỗi hệ thống khi tạo tài khoản" })),
            )
        }
    }
}
pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Value>) {

    // 1. Tìm user trong DB bằng email
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, full_name, email, password_hash, phone, address, role, created_at, updated_at FROM users WHERE email = $1",
        payload.email
    )
        .fetch_optional(&pool)
        .await;

    let user = match user_result {
        Ok(Some(u)) => u,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Email hoặc mật khẩu không chính xác" })),
            );
        }
    };

    // 2. Kiểm tra mật khẩu
    let is_valid = verify(&payload.password, &user.password_hash).unwrap_or(false);

    if !is_valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Email hoặc mật khẩu không chính xác" })),
        );
    }

    // 3. Tạo JWT Token
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Set thời gian hết hạn là 24 giờ
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Lỗi tính toán thời gian")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user.role.unwrap_or_else(|| "customer".to_string()),
        exp: expiration,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Lỗi khi tạo token đăng nhập" })),
            );
        }
    };

    // 4. Trả về Token và thông tin cơ bản cho Frontend
    (
        StatusCode::OK,
        Json(json!({
            "message": "Đăng nhập thành công",
            "token": token,
            "user": {
                "id": user.id,
                "full_name": user.full_name,
                "email": user.email,
                "role": claims.role
            }
        })),
    )
}