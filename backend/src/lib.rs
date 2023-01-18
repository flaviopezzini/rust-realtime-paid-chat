use axum::{http::StatusCode, response::Html, response::IntoResponse, routing::get, Router};
use std::sync::Arc;

use crate::chat::*;

mod chat;

pub fn run() -> axum::Router {
    let app_state = Arc::new(AppState::new());

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
