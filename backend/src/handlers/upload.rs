use axum::{extract::Multipart, http::StatusCode, Json};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

pub async fn upload_images(mut multipart: Multipart) -> Result<Json<Value>, (StatusCode, String)> {
    let upload_dir = "./uploads";
    if !Path::new(upload_dir).exists() {
        fs::create_dir_all(upload_dir).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let mut urls = Vec::new();

    // SỬA: Thay vì dùng .unwrap() gây sập Server, ta dùng while let Ok(Some(field))
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = field.file_name().unwrap_or("unknown.jpg").to_string();
        
        let ext = file_name.split('.').last().unwrap_or("jpg");
        let new_name = format!("{}.{}", uuid::Uuid::new_v4(), ext);
        let filepath = format!("{}/{}", upload_dir, new_name);

        // Bắt lỗi an toàn khi đọc data của file
        if let Ok(data) = field.bytes().await {
            // Lưu file vào ổ cứng
            if tokio::fs::write(&filepath, &data).await.is_ok() {
                urls.push(format!("http://localhost:3000/uploads/{}", new_name));
            }
        }
    }

    Ok(Json(json!({
        "status": "success",
        "urls": urls
    })))
}