mod spawn_app;

use testcontainers::clients;

use crate::spawn_app::TestApp;

#[tokio::test]
async fn health_check_works() {

    let docker = clients::Cli::default();
    let test_app = TestApp::spawn_app(&docker).await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", test_app.addr))
        .send()
        .await
        .expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
