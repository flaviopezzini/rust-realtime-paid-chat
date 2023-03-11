use redis::aio::Connection;
use redis::{AsyncCommands, RedisError};

#[derive(Clone)]
pub struct RedisWrapper {
    pub client: redis::Client
}

impl RedisWrapper {
    pub fn new(client: redis::Client) -> Self {
        Self {
            client
        }
    }

    async fn connect(&self) -> Result<Connection, RedisError> {
        self.client.get_async_connection()
        .await
    }

    pub async fn exists(&self, key: String) -> Result<bool, RedisError> {
        self.connect().await?.exists(&key).await?;
        Ok(true)
    }

    pub async fn del(&self, key: String) -> Result<(), RedisError> {
        self.connect().await?.del::<String, String>(key);
        Ok(())
    }

    pub async fn set(&self, key: String, value: String) -> Result<(), RedisError> {
        self.connect().await?.set(key, value).await?;
        Ok(())
    }

    pub async fn add_to_set(&self, set: String, key: String, value: i32) -> Result<(), RedisError> {
        redis::cmd("ZADD")
            .arg(set)
            .arg(value)
            .arg(key)
            .query_async(&mut self.connect().await?).await?;
        Ok(())
    }

    pub async fn remove_from_set(&self, set: String, key: String) -> Result<(), RedisError> {
        redis::cmd("ZREM")
            .arg(set)
            .arg(key)
            .query_async::<Connection, String>(&mut self.connect().await?)
            .await?;
        Ok(())
    }
}