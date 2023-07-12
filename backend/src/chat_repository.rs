

use sqlx::{Pool, Postgres};

use crate::models::Chat;

pub async fn save(pool: &Pool<Postgres>, chat: Chat) -> Result<(), anyhow::Error> {

    sqlx::query!(
        r#"
        INSERT INTO chat (id, sender, receiver, created_date, "content")
        VALUES ($1, $2, $3, $4, $5)
        "#,
                 chat.id,
                chat.sender,
                chat.receiver,
                chat.created_date,
                chat.content
                )
        .execute(pool).await?;

    Ok(())
}