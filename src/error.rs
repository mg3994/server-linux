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
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Resource not found")]
    NotFound,
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Network error: {0}")]
    Network(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Internal(err) => {
                tracing::error!("Internal server error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", msg)),
            AppError::Network(msg) => (StatusCode::BAD_GATEWAY, format!("Network error: {}", msg)),
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

pub type Result<T> = std::result::Result<T, AppError>;