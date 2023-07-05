use axum::{http::StatusCode, response::Html, response::IntoResponse, routing::get, Router};

use crate::chat::*;
use crate::redis_wrapper::RedisWrapper;

use deadpool_diesel::postgres::{Runtime, Manager, Pool};

use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::RunQueryDsl;

mod chat;
mod redis_wrapper;
mod advisor_list;
mod database;
mod models;
mod schema;
mod chat_repository;

pub async fn run(redis_port: u16, database_url: String) -> axum::Router {
    let client = redis::Client::open(format!("redis://127.0.0.1:{redis_port}")).unwrap();

    // let manager = Manager::new(database_url, Runtime::Tokio1);
    // let pool = Pool::builder(manager)
    //     .max_size(8)
    //     .build()
    //     .unwrap();

    // create a new connection pool with the default config
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(std::env::var("DATABASE_URL")?);
    let pool = Pool::builder(config).build()?;

    let app_state =
        AppState::new(
            RedisWrapper::new(client),
            pool
        );

    Router::new()
        .route("/", get(index))
        .route("/health_check", get(health_check))
        .route("/advisor-list", get(advisor_list::advisor_list))
        .route("/websocket/:advisor/:customer", get(websocket_handler))
        .with_state(app_state)
}

// Include utf-8 file at **compile** time.
pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../../web/public/index.html"))
}

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
