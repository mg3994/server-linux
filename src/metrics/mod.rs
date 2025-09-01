use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::{DateTime, Utc};
use prometheus::{
    Counter, Histogram, HistogramOpts, IntCounter, IntGauge, Opts, Registry, TextEncoder,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::delivery::metrics::DeliveryMetrics;

#[derive(Clone)]
pub struct MetricsCollector {
    registry: Registry,

    // HTTP metrics
    pub http_requests_total: Counter,
    pub http_request_duration: Histogram,
    pub http_requests_in_flight: IntGauge,

    // Order metrics
    pub orders_created_total: IntCounter,
    pub orders_completed_total: IntCounter,
    pub orders_cancelled_total: IntCounter,
    pub active_orders: IntGauge,

    // Payment metrics
    pub payments_processed_total: IntCounter,
    pub payments_failed_total: IntCounter,
    pub payment_amount_total: Counter,

    // FCM metrics
    pub notifications_sent_total: IntCounter,
    pub notifications_failed_total: IntCounter,

    // WebSocket metrics
    pub websocket_connections: IntGauge,
    pub websocket_messages_sent: IntCounter,

    // System metrics
    pub system_info: Arc<RwLock<SystemInfo>>,
    
    // Delivery metrics
    pub delivery_metrics: Option<DeliveryMetrics>,
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub start_time: DateTime<Utc>,
    pub version: String,
    pub build_info: String,
}

impl MetricsCollector {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let registry = Registry::new();

        // HTTP metrics
        let http_requests_total = Counter::with_opts(
            Opts::new("http_requests_total", "Total number of HTTP requests")
                .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(http_requests_total.clone()))?;

        let http_request_duration = Histogram::with_opts(
            HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request duration in seconds",
            )
            .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(http_request_duration.clone()))?;

        let http_requests_in_flight = IntGauge::with_opts(
            Opts::new(
                "http_requests_in_flight",
                "Number of HTTP requests currently being processed",
            )
            .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(http_requests_in_flight.clone()))?;

        // Order metrics
        let orders_created_total = IntCounter::with_opts(
            Opts::new("orders_created_total", "Total number of orders created")
                .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(orders_created_total.clone()))?;

        let orders_completed_total = IntCounter::with_opts(
            Opts::new("orders_completed_total", "Total number of orders completed")
                .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(orders_completed_total.clone()))?;

        let orders_cancelled_total = IntCounter::with_opts(
            Opts::new("orders_cancelled_total", "Total number of orders cancelled")
                .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(orders_cancelled_total.clone()))?;

        let active_orders = IntGauge::with_opts(
            Opts::new("active_orders", "Number of currently active orders")
                .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(active_orders.clone()))?;

        // Payment metrics
        let payments_processed_total = IntCounter::with_opts(
            Opts::new(
                "payments_processed_total",
                "Total number of payments processed",
            )
            .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(payments_processed_total.clone()))?;

        let payments_failed_total = IntCounter::with_opts(
            Opts::new("payments_failed_total", "Total number of failed payments")
                .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(payments_failed_total.clone()))?;

        let payment_amount_total = Counter::with_opts(
            Opts::new("payment_amount_total", "Total payment amount processed")
                .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(payment_amount_total.clone()))?;

        // FCM metrics
        let notifications_sent_total = IntCounter::with_opts(
            Opts::new(
                "notifications_sent_total",
                "Total number of notifications sent",
            )
            .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(notifications_sent_total.clone()))?;

        let notifications_failed_total = IntCounter::with_opts(
            Opts::new(
                "notifications_failed_total",
                "Total number of failed notifications",
            )
            .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(notifications_failed_total.clone()))?;

        // WebSocket metrics
        let websocket_connections = IntGauge::with_opts(
            Opts::new(
                "websocket_connections",
                "Number of active WebSocket connections",
            )
            .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(websocket_connections.clone()))?;

        let websocket_messages_sent = IntCounter::with_opts(
            Opts::new(
                "websocket_messages_sent_total",
                "Total number of WebSocket messages sent",
            )
            .const_label("service", "delivery_server"),
        )?;
        registry.register(Box::new(websocket_messages_sent.clone()))?;

        let system_info = Arc::new(RwLock::new(SystemInfo {
            start_time: Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_info: format!("{}@{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        }));

        // Initialize delivery metrics
        let delivery_metrics = DeliveryMetrics::new(&registry).ok();

        Ok(Self {
            registry,
            http_requests_total,
            http_request_duration,
            http_requests_in_flight,
            orders_created_total,
            orders_completed_total,
            orders_cancelled_total,
            active_orders,
            payments_processed_total,
            payments_failed_total,
            payment_amount_total,
            notifications_sent_total,
            notifications_failed_total,
            websocket_connections,
            websocket_messages_sent,
            system_info,
            delivery_metrics,
        })
    }

    pub fn record_http_request(&self, duration: f64) {
        self.http_requests_total.inc();
        self.http_request_duration.observe(duration);
    }

    pub fn inc_http_in_flight(&self) {
        self.http_requests_in_flight.inc();
    }

    pub fn dec_http_in_flight(&self) {
        self.http_requests_in_flight.dec();
    }

    pub fn record_order_created(&self) {
        self.orders_created_total.inc();
        self.active_orders.inc();
    }

    pub fn record_order_completed(&self) {
        self.orders_completed_total.inc();
        self.active_orders.dec();
    }

    pub fn record_order_cancelled(&self) {
        self.orders_cancelled_total.inc();
        self.active_orders.dec();
    }

    pub fn record_payment_processed(&self, amount: f64) {
        self.payments_processed_total.inc();
        self.payment_amount_total.inc_by(amount);
    }

    pub fn record_payment_failed(&self) {
        self.payments_failed_total.inc();
    }

    pub fn record_notification_sent(&self) {
        self.notifications_sent_total.inc();
    }

    pub fn record_notification_failed(&self) {
        self.notifications_failed_total.inc();
    }

    pub fn inc_websocket_connections(&self) {
        self.websocket_connections.inc();
    }

    pub fn dec_websocket_connections(&self) {
        self.websocket_connections.dec();
    }

    pub fn record_websocket_message_sent(&self) {
        self.websocket_messages_sent.inc();
    }
}

pub async fn metrics_handler(State(metrics): State<MetricsCollector>) -> Response {
    let encoder = TextEncoder::new();
    let metric_families = metrics.registry.gather();

    match encoder.encode_to_string(&metric_families) {
        Ok(output) => (
            StatusCode::OK,
            [("content-type", "text/plain; version=0.0.4")],
            output,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to encode metrics",
                "details": e.to_string()
            })),
        )
            .into_response(),
    }
}

pub async fn health_detailed_handler(
    State(metrics): State<MetricsCollector>,
) -> Json<serde_json::Value> {
    let system_info = metrics.system_info.read().await;
    let uptime = Utc::now().signed_duration_since(system_info.start_time);

    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now(),
        "uptime_seconds": uptime.num_seconds(),
        "version": system_info.version,
        "build_info": system_info.build_info,
        "metrics": {
            "http_requests_total": metrics.http_requests_total.get(),
            "active_orders": metrics.active_orders.get(),
            "websocket_connections": metrics.websocket_connections.get(),
            "notifications_sent_total": metrics.notifications_sent_total.get(),
            "payments_processed_total": metrics.payments_processed_total.get(),
        }
    }))
}
