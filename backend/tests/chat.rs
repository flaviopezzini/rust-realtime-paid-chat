mod spawn_app;

use tokio_tungstenite::tungstenite;

use futures::{SinkExt, StreamExt};

#[tokio::test]
async fn chat_works() {
    let addr = spawn_app::spawn_app().await;

    let (mut socket_sender, _response) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/websocket"))
            .await
            .unwrap();

    let (mut socket_receiver, _response) =
        tokio_tungstenite::connect_async(format!("ws://{addr}/websocket"))
            .await
            .unwrap();

    socket_sender
        .send(tungstenite::Message::text("sender_username"))
        .await
        .unwrap();

    let msg_sender_joined = match socket_sender.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_sender_joined, "sender_username joined.");

    socket_receiver
        .send(tungstenite::Message::text("receiver_username"))
        .await
        .unwrap();

    let msg_receiver_joined = match socket_receiver.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_receiver_joined, "receiver_username joined.");

    socket_sender
        .send(tungstenite::Message::text("Hello from Sender"))
        .await
        .unwrap();

    let msg_hello_from_sender = match socket_receiver.next().await.unwrap().unwrap() {
        tungstenite::Message::Text(msg) => msg,
        other => panic!("expected a text message but got {other:?}"),
    };
    assert_eq!(msg_hello_from_sender, "sender_username: Hello from Sender");
}
