use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{IntoResponse},
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

// Our shared state
pub struct AppState {
    pub user_set: Mutex<HashSet<String>>,
    pub tx: broadcast::Sender<String>,
}

impl AppState {
    pub fn new() -> AppState {
        let user_set = Mutex::new(HashSet::new());
        let (tx, _rx) = broadcast::channel(100);

        AppState { user_set, tx }
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            check_username(&state, &mut username, &name);

            // If not empty we want to quit the loop else we want to quit function.
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    // Subscribe before sending joined message.
    let mut rx = state.tx.subscribe();

    // Send joined message to all subscribers.
    let msg = format!("{} joined.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

    // This task will receive broadcast messages and send text message to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    // This task will receive messages from client and send them to broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Add username before message.
            let _ = tx.send(format!("{}: {}", name, text));
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Send user left message.
    let msg = format!("{} left.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);
    // Remove username from map so new clients can take it.
    state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}