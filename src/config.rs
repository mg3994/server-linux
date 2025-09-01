use anyhow::Result;
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub firebase_project_id: String,
    pub firebase_service_account_key: String,
    pub tls_cert_path: String,
    pub tls_key_path: String,
    // FCM now uses service account authentication instead of server key
    
    // Delivery configuration
    pub delivery_assignment_timeout_seconds: u64,
    pub delivery_person_location_update_interval_seconds: u64,
    pub max_delivery_distance_km: f64,
    pub default_delivery_fee: f64,
    pub peak_hour_surcharge_percentage: f64,
    pub weekend_surcharge_percentage: f64,
    pub festival_surcharge_percentage: f64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        Ok(Config {
            server_host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8443".to_string())
                .parse()?,
            firebase_project_id: env::var("FIREBASE_PROJECT_ID")
                .expect("FIREBASE_PROJECT_ID must be set"),
            firebase_service_account_key: env::var("FIREBASE_SERVICE_ACCOUNT_KEY")
                .expect("FIREBASE_SERVICE_ACCOUNT_KEY must be set"),
            tls_cert_path: env::var("TLS_CERT_PATH")
                .unwrap_or_else(|_| "cert.pem".to_string()),
            tls_key_path: env::var("TLS_KEY_PATH")
                .unwrap_or_else(|_| "key.pem".to_string()),
            // FCM authentication is handled via service account key
            
            // Delivery configuration with defaults
            delivery_assignment_timeout_seconds: env::var("DELIVERY_ASSIGNMENT_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .unwrap_or(300),
            delivery_person_location_update_interval_seconds: env::var("DELIVERY_LOCATION_UPDATE_INTERVAL_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            max_delivery_distance_km: env::var("MAX_DELIVERY_DISTANCE_KM")
                .unwrap_or_else(|_| "20.0".to_string())
                .parse()
                .unwrap_or(20.0),
            default_delivery_fee: env::var("DEFAULT_DELIVERY_FEE")
                .unwrap_or_else(|_| "30.0".to_string())
                .parse()
                .unwrap_or(30.0),
            peak_hour_surcharge_percentage: env::var("PEAK_HOUR_SURCHARGE_PERCENTAGE")
                .unwrap_or_else(|_| "25.0".to_string())
                .parse()
                .unwrap_or(25.0),
            weekend_surcharge_percentage: env::var("WEEKEND_SURCHARGE_PERCENTAGE")
                .unwrap_or_else(|_| "15.0".to_string())
                .parse()
                .unwrap_or(15.0),
            festival_surcharge_percentage: env::var("FESTIVAL_SURCHARGE_PERCENTAGE")
                .unwrap_or_else(|_| "35.0".to_string())
                .parse()
                .unwrap_or(35.0),
        })
    }
}