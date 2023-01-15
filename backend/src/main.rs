mod chat;

use axum::{response::Html, routing::get, Router};
use std::{net::SocketAddr, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::chat::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vop_rust=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(index))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Include utf-8 file at **compile** time.
async fn index() -> Html<&'static str> {
    Html(std::include_str!("../../web/public/index.html"))
}
