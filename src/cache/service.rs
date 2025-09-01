use crate::cache::{CacheConfig, CacheEntry, CacheInvalidationEvent, CacheKey, CacheStats};
use crate::error::{AppError, Result};
use chrono::Utc;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;

#[derive(Clone)]
pub struct CacheService {
    inner: Arc<RwLock<CacheServiceInner>>,
    config: CacheConfig,
}

struct CacheServiceInner {
    entries: HashMap<String, CacheEntry<Vec<u8>>>,
    stats: CacheStats,
}

impl CacheService {
    pub fn new(config: CacheConfig) -> Self {
        let service = Self {
            inner: Arc::new(RwLock::new(CacheServiceInner {
                entries: HashMap::new(),
                stats: CacheStats {
                    total_entries: 0,
                    hit_count: 0,
                    miss_count: 0,
                    hit_rate: 0.0,
                    memory_usage: 0,
                    expired_entries: 0,
                },
            })),
            config,
        };

        // Start cleanup task
        let service_clone = service.clone();
        tokio::spawn(async move {
            service_clone.cleanup_task().await;
        });

        service
    }

    pub async fn get<T>(&self, key: &CacheKey) -> Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let key_str = key.to_string();
        let mut inner = self.inner.write().await;

        // Check if entry exists and handle expiration
        let entry_data = if let Some(entry) = inner.entries.get(&key_str) {
            if entry.expires_at < Utc::now() {
                None // Mark as expired
            } else {
                Some(entry.data.clone())
            }
        } else {
            None
        };

        match entry_data {
            Some(data) => {
                // Update access stats for existing entry
                if let Some(entry) = inner.entries.get_mut(&key_str) {
                    entry.access_count += 1;
                    entry.last_accessed = Utc::now();
                }
                inner.stats.hit_count += 1;
                self.update_hit_rate(&mut inner);

                // Deserialize data
                let value: T = serde_json::from_slice(&data)
                    .map_err(|e| AppError::BadRequest(format!("Cache deserialization error: {}", e)))?;

                Ok(Some(value))
            }
            None => {
                // Remove expired entry if it exists
                if inner.entries.contains_key(&key_str) {
                    inner.entries.remove(&key_str);
                    inner.stats.expired_entries += 1;
                }
                inner.stats.miss_count += 1;
                self.update_hit_rate(&mut inner);
                Ok(None)
            }
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
        let key_str = key.to_string();
        let data = serde_json::to_vec(value)
            .map_err(|e| AppError::BadRequest(format!("Cache serialization error: {}", e)))?;

        let mut inner = self.inner.write().await;

        // Check if we need to evict entries
        if inner.entries.len() >= self.config.max_entries {
            self.evict_lru(&mut inner).await;
        }

        let now = Utc::now();
        let expires_at = now + chrono::Duration::from_std(ttl).unwrap();

        let entry = CacheEntry {
            data,
            expires_at,
            created_at: now,
            access_count: 0,
            last_accessed: now,
        };

        inner.entries.insert(key_str, entry);
        inner.stats.total_entries = inner.entries.len();
        self.update_memory_usage(&mut inner);

        Ok(())
    }

    pub async fn delete(&self, key: &CacheKey) -> Result<bool> {
        let key_str = key.to_string();
        let mut inner = self.inner.write().await;
        
        let removed = inner.entries.remove(&key_str).is_some();
        inner.stats.total_entries = inner.entries.len();
        self.update_memory_usage(&mut inner);
        
        Ok(removed)
    }

    pub async fn invalidate_pattern(&self, pattern: &str) -> Result<usize> {
        let mut inner = self.inner.write().await;
        let mut removed_count = 0;

        if pattern == "*" {
            removed_count = inner.entries.len();
            inner.entries.clear();
        } else {
            let keys_to_remove: Vec<String> = inner
                .entries
                .keys()
                .filter(|key| self.matches_pattern(key, pattern))
                .cloned()
                .collect();

            for key in keys_to_remove {
                inner.entries.remove(&key);
                removed_count += 1;
            }
        }

        inner.stats.total_entries = inner.entries.len();
        self.update_memory_usage(&mut inner);

        Ok(removed_count)
    }

    pub async fn handle_invalidation_event(&self, event: &CacheInvalidationEvent) -> Result<()> {
        for pattern in event.event_type.get_affected_patterns() {
            self.invalidate_pattern(&pattern).await?;
        }
        Ok(())
    }

    pub async fn get_stats(&self) -> CacheStats {
        let inner = self.inner.read().await;
        inner.stats.clone()
    }

    pub async fn clear(&self) -> Result<()> {
        let mut inner = self.inner.write().await;
        inner.entries.clear();
        inner.stats.total_entries = 0;
        inner.stats.memory_usage = 0;
        Ok(())
    }

    pub async fn exists(&self, key: &CacheKey) -> bool {
        let key_str = key.to_string();
        let inner = self.inner.read().await;
        
        if let Some(entry) = inner.entries.get(&key_str) {
            entry.expires_at > Utc::now()
        } else {
            false
        }
    }

    pub async fn extend_ttl(&self, key: &CacheKey, additional_ttl: Duration) -> Result<bool> {
        let key_str = key.to_string();
        let mut inner = self.inner.write().await;

        if let Some(entry) = inner.entries.get_mut(&key_str) {
            if entry.expires_at > Utc::now() {
                entry.expires_at = entry.expires_at + chrono::Duration::from_std(additional_ttl).unwrap();
                return Ok(true);
            }
        }

        Ok(false)
    }

    // Private helper methods
    fn update_hit_rate(&self, inner: &mut CacheServiceInner) {
        let total_requests = inner.stats.hit_count + inner.stats.miss_count;
        if total_requests > 0 {
            inner.stats.hit_rate = inner.stats.hit_count as f64 / total_requests as f64 * 100.0;
        }
    }

    fn update_memory_usage(&self, inner: &mut CacheServiceInner) {
        inner.stats.memory_usage = inner
            .entries
            .values()
            .map(|entry| entry.data.len())
            .sum();
    }

    async fn evict_lru(&self, inner: &mut CacheServiceInner) {
        // Find the least recently used entry
        if let Some((lru_key, _)) = inner
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.last_accessed)
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            inner.entries.remove(&lru_key);
        }
    }

    fn matches_pattern(&self, key: &str, pattern: &str) -> bool {
        if pattern.ends_with('*') {
            let prefix = &pattern[..pattern.len() - 1];
            key.starts_with(prefix)
        } else {
            key == pattern
        }
    }

    async fn cleanup_task(&self) {
        let mut interval = interval(self.config.cleanup_interval);
        
        loop {
            interval.tick().await;
            self.cleanup_expired().await;
        }
    }

    async fn cleanup_expired(&self) {
        let mut inner = self.inner.write().await;
        let now = Utc::now();
        let mut expired_count = 0;

        inner.entries.retain(|_, entry| {
            if entry.expires_at < now {
                expired_count += 1;
                false
            } else {
                true
            }
        });

        inner.stats.expired_entries += expired_count;
        inner.stats.total_entries = inner.entries.len();
        self.update_memory_usage(&mut inner);
    }
}

// Cache warming utilities
impl CacheService {
    pub async fn warm_cache(&self) -> Result<()> {
        tracing::info!("Starting cache warming...");
        
        // This would typically pre-load frequently accessed data
        // For now, we'll just log that warming is available
        
        tracing::info!("Cache warming completed");
        Ok(())
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
}