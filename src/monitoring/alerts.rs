use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub title: String,
    pub description: String,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub status: AlertStatus,
    pub metadata: HashMap<String, String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertType {
    SystemHealth,
    Performance,
    Security,
    Business,
    Infrastructure,
    Database,
    Cache,
    ExternalService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertStatus {
    Active,
    Acknowledged,
    Resolved,
    Suppressed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub condition: AlertCondition,
    pub severity: AlertSeverity,
    pub enabled: bool,
    pub cooldown_minutes: u32,
    pub notification_channels: Vec<NotificationChannel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    MetricThreshold {
        metric: String,
        operator: ComparisonOperator,
        threshold: f64,
        duration_minutes: u32,
    },
    HealthCheck {
        component: String,
        status: String,
    },
    ErrorRate {
        threshold_percentage: f64,
        time_window_minutes: u32,
    },
    ResponseTime {
        threshold_ms: u64,
        percentile: u8, // 50, 90, 95, 99
        time_window_minutes: u32,
    },
    Custom {
        query: String,
        expected_result: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email { addresses: Vec<String> },
    Slack { webhook_url: String, channel: String },
    Discord { webhook_url: String },
    Webhook { url: String, headers: HashMap<String, String> },
    SMS { phone_numbers: Vec<String> },
    PagerDuty { integration_key: String },
}

#[derive(Clone)]
pub struct AlertManager {
    active_alerts: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, Alert>>>,
    alert_rules: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, AlertRule>>>,
    notification_service: NotificationService,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            active_alerts: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            alert_rules: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            notification_service: NotificationService::new(),
        }
    }

    pub async fn create_alert(&self, alert: Alert) -> Uuid {
        let alert_id = alert.id;
        let mut alerts = self.active_alerts.write().await;
        alerts.insert(alert_id, alert.clone());

        // Send notifications
        self.notification_service.send_alert_notification(&alert).await;

        alert_id
    }

    pub async fn resolve_alert(&self, alert_id: Uuid, resolved_by: Option<String>) -> bool {
        let mut alerts = self.active_alerts.write().await;
        
        if let Some(alert) = alerts.get_mut(&alert_id) {
            alert.status = AlertStatus::Resolved;
            alert.resolved_at = Some(Utc::now());
            alert.resolved_by = resolved_by;
            
            // Send resolution notification
            self.notification_service.send_resolution_notification(alert).await;
            
            true
        } else {
            false
        }
    }

    pub async fn acknowledge_alert(&self, alert_id: Uuid, acknowledged_by: String) -> bool {
        let mut alerts = self.active_alerts.write().await;
        
        if let Some(alert) = alerts.get_mut(&alert_id) {
            alert.status = AlertStatus::Acknowledged;
            alert.metadata.insert("acknowledged_by".to_string(), acknowledged_by);
            alert.metadata.insert("acknowledged_at".to_string(), Utc::now().to_rfc3339());
            true
        } else {
            false
        }
    }

    pub async fn get_active_alerts(&self) -> Vec<Alert> {
        let alerts = self.active_alerts.read().await;
        alerts.values()
            .filter(|alert| matches!(alert.status, AlertStatus::Active | AlertStatus::Acknowledged))
            .cloned()
            .collect()
    }

    pub async fn add_alert_rule(&self, rule: AlertRule) {
        let mut rules = self.alert_rules.write().await;
        rules.insert(rule.id, rule);
    }

    pub async fn evaluate_rules(&self, metrics: &HashMap<String, f64>) {
        let rules = self.alert_rules.read().await;
        
        for rule in rules.values() {
            if !rule.enabled {
                continue;
            }

            if self.should_trigger_alert(rule, metrics).await {
                let alert = Alert {
                    id: Uuid::new_v4(),
                    alert_type: AlertType::Performance, // Would be determined by rule
                    severity: rule.severity.clone(),
                    title: rule.name.clone(),
                    description: rule.description.clone(),
                    source: "alert_manager".to_string(),
                    timestamp: Utc::now(),
                    status: AlertStatus::Active,
                    metadata: HashMap::new(),
                    resolved_at: None,
                    resolved_by: None,
                };

                self.create_alert(alert).await;
            }
        }
    }

    async fn should_trigger_alert(&self, rule: &AlertRule, metrics: &HashMap<String, f64>) -> bool {
        match &rule.condition {
            AlertCondition::MetricThreshold { metric, operator, threshold, .. } => {
                if let Some(value) = metrics.get(metric) {
                    match operator {
                        ComparisonOperator::GreaterThan => value > threshold,
                        ComparisonOperator::LessThan => value < threshold,
                        ComparisonOperator::Equal => (value - threshold).abs() < f64::EPSILON,
                        ComparisonOperator::NotEqual => (value - threshold).abs() >= f64::EPSILON,
                        ComparisonOperator::GreaterThanOrEqual => value >= threshold,
                        ComparisonOperator::LessThanOrEqual => value <= threshold,
                    }
                } else {
                    false
                }
            }
            AlertCondition::ErrorRate { threshold_percentage, .. } => {
                if let Some(error_rate) = metrics.get("error_rate") {
                    error_rate > threshold_percentage
                } else {
                    false
                }
            }
            AlertCondition::ResponseTime { threshold_ms, .. } => {
                if let Some(response_time) = metrics.get("avg_response_time_ms") {
                    response_time > &(*threshold_ms as f64)
                } else {
                    false
                }
            }
            _ => false, // Other conditions would be implemented
        }
    }

    pub async fn cleanup_resolved_alerts(&self, retention_hours: u32) {
        let cutoff_time = Utc::now() - chrono::Duration::hours(retention_hours as i64);
        let mut alerts = self.active_alerts.write().await;
        
        alerts.retain(|_, alert| {
            match alert.status {
                AlertStatus::Resolved => {
                    alert.resolved_at.map_or(true, |resolved_at| resolved_at > cutoff_time)
                }
                _ => true,
            }
        });
    }
}

#[derive(Clone)]
pub struct NotificationService {
    // In a real implementation, this would have HTTP clients for various services
}

impl NotificationService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn send_alert_notification(&self, alert: &Alert) {
        // In a real implementation, this would send notifications via configured channels
        tracing::warn!(
            "ALERT: {} - {} (Severity: {:?})",
            alert.title,
            alert.description,
            alert.severity
        );
    }

    pub async fn send_resolution_notification(&self, alert: &Alert) {
        tracing::info!(
            "RESOLVED: {} - Alert has been resolved",
            alert.title
        );
    }

    async fn send_email_notification(&self, _addresses: &[String], _alert: &Alert) {
        // Implementation would send email via SMTP or email service
    }

    async fn send_slack_notification(&self, _webhook_url: &str, _channel: &str, _alert: &Alert) {
        // Implementation would send Slack message via webhook
    }

    async fn send_webhook_notification(&self, _url: &str, _headers: &HashMap<String, String>, _alert: &Alert) {
        // Implementation would send HTTP POST to webhook URL
    }
}

// Predefined alert rules for common scenarios
impl AlertManager {
    pub async fn setup_default_rules(&self) {
        // High error rate alert
        let error_rate_rule = AlertRule {
            id: Uuid::new_v4(),
            name: "High Error Rate".to_string(),
            description: "Error rate exceeds 5% for more than 5 minutes".to_string(),
            condition: AlertCondition::ErrorRate {
                threshold_percentage: 5.0,
                time_window_minutes: 5,
            },
            severity: AlertSeverity::High,
            enabled: true,
            cooldown_minutes: 15,
            notification_channels: vec![
                NotificationChannel::Email {
                    addresses: vec!["admin@example.com".to_string()],
                },
            ],
        };

        // High response time alert
        let response_time_rule = AlertRule {
            id: Uuid::new_v4(),
            name: "High Response Time".to_string(),
            description: "95th percentile response time exceeds 1 second".to_string(),
            condition: AlertCondition::ResponseTime {
                threshold_ms: 1000,
                percentile: 95,
                time_window_minutes: 5,
            },
            severity: AlertSeverity::Medium,
            enabled: true,
            cooldown_minutes: 10,
            notification_channels: vec![
                NotificationChannel::Slack {
                    webhook_url: "https://hooks.slack.com/services/...".to_string(),
                    channel: "#alerts".to_string(),
                },
            ],
        };

        // Database connection alert
        let db_health_rule = AlertRule {
            id: Uuid::new_v4(),
            name: "Database Health Check Failed".to_string(),
            description: "Database health check is failing".to_string(),
            condition: AlertCondition::HealthCheck {
                component: "database".to_string(),
                status: "unhealthy".to_string(),
            },
            severity: AlertSeverity::Critical,
            enabled: true,
            cooldown_minutes: 5,
            notification_channels: vec![
                NotificationChannel::Email {
                    addresses: vec!["admin@example.com".to_string()],
                },
                NotificationChannel::Slack {
                    webhook_url: "https://hooks.slack.com/services/...".to_string(),
                    channel: "#critical-alerts".to_string(),
                },
            ],
        };

        self.add_alert_rule(error_rate_rule).await;
        self.add_alert_rule(response_time_rule).await;
        self.add_alert_rule(db_health_rule).await;
    }
}