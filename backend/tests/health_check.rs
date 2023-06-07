mod spawn_app;

use testcontainers::clients;

#[tokio::test]
async fn health_check_works() {

    let docker = clients::Cli::default();
    let container = docker.run(testcontainers::images::redis::Redis);
    let redis_port = container.get_host_port_ipv4(6379);

    let addr = spawn_app::spawn_app(redis_port).await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
