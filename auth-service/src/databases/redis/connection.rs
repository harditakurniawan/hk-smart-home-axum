use bb8::Pool;
use bb8_redis::{redis::RedisError, RedisConnectionManager};

use crate::repositories::redis_repository;



pub async fn setup_redis(redis_url: &str) -> Pool<RedisConnectionManager> {
    let manager: RedisConnectionManager = RedisConnectionManager::new(redis_url).unwrap();
    let pool: Pool<RedisConnectionManager> = Pool::builder().max_size(10).build(manager).await.unwrap();

    ping_redis(&pool).await.unwrap();
    
    println!("successfully connected to redis and pinged it");

    return pool;
}

async fn ping_redis(pool: &Pool<RedisConnectionManager>) -> Result<(), RedisError> {
    redis_repository::set_record(&pool, "foo", "bar").await.unwrap();

    // let resut: String = redis_repository::get_record(&pool, "foo").await.unwrap().unwrap();
    let resut: Result<String, &'static str> = match redis_repository::get_record(&pool, "foo").await {
        Ok(Some(record)) => {
            if record.is_empty() {
                Err("Empty configuration value")
            } else {
                Ok(record)
            }
        }
        Ok(None) => {
            Err("Configuration key not found")
        }
        Err(_) => {
            Err("Failed to fetch configuration from Redis")
        }
    };
    println!("result : {:?}", resut.unwrap());

    redis_repository::del_record(&pool, "foo").await.unwrap();

    Ok(())
}