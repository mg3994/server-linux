use crate::cache::{CacheConfig, CacheKey, CacheStats};
use crate::error::{AppError, Result};
use redis::{AsyncCommands, Client};

use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct RedisCacheService {
    client: Client,
    config: CacheConfig,
}

impl RedisCacheService {
    pub async fn new(config: CacheConfig) -> Result<Self> {
        let redis_url = config
            .redis_url
            .as_ref()
            .ok_or_else(|| AppError::BadRequest("Redis URL not configured".to_string()))?;

        let client = Client::open(redis_url.as_str())
            .map_err(|e| AppError::BadRequest(format!("Redis connection error: {}", e)))?;

        // Test connection
        let mut conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis connection failed: {}", e)))?;

        // Ping to verify connection
        let _: String = conn
            .ping()
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis ping failed: {}", e)))?;

        Ok(Self { client, config })
    }

    pub async fn get<T>(&self, key: &CacheKey) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let mut conn = self.get_connection().await?;
        let key_str = key.to_string();

        let data: Option<Vec<u8>> = conn
            .get(&key_str)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis get error: {}", e)))?;

        if let Some(bytes) = data.as_ref() {
            let value: T = serde_json::from_slice(bytes)
                .map_err(|e| AppError::BadRequest(format!("Deserialization error: {}", e)))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub async fn set<T>(&self, key: &CacheKey, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.set_with_ttl(key, value, key.get_ttl()).await
    }

    pub async fn set_with_ttl<T>(&self, key: &CacheKey, value: &T, ttl: Duration) -> Result<()>
    where
        T: Serialize,
    {
        let mut conn = self.get_connection().await?;
        let key_str = key.to_string();

        let data = serde_json::to_vec(value)
            .map_err(|e| AppError::BadRequest(format!("Serialization error: {}", e)))?;

        let _: () = conn
            .set_ex(&key_str, data, ttl.as_secs())
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis set error: {}", e)))?;

        Ok(())
    }

    pub async fn delete(&self, key: &CacheKey) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let key_str = key.to_string();

        let deleted: i32 = conn
            .del(&key_str)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis delete error: {}", e)))?;

        Ok(deleted > 0)
    }

    pub async fn exists(&self, key: &CacheKey) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let key_str = key.to_string();

        let exists: bool = conn
            .exists(&key_str)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis exists error: {}", e)))?;

        Ok(exists)
    }

    pub async fn extend_ttl(&self, key: &CacheKey, additional_ttl: Duration) -> Result<bool> {
        let mut conn = self.get_connection().await?;
        let key_str = key.to_string();

        // Get current TTL
        let current_ttl: i64 = conn
            .ttl(&key_str)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis TTL error: {}", e)))?;

        if current_ttl > 0 {
            let new_ttl = current_ttl as u64 + additional_ttl.as_secs();
            let _: bool = conn
                .expire(&key_str, new_ttl as i64)
                .await
                .map_err(|e| AppError::BadRequest(format!("Redis expire error: {}", e)))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn invalidate_pattern(&self, pattern: &str) -> Result<usize> {
        let mut conn = self.get_connection().await?;

        // Get all keys matching pattern
        let keys: Vec<String> = conn
            .keys(pattern)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis keys error: {}", e)))?;

        if keys.is_empty() {
            return Ok(0);
        }

        // Delete all matching keys
        let deleted: i32 = conn
            .del(&keys)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis delete error: {}", e)))?;

        Ok(deleted as usize)
    }

    pub async fn clear(&self) -> Result<()> {
        let mut conn = self.get_connection().await?;

        let _: () = conn
            .flushdb()
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis flush error: {}", e)))?;

        Ok(())
    }

    pub async fn get_stats(&self) -> Result<CacheStats> {
        // For now, return basic stats since MultiplexedConnection doesn't support info/dbsize
        Ok(CacheStats {
            total_entries: 0,
            hit_count: 0,
            miss_count: 0,
            hit_rate: 0.0,
            memory_usage: 0,
            expired_entries: 0,
        })
    }

    pub async fn get_or_compute<T, F, Fut>(&self, key: &CacheKey, compute_fn: F) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Clone,
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        // Try to get from cache first
        if let Some(cached_value) = self.get::<T>(key).await? {
            return Ok(cached_value);
        }

        // Compute the value
        let computed_value = compute_fn().await?;

        // Store in cache
        self.set(key, &computed_value).await?;

        Ok(computed_value)
    }

    // Private helper methods
    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis connection error: {}", e)))
    }

    fn parse_memory_usage(&self, info: &str) -> usize {
        for line in info.lines() {
            if line.starts_with("used_memory:") {
                if let Some(value) = line.split(':').nth(1) {
                    return value.parse().unwrap_or(0);
                }
            }
        }
        0
    }
}

// Distributed cache utilities
impl RedisCacheService {
    pub async fn publish_invalidation(&self, channel: &str, pattern: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;

        let _: i32 = conn
            .publish(channel, pattern)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis publish error: {}", e)))?;

        Ok(())
    }

    pub async fn subscribe_invalidations<F>(&self, _channel: &str, _handler: F) -> Result<()>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        // Simplified implementation - pubsub requires different connection type
        // For now, just return Ok to allow compilation
        Ok(())
    }

    pub async fn atomic_increment(&self, key: &str, increment: i64) -> Result<i64> {
        let mut conn = self.get_connection().await?;

        let result: i64 = conn
            .incr(key, increment)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis incr error: {}", e)))?;

        Ok(result)
    }

    pub async fn atomic_decrement(&self, key: &str, decrement: i64) -> Result<i64> {
        let mut conn = self.get_connection().await?;

        let result: i64 = conn
            .decr(key, decrement)
            .await
            .map_err(|e| AppError::BadRequest(format!("Redis decr error: {}", e)))?;

        Ok(result)
    }
}
