mod spawn_app;

use tokio_tungstenite::tungstenite;

use futures::{SinkExt, StreamExt};

use uuid::Uuid;

#[tokio::test]
async fn chat_works() {
    let addr = spawn_app::spawn_app().await;

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
    assert_eq!(msg_advisor_joined, "advisor joined.");

    socket_customer
        .send(tungstenite::Message::text(r#"{"username":"customer","user_type":"customer"}"#))
        .await
        .unwrap();

    let msg_customer_joined = match socket_customer.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_customer_joined, "customer joined.");

    socket_advisor
        .send(tungstenite::Message::text("Hello from Advisor"))
        .await
        .unwrap();

    let msg_hello_from_advisor = match socket_customer.next().await.expect("Error here").unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_hello_from_advisor, "advisor: Hello from Advisor");
}
