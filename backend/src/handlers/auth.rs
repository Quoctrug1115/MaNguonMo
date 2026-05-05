use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::env;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use axum::response::IntoResponse;
use uuid::Uuid;
use serde::{Deserialize, Serialize, ser};

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
) -> Result<axum::Json<serde_json::Value>, (axum::http::StatusCode, axum::Json<serde_json::Value>)> {

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
            return Err((
                axum::http::StatusCode::UNAUTHORIZED,
                Json(json!({ "error": "Email hoặc mật khẩu không chính xác" })),
            ));
        }
    };

    // 2. Kiểm tra mật khẩu
    let is_valid = match &user.password_hash {
        Some(hash) => bcrypt::verify(&payload.password, hash).unwrap_or(false),
        None => false,
    };

    if !is_valid {
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Email hoặc mật khẩu không chính xác" })),
        ));
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
        role: user.role.clone().unwrap_or_else(|| "customer".to_string()),
        exp: expiration,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Lỗi khi tạo token đăng nhập" })),
            ));
        }
    };

    // 4. Trả về Token và thông tin cơ bản cho Frontend
    Ok(Json(json!({
            "message": "Đăng nhập thành công!",
            "token": token,
            "user": {
                "id": user.id,
                "full_name": &user.full_name,
                "role": &user.role
            }
        })))
}

// API Lấy thông tin Profile (Được bảo vệ bởi Extractor Claims)
// SỬA LẠI: Trả về trực tiếp (StatusCode, Json<Value>) cho đồng bộ với register và login
pub async fn get_my_profile(
    claims: Claims,
    State(pool): State<PgPool>,
) -> (StatusCode, Json<Value>) {

    // 1. Chuyển ID từ dạng Chuỗi sang dạng UUID để query Database
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "ID người dùng không hợp lệ" })),
            );
        }
    };

    // 2. Lấy thông tin mới nhất của User từ Database
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, full_name, email, password_hash, phone, address, role, created_at, updated_at FROM users WHERE id = $1",
        user_id
    )
        .fetch_optional(&pool)
        .await;

    // 3. Trả kết quả về cho Frontend (Bỏ hết các đuôi .into_response() rườm rà)
    match user_result {
        Ok(Some(user)) => (
            StatusCode::OK,
            Json(json!({
                "message": "Lấy thông tin thành công!",
                "user": {
                    "id": user.id,
                    "full_name": user.full_name,
                    "email": user.email,
                    "phone": user.phone,
                    "address": user.address,
                    "role": user.role,
                    "created_at": user.created_at
                }
            })),
        ),

        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Không tìm thấy tài khoản người dùng này trong hệ thống" })),
        ),

        Err(e) => {
            tracing::error!("Lỗi DB khi lấy profile: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "Lỗi hệ thống khi lấy thông tin" })),
            )
        }
    }
}

// Struct nhận Token từ Vue gửi lên
#[derive(Deserialize)]
pub struct GoogleLoginReq {
    pub token: String,
}

// Struct để hứng dữ liệu Google trả về khi mình mang Token đi hỏi
#[derive(Deserialize)]
pub struct GoogleUserInfo {
    pub email: String,
    pub name: String,
    pub sub: String, // Google gọi ID của họ là "sub"
    pub picture: Option<String>,
}

pub async fn google_login(
    State(pool): State<PgPool>,
    Json(payload): Json<GoogleLoginReq>,
) -> Result<Json<Value>, (StatusCode, String)> {
    
    // 1. Mang Token của khách đi hỏi Google xem có chuẩn không
    let client = reqwest::Client::new();
    let google_res = client
        .get(format!(
            "https://oauth2.googleapis.com/tokeninfo?id_token={}",
            payload.token
        ))
        .send()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Không thể kết nối đến Google".to_string()))?;

    if !google_res.status().is_success() {
        return Err((StatusCode::UNAUTHORIZED, "Token Google không hợp lệ hoặc đã hết hạn!".to_string()));
    }

    // 2. Lấy thông tin khách hàng từ Google (Email, Tên, Ảnh)
    let google_user: GoogleUserInfo = google_res
        .json()
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lỗi đọc dữ liệu Google".to_string()))?;

    println!("Khách hàng chuẩn bị đăng nhập: {} ({})", google_user.name, google_user.email);

    // 3. Lưu vào Database (PostgreSQL)
    // Dùng tuyệt chiêu ON CONFLICT: Nếu email chưa có thì Thêm mới, nếu có rồi thì Cập nhật google_id
    let user_record = sqlx::query!(
        r#"
        INSERT INTO users (email, full_name, google_id, avatar_url, role)
        VALUES ($1, $2, $3, $4, 'customer')
        ON CONFLICT (email) 
        DO UPDATE SET 
            google_id = EXCLUDED.google_id,
            avatar_url = EXCLUDED.avatar_url,
            updated_at = NOW()
        RETURNING id, full_name, role
        "#,
        google_user.email,
        google_user.name,
        google_user.sub,
        google_user.picture
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Lỗi Database: {}", e)))?;

    // 4. Trả kết quả thành công về cho Vue
    // 1. Tạo JWT Token cho khách dùng Google giống hệt khách thường
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Lỗi tính toán thời gian")
        .timestamp() as usize;

    let claims = Claims { // Đảm bảo bạn đã khai báo struct Claims ở file này
        sub: user_record.id.to_string(),
        email: google_user.email,
        role: user_record.role.clone().unwrap_or_else(|| "customer".to_string()),
        exp: expiration,
    };

    let token = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(_) => {
            return Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Lỗi khi tạo token đăng nhập Google".to_string(),
            ));
        }
    };

    // 2. Trả về đúng cấu trúc mà Frontend Vue đang chờ đợi
    Ok(Json(json!({
        "message": "Đăng nhập Google thành công!",
        "token": token,
        "user": {
            "id": user_record.id,
            "full_name": user_record.full_name,
            "role": user_record.role
        }
    })))
}
