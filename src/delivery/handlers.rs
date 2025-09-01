use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::models::User;
use crate::error::Result;
use crate::delivery::{
    models::*,
    service::DeliveryService,
};
use crate::routes::AppState;

#[derive(Debug, Deserialize)]
pub struct DeliveryPersonListQuery {
    pub city: Option<String>,
    pub vehicle_type: Option<VehicleType>,
    pub is_available: Option<bool>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct NearbyDeliveryPersonQuery {
    pub latitude: f64,
    pub longitude: f64,
    pub radius_km: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryAssignmentQuery {
    pub status: Option<DeliveryStatus>,
}

#[derive(Debug, Deserialize)]
pub struct DeliveryTimeEstimateQuery {
    pub pickup_lat: f64,
    pub pickup_lng: f64,
    pub delivery_lat: f64,
    pub delivery_lng: f64,
    pub city: String,
}

// Delivery Person Management
pub async fn register_delivery_person(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<RegisterDeliveryPersonRequest>,
) -> Result<Json<DeliveryPersonResponse>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let delivery_person = delivery_service
        .register_delivery_person(user.id, request)
        .await?;
    
    Ok(Json(DeliveryPersonResponse::from(delivery_person)))
}

pub async fn get_delivery_person(
    State(state): State<AppState>,
    Path(delivery_person_id): Path<Uuid>,
) -> Result<Json<DeliveryPersonResponse>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let delivery_person = delivery_service
        .get_delivery_person(delivery_person_id)
        .await?;
    
    Ok(Json(DeliveryPersonResponse::from(delivery_person)))
}

pub async fn update_delivery_person(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(delivery_person_id): Path<Uuid>,
    Json(request): Json<UpdateDeliveryPersonRequest>,
) -> Result<Json<DeliveryPersonResponse>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let delivery_person = delivery_service
        .update_delivery_person(delivery_person_id, user.id, request)
        .await?;
    
    Ok(Json(DeliveryPersonResponse::from(delivery_person)))
}

pub async fn update_location(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(delivery_person_id): Path<Uuid>,
    Json(request): Json<UpdateLocationRequest>,
) -> Result<StatusCode> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    delivery_service
        .update_location(delivery_person_id, user.id, request)
        .await?;
    
    Ok(StatusCode::OK)
}

pub async fn get_delivery_person_stats(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(delivery_person_id): Path<Uuid>,
) -> Result<Json<DeliveryStatsResponse>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let stats = delivery_service
        .get_delivery_person_stats(delivery_person_id, user.id)
        .await?;
    
    Ok(Json(stats))
}

// Order Assignment and Management
pub async fn assign_order(
    State(state): State<AppState>,
    Json(request): Json<OrderAssignmentRequest>,
) -> Result<Json<DeliveryAssignment>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let assignment = delivery_service
        .assign_order(request)
        .await?;
    
    Ok(Json(assignment))
}

pub async fn update_delivery_status(
    State(state): State<AppState>,
    Extension(_user): Extension<User>,
    Path((assignment_id, delivery_person_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateDeliveryStatusRequest>,
) -> Result<Json<DeliveryAssignment>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let assignment = delivery_service
        .update_delivery_status(assignment_id, delivery_person_id, request)
        .await?;
    
    Ok(Json(assignment))
}

pub async fn get_delivery_assignments(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(delivery_person_id): Path<Uuid>,
    Query(params): Query<DeliveryAssignmentQuery>,
) -> Result<Json<Vec<DeliveryAssignment>>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let assignments = delivery_service
        .get_delivery_assignments(delivery_person_id, user.id, params.status)
        .await?;
    
    Ok(Json(assignments))
}

// Discovery and Search
pub async fn get_nearby_delivery_persons(
    State(state): State<AppState>,
    Query(params): Query<NearbyDeliveryPersonQuery>,
) -> Result<Json<Vec<NearbyDeliveryPersonResponse>>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let radius = params.radius_km.unwrap_or(10.0);
    let nearby_persons = delivery_service
        .get_nearby_delivery_persons(params.latitude, params.longitude, radius)
        .await?;
    
    Ok(Json(nearby_persons))
}

// India-specific endpoints
pub async fn get_india_delivery_zones(
    State(state): State<AppState>,
) -> Result<Json<Vec<IndiaDeliveryZone>>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let zones = delivery_service
        .get_india_delivery_zones()
        .await?;
    
    Ok(Json(zones))
}

pub async fn calculate_delivery_time_estimate(
    State(state): State<AppState>,
    Query(params): Query<DeliveryTimeEstimateQuery>,
) -> Result<Json<DeliveryTimeEstimate>> {
    let delivery_service = DeliveryService::new(state.database.clone());
    
    let estimate = delivery_service
        .calculate_delivery_time_estimate(
            params.pickup_lat,
            params.pickup_lng,
            params.delivery_lat,
            params.delivery_lng,
            &params.city,
        )
        .await?;
    
    Ok(Json(estimate))
}

// Admin endpoints for delivery person management
pub async fn verify_delivery_person(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(delivery_person_id): Path<Uuid>,
) -> Result<Json<DeliveryPersonResponse>> {
    // Check if user is admin (simplified - in production, check role)
    if user.role != "admin" {
        return Err(crate::error::AppError::Forbidden("Admin access required".to_string()));
    }

    // Update verification status
    sqlx::query(
        "UPDATE delivery_persons SET is_verified = true, updated_at = $1 WHERE id = $2"
    )
    .bind(chrono::Utc::now())
    .bind(delivery_person_id)
    .execute(state.database.pool())
    .await
    .map_err(|e| crate::error::AppError::DatabaseError(e.to_string()))?;

    let delivery_service = DeliveryService::new(state.database.clone());
    let delivery_person = delivery_service
        .get_delivery_person(delivery_person_id)
        .await?;
    
    Ok(Json(DeliveryPersonResponse::from(delivery_person)))
}

pub async fn deactivate_delivery_person(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(delivery_person_id): Path<Uuid>,
) -> Result<StatusCode> {
    // Check if user is admin (simplified - in production, check role)
    if user.role != "admin" {
        return Err(crate::error::AppError::Forbidden("Admin access required".to_string()));
    }

    sqlx::query(
        "UPDATE delivery_persons SET is_active = false, is_available = false, updated_at = $1 WHERE id = $2"
    )
    .bind(chrono::Utc::now())
    .bind(delivery_person_id)
    .execute(state.database.pool())
    .await
    .map_err(|e| crate::error::AppError::DatabaseError(e.to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
}

// Analytics endpoints
pub async fn get_delivery_analytics(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<serde_json::Value>> {
    // Check if user is admin
    if user.role != "admin" {
        return Err(crate::error::AppError::Forbidden("Admin access required".to_string()));
    }

    // Get delivery analytics (simplified)
    let total_delivery_persons = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM delivery_persons WHERE is_active = true"
    )
    .fetch_one(state.database.pool())
    .await
    .map_err(|e| crate::error::AppError::DatabaseError(e.to_string()))?;

    let active_delivery_persons = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM delivery_persons WHERE is_active = true AND is_available = true"
    )
    .fetch_one(state.database.pool())
    .await
    .map_err(|e| crate::error::AppError::DatabaseError(e.to_string()))?;

    let total_deliveries_today = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM delivery_assignments WHERE DATE(created_at) = CURRENT_DATE"
    )
    .fetch_one(state.database.pool())
    .await
    .map_err(|e| crate::error::AppError::DatabaseError(e.to_string()))?;

    let successful_deliveries_today = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM delivery_assignments WHERE DATE(created_at) = CURRENT_DATE AND status = 'delivered'"
    )
    .fetch_one(state.database.pool())
    .await
    .map_err(|e| crate::error::AppError::DatabaseError(e.to_string()))?;

    let analytics = serde_json::json!({
        "total_delivery_persons": total_delivery_persons,
        "active_delivery_persons": active_delivery_persons,
        "total_deliveries_today": total_deliveries_today,
        "successful_deliveries_today": successful_deliveries_today,
        "success_rate_today": if total_deliveries_today > 0 {
            (successful_deliveries_today as f64 / total_deliveries_today as f64) * 100.0
        } else {
            0.0
        },
        "availability_rate": if total_delivery_persons > 0 {
            (active_delivery_persons as f64 / total_delivery_persons as f64) * 100.0
        } else {
            0.0
        }
    });

    Ok(Json(analytics))
}