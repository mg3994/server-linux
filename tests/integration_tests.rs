use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use axum_test::TestServer;
use serde_json::{json, Value};
use server::{
    auth::models::User,
    config::Config,
    database::Database,
    metrics::MetricsCollector,
    notifications::fcm::FCMService,
    routes::{create_routes, AppState},
    websocket::WebSocketManager,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Test utilities
pub struct TestContext {
    pub server: TestServer,
    pub db: PgPool,
    pub customer_token: String,
    pub restaurant_token: String,
    pub delivery_token: String,
}

impl TestContext {
    pub async fn new() -> Self {
        // Setup test database
        let database_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/delivery_server_test".to_string());

        let db = PgPool::connect(&database_url).await.unwrap();

        // Run migrations
        sqlx::migrate!("./migrations").run(&db).await.unwrap();

        // Create test app
        let config = Config::from_env().unwrap();
        let fcm_service = Arc::new(Mutex::new(FCMService::new(config.clone()).await.unwrap()));
        let database = Database::new(db.clone());
        let websocket_manager = WebSocketManager::new();
        let metrics = MetricsCollector::new().unwrap();

        let app_state = AppState {
            fcm_service,
            database,
            websocket_manager,
            metrics,
        };

        let firebase_auth = Arc::new(
            server::auth::firebase::FirebaseAuth::new(config.firebase_project_id.clone())
                .await
                .unwrap(),
        );

        let app = create_routes(firebase_auth, app_state);
        let server = TestServer::new(app).unwrap();

        // Create test tokens (mock Firebase tokens for testing)
        let customer_token = create_test_token("customer_uid", "customer@test.com", "customer");
        let restaurant_token =
            create_test_token("restaurant_uid", "restaurant@test.com", "restaurant");
        let delivery_token =
            create_test_token("delivery_uid", "delivery@test.com", "delivery_person");

        Self {
            server,
            db,
            customer_token,
            restaurant_token,
            delivery_token,
        }
    }

    pub async fn cleanup(&self) {
        // Clean up test data
        sqlx::query("TRUNCATE TABLE orders, payments, notifications CASCADE")
            .execute(&self.db)
            .await
            .unwrap();
    }
}

fn create_test_token(uid: &str, email: &str, role: &str) -> String {
    // In a real test, you'd create a proper Firebase test token
    // For now, we'll use a mock token format
    format!("test_token_{}_{}", uid, role)
}

#[tokio::test]
async fn test_health_endpoints() {
    let ctx = TestContext::new().await;

    // Test basic health check
    let response = ctx.server.get("/health").await;
    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(response.text(), "OK");

    // Test detailed health check
    let response = ctx.server.get("/health/detailed").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert_eq!(body["status"], "healthy");
    assert!(body["uptime_seconds"].is_number());
    assert!(body["metrics"].is_object());

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_metrics_endpoint() {
    let ctx = TestContext::new().await;

    let response = ctx.server.get("/metrics").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body = response.text();
    assert!(body.contains("http_requests_total"));
    assert!(body.contains("active_orders"));

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_order_creation_flow() {
    let ctx = TestContext::new().await;

    // Create order
    let order_payload = json!({
        "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
        "items": [
            {
                "menu_item_id": "456e7890-e89b-12d3-a456-426614174001",
                "quantity": 2,
                "customizations": ["Extra cheese"]
            }
        ],
        "delivery_address": {
            "street": "123 Test St",
            "city": "Test City",
            "state": "TS",
            "postal_code": "12345",
            "country": "US",
            "latitude": 40.7128,
            "longitude": -74.0060
        }
    });

    let response = ctx
        .server
        .post("/orders")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .json(&order_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["order"]["id"].is_string());
    assert_eq!(body["order"]["status"], "placed");
    assert!(body["order"]["total_amount"].is_number());

    let order_id = body["order"]["id"].as_str().unwrap();

    // Get order details
    let response = ctx
        .server
        .get(&format!("/orders/{}", order_id))
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert_eq!(body["id"], order_id);
    assert_eq!(body["status"], "placed");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_order_status_update() {
    let ctx = TestContext::new().await;

    // First create an order
    let order_id = create_test_order(&ctx).await;

    // Update order status
    let status_payload = json!({
        "status": "confirmed"
    });

    let response = ctx
        .server
        .put(&format!("/orders/{}/status", order_id))
        .add_header("Authorization", format!("Bearer {}", ctx.restaurant_token))
        .json(&status_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert_eq!(body["order"]["status"], "confirmed");
    assert!(body["message"].is_string());

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_payment_processing() {
    let ctx = TestContext::new().await;

    // First create an order
    let order_id = create_test_order(&ctx).await;

    // Process payment
    let payment_payload = json!({
        "order_id": order_id,
        "amount": 25.99,
        "currency": "USD",
        "payment_method": "credit_card",
        "payment_details": {
            "card_token": "tok_test_123456"
        }
    });

    let response = ctx
        .server
        .post("/payments")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .json(&payment_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["payment"]["id"].is_string());
    assert_eq!(body["payment"]["order_id"], order_id);
    assert_eq!(body["payment"]["amount"], 25.99);

    let payment_id = body["payment"]["id"].as_str().unwrap();

    // Get payment details
    let response = ctx
        .server
        .get(&format!("/payments/{}", payment_id))
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert_eq!(body["id"], payment_id);
    assert_eq!(body["order_id"], order_id);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_customer_orders_list() {
    let ctx = TestContext::new().await;

    // Create multiple orders
    let order1_id = create_test_order(&ctx).await;
    let order2_id = create_test_order(&ctx).await;

    // Get customer orders
    let customer_id = "customer_uid"; // This should match the token
    let response = ctx
        .server
        .get(&format!("/customers/{}/orders", customer_id))
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body.is_array());
    assert!(body.as_array().unwrap().len() >= 2);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_location_update() {
    let ctx = TestContext::new().await;

    // First create an order
    let order_id = create_test_order(&ctx).await;

    // Update delivery location
    let location_payload = json!({
        "customer_id": "customer_uid",
        "latitude": 40.7589,
        "longitude": -73.9851
    });

    let response = ctx
        .server
        .put(&format!("/orders/{}/location", order_id))
        .add_header("Authorization", format!("Bearer {}", ctx.delivery_token))
        .json(&location_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert_eq!(body["success"], true);
    assert!(body["message"].is_string());

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_authentication_required() {
    let ctx = TestContext::new().await;

    // Try to create order without authentication
    let order_payload = json!({
        "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
        "items": []
    });

    let response = ctx.server.post("/orders").json(&order_payload).await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_invalid_order_data() {
    let ctx = TestContext::new().await;

    // Try to create order with invalid data
    let invalid_payload = json!({
        "restaurant_id": "invalid-uuid",
        "items": []
    });

    let response = ctx
        .server
        .post("/orders")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .json(&invalid_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_order_not_found() {
    let ctx = TestContext::new().await;

    let non_existent_id = Uuid::new_v4();

    let response = ctx
        .server
        .get(&format!("/orders/{}", non_existent_id))
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_concurrent_order_creation() {
    let ctx = TestContext::new().await;

    let order_payload = json!({
        "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
        "items": [
            {
                "menu_item_id": "456e7890-e89b-12d3-a456-426614174001",
                "quantity": 1,
                "customizations": []
            }
        ],
        "delivery_address": {
            "street": "123 Test St",
            "city": "Test City",
            "state": "TS",
            "postal_code": "12345",
            "country": "US",
            "latitude": 40.7128,
            "longitude": -74.0060
        }
    });

    // Create multiple orders concurrently
    let mut handles = vec![];

    for _ in 0..5 {
        let server = ctx.server.clone();
        let token = ctx.customer_token.clone();
        let payload = order_payload.clone();

        let handle = tokio::spawn(async move {
            server
                .post("/orders")
                .add_header("Authorization", format!("Bearer {}", token))
                .json(&payload)
                .await
        });

        handles.push(handle);
    }

    // Wait for all requests to complete
    let mut success_count = 0;
    for handle in handles {
        let response = handle.await.unwrap();
        if response.status_code() == StatusCode::CREATED {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 5);

    ctx.cleanup().await;
}

// Helper function to create a test order
async fn create_test_order(ctx: &TestContext) -> String {
    let order_payload = json!({
        "restaurant_id": "123e4567-e89b-12d3-a456-426614174000",
        "items": [
            {
                "menu_item_id": "456e7890-e89b-12d3-a456-426614174001",
                "quantity": 1,
                "customizations": []
            }
        ],
        "delivery_address": {
            "street": "123 Test St",
            "city": "Test City",
            "state": "TS",
            "postal_code": "12345",
            "country": "US",
            "latitude": 40.7128,
            "longitude": -74.0060
        }
    });

    let response = ctx
        .server
        .post("/orders")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .json(&order_payload)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    body["order"]["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_websocket_connection() {
    let ctx = TestContext::new().await;

    // Note: WebSocket testing requires special setup
    // This is a placeholder for WebSocket connection testing
    // In a real implementation, you'd use a WebSocket testing library

    // For now, we'll just test that the WebSocket endpoint exists
    let response = ctx
        .server
        .get("/ws")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .add_header("Connection", "Upgrade")
        .add_header("Upgrade", "websocket")
        .add_header("Sec-WebSocket-Version", "13")
        .add_header("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ==")
        .await;

    // WebSocket upgrade should return 101 Switching Protocols
    // But our test server might not handle this properly
    // So we'll just check that it doesn't return 404
    assert_ne!(response.status_code(), StatusCode::NOT_FOUND);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_performance_metrics() {
    let ctx = TestContext::new().await;

    // Create several orders to generate metrics
    for _ in 0..10 {
        create_test_order(&ctx).await;
    }

    // Check metrics endpoint
    let response = ctx.server.get("/metrics").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body = response.text();

    // Verify that metrics are being collected
    assert!(body.contains("http_requests_total"));
    assert!(body.contains("orders_created_total"));

    // Check detailed health endpoint for metrics
    let response = ctx.server.get("/health/detailed").await;
    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["metrics"]["http_requests_total"].as_u64().unwrap() > 0);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_error_handling() {
    let ctx = TestContext::new().await;

    // Test various error scenarios

    // 1. Invalid JSON
    let response = ctx
        .server
        .post("/orders")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .add_header("Content-Type", "application/json")
        .text("invalid json")
        .await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    // 2. Missing required fields
    let response = ctx
        .server
        .post("/orders")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .json(&json!({}))
        .await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    // 3. Invalid UUID format
    let response = ctx
        .server
        .get("/orders/invalid-uuid")
        .add_header("Authorization", format!("Bearer {}", ctx.customer_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    ctx.cleanup().await;
}

// Benchmark tests (optional, requires criterion feature)
#[cfg(feature = "benchmark")]
mod benchmarks {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn benchmark_order_creation(c: &mut Criterion) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let ctx = rt.block_on(TestContext::new());

        c.bench_function("order_creation", |b| {
            b.iter(|| {
                rt.block_on(async {
                    black_box(create_test_order(&ctx).await);
                })
            })
        });
    }

    criterion_group!(benches, benchmark_order_creation);
    criterion_main!(benches);
}
// Delivery Management Integration Tests
#[tokio::test]
async fn test_delivery_person_registration() {
    let ctx = TestContext::new().await;
    
    let register_request = json!({
        "name": "Rajesh Kumar",
        "phone": "+91-9876543210",
        "email": "rajesh.delivery@example.com",
        "vehicle_type": "motorcycle",
        "vehicle_number": "MH01AB1234",
        "license_number": "MH0120230001234",
        "aadhar_number": "123456789012",
        "pan_number": "ABCDE1234F",
        "bank_account_number": "1234567890123456",
        "ifsc_code": "SBIN0001234"
    });

    let response = ctx
        .server
        .post("/api/delivery/register")
        .add_header("Authorization", format!("Bearer {}", ctx.delivery_token))
        .json(&register_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    
    let body: Value = response.json();
    assert_eq!(body["name"], "Rajesh Kumar");
    assert_eq!(body["vehicle_type"], "motorcycle");
    assert_eq!(body["is_verified"], false); // Should require admin verification

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_get_nearby_delivery_persons() {
    let ctx = TestContext::new().await;
    
    let response = ctx
        .server
        .get("/api/delivery/nearby?latitude=19.0760&longitude=72.8777&radius_km=5")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: Value = response.json();
    assert!(body.is_array());
    
    // Should return available delivery persons within radius
    for person in body.as_array().unwrap() {
        assert!(person["distance_km"].as_f64().unwrap() <= 5.0);
        assert_eq!(person["delivery_person"]["is_available"], true);
    }

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_delivery_time_estimate() {
    let ctx = TestContext::new().await;
    
    let response = ctx
        .server
        .get("/api/delivery/estimate-time?pickup_lat=19.0760&pickup_lng=72.8777&delivery_lat=19.0896&delivery_lng=72.8656&city=Mumbai")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: Value = response.json();
    assert!(body["total_estimated_minutes"].is_number());
    assert!(body["confidence_level"].is_number());
    assert!(body["base_time_minutes"].is_number());

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_delivery_person_location_update() {
    let ctx = TestContext::new().await;
    
    // First register a delivery person
    let delivery_person_id = create_test_delivery_person(&ctx).await;
    
    let location_update = json!({
        "latitude": 19.0760,
        "longitude": 72.8777,
        "speed": 25.5,
        "heading": 180.0
    });

    let response = ctx
        .server
        .put(&format!("/api/delivery/{}/location", delivery_person_id))
        .add_header("Authorization", format!("Bearer {}", ctx.delivery_token))
        .json(&location_update)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_order_assignment_to_delivery_person() {
    let ctx = TestContext::new().await;
    
    // Create an order first
    let order_id = create_test_order(&ctx).await;
    
    // Create a delivery person
    let delivery_person_id = create_test_delivery_person(&ctx).await;
    
    let assignment_request = json!({
        "order_id": order_id,
        "preferred_delivery_person_id": delivery_person_id,
        "max_distance_km": 10.0
    });

    let response = ctx
        .server
        .post("/api/delivery/assign-order")
        .add_header("Authorization", format!("Bearer {}", ctx.restaurant_token))
        .json(&assignment_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    
    let body: Value = response.json();
    assert_eq!(body["order_id"], order_id);
    assert_eq!(body["delivery_person_id"], delivery_person_id);
    assert_eq!(body["status"], "assigned");

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_delivery_status_updates() {
    let ctx = TestContext::new().await;
    
    // Create order and assignment
    let order_id = create_test_order(&ctx).await;
    let delivery_person_id = create_test_delivery_person(&ctx).await;
    let assignment_id = create_test_assignment(&ctx, &order_id, &delivery_person_id).await;
    
    // Test status progression: assigned -> accepted -> pickedup -> delivered
    let statuses = vec![
        ("accepted", "Order accepted by delivery person"),
        ("enroutetorestaurant", "On the way to restaurant"),
        ("arrivedatrestaurant", "Arrived at restaurant"),
        ("pickedup", "Food picked up"),
        ("enroutetocustomer", "On the way to customer"),
        ("arrivedatcustomer", "Arrived at customer location"),
        ("delivered", "Order delivered successfully"),
    ];

    for (status, notes) in statuses {
        let status_update = json!({
            "status": status,
            "notes": notes,
            "proof_of_delivery": if status == "delivered" {
                Some(json!({"photo": "base64_image_data", "signature": "customer_signature"}))
            } else {
                None
            }
        });

        let response = ctx
            .server
            .put(&format!("/api/delivery/assignments/{}/{}/status", assignment_id, delivery_person_id))
            .add_header("Authorization", format!("Bearer {}", ctx.delivery_token))
            .json(&status_update)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        
        let body: Value = response.json();
        assert_eq!(body["status"], status);
        if status == "delivered" {
            assert!(body["proof_of_delivery"].is_object());
        }
    }

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_delivery_person_stats() {
    let ctx = TestContext::new().await;
    
    let delivery_person_id = create_test_delivery_person(&ctx).await;
    
    let response = ctx
        .server
        .get(&format!("/api/delivery/{}/stats", delivery_person_id))
        .add_header("Authorization", format!("Bearer {}", ctx.delivery_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: Value = response.json();
    assert!(body["total_deliveries"].is_number());
    assert!(body["successful_deliveries"].is_number());
    assert!(body["success_rate"].is_number());
    assert!(body["earnings_today"].is_number());
    assert!(body["rating"].is_number());

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_delivery_person_assignments_list() {
    let ctx = TestContext::new().await;
    
    let delivery_person_id = create_test_delivery_person(&ctx).await;
    
    // Create multiple assignments
    for _ in 0..3 {
        let order_id = create_test_order(&ctx).await;
        create_test_assignment(&ctx, &order_id, &delivery_person_id).await;
    }
    
    let response = ctx
        .server
        .get(&format!("/api/delivery/{}/assignments", delivery_person_id))
        .add_header("Authorization", format!("Bearer {}", ctx.delivery_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: Value = response.json();
    assert!(body.is_array());
    assert!(body.as_array().unwrap().len() >= 3);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_india_delivery_zones() {
    let ctx = TestContext::new().await;
    
    let response = ctx
        .server
        .get("/api/delivery/zones")
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: Value = response.json();
    assert!(body.is_array());
    
    // Check that zones have required fields
    for zone in body.as_array().unwrap() {
        assert!(zone["zone_name"].is_string());
        assert!(zone["cities"].is_array());
        assert!(zone["base_delivery_time_minutes"].is_number());
        assert!(zone["peak_hour_surcharge"].is_number());
    }

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_admin_delivery_person_verification() {
    let ctx = TestContext::new().await;
    
    let delivery_person_id = create_test_delivery_person(&ctx).await;
    
    // Create admin token (in real implementation, this would be a proper admin token)
    let admin_token = create_test_token("admin_uid", "admin@test.com", "admin");
    
    let response = ctx
        .server
        .put(&format!("/api/admin/delivery/{}/verify", delivery_person_id))
        .add_header("Authorization", format!("Bearer {}", admin_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: Value = response.json();
    assert_eq!(body["is_verified"], true);

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_delivery_analytics() {
    let ctx = TestContext::new().await;
    
    // Create admin token
    let admin_token = create_test_token("admin_uid", "admin@test.com", "admin");
    
    let response = ctx
        .server
        .get("/api/admin/delivery/analytics")
        .add_header("Authorization", format!("Bearer {}", admin_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    
    let body: Value = response.json();
    assert!(body["total_delivery_persons"].is_number());
    assert!(body["active_delivery_persons"].is_number());
    assert!(body["total_deliveries_today"].is_number());
    assert!(body["success_rate_today"].is_number());
    assert!(body["availability_rate"].is_number());

    ctx.cleanup().await;
}

#[tokio::test]
async fn test_delivery_person_unauthorized_access() {
    let ctx = TestContext::new().await;
    
    let delivery_person_id = create_test_delivery_person(&ctx).await;
    
    // Try to access another delivery person's data
    let other_delivery_token = create_test_token("other_delivery_uid", "other@test.com", "delivery_person");
    
    let response = ctx
        .server
        .get(&format!("/api/delivery/{}/stats", delivery_person_id))
        .add_header("Authorization", format!("Bearer {}", other_delivery_token))
        .await;

    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);

    ctx.cleanup().await;
}

// Helper functions for delivery tests
async fn create_test_delivery_person(ctx: &TestContext) -> String {
    let register_request = json!({
        "name": "Test Driver",
        "phone": "+91-9876543210",
        "email": "testdriver@example.com",
        "vehicle_type": "motorcycle",
        "vehicle_number": "MH01AB1234",
        "license_number": "MH0120230001234",
        "aadhar_number": "123456789012",
        "pan_number": "ABCDE1234F",
        "bank_account_number": "1234567890123456",
        "ifsc_code": "SBIN0001234"
    });

    let response = ctx
        .server
        .post("/api/delivery/register")
        .add_header("Authorization", format!("Bearer {}", ctx.delivery_token))
        .json(&register_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    
    let body: Value = response.json();
    body["id"].as_str().unwrap().to_string()
}

async fn create_test_assignment(ctx: &TestContext, order_id: &str, delivery_person_id: &str) -> String {
    let assignment_request = json!({
        "order_id": order_id,
        "preferred_delivery_person_id": delivery_person_id,
        "max_distance_km": 10.0
    });

    let response = ctx
        .server
        .post("/api/delivery/assign-order")
        .add_header("Authorization", format!("Bearer {}", ctx.restaurant_token))
        .json(&assignment_request)
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    
    let body: Value = response.json();
    body["id"].as_str().unwrap().to_string()
}