use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub access_count: u64,
    pub last_accessed: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CacheConfig {
    pub default_ttl: Duration,
    pub max_entries: usize,
    pub cleanup_interval: Duration,
    pub redis_url: Option<String>,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            default_ttl: Duration::from_secs(300), // 5 minutes
            max_entries: 10000,
            cleanup_interval: Duration::from_secs(60), // 1 minute
            redis_url: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_entries: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub hit_rate: f64,
    pub memory_usage: usize,
    pub expired_entries: usize,
}

#[derive(Debug, Clone)]
pub enum CacheKey {
    Restaurant(Uuid),
    RestaurantMenu(Uuid),
    RestaurantsByCity(String),
    RestaurantsByCuisine(String),
    DeliveryPerson(Uuid),
    NearbyDeliveryPersons { lat: f64, lng: f64, radius: f64 },
    Order(Uuid),
    CustomerOrders(Uuid),
    Analytics { period: String, filters: String },
    RealTimeMetrics,
    IndiaConfig,
    DeliveryZones,
    Custom(String),
}

impl CacheKey {
    pub fn to_string(&self) -> String {
        match self {
            CacheKey::Restaurant(id) => format!("restaurant:{}", id),
            CacheKey::RestaurantMenu(id) => format!("restaurant:{}:menu", id),
            CacheKey::RestaurantsByCity(city) => format!("restaurants:city:{}", city),
            CacheKey::RestaurantsByCuisine(cuisine) => format!("restaurants:cuisine:{}", cuisine),
            CacheKey::DeliveryPerson(id) => format!("delivery_person:{}", id),
            CacheKey::NearbyDeliveryPersons { lat, lng, radius } => {
                format!("nearby_delivery:{}:{}:{}", lat, lng, radius)
            }
            CacheKey::Order(id) => format!("order:{}", id),
            CacheKey::CustomerOrders(id) => format!("customer:{}:orders", id),
            CacheKey::Analytics { period, filters } => {
                format!("analytics:{}:{}", period, filters)
            }
            CacheKey::RealTimeMetrics => "real_time_metrics".to_string(),
            CacheKey::IndiaConfig => "india_config".to_string(),
            CacheKey::DeliveryZones => "delivery_zones".to_string(),
            CacheKey::Custom(key) => key.clone(),
        }
    }

    pub fn get_ttl(&self) -> Duration {
        match self {
            CacheKey::Restaurant(_) => Duration::from_secs(600), // 10 minutes
            CacheKey::RestaurantMenu(_) => Duration::from_secs(300), // 5 minutes
            CacheKey::RestaurantsByCity(_) => Duration::from_secs(180), // 3 minutes
            CacheKey::RestaurantsByCuisine(_) => Duration::from_secs(180), // 3 minutes
            CacheKey::DeliveryPerson(_) => Duration::from_secs(60), // 1 minute
            CacheKey::NearbyDeliveryPersons { .. } => Duration::from_secs(30), // 30 seconds
            CacheKey::Order(_) => Duration::from_secs(120), // 2 minutes
            CacheKey::CustomerOrders(_) => Duration::from_secs(60), // 1 minute
            CacheKey::Analytics { .. } => Duration::from_secs(900), // 15 minutes
            CacheKey::RealTimeMetrics => Duration::from_secs(10), // 10 seconds
            CacheKey::IndiaConfig => Duration::from_secs(3600), // 1 hour
            CacheKey::DeliveryZones => Duration::from_secs(1800), // 30 minutes
            CacheKey::Custom(_) => Duration::from_secs(300), // 5 minutes default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInvalidationEvent {
    pub event_type: CacheInvalidationType,
    pub affected_keys: Vec<String>,
    pub timestamp: DateTime<Utc>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheInvalidationType {
    RestaurantUpdated(Uuid),
    MenuUpdated(Uuid),
    DeliveryPersonUpdated(Uuid),
    OrderStatusChanged(Uuid),
    LocationUpdated(Uuid),
    ConfigChanged,
    BulkInvalidation,
}

impl CacheInvalidationType {
    pub fn get_affected_patterns(&self) -> Vec<String> {
        match self {
            CacheInvalidationType::RestaurantUpdated(id) => vec![
                format!("restaurant:{}", id),
                format!("restaurant:{}:menu", id),
                "restaurants:city:*".to_string(),
                "restaurants:cuisine:*".to_string(),
            ],
            CacheInvalidationType::MenuUpdated(id) => vec![
                format!("restaurant:{}:menu", id),
            ],
            CacheInvalidationType::DeliveryPersonUpdated(id) => vec![
                format!("delivery_person:{}", id),
                "nearby_delivery:*".to_string(),
            ],
            CacheInvalidationType::OrderStatusChanged(id) => vec![
                format!("order:{}", id),
                "customer:*:orders".to_string(),
                "real_time_metrics".to_string(),
            ],
            CacheInvalidationType::LocationUpdated(_) => vec![
                "nearby_delivery:*".to_string(),
                "real_time_metrics".to_string(),
            ],
            CacheInvalidationType::ConfigChanged => vec![
                "india_config".to_string(),
                "delivery_zones".to_string(),
            ],
            CacheInvalidationType::BulkInvalidation => vec![
                "*".to_string(),
            ],
        }
    }
}