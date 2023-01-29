use redis::aio::Connection;
use redis::{AsyncCommands, RedisError};

pub struct RedisWrapper {
    pub redis_conn: Connection,
}

impl RedisWrapper {
    pub fn new(redis_conn: Connection) -> Self {
        Self {
            redis_conn
        }
    }

    pub async fn exists(&mut self, key: String) -> Result<bool, RedisError> {
        self.redis_conn.exists(&key).await?;
        Ok(true)
    }

    pub async fn del(&mut self, key: String) -> Result<(), RedisError> {
        self.redis_conn.del::<String, String>(key);
        Ok(())
    }

    pub async fn set(&mut self, key: String, value: String) -> Result<(), RedisError> {
        self.redis_conn.set(key, value).await?;
        Ok(())
    }

    pub async fn add_to_set(&mut self, set: String, key: String, value: i32) -> Result<(), RedisError> {
        redis::cmd("ZADD")
            .arg(set)
            .arg(value)
            .arg(key)
            .query_async(&mut self.redis_conn).await?;
        Ok(())
    }

    pub async fn remove_from_set(&mut self, set: String, key: String) -> Result<(), RedisError> {
        redis::cmd("ZREM")
            .arg(set)
            .arg(key)
            .query_async::<Connection, String>(&mut self.redis_conn)
            .await?;
        Ok(())
    }
}