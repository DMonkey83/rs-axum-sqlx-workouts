use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use config::Config;
use dotenv::dotenv;
use redis::Client;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod api;
mod config;
mod helpers;
mod models;
mod repository;
mod auth;

use crate::api::routes::create_router;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
    pub redis_client: Client,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = config::Config::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to the database was successful");
            pool
        }
        Err(e) => {
            println!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    };

    let redis_client = match Client::open(config.redis_url.to_owned()) {
        Ok(client) => {
            println!("✅Connection to the redis is successful!");
            client
        }
        Err(e) => {
            println!("🔥 Error connecting to Redis: {}", e);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let port = std::env::var("PORT").expect("PORT must be set");
    let host = std::env::var("HOST").expect("HOST must be set");
    let address = format!("{}:{}", host, port);
    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
        redis_client: redis_client.clone(),
    }))
    .layer(cors);

    println!("Server running on port {}", port.clone());
    println!("Address http://{}", address.clone());
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
