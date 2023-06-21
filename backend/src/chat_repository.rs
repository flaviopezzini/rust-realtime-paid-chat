
use deadpool_diesel::Manager;
use deadpool_diesel::Pool;
use deadpool_diesel::PoolError;
use diesel::PgConnection;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;

use crate::{models::Chat, schema::chat};

pub async fn save(pool: Pool<Manager<PgConnection>>, chat: Chat) -> Result<Chat, PoolError> {
    let conn = &mut pool.get().await?;
    Ok(diesel::insert_into(chat::table)
        .values(&chat)
        .returning(Chat::as_returning())
        .get_result(conn)
        .expect("Error saving new post"))
}