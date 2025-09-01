use axum::{
    extract::{Path, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::models::User;
use crate::delivery::enhanced_service::EnhancedDeliveryService;
use crate::delivery::models::*;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct EmergencyAlertRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub message: String,
}

/// Enhanced location update with real-time broadcasting
pub async fn update_location_enhanced(
    Path(delivery_person_id): Path<Uuid>,
    State(service): State<Arc<EnhancedDeliveryService>>,
    auth_user: User,
    Json(request): Json<UpdateLocationRequest>,
) -> Result<Json<serde_json::Value>> {
    service
        .update_location_with_broadcast(delivery_person_id, auth_user.id, request)
        .await?;

    Ok(Json(serde_json::json!({
        "message": "Location updated and broadcasted successfully",
        "delivery_person_id": delivery_person_id,
        "timestamp": chrono::Utc::now()
    })))
}

/// Enhanced delivery status update with real-time broadcasting
pub async fn update_delivery_status_enhanced(
    Path(assignment_id): Path<Uuid>,
    State(service): State<Arc<EnhancedDeliveryService>>,
    auth_user: User,
    Json(request): Json<UpdateDeliveryStatusRequest>,
) -> Result<Json<DeliveryAssignment>> {
    // Get delivery person ID from the assignment
    let assignment = service
        .update_delivery_status_with_broadcast(assignment_id, auth_user.id, request)
        .await?;

    Ok(Json(assignment))
}

/// Enhanced order assignment with real-time broadcasting
pub async fn assign_order_enhanced(
    State(service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
    Json(request): Json<OrderAssignmentRequest>,
) -> Result<Json<DeliveryAssignment>> {
    let assignment = service.assign_order_with_broadcast(request).await?;
    Ok(Json(assignment))
}

/// Handle emergency alert from delivery person
pub async fn handle_emergency_alert(
    Path(delivery_person_id): Path<Uuid>,
    State(service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
    Json(request): Json<EmergencyAlertRequest>,
) -> Result<Json<serde_json::Value>> {
    service
        .handle_emergency_alert(
            delivery_person_id,
            request.latitude,
            request.longitude,
            request.message,
        )
        .await?;

    Ok(Json(serde_json::json!({
        "message": "Emergency alert sent successfully",
        "delivery_person_id": delivery_person_id,
        "alert_sent_at": chrono::Utc::now(),
        "coordinates": {
            "latitude": request.latitude,
            "longitude": request.longitude
        }
    })))
}

/// Get real-time delivery tracking information
pub async fn get_delivery_tracking(
    Path(assignment_id): Path<Uuid>,
    State(service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
) -> Result<Json<crate::delivery::enhanced_service::DeliveryTrackingInfo>> {
    let tracking_info = service.get_real_time_tracking(assignment_id).await?;
    Ok(Json(tracking_info))
}

/// Get real-time delivery analytics
pub async fn get_real_time_delivery_analytics(
    State(service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
) -> Result<Json<crate::delivery::enhanced_service::DeliveryAnalytics>> {
    let analytics = service.get_real_time_analytics().await?;
    Ok(Json(analytics))
}

/// Get delivery person's current status and active assignments
pub async fn get_delivery_person_status(
    Path(delivery_person_id): Path<Uuid>,
    State(_service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
) -> Result<Json<serde_json::Value>> {
    // This would typically fetch from the regular delivery service
    // For now, return a placeholder response
    Ok(Json(serde_json::json!({
        "delivery_person_id": delivery_person_id,
        "status": "online",
        "active_assignments": 0,
        "current_location": null,
        "last_update": chrono::Utc::now()
    })))
}

/// Batch update multiple delivery statuses (for admin use)
#[derive(Debug, Deserialize)]
pub struct BatchStatusUpdate {
    pub updates: Vec<BatchStatusUpdateItem>,
}

#[derive(Debug, Deserialize)]
pub struct BatchStatusUpdateItem {
    pub assignment_id: Uuid,
    pub delivery_person_id: Uuid,
    pub status: DeliveryStatus,
    pub notes: Option<String>,
}

pub async fn batch_update_delivery_status(
    State(service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
    Json(request): Json<BatchStatusUpdate>,
) -> Result<Json<serde_json::Value>> {
    let mut updated_assignments = Vec::new();
    let mut errors = Vec::new();

    let total_updates = request.updates.len();
    for update in request.updates {
        let status_request = UpdateDeliveryStatusRequest {
            status: update.status,
            notes: update.notes,
            proof_of_delivery: None,
        };

        match service
            .update_delivery_status_with_broadcast(
                update.assignment_id,
                update.delivery_person_id,
                status_request,
            )
            .await
        {
            Ok(assignment) => updated_assignments.push(assignment),
            Err(e) => errors.push(serde_json::json!({
                "assignment_id": update.assignment_id,
                "error": e.to_string()
            })),
        }
    }

    Ok(Json(serde_json::json!({
        "updated_assignments": updated_assignments,
        "errors": errors,
        "total_processed": total_updates,
        "successful": updated_assignments.len(),
        "failed": errors.len(),
        "timestamp": chrono::Utc::now()
    })))
}

/// Get delivery heatmap data for analytics
pub async fn get_delivery_heatmap(
    State(_service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
) -> Result<Json<serde_json::Value>> {
    // This would typically aggregate location data to create heatmap points
    // For now, return a placeholder response
    Ok(Json(serde_json::json!({
        "heatmap_data": [],
        "generated_at": chrono::Utc::now(),
        "data_points": 0
    })))
}

/// Get live delivery metrics for dashboard
pub async fn get_live_delivery_metrics(
    State(service): State<Arc<EnhancedDeliveryService>>,
    _auth_user: User,
) -> Result<Json<serde_json::Value>> {
    let analytics = service.get_real_time_analytics().await?;

    Ok(Json(serde_json::json!({
        "metrics": analytics,
        "websocket_connections": {
            "total": 0, // Would get from WebSocket manager
            "delivery_persons": 0,
            "customers": 0,
            "restaurants": 0,
            "admins": 0
        },
        "system_status": "operational",
        "last_updated": chrono::Utc::now()
    })))
}
