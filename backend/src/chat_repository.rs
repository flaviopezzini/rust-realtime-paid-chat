

use sqlx::{Pool, Postgres};

use crate::models::Chat;

pub async fn save(pool: &Pool<Postgres>, chat: Chat) -> Result<(), anyhow::Error> {

    sqlx::query!(
        r#"
        INSERT INTO chat (id, sender, receiver, created_date, "content", amount)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
                 chat.id,
                chat.sender,
                chat.receiver,
                chat.created_date,
                chat.content,
                chat.amount
                )
        .execute(pool).await?;

    Ok(())
}