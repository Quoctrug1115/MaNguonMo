use axum::{routing::{get, post, put, delete}, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::services::ServeDir;
use crate::handlers::auth::google_login;
use crate::handlers::order::{checkout, get_user_orders};
use crate::handlers::wishlist::{add_to_wishlist, get_wishlist, remove_from_wishlist};
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
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/users/me", get(handlers::auth::get_profile))
        .route("/api/products", get(handlers::product::get_products))
        .route("/api/products", post(handlers::product::create_product))
        .route("/api/products/:id", get(handlers::product::get_product_by_id))
        .nest_service("/images", ServeDir::new("../images_product"))
        .route("/api/auth/google", post(handlers::auth::google_login))
        .route("/api/cart", post(handlers::cart::add_to_cart))
        .route("/api/cart/:user_id", get(handlers::cart::get_cart))
        .route("/api/cart/item/:id", put(handlers::cart::update_cart_quantity))
        .route("/api/cart/item/:id", delete(handlers::cart::delete_cart_item))
        .route("/api/orders/checkout", post(handlers::order::checkout))
        .route("/api/orders/user/:user_id", get(handlers::order::get_user_orders))
        .route("/api/wishlist", post(handlers::wishlist::add_to_wishlist))
        .route("/api/wishlist/:user_id", get(handlers::wishlist::get_wishlist))
        .route("/api/wishlist/:user_id/:product_id", delete(handlers::wishlist::remove_from_wishlist))
        .route("/api/profile/:user_id", get(handlers::auth::get_profile).put(handlers::auth::update_user_profile))
        .route("/api/admin/product-variants", get(handlers::product::get_product_variants))
        .route("/api/admin/products", post(handlers::product::create_product))
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