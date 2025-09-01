use crate::database::Database;
use crate::delivery::models::*;
use crate::delivery::websocket::DeliveryWebSocketManager;
use crate::error::{AppError, Result};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

/// Enhanced delivery service with WebSocket integration
pub struct EnhancedDeliveryService {
    db: Database,
    ws_manager: Arc<DeliveryWebSocketManager>,
}

impl EnhancedDeliveryService {
    pub fn new(db: Database, ws_manager: Arc<DeliveryWebSocketManager>) -> Self {
        Self { db, ws_manager }
    }

    /// Update delivery person location with real-time broadcasting
    pub async fn update_location_with_broadcast(
        &self,
        delivery_person_id: Uuid,
        user_id: Uuid,
        request: UpdateLocationRequest,
    ) -> Result<()> {
        // First update the location in the database
        sqlx::query(
            r#"
            UPDATE delivery_persons 
            SET current_latitude = $1, current_longitude = $2, updated_at = $3
            WHERE id = $4 AND user_id = $5
            "#,
        )
        .bind(request.latitude)
        .bind(request.longitude)
        .bind(Utc::now())
        .bind(delivery_person_id)
        .bind(user_id)
        .execute(self.db.pool())
        .await?;

        // Create location update for broadcasting
        let location_update = LocationUpdate {
            delivery_person_id,
            latitude: request.latitude,
            longitude: request.longitude,
            speed: request.speed,
            heading: request.heading,
            timestamp: Utc::now(),
        };

        // Broadcast the location update via WebSocket
        self.ws_manager
            .broadcast_location_update(location_update.clone())
            .await?;

        // Store location update in database for history
        sqlx::query(
            r#"
            INSERT INTO location_updates (id, delivery_person_id, latitude, longitude, speed, heading, timestamp)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(delivery_person_id)
        .bind(request.latitude)
        .bind(request.longitude)
        .bind(request.speed)
        .bind(request.heading)
        .bind(location_update.timestamp)
        .execute(self.db.pool())
        .await?;

        Ok(())
    }

    /// Update delivery status with real-time broadcasting
    pub async fn update_delivery_status_with_broadcast(
        &self,
        assignment_id: Uuid,
        delivery_person_id: Uuid,
        request: UpdateDeliveryStatusRequest,
    ) -> Result<DeliveryAssignment> {
        let now = Utc::now();

        // Update the assignment status in database
        let assignment = sqlx::query_as::<_, DeliveryAssignment>(
            r#"
            UPDATE delivery_assignments 
            SET status = $1, updated_at = $2, notes = COALESCE($3, notes)
            WHERE id = $4 AND delivery_person_id = $5
            RETURNING id, order_id, delivery_person_id, status as status_str, 
                     pickup_address, delivery_address, estimated_pickup_time, 
                     estimated_delivery_time, actual_pickup_time, actual_delivery_time,
                     distance_km, delivery_fee, notes, created_at, updated_at
            "#,
        )
        .bind(request.status.as_str())
        .bind(now)
        .bind(&request.notes)
        .bind(assignment_id)
        .bind(delivery_person_id)
        .fetch_one(self.db.pool())
        .await?;

        // Calculate estimated arrival time based on status
        let estimated_arrival = match request.status {
            DeliveryStatus::PickedUp => {
                // Estimate delivery time based on distance and traffic
                Some(now + chrono::Duration::minutes(assignment.actual_distance_km.unwrap_or(5.0) as i64 * 2))
            }
            DeliveryStatus::EnRouteToCustomer => {
                // More precise estimate when out for delivery
                Some(now + chrono::Duration::minutes(assignment.actual_distance_km.unwrap_or(5.0) as i64))
            }
            _ => None,
        };

        // Broadcast status update via WebSocket
        self.ws_manager
            .broadcast_status_update(
                assignment_id,
                delivery_person_id,
                request.status,
                estimated_arrival,
                request.notes,
            )
            .await?;

        Ok(assignment)
    }

    /// Assign order with real-time broadcasting
    pub async fn assign_order_with_broadcast(
        &self,
        request: OrderAssignmentRequest,
    ) -> Result<DeliveryAssignment> {
        let assignment_id = Uuid::new_v4();
        let now = Utc::now();

        // Find a suitable delivery person if not specified
        let delivery_person_id = if let Some(preferred_id) = request.preferred_delivery_person_id {
            preferred_id
        } else {
            // Find nearest available delivery person
            self.find_nearest_delivery_person(request.order_id, request.max_distance_km.unwrap_or(10.0))
                .await?
                .ok_or_else(|| AppError::NotFound("No available delivery person found".to_string()))?
        };

        // Create the assignment in database with basic schema matching the actual table
        let assignment = sqlx::query_as::<_, DeliveryAssignment>(
            r#"
            INSERT INTO delivery_assignments (
                id, order_id, delivery_person_id, status, assigned_at, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, order_id, delivery_person_id, restaurant_id, customer_id,
                     pickup_address, delivery_address, status as status_str, 
                     assigned_at, accepted_at, picked_up_at, delivered_at,
                     estimated_pickup_time, estimated_delivery_time, actual_distance_km,
                     delivery_fee, tip_amount, delivery_notes, proof_of_delivery,
                     created_at, updated_at
            "#,
        )
        .bind(assignment_id)
        .bind(request.order_id)
        .bind(delivery_person_id)
        .bind(DeliveryStatus::Assigned.as_str())
        .bind(now)
        .bind(now)
        .bind(now)
        .fetch_one(self.db.pool())
        .await?;

        // Broadcast order assignment via WebSocket
        self.ws_manager
            .broadcast_order_assignment(
                assignment_id,
                delivery_person_id,
                request.order_id,
                assignment.pickup_address.clone(),
                assignment.delivery_address.clone(),
                assignment.estimated_pickup_time,
                assignment.estimated_delivery_time,
            )
            .await?;

        Ok(assignment)
    }

    /// Find nearest available delivery person
    async fn find_nearest_delivery_person(
        &self,
        _order_id: Uuid,
        _max_distance_km: f64,
    ) -> Result<Option<Uuid>> {
        // This is a simplified version - in production you'd use proper geospatial queries
        let delivery_person = sqlx::query_scalar::<_, Uuid>(
            r#"
            SELECT id FROM delivery_persons 
            WHERE is_available = true AND is_active = true 
            ORDER BY RANDOM() 
            LIMIT 1
            "#,
        )
        .fetch_optional(self.db.pool())
        .await?;

        Ok(delivery_person)
    }

    /// Handle emergency alert with immediate broadcasting
    pub async fn handle_emergency_alert(
        &self,
        delivery_person_id: Uuid,
        latitude: f64,
        longitude: f64,
        message: String,
    ) -> Result<()> {
        // Store emergency alert in database
        let alert_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO emergency_alerts (id, delivery_person_id, latitude, longitude, message, timestamp, resolved)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(alert_id)
        .bind(delivery_person_id)
        .bind(latitude)
        .bind(longitude)
        .bind(&message)
        .bind(Utc::now())
        .bind(false)
        .execute(self.db.pool())
        .await?;

        // Immediately broadcast emergency alert
        self.ws_manager
            .broadcast_emergency_alert(delivery_person_id, latitude, longitude, message)
            .await?;

        Ok(())
    }

    /// Get real-time delivery tracking information
    pub async fn get_real_time_tracking(
        &self,
        assignment_id: Uuid,
    ) -> Result<DeliveryTrackingInfo> {
        let assignment = sqlx::query_as::<_, DeliveryAssignment>(
            r#"
            SELECT id, order_id, delivery_person_id, status as status_str, 
                   pickup_address, delivery_address, estimated_pickup_time, 
                   estimated_delivery_time, actual_pickup_time, actual_delivery_time,
                   distance_km, delivery_fee, notes, created_at, updated_at
            FROM delivery_assignments 
            WHERE id = $1
            "#,
        )
        .bind(assignment_id)
        .fetch_one(self.db.pool())
        .await?;

        // Get latest location of delivery person
        let location = sqlx::query_as::<_, LocationUpdate>(
            r#"
            SELECT delivery_person_id, latitude, longitude, speed, heading, timestamp
            FROM location_updates 
            WHERE delivery_person_id = $1 
            ORDER BY timestamp DESC 
            LIMIT 1
            "#,
        )
        .bind(assignment.delivery_person_id)
        .fetch_optional(self.db.pool())
        .await?;

        // Get delivery person info
        let delivery_person = sqlx::query_as::<_, DeliveryPerson>(
            r#"
            SELECT id, user_id, name, phone, email, vehicle_type as vehicle_type_str, 
                   vehicle_number, license_number, aadhar_number, pan_number, 
                   bank_account_number, ifsc_code, current_latitude, current_longitude,
                   is_available, is_verified, is_active, rating, total_deliveries,
                   successful_deliveries, average_delivery_time, earnings_today,
                   earnings_this_month, created_at, updated_at
            FROM delivery_persons 
            WHERE id = $1
            "#,
        )
        .bind(assignment.delivery_person_id)
        .fetch_one(self.db.pool())
        .await?;

        Ok(DeliveryTrackingInfo {
            assignment,
            delivery_person,
            current_location: location,
            estimated_arrival: None, // Calculate based on current location and destination
        })
    }

    /// Get delivery analytics with real-time data
    pub async fn get_real_time_analytics(&self) -> Result<DeliveryAnalytics> {
        let active_deliveries = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM delivery_assignments WHERE status IN ('assigned', 'picked_up', 'out_for_delivery')"
        )
        .fetch_one(self.db.pool())
        .await?;

        let online_delivery_persons = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM delivery_persons WHERE is_available = true AND is_active = true"
        )
        .fetch_one(self.db.pool())
        .await?;

        let completed_today = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM delivery_assignments WHERE status = 'delivered' AND DATE(created_at) = CURRENT_DATE"
        )
        .fetch_one(self.db.pool())
        .await?;

        let average_delivery_time = sqlx::query_scalar::<_, Option<f64>>(
            r#"
            SELECT AVG(EXTRACT(EPOCH FROM (actual_delivery_time - actual_pickup_time))/60) 
            FROM delivery_assignments 
            WHERE status = 'delivered' AND actual_pickup_time IS NOT NULL AND actual_delivery_time IS NOT NULL
            AND DATE(created_at) = CURRENT_DATE
            "#
        )
        .fetch_one(self.db.pool())
        .await?;

        Ok(DeliveryAnalytics {
            active_deliveries: active_deliveries as u32,
            online_delivery_persons: online_delivery_persons as u32,
            completed_deliveries_today: completed_today as u32,
            average_delivery_time_minutes: average_delivery_time.unwrap_or(0.0) as u32,
            total_distance_covered_today: 0.0, // Would need to calculate from location updates
            total_earnings_today: 0.0,         // Would need to sum from completed deliveries
        })
    }
}

// Additional models for enhanced functionality
#[derive(Debug, serde::Serialize)]
pub struct DeliveryTrackingInfo {
    pub assignment: DeliveryAssignment,
    pub delivery_person: DeliveryPerson,
    pub current_location: Option<LocationUpdate>,
    pub estimated_arrival: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct DeliveryAnalytics {
    pub active_deliveries: u32,
    pub online_delivery_persons: u32,
    pub completed_deliveries_today: u32,
    pub average_delivery_time_minutes: u32,
    pub total_distance_covered_today: f64,
    pub total_earnings_today: f64,
}