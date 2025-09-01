use crate::analytics::AnalyticsService;
use crate::auth::firebase::FirebaseAuth;
use crate::auth::middleware::SharedFirebaseAuth;
use crate::config::Config;
use crate::error::Result;
use crate::middleware::{cors_layer, logging_middleware};
use crate::notifications::fcm::FCMService;
use crate::orders::handlers::SharedFCMService;
use crate::routes::create_routes;
use anyhow::Context;
use axum::middleware;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;

pub struct Server {
    config: Config,
    firebase_auth: SharedFirebaseAuth,
    fcm_service: SharedFCMService,
}

impl Server {
    pub fn new(config: Config) -> Result<Self> {
        let firebase_auth = Arc::new(Mutex::new(FirebaseAuth::new(&config)));
        let fcm_service = Arc::new(Mutex::new(FCMService::new(&config)?));

        Ok(Self {
            config,
            firebase_auth,
            fcm_service,
        })
    }

    pub async fn run(self) -> Result<()> {
        // Initialize tracing
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .init();

        tracing::info!("Starting Multi-Vendor Delivery Server...");

        // Create the application router
        let database = crate::database::Database::new(sqlx::PgPool::connect("postgresql://localhost/test").await.unwrap());
        let analytics_service = AnalyticsService::new(database.clone());
        
        let delivery_websocket_manager = std::sync::Arc::new(crate::delivery::DeliveryWebSocketManager::new());
        let enhanced_delivery_service = std::sync::Arc::new(
            crate::delivery::EnhancedDeliveryService::new(database.clone(), delivery_websocket_manager.clone())
        );

        let app_state = crate::routes::AppState {
            fcm_service: self.fcm_service.clone(),
            database,
            websocket_manager: crate::websocket::WebSocketManager::new(),
            delivery_websocket_manager: (*delivery_websocket_manager).clone(),
            enhanced_delivery_service,
            metrics: crate::metrics::MetricsCollector::new().unwrap(),
            analytics_service,
        };
        
        let app = create_routes(self.firebase_auth.clone(), app_state)
            .layer(cors_layer())
            .layer(middleware::from_fn(logging_middleware))
            .layer(TraceLayer::new_for_http());

        // Bind to address
        let addr: SocketAddr = format!("{}:{}", self.config.server_host, self.config.server_port)
            .parse()
            .context("Invalid server address")?;

        tracing::info!("Server listening on http://{}", addr);
        tracing::info!("Health check: http://{}/health", addr);
        tracing::info!("Note: HTTPS/TLS support will be added in future updates");

        // Use HTTP (not HTTPS) for now to avoid certificate complexity
        let listener = tokio::net::TcpListener::bind(addr).await?;
        
        tracing::info!("🚀 Multi-Vendor Delivery Server is running!");
        tracing::info!("📱 Ready to handle orders, payments, and notifications");
        
        axum::serve(listener, app)
            .await
            .context("Server error")?;

        Ok(())
    }

    // Helper method to create self-signed certificates for development (future use)
    pub async fn create_dev_certificates(&self) -> Result<()> {
        tracing::info!("Certificate creation skipped - using HTTP for development");
        tracing::info!("For production deployment, configure TLS certificates");
        tracing::info!("You can use Let's Encrypt, CloudFlare, or a reverse proxy like Nginx");
        Ok(())
    }
}

// Future HTTPS/TLS server implementation
#[allow(dead_code)]
pub struct HttpsServer {
    config: Config,
    firebase_auth: SharedFirebaseAuth,
    fcm_service: SharedFCMService,
}

#[allow(dead_code)]
impl HttpsServer {
    pub fn new(config: Config) -> Result<Self> {
        let firebase_auth = Arc::new(Mutex::new(FirebaseAuth::new(&config)));
        let fcm_service = Arc::new(Mutex::new(FCMService::new(&config)?));

        Ok(Self {
            config,
            firebase_auth,
            fcm_service,
        })
    }

    pub async fn run(self) -> Result<()> {
        tracing::info!("HTTPS server implementation coming soon...");
        tracing::info!("Will support:");
        tracing::info!("  - TLS 1.3 encryption");
        tracing::info!("  - HTTP/2 with server push");
        tracing::info!("  - HTTP/3 with QUIC (future)");
        tracing::info!("  - Automatic certificate management");
        
        // For now, delegate to HTTP server
        let http_server = Server {
            config: self.config,
            firebase_auth: self.firebase_auth,
            fcm_service: self.fcm_service,
        };
        
        http_server.run().await
    }
}