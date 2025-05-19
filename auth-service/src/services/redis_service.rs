use bb8::Pool;
use bb8_redis::{redis::RedisError, RedisConnectionManager};

use crate::{core::enums::redis_enum::RedisKey, repositories::redis_repository::get_record};

pub async fn get_config(redis: Pool<RedisConnectionManager>, parameter: &str) -> Result<String, &'static str> {
    let key = format!("{}-{}", RedisKey::HK_SMART_HOME_SYSTEMCONFIG, parameter);
    println!("Fetching Redis key: {}", key);

    match get_record(&redis, &key).await {
        Ok(Some(record)) => {
            if record.is_empty() {
                // log::warn!("Empty value found for key: {}", key);
                // Decide what to return for empty string; here we treat it as an error
                Err("Empty configuration value")
            } else {
                // log::debug!("Record found: {}", record);
                Ok(record)
            }
        }
        Ok(None) => {
            // log::warn!("No record found for key: {}", key);
            Err("Configuration key not found")
        }
        Err(_) => {
            // log::error!("Error fetching record for key: {}", key);
            Err("Failed to fetch configuration from Redis")
        }
    }
}