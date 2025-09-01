use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::models::Claims;
use crate::error::{AppError, Result};
use crate::routes::AppState;
use crate::users::models::*;
use crate::users::service::UserService;

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub role: Option<UserRole>,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>> {
    let user_service = UserService::new(state.database.clone());
    
    // Check if user already exists
    if let Some(_) = user_service.get_user_by_firebase_uid(&request.firebase_uid).await? {
        return Err(AppError::BadRequest("User already exists".to_string()));
    }

    // Check email uniqueness if provided
    if let Some(ref email) = request.email {
        if let Some(_) = user_service.get_user_by_email(email).await? {
            return Err(AppError::BadRequest("Email already registered".to_string()));
        }
    }

    // Check phone uniqueness if provided
    if let Some(ref phone) = request.phone {
        if let Some(_) = user_service.get_user_by_phone(phone).await? {
            return Err(AppError::BadRequest("Phone number already registered".to_string()));
        }
    }

    let user = user_service.create_user(request).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
) -> Result<Json<UserResponse>> {
    let user_service = UserService::new(state.database.clone());
    
    // Users can only access their own profile unless they're admin
    if claims.sub != user_id.to_string() && claims.role != Some("admin".to_string()) {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let user = user_service
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn get_current_user(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<UserResponse>> {
    let user_service = UserService::new(state.database.clone());
    
    let user = user_service
        .get_user_by_firebase_uid(&claims.sub)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
    Json(request): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>> {
    let user_service = UserService::new(state.database.clone());
    
    // Users can only update their own profile unless they're admin
    if claims.sub != user_id.to_string() && claims.role != Some("admin".to_string()) {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    // Check email uniqueness if being updated
    if let Some(ref email) = request.email {
        if let Some(existing_user) = user_service.get_user_by_email(email).await? {
            if existing_user.id != user_id {
                return Err(AppError::BadRequest("Email already registered".to_string()));
            }
        }
    }

    // Check phone uniqueness if being updated
    if let Some(ref phone) = request.phone {
        if let Some(existing_user) = user_service.get_user_by_phone(phone).await? {
            if existing_user.id != user_id {
                return Err(AppError::BadRequest("Phone number already registered".to_string()));
            }
        }
    }

    let user = user_service.update_user(user_id, request).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn verify_email(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
    Json(_request): Json<VerifyEmailRequest>,
) -> Result<Json<UserResponse>> {
    let user_service = UserService::new(state.database.clone());
    
    // Users can only verify their own email
    if claims.sub != user_id.to_string() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    // In a real implementation, you would validate the verification code here
    // For now, we'll just mark the email as verified
    let user = user_service.verify_email(user_id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn verify_phone(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
    Json(_request): Json<VerifyPhoneRequest>,
) -> Result<Json<UserResponse>> {
    let user_service = UserService::new(state.database.clone());
    
    // Users can only verify their own phone
    if claims.sub != user_id.to_string() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    // In a real implementation, you would validate the verification code here
    // For now, we'll just mark the phone as verified
    let user = user_service.verify_phone(user_id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn deactivate_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    claims: Claims,
) -> Result<StatusCode> {
    let user_service = UserService::new(state.database.clone());
    
    // Users can deactivate their own account or admin can deactivate any account
    if claims.sub != user_id.to_string() && claims.role != Some("admin".to_string()) {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    user_service.deactivate_user(user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
    claims: Claims,
) -> Result<Json<UserListResponse>> {
    // Only admins can list users
    if claims.role != Some("admin".to_string()) {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let user_service = UserService::new(state.database.clone());
    
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20).min(100); // Max 100 per page

    let response = user_service.list_users(page, per_page, query.role).await?;
    Ok(Json(response))
}