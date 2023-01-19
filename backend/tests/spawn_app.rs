use axum::Router;

use std::net::{SocketAddr, TcpListener};

pub async fn spawn_app() -> std::net::SocketAddr {
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    let app: Router = vop_rust::run();

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
    addr
}
