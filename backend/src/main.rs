use axum::{routing::{get, post, put, delete}, Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::services::ServeDir;
use axum::extract::DefaultBodyLimit;
use crate::handlers::auth::google_login;
use crate::handlers::order::{checkout, get_user_orders};
use crate::handlers::wishlist::{add_to_wishlist, get_wishlist, remove_from_wishlist};
use tower_http::cors::{Any, CorsLayer};
use axum::http::{Method};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};

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

    let cors = CorsLayer::new()
        .allow_origin(Any) // Trong môi trường dev, cho phép mọi origin gọi API
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

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
        .route("/api/cart", get(handlers::cart::get_cart).post(handlers::cart::add_to_cart))
        .route("/api/cart/item/:id", put(handlers::cart::update_cart_quantity).delete(handlers::cart::delete_cart_item))
        .route("/api/orders/checkout", post(handlers::order::checkout))
        .route("/api/orders/user", get(handlers::order::get_user_orders))
        .route("/api/wishlist",get(handlers::wishlist::get_wishlist).post(handlers::wishlist::add_to_wishlist))
        .route("/api/wishlist/:product_id", delete(handlers::wishlist::remove_from_wishlist))
        .route("/api/profile", get(handlers::auth::get_profile).put(handlers::auth::update_user_profile))
        .route("/api/admin/product-variants", get(handlers::product::get_product_variants))
        .route("/api/admin/products", post(handlers::product::create_product))
        .route("/api/admin/upload", post(handlers::upload::upload_images))
        .nest_service("/uploads", ServeDir::new("uploads"))
        .route("/api/admin/products/:id", delete(handlers::product::delete_product))
        .route("/api/categories", get(handlers::category::get_all_categories))
        .route("/api/admin/products/:id", get(handlers::product::get_product_detail))
        .route("/api/admin/products/:id", put(handlers::product::update_product))
        .route("/api/admin/orders", get(handlers::order::get_all_orders_admin))
        .route("/api/admin/orders/:id/status", put(handlers::order::update_order_status))
        .layer(DefaultBodyLimit::max(52_428_800))
        .layer(cors)
        .with_state(pool);

        let port = std::env::var("PORT").unwrap_or_else(|_| "10000".to_string());
        let addr = format!("0.0.0.0:{}", port).parse::<SocketAddr>().unwrap();
    
        tracing::info!("Server is running on http://{}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Server is healthy and running!"
}