mod spawn_app;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app::spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
