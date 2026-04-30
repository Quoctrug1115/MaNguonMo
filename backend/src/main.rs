use axum::{routing::{get, post}, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import thêm thư viện CORS
use tower_http::cors::{Any, CorsLayer};
use axum::http::{Method, header};

mod models;
mod handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to Postgres");

    tracing::info!("Connected to the database successfully!");

    // CẤU HÌNH CORS Ở ĐÂY
    let cors = CorsLayer::new()
        .allow_origin(Any) // Trong môi trường dev, cho phép mọi origin gọi API
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/register", post(handlers::auth::register))
        // Thêm route Login vào đây:
        .route("/api/auth/login", post(handlers::auth::login))
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server is running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Server is healthy and running!"
}