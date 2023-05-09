use axum::{extract::State, response::IntoResponse};

use crate::chat::AppState;


pub async fn advisor_list(State(state): State<AppState>,) -> impl IntoResponse {
    state.redis.fetch_set("advisor_list".to_owned()).await.unwrap()
}