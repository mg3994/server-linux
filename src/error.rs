use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Internal server error: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("Authentication failed")]
    Unauthorized,
    #[error("Access forbidden: {0}")]
    Forbidden(String),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("WebSocket error: {0}")]
    WebSocketError(String),
    #[error("Delivery person not found")]
    DeliveryPersonNotFound,
    #[error("Delivery person not available")]
    DeliveryPersonNotAvailable,
    #[error("Delivery person not verified")]
    DeliveryPersonNotVerified,
    #[error("Invalid delivery status transition: {0}")]
    InvalidStatusTransition(String),
    #[error("Order already assigned to delivery person")]
    OrderAlreadyAssigned,
    #[error("No delivery persons available in the area")]
    NoDeliveryPersonsAvailable,
    #[error("Delivery assignment not found")]
    DeliveryAssignmentNotFound,
    #[error("Location update failed: {0}")]
    LocationUpdateFailed(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Internal(err) => {
                tracing::error!("Internal server error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            AppError::Network(msg) => (StatusCode::BAD_GATEWAY, format!("Network error: {}", msg)),
            AppError::WebSocketError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("WebSocket error: {}", msg)),
            AppError::DeliveryPersonNotFound => (StatusCode::NOT_FOUND, "Delivery person not found".to_string()),
            AppError::DeliveryPersonNotAvailable => (StatusCode::CONFLICT, "Delivery person not available".to_string()),
            AppError::DeliveryPersonNotVerified => (StatusCode::FORBIDDEN, "Delivery person not verified".to_string()),
            AppError::InvalidStatusTransition(msg) => (StatusCode::BAD_REQUEST, format!("Invalid status transition: {}", msg)),
            AppError::OrderAlreadyAssigned => (StatusCode::CONFLICT, "Order already assigned".to_string()),
            AppError::NoDeliveryPersonsAvailable => (StatusCode::SERVICE_UNAVAILABLE, "No delivery persons available".to_string()),
            AppError::DeliveryAssignmentNotFound => (StatusCode::NOT_FOUND, "Delivery assignment not found".to_string()),
            AppError::LocationUpdateFailed(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Location update failed: {}", msg)),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }));

        (status, body).into_response()
    }
}

// Implement From trait for common error types
impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::Internal(anyhow::Error::from(err))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ValidationError(format!("JSON parsing error: {}", err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;