use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::env;
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};
use uuid::Uuid;
use serde::{Deserialize};
use bcrypt::{hash as bcrypt_hash, verify, DEFAULT_COST};
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
    let password_hash = match bcrypt_hash(&payload.password, DEFAULT_COST) {
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

    // 1. Trích xuất role ra một biến riêng trước (dùng clone để không làm mất dữ liệu gốc)
    let user_role = user.role.clone().unwrap_or_else(|| "user".to_string());

    // 2. Tạo JWT Token
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Lỗi tính toán thời gian")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: user_role.clone(), // <-- Dùng biến vừa tạo (Lần 1)
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

    // 3. Trả về Token và thông tin cơ bản cho Frontend
    Ok(Json(json!({
        "status": "success",
        "message": "Đăng nhập thành công!",
        "token": token,
        "user": {
            "email": user.email,
            "role": user_role // <-- Dùng biến vừa tạo (Lần 2)
        }
    })))
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}

pub async fn get_profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Value>, (StatusCode, String)> {
    
    let user = sqlx::query!(
        r#"SELECT id, full_name, email, phone, address FROM users WHERE id = $1"#,
        user_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lỗi DB".to_string()))?;

    match user {
        Some(u) => Ok(Json(json!({
            "id": u.id,
            "full_name": u.full_name,
            "email": u.email,
            "phone": u.phone,
            "address": u.address
        }))),
        None => Err((StatusCode::NOT_FOUND, "Không tìm thấy user".to_string())),
    }
}

// 2. API Cập nhật thông tin Cá nhân (PUT)
pub async fn update_user_profile(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<Value>, (StatusCode, String)> {
    
    // 1. Lấy thông tin user hiện tại từ DB để xem họ ĐÃ CÓ mật khẩu chưa
    let user_db = sqlx::query!("SELECT password_hash FROM users WHERE id = $1", user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lỗi truy xuất dữ liệu".to_string()))?;

    let mut new_hashed_password: Option<String> = None;

    // 2. Nếu người dùng muốn đổi/tạo mật khẩu (có nhập ô Mật khẩu mới)
    if let Some(new_pwd) = &payload.new_password {
        
        // Kiểm tra xem User này đã có password trong DB chưa
        match &user_db.password_hash {
            Some(existing_hash) if !existing_hash.is_empty() => {
                // TRƯỜNG HỢP 1: TÀI KHOẢN THƯỜNG (Đã có pass) -> BẮT BUỘC kiểm tra mật khẩu cũ
                let curr_pwd = payload.current_password.as_ref()
                    .ok_or((StatusCode::BAD_REQUEST, "Vui lòng nhập mật khẩu cũ!".to_string()))?;
                
                let is_valid = verify(curr_pwd, existing_hash).unwrap_or(false);
                if !is_valid {
                    return Err((StatusCode::BAD_REQUEST, "Mật khẩu cũ không chính xác!".to_string()));
                }
            },
            _ => {
                // TRƯỜNG HỢP 2: TÀI KHOẢN GOOGLE (Chưa có pass) -> BỎ QUA check mật khẩu cũ
                // Tự động cho phép đi tiếp xuống bước mã hóa luôn!
            }
        }

        // Mã hóa mật khẩu mới
        let hashed = bcrypt_hash(new_pwd, DEFAULT_COST)
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Lỗi hệ thống khi mã hóa mật khẩu".to_string()))?;
        
        new_hashed_password = Some(hashed);
    }

    // 3. Câu lệnh UPDATE giữ nguyên như cũ...
    let update_result = sqlx::query!(
        r#"
        UPDATE users 
        SET first_name = COALESCE($1, first_name),
            last_name = COALESCE($2, last_name),
            email = COALESCE($3, email),
            address = COALESCE($4, address),
            password_hash = COALESCE($5, password_hash)
        WHERE id = $6
        "#,
        payload.first_name,
        payload.last_name,
        payload.email,
        payload.address,
        new_hashed_password, 
        user_id
    )
    .execute(&pool)
    .await;

    match update_result {
        Ok(_) => Ok(Json(json!({ "message": "Cập nhật thành công!" }))),
        Err(e) => {
            tracing::error!("Lỗi cập nhật user: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Không thể cập nhật hồ sơ".to_string()))
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
