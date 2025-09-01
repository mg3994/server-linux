use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::sleep;

#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub base_url: String,
    pub concurrent_users: usize,
    pub requests_per_user: usize,
    pub ramp_up_duration: Duration,
    pub test_duration: Duration,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:3000".to_string(),
            concurrent_users: 100,
            requests_per_user: 10,
            ramp_up_duration: Duration::from_secs(30),
            test_duration: Duration::from_secs(300), // 5 minutes
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadTestResult {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub average_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub response_time_percentiles: ResponseTimePercentiles,
}

#[derive(Debug, Clone)]
pub struct ResponseTimePercentiles {
    pub p50: Duration,
    pub p90: Duration,
    pub p95: Duration,
    pub p99: Duration,
}

#[derive(Debug, Clone)]
pub struct RequestResult {
    pub success: bool,
    pub response_time: Duration,
    pub status_code: Option<u16>,
    pub error: Option<String>,
}

pub struct LoadTester {
    config: LoadTestConfig,
    client: Client,
}

impl LoadTester {
    pub fn new(config: LoadTestConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    pub async fn run_load_test(&self) -> LoadTestResult {
        println!("🚀 Starting load test with {} concurrent users", self.config.concurrent_users);
        println!("📊 Each user will make {} requests", self.config.requests_per_user);
        println!("⏱️  Test duration: {:?}", self.config.test_duration);

        let semaphore = Arc::new(Semaphore::new(self.config.concurrent_users));
        let mut handles = Vec::new();
        let start_time = Instant::now();

        // Spawn concurrent users
        for user_id in 0..self.config.concurrent_users {
            let semaphore = semaphore.clone();
            let client = self.client.clone();
            let config = self.config.clone();

            let handle = tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                
                // Stagger user start times for ramp-up
                let delay = config.ramp_up_duration.as_millis() as u64 * user_id as u64 / config.concurrent_users as u64;
                sleep(Duration::from_millis(delay)).await;

                simulate_user_session(client, config, user_id).await
            });

            handles.push(handle);
        }

        // Collect results
        let mut all_results = Vec::new();
        for handle in handles {
            if let Ok(user_results) = handle.await {
                all_results.extend(user_results);
            }
        }

        let test_duration = start_time.elapsed();
        self.analyze_results(all_results, test_duration)
    }

    fn analyze_results(&self, results: Vec<RequestResult>, test_duration: Duration) -> LoadTestResult {
        let total_requests = results.len();
        let successful_requests = results.iter().filter(|r| r.success).count();
        let failed_requests = total_requests - successful_requests;

        let response_times: Vec<Duration> = results.iter().map(|r| r.response_time).collect();
        
        let average_response_time = if !response_times.is_empty() {
            response_times.iter().sum::<Duration>() / response_times.len() as u32
        } else {
            Duration::from_millis(0)
        };

        let min_response_time = response_times.iter().min().copied().unwrap_or_default();
        let max_response_time = response_times.iter().max().copied().unwrap_or_default();

        let requests_per_second = total_requests as f64 / test_duration.as_secs_f64();
        let error_rate = failed_requests as f64 / total_requests as f64 * 100.0;

        let response_time_percentiles = self.calculate_percentiles(&response_times);

        LoadTestResult {
            total_requests,
            successful_requests,
            failed_requests,
            average_response_time,
            min_response_time,
            max_response_time,
            requests_per_second,
            error_rate,
            response_time_percentiles,
        }
    }

    fn calculate_percentiles(&self, mut response_times: &Vec<Duration>) -> ResponseTimePercentiles {
        let mut sorted_times = response_times.clone();
        sorted_times.sort();

        let len = sorted_times.len();
        if len == 0 {
            return ResponseTimePercentiles {
                p50: Duration::from_millis(0),
                p90: Duration::from_millis(0),
                p95: Duration::from_millis(0),
                p99: Duration::from_millis(0),
            };
        }

        ResponseTimePercentiles {
            p50: sorted_times[len * 50 / 100],
            p90: sorted_times[len * 90 / 100],
            p95: sorted_times[len * 95 / 100],
            p99: sorted_times[len * 99 / 100],
        }
    }
}

async fn simulate_user_session(client: Client, config: LoadTestConfig, user_id: usize) -> Vec<RequestResult> {
    let mut results = Vec::new();

    for request_id in 0..config.requests_per_user {
        // Simulate different types of requests
        let result = match request_id % 5 {
            0 => make_health_check_request(&client, &config.base_url).await,
            1 => make_restaurant_list_request(&client, &config.base_url).await,
            2 => make_restaurant_search_request(&client, &config.base_url).await,
            3 => make_delivery_zones_request(&client, &config.base_url).await,
            4 => make_india_config_request(&client, &config.base_url).await,
            _ => make_health_check_request(&client, &config.base_url).await,
        };

        results.push(result);

        // Add some delay between requests to simulate real user behavior
        sleep(Duration::from_millis(100 + (user_id % 500) as u64)).await;
    }

    results
}

async fn make_health_check_request(client: &Client, base_url: &str) -> RequestResult {
    let start_time = Instant::now();
    
    match client.get(&format!("{}/health", base_url)).send().await {
        Ok(response) => {
            let response_time = start_time.elapsed();
            let status_code = response.status().as_u16();
            let success = response.status().is_success();

            RequestResult {
                success,
                response_time,
                status_code: Some(status_code),
                error: None,
            }
        }
        Err(e) => RequestResult {
            success: false,
            response_time: start_time.elapsed(),
            status_code: None,
            error: Some(e.to_string()),
        },
    }
}

async fn make_restaurant_list_request(client: &Client, base_url: &str) -> RequestResult {
    let start_time = Instant::now();
    
    match client.get(&format!("{}/restaurants", base_url)).send().await {
        Ok(response) => {
            let response_time = start_time.elapsed();
            let status_code = response.status().as_u16();
            let success = response.status().is_success();

            RequestResult {
                success,
                response_time,
                status_code: Some(status_code),
                error: None,
            }
        }
        Err(e) => RequestResult {
            success: false,
            response_time: start_time.elapsed(),
            status_code: None,
            error: Some(e.to_string()),
        },
    }
}

async fn make_restaurant_search_request(client: &Client, base_url: &str) -> RequestResult {
    let start_time = Instant::now();
    
    match client.get(&format!("{}/restaurants/search?q=pizza", base_url)).send().await {
        Ok(response) => {
            let response_time = start_time.elapsed();
            let status_code = response.status().as_u16();
            let success = response.status().is_success();

            RequestResult {
                success,
                response_time,
                status_code: Some(status_code),
                error: None,
            }
        }
        Err(e) => RequestResult {
            success: false,
            response_time: start_time.elapsed(),
            status_code: None,
            error: Some(e.to_string()),
        },
    }
}

async fn make_delivery_zones_request(client: &Client, base_url: &str) -> RequestResult {
    let start_time = Instant::now();
    
    match client.get(&format!("{}/delivery/zones", base_url)).send().await {
        Ok(response) => {
            let response_time = start_time.elapsed();
            let status_code = response.status().as_u16();
            let success = response.status().is_success();

            RequestResult {
                success,
                response_time,
                status_code: Some(status_code),
                error: None,
            }
        }
        Err(e) => RequestResult {
            success: false,
            response_time: start_time.elapsed(),
            status_code: None,
            error: Some(e.to_string()),
        },
    }
}

async fn make_india_config_request(client: &Client, base_url: &str) -> RequestResult {
    let start_time = Instant::now();
    
    match client.get(&format!("{}/india/config", base_url)).send().await {
        Ok(response) => {
            let response_time = start_time.elapsed();
            let status_code = response.status().as_u16();
            let success = response.status().is_success();

            RequestResult {
                success,
                response_time,
                status_code: Some(status_code),
                error: None,
            }
        }
        Err(e) => RequestResult {
            success: false,
            response_time: start_time.elapsed(),
            status_code: None,
            error: Some(e.to_string()),
        },
    }
}

#[tokio::test]
async fn test_basic_load() {
    let config = LoadTestConfig {
        concurrent_users: 10,
        requests_per_user: 5,
        test_duration: Duration::from_secs(30),
        ..Default::default()
    };

    let load_tester = LoadTester::new(config);
    let result = load_tester.run_load_test().await;

    println!("📊 Load Test Results:");
    println!("   Total Requests: {}", result.total_requests);
    println!("   Successful: {}", result.successful_requests);
    println!("   Failed: {}", result.failed_requests);
    println!("   Error Rate: {:.2}%", result.error_rate);
    println!("   Requests/sec: {:.2}", result.requests_per_second);
    println!("   Avg Response Time: {:?}", result.average_response_time);
    println!("   P95 Response Time: {:?}", result.response_time_percentiles.p95);

    // Basic assertions
    assert!(result.error_rate < 5.0, "Error rate should be less than 5%");
    assert!(result.average_response_time < Duration::from_millis(1000), "Average response time should be less than 1s");
}

#[tokio::test]
async fn test_stress_load() {
    let config = LoadTestConfig {
        concurrent_users: 50,
        requests_per_user: 20,
        test_duration: Duration::from_secs(60),
        ..Default::default()
    };

    let load_tester = LoadTester::new(config);
    let result = load_tester.run_load_test().await;

    println!("🔥 Stress Test Results:");
    println!("   Total Requests: {}", result.total_requests);
    println!("   Successful: {}", result.successful_requests);
    println!("   Failed: {}", result.failed_requests);
    println!("   Error Rate: {:.2}%", result.error_rate);
    println!("   Requests/sec: {:.2}", result.requests_per_second);
    println!("   Avg Response Time: {:?}", result.average_response_time);
    println!("   P99 Response Time: {:?}", result.response_time_percentiles.p99);

    // Stress test assertions (more lenient)
    assert!(result.error_rate < 10.0, "Error rate should be less than 10% under stress");
    assert!(result.requests_per_second > 10.0, "Should handle at least 10 requests per second");
}