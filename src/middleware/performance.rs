use axum::{
    extract::Request,
    http::{HeaderMap, Method, Uri},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn};

/// Performance monitoring middleware that tracks request duration and logs slow requests
pub async fn performance_middleware(
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let path = uri.path().to_string();
    let method_str = method.to_string();
    
    // Get user agent for additional context
    let user_agent = headers
        .get("user-agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    // Process the request
    let response = next.run(request).await;
    
    let duration = start.elapsed();
    let status = response.status();
    
    // Log performance metrics
    let duration_ms = duration.as_millis();
    
    if duration_ms > 1000 {
        // Log slow requests (>1s) as warnings
        warn!(
            method = %method_str,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %duration_ms,
            user_agent = %user_agent,
            "Slow request detected"
        );
    } else if duration_ms > 500 {
        // Log moderately slow requests (>500ms) as info
        info!(
            method = %method_str,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %duration_ms,
            "Request completed"
        );
    } else {
        // Log fast requests as debug
        tracing::debug!(
            method = %method_str,
            path = %path,
            status = %status.as_u16(),
            duration_ms = %duration_ms,
            "Request completed"
        );
    }
    
    // Add performance headers to response
    let mut response = response;
    response.headers_mut().insert(
        "X-Response-Time",
        format!("{}ms", duration_ms).parse().unwrap(),
    );
    
    response
}

/// Request metrics collector for Prometheus
pub struct RequestMetrics {
    pub request_duration: prometheus::HistogramVec,
    pub request_count: prometheus::CounterVec,
    pub active_requests: prometheus::GaugeVec,
}

impl RequestMetrics {
    pub fn new(registry: &prometheus::Registry) -> Result<Self, prometheus::Error> {
        let request_duration = prometheus::HistogramVec::new(
            prometheus::HistogramOpts::new(
                "http_request_duration_seconds",
                "HTTP request duration in seconds"
            ).buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0, 10.0]),
            &["method", "path", "status"]
        )?;
        
        let request_count = prometheus::CounterVec::new(
            prometheus::Opts::new(
                "http_requests_total",
                "Total number of HTTP requests"
            ),
            &["method", "path", "status"]
        )?;
        
        let active_requests = prometheus::GaugeVec::new(
            prometheus::Opts::new(
                "http_active_requests",
                "Number of active HTTP requests"
            ),
            &["method", "path"]
        )?;
        
        registry.register(Box::new(request_duration.clone()))?;
        registry.register(Box::new(request_count.clone()))?;
        registry.register(Box::new(active_requests.clone()))?;
        
        Ok(Self {
            request_duration,
            request_count,
            active_requests,
        })
    }
    
    pub fn record_request(&self, method: &str, path: &str, status: u16, duration: std::time::Duration) {
        let status_str = status.to_string();
        
        self.request_duration
            .with_label_values(&[method, path, &status_str])
            .observe(duration.as_secs_f64());
            
        self.request_count
            .with_label_values(&[method, path, &status_str])
            .inc();
    }
    
    pub fn increment_active_requests(&self, method: &str, path: &str) {
        self.active_requests
            .with_label_values(&[method, path])
            .inc();
    }
    
    pub fn decrement_active_requests(&self, method: &str, path: &str) {
        self.active_requests
            .with_label_values(&[method, path])
            .dec();
    }
}