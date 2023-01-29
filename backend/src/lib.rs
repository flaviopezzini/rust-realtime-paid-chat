use axum::{http::StatusCode, response::Html, response::IntoResponse, routing::get, Router};
use std::sync::Arc;

use crate::chat::*;
use crate::redis_wrapper::RedisWrapper;

mod chat;
mod redis_wrapper;

pub async fn run() -> axum::Router {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let conn = client.get_async_connection()
        .await
        .expect("Unable to connect to Redis");

    let app_state = Arc::new(
        AppState::new(
            RedisWrapper::new(conn)
        )
    );

    Router::new()
        .route("/", get(index))
        .route("/health_check", get(health_check))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state)
}

// Include utf-8 file at **compile** time.
pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../../web/public/index.html"))
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
