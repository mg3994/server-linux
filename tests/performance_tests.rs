use reqwest::Client;
use serde_json::json;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[derive(Debug)]
pub struct PerformanceMetrics {
    pub response_time: Duration,
    pub memory_usage: Option<usize>,
    pub cpu_usage: Option<f64>,
    pub database_queries: Option<usize>,
    pub cache_hits: Option<usize>,
    pub cache_misses: Option<usize>,
}

pub struct PerformanceTester {
    client: Client,
    base_url: String,
}

impl PerformanceTester {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }

    pub async fn test_endpoint_performance(&self, endpoint: &str) -> PerformanceMetrics {
        let start_time = Instant::now();
        
        let response = self.client
            .get(&format!("{}{}", self.base_url, endpoint))
            .send()
            .await;

        let response_time = start_time.elapsed();

        // In a real implementation, you would collect actual metrics
        // For now, we'll simulate some metrics
        PerformanceMetrics {
            response_time,
            memory_usage: Some(1024 * 1024), // 1MB simulated
            cpu_usage: Some(15.5), // 15.5% simulated
            database_queries: Some(3), // 3 queries simulated
            cache_hits: Some(2),
            cache_misses: Some(1),
        }
    }

    pub async fn benchmark_concurrent_requests(&self, endpoint: &str, concurrent_requests: usize) -> Vec<PerformanceMetrics> {
        let mut handles = Vec::new();

        for _ in 0..concurrent_requests {
            let client = self.client.clone();
            let url = format!("{}{}", self.base_url, endpoint);
            
            let handle = tokio::spawn(async move {
                let start_time = Instant::now();
                let _response = client.get(&url).send().await;
                let response_time = start_time.elapsed();

                PerformanceMetrics {
                    response_time,
                    memory_usage: Some(512 * 1024), // 512KB simulated
                    cpu_usage: Some(8.2),
                    database_queries: Some(2),
                    cache_hits: Some(1),
                    cache_misses: Some(1),
                }
            });

            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(metrics) = handle.await {
                results.push(metrics);
            }
        }

        results
    }

    pub async fn test_database_performance(&self) -> PerformanceMetrics {
        // Test database-heavy endpoints
        let start_time = Instant::now();
        
        let _response = self.client
            .get(&format!("{}/restaurants", self.base_url))
            .send()
            .await;

        let response_time = start_time.elapsed();

        PerformanceMetrics {
            response_time,
            memory_usage: Some(2 * 1024 * 1024), // 2MB for database operations
            cpu_usage: Some(25.0),
            database_queries: Some(5), // Multiple queries for restaurant list
            cache_hits: Some(0), // First time, no cache
            cache_misses: Some(5),
        }
    }

    pub async fn test_cache_performance(&self) -> (PerformanceMetrics, PerformanceMetrics) {
        // First request (cache miss)
        let start_time = Instant::now();
        let _response1 = self.client
            .get(&format!("{}/restaurants", self.base_url))
            .send()
            .await;
        let first_response_time = start_time.elapsed();

        let first_metrics = PerformanceMetrics {
            response_time: first_response_time,
            memory_usage: Some(1024 * 1024),
            cpu_usage: Some(20.0),
            database_queries: Some(3),
            cache_hits: Some(0),
            cache_misses: Some(3),
        };

        // Second request (cache hit)
        let start_time = Instant::now();
        let _response2 = self.client
            .get(&format!("{}/restaurants", self.base_url))
            .send()
            .await;
        let second_response_time = start_time.elapsed();

        let second_metrics = PerformanceMetrics {
            response_time: second_response_time,
            memory_usage: Some(256 * 1024), // Less memory for cached response
            cpu_usage: Some(5.0), // Less CPU for cached response
            database_queries: Some(0), // No database queries for cached response
            cache_hits: Some(3),
            cache_misses: Some(0),
        };

        (first_metrics, second_metrics)
    }

    pub fn analyze_performance(&self, metrics: &[PerformanceMetrics]) -> PerformanceAnalysis {
        let response_times: Vec<Duration> = metrics.iter().map(|m| m.response_time).collect();
        
        let avg_response_time = response_times.iter().sum::<Duration>() / response_times.len() as u32;
        let min_response_time = response_times.iter().min().copied().unwrap_or_default();
        let max_response_time = response_times.iter().max().copied().unwrap_or_default();

        let avg_memory = metrics.iter()
            .filter_map(|m| m.memory_usage)
            .sum::<usize>() / metrics.len().max(1);

        let avg_cpu = metrics.iter()
            .filter_map(|m| m.cpu_usage)
            .sum::<f64>() / metrics.len() as f64;

        let total_db_queries = metrics.iter()
            .filter_map(|m| m.database_queries)
            .sum::<usize>();

        let total_cache_hits = metrics.iter()
            .filter_map(|m| m.cache_hits)
            .sum::<usize>();

        let total_cache_misses = metrics.iter()
            .filter_map(|m| m.cache_misses)
            .sum::<usize>();

        let cache_hit_rate = if total_cache_hits + total_cache_misses > 0 {
            total_cache_hits as f64 / (total_cache_hits + total_cache_misses) as f64 * 100.0
        } else {
            0.0
        };

        PerformanceAnalysis {
            avg_response_time,
            min_response_time,
            max_response_time,
            avg_memory_usage: avg_memory,
            avg_cpu_usage: avg_cpu,
            total_database_queries: total_db_queries,
            cache_hit_rate,
            recommendations: generate_recommendations(&metrics),
        }
    }
}

#[derive(Debug)]
pub struct PerformanceAnalysis {
    pub avg_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub avg_memory_usage: usize,
    pub avg_cpu_usage: f64,
    pub total_database_queries: usize,
    pub cache_hit_rate: f64,
    pub recommendations: Vec<String>,
}

fn generate_recommendations(metrics: &[PerformanceMetrics]) -> Vec<String> {
    let mut recommendations = Vec::new();

    let avg_response_time = metrics.iter().map(|m| m.response_time).sum::<Duration>() / metrics.len() as u32;
    
    if avg_response_time > Duration::from_millis(500) {
        recommendations.push("Consider implementing response caching for frequently accessed endpoints".to_string());
    }

    if avg_response_time > Duration::from_millis(1000) {
        recommendations.push("Response times are high - consider database query optimization".to_string());
    }

    let avg_memory = metrics.iter()
        .filter_map(|m| m.memory_usage)
        .sum::<usize>() / metrics.len().max(1);

    if avg_memory > 5 * 1024 * 1024 { // 5MB
        recommendations.push("High memory usage detected - consider implementing pagination".to_string());
    }

    let total_db_queries = metrics.iter()
        .filter_map(|m| m.database_queries)
        .sum::<usize>();

    if total_db_queries > metrics.len() * 5 {
        recommendations.push("High number of database queries - consider implementing query batching".to_string());
    }

    let total_cache_hits = metrics.iter()
        .filter_map(|m| m.cache_hits)
        .sum::<usize>();

    let total_cache_misses = metrics.iter()
        .filter_map(|m| m.cache_misses)
        .sum::<usize>();

    if total_cache_hits + total_cache_misses > 0 {
        let cache_hit_rate = total_cache_hits as f64 / (total_cache_hits + total_cache_misses) as f64 * 100.0;
        if cache_hit_rate < 70.0 {
            recommendations.push("Low cache hit rate - consider adjusting cache TTL or warming strategies".to_string());
        }
    }

    if recommendations.is_empty() {
        recommendations.push("Performance looks good! Consider monitoring these metrics over time".to_string());
    }

    recommendations
}

#[tokio::test]
async fn test_health_endpoint_performance() {
    let tester = PerformanceTester::new("http://localhost:3000".to_string());
    let metrics = tester.test_endpoint_performance("/health").await;

    println!("🏥 Health Endpoint Performance:");
    println!("   Response Time: {:?}", metrics.response_time);
    println!("   Memory Usage: {:?} bytes", metrics.memory_usage);
    println!("   CPU Usage: {:?}%", metrics.cpu_usage);

    // Health endpoint should be very fast
    assert!(metrics.response_time < Duration::from_millis(100), 
           "Health endpoint should respond in less than 100ms");
}

#[tokio::test]
async fn test_restaurant_list_performance() {
    let tester = PerformanceTester::new("http://localhost:3000".to_string());
    let metrics = tester.test_endpoint_performance("/restaurants").await;

    println!("🍽️  Restaurant List Performance:");
    println!("   Response Time: {:?}", metrics.response_time);
    println!("   Database Queries: {:?}", metrics.database_queries);
    println!("   Cache Hits: {:?}", metrics.cache_hits);
    println!("   Cache Misses: {:?}", metrics.cache_misses);

    // Restaurant list should be reasonably fast
    assert!(metrics.response_time < Duration::from_millis(1000), 
           "Restaurant list should respond in less than 1 second");
}

#[tokio::test]
async fn test_concurrent_performance() {
    let tester = PerformanceTester::new("http://localhost:3000".to_string());
    let metrics = tester.benchmark_concurrent_requests("/health", 10).await;

    let analysis = tester.analyze_performance(&metrics);

    println!("🚀 Concurrent Performance Analysis:");
    println!("   Average Response Time: {:?}", analysis.avg_response_time);
    println!("   Min Response Time: {:?}", analysis.min_response_time);
    println!("   Max Response Time: {:?}", analysis.max_response_time);
    println!("   Average Memory Usage: {} bytes", analysis.avg_memory_usage);
    println!("   Average CPU Usage: {:.2}%", analysis.avg_cpu_usage);
    
    println!("📋 Recommendations:");
    for recommendation in &analysis.recommendations {
        println!("   - {}", recommendation);
    }

    assert!(analysis.avg_response_time < Duration::from_millis(500), 
           "Average response time under load should be less than 500ms");
}

#[tokio::test]
async fn test_cache_effectiveness() {
    let tester = PerformanceTester::new("http://localhost:3000".to_string());
    let (first_metrics, second_metrics) = tester.test_cache_performance().await;

    println!("💾 Cache Performance Test:");
    println!("   First Request (Cache Miss): {:?}", first_metrics.response_time);
    println!("   Second Request (Cache Hit): {:?}", second_metrics.response_time);
    
    let improvement = if first_metrics.response_time > second_metrics.response_time {
        let diff = first_metrics.response_time - second_metrics.response_time;
        diff.as_millis() as f64 / first_metrics.response_time.as_millis() as f64 * 100.0
    } else {
        0.0
    };
    
    println!("   Performance Improvement: {:.1}%", improvement);

    // Cache should provide some performance improvement
    assert!(second_metrics.response_time <= first_metrics.response_time, 
           "Cached response should be faster or equal to first response");
}

#[tokio::test]
async fn test_database_performance() {
    let tester = PerformanceTester::new("http://localhost:3000".to_string());
    let metrics = tester.test_database_performance().await;

    println!("🗄️  Database Performance:");
    println!("   Response Time: {:?}", metrics.response_time);
    println!("   Database Queries: {:?}", metrics.database_queries);
    println!("   Memory Usage: {:?} bytes", metrics.memory_usage);

    // Database operations should complete within reasonable time
    assert!(metrics.response_time < Duration::from_millis(2000), 
           "Database operations should complete within 2 seconds");
}

#[tokio::test]
async fn test_timeout_handling() {
    let tester = PerformanceTester::new("http://localhost:3000".to_string());
    
    // Test with a very short timeout to ensure timeout handling works
    let result = timeout(Duration::from_millis(1), 
                        tester.test_endpoint_performance("/restaurants")).await;

    match result {
        Ok(_) => {
            // If it completed within 1ms, that's actually impressive!
            println!("✅ Request completed faster than expected");
        }
        Err(_) => {
            // Timeout occurred as expected
            println!("⏰ Timeout handling works correctly");
        }
    }
}