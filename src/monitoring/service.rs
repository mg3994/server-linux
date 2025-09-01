use crate::monitoring::{AlertManager, HealthChecker, MetricsCollector};
use crate::database::Database;

use std::time::Duration;
use tokio::time::interval;

#[derive(Clone)]
pub struct MonitoringService {
    pub health_checker: HealthChecker,
    pub metrics_collector: MetricsCollector,
    pub alert_manager: AlertManager,
}

impl MonitoringService {
    pub fn new(database: Database) -> Self {
        let health_checker = HealthChecker::new(database);
        let metrics_collector = MetricsCollector::new();
        let alert_manager = AlertManager::new();

        Self {
            health_checker,
            metrics_collector,
            alert_manager,
        }
    }

    pub async fn start_background_tasks(&self) {
        // Start metrics collection task
        let metrics_collector = self.metrics_collector.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Collect every minute
            loop {
                interval.tick().await;
                metrics_collector.cleanup_old_metrics(24).await; // Keep 24 hours of metrics
            }
        });

        // Start health monitoring task
        let health_checker = self.health_checker.clone();
        let alert_manager = self.alert_manager.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // Check every 30 seconds
            loop {
                interval.tick().await;
                
                match health_checker.check_health().await {
                    Ok(health_status) => {
                        // Check if any components are unhealthy and create alerts
                        for (component_name, component_health) in &health_status.components {
                            match component_health.status {
                                crate::monitoring::health::ServiceStatus::Unhealthy => {
                                    let alert = crate::monitoring::alerts::Alert {
                                        id: uuid::Uuid::new_v4(),
                                        alert_type: crate::monitoring::alerts::AlertType::SystemHealth,
                                        severity: crate::monitoring::alerts::AlertSeverity::Critical,
                                        title: format!("{} Component Unhealthy", component_name),
                                        description: component_health.error_message
                                            .clone()
                                            .unwrap_or_else(|| format!("{} is not responding", component_name)),
                                        source: "health_monitor".to_string(),
                                        timestamp: chrono::Utc::now(),
                                        status: crate::monitoring::alerts::AlertStatus::Active,
                                        metadata: std::collections::HashMap::new(),
                                        resolved_at: None,
                                        resolved_by: None,
                                    };
                                    alert_manager.create_alert(alert).await;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("Health check failed: {}", e);
                    }
                }
            }
        });

        // Start alert evaluation task
        let alert_manager = self.alert_manager.clone();
        let metrics_collector = self.metrics_collector.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Evaluate every minute
            loop {
                interval.tick().await;
                
                let metrics = metrics_collector.get_all_metrics().await;
                alert_manager.evaluate_rules(&metrics).await;
            }
        });

        // Start alert cleanup task
        let alert_manager = self.alert_manager.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(3600)); // Cleanup every hour
            loop {
                interval.tick().await;
                alert_manager.cleanup_resolved_alerts(24).await; // Keep resolved alerts for 24 hours
            }
        });

        // Setup default alert rules
        self.alert_manager.setup_default_rules().await;

        tracing::info!("Monitoring service background tasks started");
    }

    pub async fn record_request_metrics(&self, method: &str, path: &str, status_code: u16, duration_ms: f64) {
        self.metrics_collector.record_request(method, path, status_code, duration_ms).await;
    }

    pub async fn record_business_metrics(&self, event_type: &str, entity_id: &str, value: f64) {
        let mut labels = std::collections::HashMap::new();
        labels.insert("event_type".to_string(), event_type.to_string());
        labels.insert("entity_id".to_string(), entity_id.to_string());
        
        self.metrics_collector.record_metric_point("business_events", value, labels).await;
    }

    pub async fn get_system_overview(&self) -> Result<SystemOverview, crate::error::AppError> {
        let health_status = self.health_checker.check_health().await?;
        let active_alerts = self.alert_manager.get_active_alerts().await;
        let metrics = self.metrics_collector.get_all_metrics().await;

        Ok(SystemOverview {
            health_status,
            active_alerts_count: active_alerts.len(),
            critical_alerts_count: active_alerts.iter()
                .filter(|alert| matches!(alert.severity, crate::monitoring::alerts::AlertSeverity::Critical))
                .count(),
            key_metrics: extract_key_metrics(&metrics),
            timestamp: chrono::Utc::now(),
        })
    }
}

#[derive(serde::Serialize)]
pub struct SystemOverview {
    pub health_status: crate::monitoring::health::HealthStatus,
    pub active_alerts_count: usize,
    pub critical_alerts_count: usize,
    pub key_metrics: KeyMetrics,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize)]
pub struct KeyMetrics {
    pub requests_per_minute: f64,
    pub error_rate_percent: f64,
    pub average_response_time_ms: f64,
    pub active_connections: f64,
    pub database_queries_per_minute: f64,
    pub cache_hit_rate_percent: f64,
}

fn extract_key_metrics(metrics: &std::collections::HashMap<String, f64>) -> KeyMetrics {
    KeyMetrics {
        requests_per_minute: metrics.get("http_requests_total").copied().unwrap_or(0.0),
        error_rate_percent: calculate_error_rate(metrics),
        average_response_time_ms: metrics.get("http_request_duration_ms").copied().unwrap_or(0.0),
        active_connections: metrics.get("active_connections").copied().unwrap_or(0.0),
        database_queries_per_minute: metrics.get("database_queries_total").copied().unwrap_or(0.0),
        cache_hit_rate_percent: calculate_cache_hit_rate(metrics),
    }
}

fn calculate_error_rate(metrics: &std::collections::HashMap<String, f64>) -> f64 {
    let total_requests = metrics.get("http_requests_total").copied().unwrap_or(0.0);
    let error_requests = metrics.get("http_errors_total").copied().unwrap_or(0.0);
    
    if total_requests > 0.0 {
        (error_requests / total_requests) * 100.0
    } else {
        0.0
    }
}

fn calculate_cache_hit_rate(metrics: &std::collections::HashMap<String, f64>) -> f64 {
    let cache_hits = metrics.iter()
        .filter(|(key, _)| key.contains("cache_operations_total") && key.contains("result=hit"))
        .map(|(_, value)| *value)
        .sum::<f64>();
    
    let cache_total = metrics.iter()
        .filter(|(key, _)| key.contains("cache_operations_total"))
        .map(|(_, value)| *value)
        .sum::<f64>();
    
    if cache_total > 0.0 {
        (cache_hits / cache_total) * 100.0
    } else {
        0.0
    }
}

// HTTP handlers for monitoring endpoints
use axum::{extract::State, response::Json};

pub async fn get_system_overview(
    State(monitoring_service): State<MonitoringService>,
) -> Result<Json<SystemOverview>, crate::error::AppError> {
    let overview = monitoring_service.get_system_overview().await?;
    Ok(Json(overview))
}

pub async fn get_metrics_prometheus(
    State(monitoring_service): State<MonitoringService>,
) -> String {
    monitoring_service.metrics_collector.export_prometheus_metrics().await
}

pub async fn get_active_alerts(
    State(monitoring_service): State<MonitoringService>,
) -> Json<Vec<crate::monitoring::alerts::Alert>> {
    let alerts = monitoring_service.alert_manager.get_active_alerts().await;
    Json(alerts)
}

pub async fn acknowledge_alert(
    State(monitoring_service): State<MonitoringService>,
    axum::extract::Path(alert_id): axum::extract::Path<uuid::Uuid>,
    Json(request): Json<AcknowledgeAlertRequest>,
) -> Result<Json<serde_json::Value>, crate::error::AppError> {
    let success = monitoring_service.alert_manager
        .acknowledge_alert(alert_id, request.acknowledged_by)
        .await;
    
    Ok(Json(serde_json::json!({
        "success": success,
        "alert_id": alert_id
    })))
}

pub async fn resolve_alert(
    State(monitoring_service): State<MonitoringService>,
    axum::extract::Path(alert_id): axum::extract::Path<uuid::Uuid>,
    Json(request): Json<ResolveAlertRequest>,
) -> Result<Json<serde_json::Value>, crate::error::AppError> {
    let success = monitoring_service.alert_manager
        .resolve_alert(alert_id, request.resolved_by)
        .await;
    
    Ok(Json(serde_json::json!({
        "success": success,
        "alert_id": alert_id
    })))
}

#[derive(serde::Deserialize)]
pub struct AcknowledgeAlertRequest {
    pub acknowledged_by: String,
}

#[derive(serde::Deserialize)]
pub struct ResolveAlertRequest {
    pub resolved_by: Option<String>,
}