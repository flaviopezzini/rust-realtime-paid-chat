mod spawn_app;

use testcontainers::clients;
use tokio_tungstenite::tungstenite;

use futures::SinkExt;

use uuid::Uuid;

use crate::spawn_app::TestApp;

#[tokio::test]
async fn advisor_list_works() {

    let docker = clients::Cli::default();
    let test_app = TestApp::spawn_app(&docker).await;

    let client = reqwest::Client::new();

    let advisor1 = Uuid::new_v4();

    let addr = test_app.addr;

    let (mut socket_advisor1, _response) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/websocket/{advisor1}/customer1"))
            .await
            .expect("Failed to connect to the websocket from the advisor side");

    socket_advisor1
            .send(tungstenite::Message::text(format!(r#"{{"username":"{advisor1}","user_type":"advisor"}}"#)))
            .await
            .unwrap();
        
    let advisor2 = Uuid::new_v4();
    let (mut socket_advisor2, _response) =
            tokio_tungstenite::connect_async(format!("ws://{addr}/websocket/{advisor2}/customer2"))
                .await
                .expect("Failed to connect to the websocket from the advisor side");

    socket_advisor2
                .send(tungstenite::Message::text(format!(r#"{{"username":"{advisor2}","user_type":"advisor"}}"#)))
                .await
                .unwrap();

    let response = client
        .get(format!("http://{}/advisor-list", addr))
        .send()
        .await
        .expect("Failed to execute health check request");

    println!("=> {:?}", response.text().await.unwrap());

    // assert!(response.status().is_success());
    // assert_ne!(Some(0), response.content_length());

    socket_advisor1.close(None).await.unwrap();
    socket_advisor2.close(None).await.unwrap();    
}
