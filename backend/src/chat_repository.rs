
use deadpool_diesel::postgres::Pool;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;

use crate::{models::Chat, schema::chat};

pub async fn save(pool: &Pool, chat: Chat) -> Result<(), anyhow::Error> {
    let conn = pool.get().await?;

    let x = conn.interact(move |conn| {
        diesel::insert_into(chat::table)
        .values(&(chat.clone()))
        .returning(Chat::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
    }).await?;

    Ok(())
}