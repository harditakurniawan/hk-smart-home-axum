use std::time::Instant;

use bb8::{Pool, PooledConnection};
use bb8_redis::{redis::{AsyncCommands, RedisError, ToRedisArgs}, RedisConnectionManager};
use serde_json::json;

use crate::utils::log_util::Log;

pub async fn get_record(redis: &Pool<RedisConnectionManager>, key: &str) -> Result<Option<String>, RedisError> {
    let mut conn = redis.get().await.unwrap();

    let value: Option<String> = conn.get(key).await?;

    Ok(value)
}

pub async fn set_record<K, V>(redis: &Pool<RedisConnectionManager>, key: K, value: V) -> Result<(), RedisError>
where
    K: ToRedisArgs + Send + Sync + serde::Serialize,
    V: ToRedisArgs + Send + Sync + serde::Serialize,
{
    let start_time: Instant = Instant::now();

    let mut conn: PooledConnection<RedisConnectionManager> = redis.get().await.unwrap();

    conn.set(&key, &value).await.map_err(|err| {
        let duration = start_time.elapsed();

        if let Err(log_err) = Log::error(
            "REDIS".to_string(),
            "set_record".to_string(),
            serde_json::to_string(&json!({
                "key": key,
                "value": value,
            })).expect("Failed to serialize log data"),
            err.to_string(),
            duration.as_millis(),
        ) {
            eprintln!("Failed to log error: {:?}", log_err);
        }

        err
    })
}

pub async fn del_record(redis: &Pool<RedisConnectionManager>, key: &str) -> Result<(), RedisError> {
    let start_time: Instant = Instant::now();

    let mut conn: PooledConnection<'_, RedisConnectionManager> = redis.get().await.unwrap();

    conn.del::<&str, ()>(key).await.map_err(|err| {
        let duration = start_time.elapsed();

        if let Err(log_err) = Log::error(
            "REDIS".to_string(),
            "set_record".to_string(),
            serde_json::to_string(&json!({ "key": key })).expect("Failed to serialize log data"),
            err.to_string(),
            duration.as_millis(),
        ) {
            eprintln!("Failed to log error: {:?}", log_err);
        }

        err
    })
}