use std::net::SocketAddr;

use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vop_rust=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("SERVER_PORT").expect("Environment variable SERVER_PORT not set");
    let port = port
        .parse::<u16>()
        .expect("Environment variable SERVER_PORT must be a valid number");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);

    let app = vop_rust::run();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
