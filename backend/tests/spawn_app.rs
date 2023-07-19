use axum::Router;

use testcontainers::{clients, Container};
use testcontainers::images::redis::Redis;
use testcontainers::images::postgres::Postgres;

use testcontainers::clients::Cli;

use std::net::{SocketAddr, TcpListener};

pub struct TestApp<'a> {
    pub addr: std::net::SocketAddr,
    redis_container: Container<'a, Redis>,
    pg_container: Container<'a, Postgres>
}

impl<'a> TestApp<'a> {

pub async fn spawn_app(docker: &'a Cli) -> TestApp<'a> {
    let redis_container = docker.run(testcontainers::images::redis::Redis);
    let redis_port = redis_container.get_host_port_ipv4(6379);

    let pg_container = docker.run(testcontainers::images::postgres::Postgres::default());
    let pg_port = pg_container.get_host_port_ipv4(5432);

    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    let database_url = format!("postgresql://postgres:@localhost:{pg_port}");

    let app: Router = vop_rust::run(redis_port, database_url).await;

    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
    TestApp {
        addr,
        redis_container: redis_container,
        pg_container: pg_container
    }
    
}    
}


