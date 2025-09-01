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
        })
    }
}