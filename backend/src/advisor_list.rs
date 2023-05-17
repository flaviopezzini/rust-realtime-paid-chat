use axum::{extract::State, Json};

use crate::chat::AppState;


pub async fn advisor_list(State(state): State<AppState>,) -> Json<Vec<String>> {
    let result = state.redis.fetch_set("advisor_list".to_owned()).await;

    match result {
        Ok(vec) => Json(vec),
        Err(e) => Json(vec![format!("There was an error {}", e)]),
    }
}