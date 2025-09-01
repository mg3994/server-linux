use crate::auth::middleware::{auth_middleware, SharedFirebaseAuth};
use crate::orders::handlers::{create_order, get_order, update_order_status, SharedFCMService};
use crate::payments::handlers::{create_payment, get_payment};
use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

pub fn create_routes(
    firebase_auth: SharedFirebaseAuth,
    fcm_service: SharedFCMService,
) -> Router {
    Router::new()
        // Public routes
        .route("/health", get(health_check))
        // Protected routes
        .route("/orders", post(create_order))
        .route("/orders/:id", get(get_order))
        .route("/orders/:id/status", put(update_order_status))
        .route("/payments", post(create_payment))
        .route("/payments/:id", get(get_payment))
        // Add auth middleware to protected routes
        .layer(middleware::from_fn_with_state(
            firebase_auth.clone(),
            auth_middleware,
        ))
        // Add shared state
        .with_state(fcm_service)
}

async fn health_check() -> &'static str {
    "OK"
}