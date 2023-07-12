mod spawn_app;

use tokio_tungstenite::tungstenite;

use futures::{SinkExt, StreamExt};

use uuid::Uuid;

use testcontainers::clients;

#[tokio::test]
async fn chat_works() {
    let docker = clients::Cli::default();
    let redis_container = docker.run(testcontainers::images::redis::Redis);
    let redis_port = redis_container.get_host_port_ipv4(6379);

    let pg_container = docker.run(testcontainers::images::postgres::Postgres::default());
    let pg_port = pg_container.get_host_port_ipv4(5432);
    let database_url = format!("postgresql://localhost:{pg_port}/vop_rust");

    let addr = spawn_app::spawn_app(redis_port, database_url).await;

    let (mut socket_advisor, _response) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/websocket/advisor/customer"))
            .await
            .expect("Failed to connect to the websocket from the advisor side");

    let advisor = Uuid::new_v4();

    let (mut socket_customer, _response) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/websocket/{advisor}/customer"))
            .await
            .unwrap();

    socket_advisor
        .send(tungstenite::Message::text(format!(r#"{{"username":"{advisor}","user_type":"advisor"}}"#)))
        .await
        .unwrap();

    let msg_advisor_joined = match socket_advisor.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_advisor_joined, format!("{advisor} joined."));

    let customer = Uuid::new_v4();

    socket_customer
        .send(tungstenite::Message::text(format!(r#"{{"username":"{customer}","user_type":"customer"}}"#)))
        .await
        .unwrap();

    let msg_customer_joined = match socket_customer.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_customer_joined, format!("{customer} joined."));

    socket_advisor
        .send(tungstenite::Message::text("Hello from Advisor"))
        .await
        .unwrap();

    let msg_hello_from_advisor = match socket_customer.next().await.expect("Error here").unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_hello_from_advisor, format!("{advisor}: Hello from Advisor"));

    socket_advisor.close(None).await.unwrap();
    socket_customer.close(None).await.unwrap();
}
