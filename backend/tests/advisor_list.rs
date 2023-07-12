mod spawn_app;

use testcontainers::clients;
use tokio_tungstenite::tungstenite;

use futures::SinkExt;

use uuid::Uuid;

#[tokio::test]
async fn advisor_list_works() {

    let docker = clients::Cli::default();
    let container = docker.run(testcontainers::images::redis::Redis);
    let redis_port = container.get_host_port_ipv4(6379);

    let pg_container = docker.run(testcontainers::images::postgres::Postgres::default());
    let pg_port = pg_container.get_host_port_ipv4(5432);
    let database_url = format!("postgresql://localhost:{pg_port}/vop_rust");

    let addr = spawn_app::spawn_app(redis_port, database_url).await;

    let client = reqwest::Client::new();

    let advisor1 = Uuid::new_v4();

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
