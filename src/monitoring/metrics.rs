use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    pub metric_name: String,
    pub points: Vec<MetricPoint>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSummary {
    pub metric_name: String,
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub percentiles: HashMap<u8, f64>, // 50, 90, 95, 99
}

#[derive(Clone)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, Vec<MetricPoint>>>>,
    counters: Arc<RwLock<HashMap<String, u64>>>,
    gauges: Arc<RwLock<HashMap<String, f64>>>,
    histograms: Arc<RwLock<HashMap<String, Vec<f64>>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            counters: Arc::new(RwLock::new(HashMap::new())),
            gauges: Arc::new(RwLock::new(HashMap::new())),
            histograms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Counter operations
    pub async fn increment_counter(&self, name: &str, labels: HashMap<String, String>) {
        let key = self.create_metric_key(name, &labels);
        let mut counters = self.counters.write().await;
        *counters.entry(key.clone()).or_insert(0) += 1;

        // Also record as time series
        self.record_metric_point(name, 1.0, labels).await;
    }

    pub async fn increment_counter_by(&self, name: &str, value: u64, labels: HashMap<String, String>) {
        let key = self.create_metric_key(name, &labels);
        let mut counters = self.counters.write().await;
        *counters.entry(key.clone()).or_insert(0) += value;

        // Also record as time series
        self.record_metric_point(name, value as f64, labels).await;
    }

    // Gauge operations
    pub async fn set_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let key = self.create_metric_key(name, &labels);
        let mut gauges = self.gauges.write().await;
        gauges.insert(key, value);

        // Also record as time series
        self.record_metric_point(name, value, labels).await;
    }

    // Histogram operations
    pub async fn record_histogram(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let key = self.create_metric_key(name, &labels);
        let mut histograms = self.histograms.write().await;
        histograms.entry(key).or_insert_with(Vec::new).push(value);

        // Also record as time series
        self.record_metric_point(name, value, labels).await;
    }

    // Time series operations
    pub async fn record_metric_point(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let point = MetricPoint {
            timestamp: Utc::now(),
            value,
            labels,
        };

        let mut metrics = self.metrics.write().await;
        metrics.entry(name.to_string()).or_insert_with(Vec::new).push(point);
    }

    // Query operations
    pub async fn get_counter(&self, name: &str, labels: &HashMap<String, String>) -> u64 {
        let key = self.create_metric_key(name, labels);
        let counters = self.counters.read().await;
        counters.get(&key).copied().unwrap_or(0)
    }

    pub async fn get_gauge(&self, name: &str, labels: &HashMap<String, String>) -> Option<f64> {
        let key = self.create_metric_key(name, labels);
        let gauges = self.gauges.read().await;
        gauges.get(&key).copied()
    }

    pub async fn get_histogram_summary(&self, name: &str, labels: &HashMap<String, String>) -> Option<MetricSummary> {
        let key = self.create_metric_key(name, labels);
        let histograms = self.histograms.read().await;
        
        if let Some(values) = histograms.get(&key) {
            if values.is_empty() {
                return None;
            }

            let mut sorted_values = values.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let count = values.len() as u64;
            let sum = values.iter().sum::<f64>();
            let min = *sorted_values.first().unwrap();
            let max = *sorted_values.last().unwrap();
            let avg = sum / count as f64;

            let mut percentiles = HashMap::new();
            percentiles.insert(50, self.calculate_percentile(&sorted_values, 50.0));
            percentiles.insert(90, self.calculate_percentile(&sorted_values, 90.0));
            percentiles.insert(95, self.calculate_percentile(&sorted_values, 95.0));
            percentiles.insert(99, self.calculate_percentile(&sorted_values, 99.0));

            Some(MetricSummary {
                metric_name: name.to_string(),
                count,
                sum,
                min,
                max,
                avg,
                percentiles,
            })
        } else {
            None
        }
    }

    pub async fn get_time_series(&self, name: &str, start_time: DateTime<Utc>, end_time: DateTime<Utc>) -> Option<TimeSeries> {
        let metrics = self.metrics.read().await;
        
        if let Some(points) = metrics.get(name) {
            let filtered_points: Vec<MetricPoint> = points
                .iter()
                .filter(|point| point.timestamp >= start_time && point.timestamp <= end_time)
                .cloned()
                .collect();

            Some(TimeSeries {
                metric_name: name.to_string(),
                points: filtered_points,
                metadata: HashMap::new(),
            })
        } else {
            None
        }
    }

    pub async fn get_all_metrics(&self) -> HashMap<String, f64> {
        let mut all_metrics = HashMap::new();

        // Add counters
        let counters = self.counters.read().await;
        for (key, value) in counters.iter() {
            all_metrics.insert(key.clone(), *value as f64);
        }

        // Add gauges
        let gauges = self.gauges.read().await;
        for (key, value) in gauges.iter() {
            all_metrics.insert(key.clone(), *value);
        }

        all_metrics
    }

    // Cleanup old metrics
    pub async fn cleanup_old_metrics(&self, retention_hours: u32) {
        let cutoff_time = Utc::now() - chrono::Duration::hours(retention_hours as i64);
        let mut metrics = self.metrics.write().await;

        for points in metrics.values_mut() {
            points.retain(|point| point.timestamp > cutoff_time);
        }

        // Remove empty metric series
        metrics.retain(|_, points| !points.is_empty());
    }

    // Helper methods
    fn create_metric_key(&self, name: &str, labels: &HashMap<String, String>) -> String {
        if labels.is_empty() {
            name.to_string()
        } else {
            let mut label_pairs: Vec<String> = labels
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            label_pairs.sort();
            format!("{}:{}", name, label_pairs.join(","))
        }
    }

    fn calculate_percentile(&self, sorted_values: &[f64], percentile: f64) -> f64 {
        let index = (percentile / 100.0 * (sorted_values.len() - 1) as f64).round() as usize;
        sorted_values[index.min(sorted_values.len() - 1)]
    }
}

// Business-specific metrics
impl MetricsCollector {
    pub async fn record_request(&self, method: &str, path: &str, status_code: u16, duration_ms: f64) {
        let mut labels = HashMap::new();
        labels.insert("method".to_string(), method.to_string());
        labels.insert("path".to_string(), path.to_string());
        labels.insert("status_code".to_string(), status_code.to_string());

        // Record request count
        self.increment_counter("http_requests_total", labels.clone()).await;

        // Record request duration
        self.record_histogram("http_request_duration_ms", duration_ms, labels.clone()).await;

        // Record error rate
        if status_code >= 400 {
            self.increment_counter("http_errors_total", labels).await;
        }
    }

    pub async fn record_database_query(&self, query_type: &str, duration_ms: f64, success: bool) {
        let mut labels = HashMap::new();
        labels.insert("query_type".to_string(), query_type.to_string());
        labels.insert("success".to_string(), success.to_string());

        self.increment_counter("database_queries_total", labels.clone()).await;
        self.record_histogram("database_query_duration_ms", duration_ms, labels).await;
    }

    pub async fn record_cache_operation(&self, operation: &str, hit: bool) {
        let mut labels = HashMap::new();
        labels.insert("operation".to_string(), operation.to_string());
        labels.insert("result".to_string(), if hit { "hit" } else { "miss" }.to_string());

        self.increment_counter("cache_operations_total", labels).await;
    }

    pub async fn record_order_event(&self, event_type: &str, restaurant_id: &str) {
        let mut labels = HashMap::new();
        labels.insert("event_type".to_string(), event_type.to_string());
        labels.insert("restaurant_id".to_string(), restaurant_id.to_string());

        self.increment_counter("order_events_total", labels).await;
    }

    pub async fn record_delivery_event(&self, event_type: &str, delivery_person_id: &str) {
        let mut labels = HashMap::new();
        labels.insert("event_type".to_string(), event_type.to_string());
        labels.insert("delivery_person_id".to_string(), delivery_person_id.to_string());

        self.increment_counter("delivery_events_total", labels).await;
    }

    pub async fn set_active_connections(&self, count: u32) {
        self.set_gauge("active_connections", count as f64, HashMap::new()).await;
    }

    pub async fn set_memory_usage(&self, bytes: u64) {
        self.set_gauge("memory_usage_bytes", bytes as f64, HashMap::new()).await;
    }

    pub async fn set_cpu_usage(&self, percentage: f64) {
        self.set_gauge("cpu_usage_percent", percentage, HashMap::new()).await;
    }
}

// Prometheus-compatible metrics export
impl MetricsCollector {
    pub async fn export_prometheus_metrics(&self) -> String {
        let mut output = String::new();

        // Export counters
        let counters = self.counters.read().await;
        for (key, value) in counters.iter() {
            output.push_str(&format!("# TYPE {} counter\n", key));
            output.push_str(&format!("{} {}\n", key, value));
        }

        // Export gauges
        let gauges = self.gauges.read().await;
        for (key, value) in gauges.iter() {
            output.push_str(&format!("# TYPE {} gauge\n", key));
            output.push_str(&format!("{} {}\n", key, value));
        }

        // Export histogram summaries
        let histograms = self.histograms.read().await;
        for (key, values) in histograms.iter() {
            if !values.is_empty() {
                let mut sorted_values = values.clone();
                sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

                output.push_str(&format!("# TYPE {} histogram\n", key));
                output.push_str(&format!("{}_count {}\n", key, values.len()));
                output.push_str(&format!("{}_sum {}\n", key, values.iter().sum::<f64>()));
                
                // Add percentile buckets
                for percentile in [50.0, 90.0, 95.0, 99.0] {
                    let value = self.calculate_percentile(&sorted_values, percentile);
                    output.push_str(&format!("{}_bucket{{le=\"{}\"}} {}\n", key, percentile, value));
                }
            }
        }

        output
    }
}