use axum_test::TestServer;
use serde_json::Value;
use server::routes::{create_routes, AppState};
use server::auth::firebase::FirebaseAuth;
use server::config::Config;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn create_test_server() -> TestServer {
    let config = Config::from_env().unwrap_or_default();
    let firebase_auth = Arc::new(Mutex::new(FirebaseAuth::new(&config)));
    
    let app_state = AppState {
        fcm_service: Arc::new(Mutex::new(
            server::notifications::fcm::FCMService::new(&config).unwrap()
        )),
        database: server::database::Database::new(
            sqlx::PgPool::connect("postgresql://localhost/test").await.unwrap()
        ),
        websocket_manager: server::websocket::WebSocketManager::new(),
        metrics: server::metrics::MetricsCollector::new().unwrap(),
    };
    
    let app = create_routes(firebase_auth, app_state);
    TestServer::new(app).unwrap()
}

#[tokio::test]
async fn test_india_cities_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/cities").await;
    assert_eq!(response.status_code(), 200);
    
    let cities: Value = response.json();
    assert!(cities.is_array());
    
    let cities_array = cities.as_array().unwrap();
    assert!(!cities_array.is_empty());
    
    // Check if Mumbai is in the list
    let mumbai_exists = cities_array.iter().any(|city| {
        city["name"].as_str() == Some("Mumbai")
    });
    assert!(mumbai_exists, "Mumbai should be in the supported cities list");
}

#[tokio::test]
async fn test_india_states_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/states").await;
    assert_eq!(response.status_code(), 200);
    
    let states: Value = response.json();
    assert!(states.is_array());
    
    let states_array = states.as_array().unwrap();
    assert!(!states_array.is_empty());
    
    // Check if Maharashtra is in the list
    let maharashtra_exists = states_array.iter().any(|state| {
        state["name"].as_str() == Some("Maharashtra") && 
        state["code"].as_str() == Some("MH")
    });
    assert!(maharashtra_exists, "Maharashtra should be in the states list");
}

#[tokio::test]
async fn test_india_cuisines_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/cuisines").await;
    assert_eq!(response.status_code(), 200);
    
    let cuisines: Value = response.json();
    assert!(cuisines.is_array());
    
    let cuisines_array = cuisines.as_array().unwrap();
    assert!(!cuisines_array.is_empty());
    
    // Check if North Indian cuisine is in the list
    let north_indian_exists = cuisines_array.iter().any(|cuisine| {
        cuisine["name"].as_str() == Some("North Indian")
    });
    assert!(north_indian_exists, "North Indian should be in the cuisines list");
}

#[tokio::test]
async fn test_gst_rates_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/gst-rates").await;
    assert_eq!(response.status_code(), 200);
    
    let gst_rates: Value = response.json();
    assert!(gst_rates.is_array());
    
    let rates_array = gst_rates.as_array().unwrap();
    assert!(!rates_array.is_empty());
    
    // Check if restaurant service GST rate exists
    let restaurant_gst_exists = rates_array.iter().any(|rate| {
        rate["category"].as_str() == Some("Restaurant Service") &&
        rate["rate"].as_f64() == Some(5.0)
    });
    assert!(restaurant_gst_exists, "Restaurant Service GST rate should be 5%");
}

#[tokio::test]
async fn test_gst_calculation() {
    let server = create_test_server().await;
    
    let response = server
        .get("/india/calculate-gst?amount=100&category=Restaurant Service")
        .await;
    assert_eq!(response.status_code(), 200);
    
    let calculation: Value = response.json();
    
    assert_eq!(calculation["base_amount"].as_f64(), Some(100.0));
    assert_eq!(calculation["gst_rate"].as_f64(), Some(5.0));
    assert_eq!(calculation["gst_amount"].as_f64(), Some(5.0));
    assert_eq!(calculation["total_amount"].as_f64(), Some(105.0));
    assert_eq!(calculation["category"].as_str(), Some("Restaurant Service"));
}

#[tokio::test]
async fn test_upi_apps_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/upi-apps").await;
    assert_eq!(response.status_code(), 200);
    
    let upi_apps: Value = response.json();
    assert!(upi_apps.is_array());
    
    let apps_array = upi_apps.as_array().unwrap();
    assert!(!apps_array.is_empty());
    
    // Check if Google Pay is in the list
    let google_pay_exists = apps_array.iter().any(|app| {
        app["name"].as_str() == Some("Google Pay")
    });
    assert!(google_pay_exists, "Google Pay should be in the UPI apps list");
    
    // Check if PhonePe is in the list
    let phonepe_exists = apps_array.iter().any(|app| {
        app["name"].as_str() == Some("PhonePe")
    });
    assert!(phonepe_exists, "PhonePe should be in the UPI apps list");
}

#[tokio::test]
async fn test_indian_banks_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/banks").await;
    assert_eq!(response.status_code(), 200);
    
    let banks: Value = response.json();
    assert!(banks.is_array());
    
    let banks_array = banks.as_array().unwrap();
    assert!(!banks_array.is_empty());
    
    // Check if SBI is in the list
    let sbi_exists = banks_array.iter().any(|bank| {
        bank["name"].as_str() == Some("State Bank of India") &&
        bank["code"].as_str() == Some("SBIN")
    });
    assert!(sbi_exists, "State Bank of India should be in the banks list");
}

#[tokio::test]
async fn test_payment_fees_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/payment-fees").await;
    assert_eq!(response.status_code(), 200);
    
    let fees: Value = response.json();
    assert!(fees.is_array());
    
    let fees_array = fees.as_array().unwrap();
    assert!(!fees_array.is_empty());
    
    // Check if UPI has zero fees
    let upi_fee_exists = fees_array.iter().any(|fee| {
        fee["payment_method"].as_str() == Some("UPI") &&
        fee["percentage_fee"].as_f64() == Some(0.0) &&
        fee["fixed_fee"].as_f64() == Some(0.0)
    });
    assert!(upi_fee_exists, "UPI should have zero fees");
}

#[tokio::test]
async fn test_delivery_zones_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/delivery-zones").await;
    assert_eq!(response.status_code(), 200);
    
    let zones: Value = response.json();
    assert!(zones.is_array());
    
    let zones_array = zones.as_array().unwrap();
    assert!(!zones_array.is_empty());
    
    // Check if Metro Cities zone exists
    let metro_zone_exists = zones_array.iter().any(|zone| {
        zone["zone_name"].as_str() == Some("Metro Cities") &&
        zone["standard_delivery_time"].as_u64() == Some(30)
    });
    assert!(metro_zone_exists, "Metro Cities zone should exist with 30 min delivery");
}

#[tokio::test]
async fn test_india_config_endpoint() {
    let server = create_test_server().await;
    
    let response = server.get("/india/config").await;
    assert_eq!(response.status_code(), 200);
    
    let config: Value = response.json();
    
    assert_eq!(config["default_currency"].as_str(), Some("INR"));
    assert_eq!(config["default_timezone"].as_str(), Some("Asia/Kolkata"));
    assert_eq!(config["default_language"].as_str(), Some("en-IN"));
    assert_eq!(config["gst_enabled"].as_bool(), Some(true));
    assert_eq!(config["minimum_order_amount"].as_f64(), Some(99.0));
    assert_eq!(config["delivery_fee"].as_f64(), Some(29.0));
    assert_eq!(config["free_delivery_above"].as_f64(), Some(299.0));
}

#[tokio::test]
async fn test_delivery_time_calculation() {
    let server = create_test_server().await;
    
    // Test Mumbai delivery time during peak hours
    let response = server
        .get("/india/delivery-time?city=Mumbai&is_peak_hour=true&is_weekend=false")
        .await;
    assert_eq!(response.status_code(), 200);
    
    let delivery_time: Value = response.json();
    
    assert_eq!(delivery_time["city"].as_str(), Some("Mumbai"));
    assert_eq!(delivery_time["zone_name"].as_str(), Some("Metro Cities"));
    
    // During peak hours, delivery time should be higher than 30 minutes
    let estimated_time = delivery_time["estimated_delivery_minutes"].as_u64().unwrap();
    assert!(estimated_time > 30, "Peak hour delivery should take longer than 30 minutes");
    
    // Check if peak hour factor is applied
    let factors = delivery_time["factors_applied"].as_array().unwrap();
    let has_peak_factor = factors.iter().any(|factor| {
        factor.as_str() == Some("Peak Hour Multiplier")
    });
    assert!(has_peak_factor, "Peak hour multiplier should be applied");
}

#[tokio::test]
async fn test_localization_config() {
    let server = create_test_server().await;
    
    let response = server.get("/india/localization").await;
    assert_eq!(response.status_code(), 200);
    
    let localization: Value = response.json();
    
    assert_eq!(localization["default_language"].as_str(), Some("English"));
    assert_eq!(localization["currency_format"]["symbol"].as_str(), Some("₹"));
    assert_eq!(localization["currency_format"]["position"].as_str(), Some("Before"));
    assert_eq!(localization["date_format"].as_str(), Some("DD/MM/YYYY"));
    assert_eq!(localization["time_format"].as_str(), Some("HH:mm"));
    
    // Check if supported languages include major Indian languages
    let supported_languages = localization["supported_languages"].as_array().unwrap();
    let has_hindi = supported_languages.iter().any(|lang| {
        lang.as_str() == Some("Hindi")
    });
    assert!(has_hindi, "Hindi should be in supported languages");
}

#[tokio::test]
async fn test_comprehensive_india_workflow() {
    let server = create_test_server().await;
    
    // 1. Get supported cities
    let cities_response = server.get("/india/cities").await;
    assert_eq!(cities_response.status_code(), 200);
    
    // 2. Get cuisines
    let cuisines_response = server.get("/india/cuisines").await;
    assert_eq!(cuisines_response.status_code(), 200);
    
    // 3. Calculate GST for an order
    let gst_response = server
        .get("/india/calculate-gst?amount=250&category=Restaurant Service")
        .await;
    assert_eq!(gst_response.status_code(), 200);
    
    let gst_calculation: Value = gst_response.json();
    assert_eq!(gst_calculation["total_amount"].as_f64(), Some(262.5)); // 250 + 5% GST
    
    // 4. Get delivery time for Mumbai
    let delivery_response = server
        .get("/india/delivery-time?city=Mumbai&is_peak_hour=false&is_weekend=false")
        .await;
    assert_eq!(delivery_response.status_code(), 200);
    
    let delivery_time: Value = delivery_response.json();
    assert_eq!(delivery_time["estimated_delivery_minutes"].as_u64(), Some(30));
    
    // 5. Get UPI apps for payment
    let upi_response = server.get("/india/upi-apps").await;
    assert_eq!(upi_response.status_code(), 200);
    
    let upi_apps: Value = upi_response.json();
    assert!(upi_apps.as_array().unwrap().len() >= 5); // Should have at least 5 UPI apps
}

#[tokio::test]
async fn test_error_handling_for_invalid_city() {
    let server = create_test_server().await;
    
    // Test with a city not in our supported list
    let response = server
        .get("/india/delivery-time?city=UnknownCity&is_peak_hour=false")
        .await;
    assert_eq!(response.status_code(), 200); // Should still work, defaulting to Tier 2
    
    let delivery_time: Value = response.json();
    assert_eq!(delivery_time["city"].as_str(), Some("UnknownCity"));
    assert_eq!(delivery_time["zone_name"].as_str(), Some("Tier 2 Cities"));
    assert_eq!(delivery_time["estimated_delivery_minutes"].as_u64(), Some(40)); // Tier 2 default
}