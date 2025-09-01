use crate::database::Database;
use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: ServiceStatus,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub uptime: Duration,
    pub components: HashMap<String, ComponentHealth>,
    pub metrics: HealthMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub status: ServiceStatus,
    pub response_time: Option<Duration>,
    pub last_check: DateTime<Utc>,
    pub error_message: Option<String>,
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub active_connections: u32,
    pub requests_per_minute: f64,
    pub error_rate: f64,
    pub average_response_time: Duration,
}

#[derive(Clone)]
pub struct HealthChecker {
    database: Database,
    start_time: Instant,
}

impl HealthChecker {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            start_time: Instant::now(),
        }
    }

    pub async fn check_health(&self) -> Result<HealthStatus> {
        let mut components = HashMap::new();

        // Check database health
        components.insert("database".to_string(), self.check_database_health().await);

        // Check cache health (if available)
        components.insert("cache".to_string(), self.check_cache_health().await);

        // Check external services
        components.insert("firebase".to_string(), self.check_firebase_health().await);
        components.insert("fcm".to_string(), self.check_fcm_health().await);

        // Check file system
        components.insert("filesystem".to_string(), self.check_filesystem_health().await);

        // Determine overall status
        let overall_status = self.determine_overall_status(&components);

        // Collect system metrics
        let metrics = self.collect_system_metrics().await;

        Ok(HealthStatus {
            status: overall_status,
            timestamp: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: self.start_time.elapsed(),
            components,
            metrics,
        })
    }

    pub async fn check_readiness(&self) -> Result<bool> {
        // Check if all critical components are ready
        let database_ready = self.is_database_ready().await;
        let cache_ready = self.is_cache_ready().await;

        Ok(database_ready && cache_ready)
    }

    pub async fn check_liveness(&self) -> Result<bool> {
        // Basic liveness check - can we respond to requests?
        Ok(true)
    }

    async fn check_database_health(&self) -> ComponentHealth {
        let start_time = Instant::now();
        let mut details = HashMap::new();

        match self.database.pool().acquire().await {
            Ok(mut conn) => {
                match sqlx::query("SELECT 1").fetch_one(&mut *conn).await {
                    Ok(_) => {
                        let response_time = start_time.elapsed();
                        details.insert("connection_pool_size".to_string(), "active".to_string());
                        details.insert("query_test".to_string(), "passed".to_string());

                        ComponentHealth {
                            status: ServiceStatus::Healthy,
                            response_time: Some(response_time),
                            last_check: Utc::now(),
                            error_message: None,
                            details,
                        }
                    }
                    Err(e) => ComponentHealth {
                        status: ServiceStatus::Unhealthy,
                        response_time: Some(start_time.elapsed()),
                        last_check: Utc::now(),
                        error_message: Some(format!("Database query failed: {}", e)),
                        details,
                    },
                }
            }
            Err(e) => ComponentHealth {
                status: ServiceStatus::Unhealthy,
                response_time: Some(start_time.elapsed()),
                last_check: Utc::now(),
                error_message: Some(format!("Database connection failed: {}", e)),
                details,
            },
        }
    }

    async fn check_cache_health(&self) -> ComponentHealth {
        let mut details = HashMap::new();
        details.insert("type".to_string(), "in-memory".to_string());

        // For now, assume cache is always healthy since we're using in-memory cache
        ComponentHealth {
            status: ServiceStatus::Healthy,
            response_time: Some(Duration::from_millis(1)),
            last_check: Utc::now(),
            error_message: None,
            details,
        }
    }

    async fn check_firebase_health(&self) -> ComponentHealth {
        let mut details = HashMap::new();
        details.insert("service".to_string(), "firebase_auth".to_string());

        // For now, assume Firebase is healthy (would need actual health check)
        ComponentHealth {
            status: ServiceStatus::Healthy,
            response_time: Some(Duration::from_millis(50)),
            last_check: Utc::now(),
            error_message: None,
            details,
        }
    }

    async fn check_fcm_health(&self) -> ComponentHealth {
        let mut details = HashMap::new();
        details.insert("service".to_string(), "firebase_messaging".to_string());

        // For now, assume FCM is healthy (would need actual health check)
        ComponentHealth {
            status: ServiceStatus::Healthy,
            response_time: Some(Duration::from_millis(100)),
            last_check: Utc::now(),
            error_message: None,
            details,
        }
    }

    async fn check_filesystem_health(&self) -> ComponentHealth {
        let mut details = HashMap::new();

        // Check if we can write to temp directory
        match std::fs::write("/tmp/health_check", "test") {
            Ok(_) => {
                let _ = std::fs::remove_file("/tmp/health_check");
                details.insert("write_test".to_string(), "passed".to_string());

                ComponentHealth {
                    status: ServiceStatus::Healthy,
                    response_time: Some(Duration::from_millis(5)),
                    last_check: Utc::now(),
                    error_message: None,
                    details,
                }
            }
            Err(e) => ComponentHealth {
                status: ServiceStatus::Degraded,
                response_time: Some(Duration::from_millis(5)),
                last_check: Utc::now(),
                error_message: Some(format!("Filesystem write failed: {}", e)),
                details,
            },
        }
    }

    fn determine_overall_status(&self, components: &HashMap<String, ComponentHealth>) -> ServiceStatus {
        let mut _healthy_count = 0;
        let mut degraded_count = 0;
        let mut unhealthy_count = 0;

        for component in components.values() {
            match component.status {
                ServiceStatus::Healthy => _healthy_count += 1,
                ServiceStatus::Degraded => degraded_count += 1,
                ServiceStatus::Unhealthy => unhealthy_count += 1,
                ServiceStatus::Maintenance => degraded_count += 1,
            }
        }

        // If any critical component is unhealthy, overall status is unhealthy
        if unhealthy_count > 0 {
            ServiceStatus::Unhealthy
        } else if degraded_count > 0 {
            ServiceStatus::Degraded
        } else {
            ServiceStatus::Healthy
        }
    }

    async fn collect_system_metrics(&self) -> HealthMetrics {
        // In a real implementation, these would come from system monitoring
        // For now, we'll return mock data
        HealthMetrics {
            cpu_usage: 25.5,
            memory_usage: 512.0 * 1024.0 * 1024.0, // 512MB
            disk_usage: 75.2,
            active_connections: 15,
            requests_per_minute: 120.0,
            error_rate: 0.5,
            average_response_time: Duration::from_millis(150),
        }
    }

    async fn is_database_ready(&self) -> bool {
        match self.database.pool().acquire().await {
            Ok(mut conn) => {
                sqlx::query("SELECT 1").fetch_one(&mut *conn).await.is_ok()
            }
            Err(_) => false,
        }
    }

    async fn is_cache_ready(&self) -> bool {
        // For in-memory cache, always ready
        true
    }
}

// Health check handlers for HTTP endpoints
use axum::{extract::State, response::Json};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn health_detailed(
    State(health_checker): State<HealthChecker>,
) -> Result<Json<HealthStatus>> {
    let health_status = health_checker.check_health().await?;
    Ok(Json(health_status))
}

pub async fn readiness_check(
    State(health_checker): State<HealthChecker>,
) -> Result<Json<serde_json::Value>> {
    let ready = health_checker.check_readiness().await?;
    Ok(Json(serde_json::json!({
        "ready": ready,
        "timestamp": Utc::now()
    })))
}

pub async fn liveness_check(
    State(health_checker): State<HealthChecker>,
) -> Result<Json<serde_json::Value>> {
    let alive = health_checker.check_liveness().await?;
    Ok(Json(serde_json::json!({
        "alive": alive,
        "timestamp": Utc::now()
    })))
}
