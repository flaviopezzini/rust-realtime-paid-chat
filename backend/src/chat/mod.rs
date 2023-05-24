use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, Path,
    },
    response::IntoResponse,
};
use futures::{sink::SinkExt, stream::StreamExt};

use std::fmt::Formatter;
use redis::{RedisError};
use tokio::sync::broadcast;

use crate::redis_wrapper::RedisWrapper;

use std::ops::{Deref};

use uuid::Uuid;

// Our shared state
#[derive(Clone)]
pub struct AppState {
    pub redis: RedisWrapper,
    pub tx: broadcast::Sender<String>,
    pub advisors: Vec<Advisor>
}

#[derive(Clone)]
pub struct Advisor {
    name: String,
    talking_to : Option<Uuid>
}

impl AppState {
    pub fn new(redis: RedisWrapper) -> AppState {
        let (tx, _rx) = broadcast::channel(100);

        AppState { redis, tx , advisors: Vec::new() }
    }
}

#[derive(serde::Deserialize)]
struct Payload {
  username: String,
  user_type: String
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Deref for AppState {
    type Target = AppState;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Path((advisor, customer)): Path<(String, String)>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state, advisor, customer))
}

async fn websocket(stream: WebSocket, state: AppState, advisor: String, customer: String) {

    // By splitting we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(payload) = message {

            let payload: Payload = match serde_json::from_str::<Payload>(&payload) {
                Ok(inner) => inner,
                Err(_) => {
                    let _ = sender
                    .send(Message::Text(String::from("Invalid format")))
                    .await;
                    return;
                } 
            };

            // If username that is sent by client is not taken, fill username string.
            check_username(
                &state.redis, 
                &mut username,
                &payload
            )
                .await.unwrap();

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

    // handle payload

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
    state.redis.del(username.clone()).await.unwrap();
    state.redis.remove_from_set("advisor_list".to_owned(), username).await.unwrap();
}

async fn check_username(
    redis_wrapper: &RedisWrapper, 
    string: &mut String, 
    payload: &Payload
) -> Result<(), RedisError> {
    let name = &payload.username;
    tracing::info!("username {} name {} exists {}", &string, &name, redis_wrapper.exists(name.to_owned()).await?);

    if !(redis_wrapper.exists(name.to_owned()).await?) {

        redis_wrapper.set(name.to_owned(), "true".to_owned()).await?;

        if "advisor" == payload.user_type {
            redis_wrapper.add_to_set(
                "advisor_list".to_owned(),
                name.to_owned()
            ).await?;
        }
        string.push_str(name);
    }

    Ok(())
}
