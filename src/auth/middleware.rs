use crate::auth::firebase::FirebaseAuth;
use crate::error::{AppError, Result};
use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type SharedFirebaseAuth = Arc<Mutex<FirebaseAuth>>;

pub async fn auth_middleware(
    State(firebase_auth): State<SharedFirebaseAuth>,
    headers: HeaderMap,
    mut request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    let token = &auth_header[7..]; // Remove "Bearer " prefix

    let mut firebase_auth = firebase_auth.lock().await;
    let firebase_token = firebase_auth.verify_token(token).await?;
    
    // Validate user requirements (email + phone verified)
    firebase_auth.validate_user_requirements(&firebase_token)?;
    
    let user = firebase_auth.token_to_user(firebase_token);

    // Add user to request extensions for handler extraction
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}