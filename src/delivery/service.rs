use crate::database::Database;
use crate::error::{AppError, Result};
use crate::delivery::models::*;
use uuid::Uuid;
use chrono::{Utc, Timelike};

use sqlx::Row;

pub struct DeliveryService {
    db: Database,
}

impl DeliveryService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    // Delivery Person Management
    pub async fn register_delivery_person(&self, user_id: Uuid, request: RegisterDeliveryPersonRequest) -> Result<DeliveryPerson> {
        let delivery_person_id = Uuid::new_v4();
        let now = Utc::now();

        let delivery_person = sqlx::query_as::<_, DeliveryPerson>(
            r#"
            INSERT INTO delivery_persons (
                id, user_id, name, phone, email, vehicle_type, vehicle_number, 
                license_number, aadhar_number, pan_number, bank_account_number, 
                ifsc_code, is_available, is_verified, is_active, rating, 
                total_deliveries, successful_deliveries, earnings_today, 
                earnings_this_month, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22
            ) RETURNING id, user_id, name, phone, email, vehicle_type as vehicle_type_str, 
                     vehicle_number, license_number, aadhar_number, pan_number, 
                     bank_account_number, ifsc_code, current_latitude, current_longitude,
                     is_available, is_verified, is_active, rating, total_deliveries,
                     successful_deliveries, average_delivery_time, earnings_today,
                     earnings_this_month, created_at, updated_at
            "#,
        )
        .bind(delivery_person_id)
        .bind(user_id)
        .bind(&request.name)
        .bind(&request.phone)
        .bind(&request.email)
        .bind(request.vehicle_type.as_str())
        .bind(&request.vehicle_number)
        .bind(&request.license_number)
        .bind(&request.aadhar_number)
        .bind(&request.pan_number)
        .bind(&request.bank_account_number)
        .bind(&request.ifsc_code)
        .bind(false) // is_available (needs admin verification first)
        .bind(false) // is_verified (needs admin verification)
        .bind(true)  // is_active
        .bind(0.0f64) // initial rating
        .bind(0i32)   // total_deliveries
        .bind(0i32)   // successful_deliveries
        .bind(0.0f64) // earnings_today
        .bind(0.0f64) // earnings_this_month
        .bind(now)
        .bind(now)
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(delivery_person)
    }

    pub async fn get_delivery_person(&self, delivery_person_id: Uuid) -> Result<DeliveryPerson> {
        let delivery_person = sqlx::query_as::<_, DeliveryPerson>(
            "SELECT id, user_id, name, phone, email, vehicle_type as vehicle_type_str, 
                    vehicle_number, license_number, aadhar_number, pan_number, 
                    bank_account_number, ifsc_code, current_latitude, current_longitude,
                    is_available, is_verified, is_active, rating, total_deliveries,
                    successful_deliveries, average_delivery_time, earnings_today,
                    earnings_this_month, created_at, updated_at
             FROM delivery_persons WHERE id = $1 AND is_active = true"
        )
        .bind(delivery_person_id)
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Delivery person not found".to_string()))?;

        Ok(delivery_person)
    }

    pub async fn update_delivery_person(&self, delivery_person_id: Uuid, user_id: Uuid, request: UpdateDeliveryPersonRequest) -> Result<DeliveryPerson> {
        // Verify ownership
        self.verify_delivery_person_ownership(delivery_person_id, user_id).await?;

        let now = Utc::now();
        let mut query_parts = Vec::new();
        let mut bind_count = 1;

        if request.name.is_some() {
            query_parts.push(format!("name = ${}", bind_count));
            bind_count += 1;
        }
        if request.phone.is_some() {
            query_parts.push(format!("phone = ${}", bind_count));
            bind_count += 1;
        }
        if request.email.is_some() {
            query_parts.push(format!("email = ${}", bind_count));
            bind_count += 1;
        }
        if request.vehicle_type.is_some() {
            query_parts.push(format!("vehicle_type = ${}", bind_count));
            bind_count += 1;
        }
        if request.vehicle_number.is_some() {
            query_parts.push(format!("vehicle_number = ${}", bind_count));
            bind_count += 1;
        }
        if request.license_number.is_some() {
            query_parts.push(format!("license_number = ${}", bind_count));
            bind_count += 1;
        }
        if request.bank_account_number.is_some() {
            query_parts.push(format!("bank_account_number = ${}", bind_count));
            bind_count += 1;
        }
        if request.ifsc_code.is_some() {
            query_parts.push(format!("ifsc_code = ${}", bind_count));
            bind_count += 1;
        }
        if request.is_available.is_some() {
            query_parts.push(format!("is_available = ${}", bind_count));
            bind_count += 1;
        }

        if query_parts.is_empty() {
            return Err(AppError::BadRequest("No fields to update".to_string()));
        }

        query_parts.push(format!("updated_at = ${}", bind_count));
        let update_clause = query_parts.join(", ");

        let query = format!(
            "UPDATE delivery_persons SET {} WHERE id = ${} AND user_id = ${} AND is_active = true RETURNING *",
            update_clause,
            bind_count + 1,
            bind_count + 2
        );

        let mut query_builder = sqlx::query_as::<_, DeliveryPerson>(&query);

        // Bind parameters in order
        if let Some(name) = &request.name {
            query_builder = query_builder.bind(name);
        }
        if let Some(phone) = &request.phone {
            query_builder = query_builder.bind(phone);
        }
        if let Some(email) = &request.email {
            query_builder = query_builder.bind(email);
        }
        if let Some(vehicle_type) = &request.vehicle_type {
            query_builder = query_builder.bind(vehicle_type.as_str());
        }
        if let Some(vehicle_number) = &request.vehicle_number {
            query_builder = query_builder.bind(vehicle_number);
        }
        if let Some(license_number) = &request.license_number {
            query_builder = query_builder.bind(license_number);
        }
        if let Some(bank_account_number) = &request.bank_account_number {
            query_builder = query_builder.bind(bank_account_number);
        }
        if let Some(ifsc_code) = &request.ifsc_code {
            query_builder = query_builder.bind(ifsc_code);
        }
        if let Some(is_available) = &request.is_available {
            query_builder = query_builder.bind(is_available);
        }

        let delivery_person = query_builder
            .bind(now)
            .bind(delivery_person_id)
            .bind(user_id)
            .fetch_optional(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Delivery person not found or not owned by user".to_string()))?;

        Ok(delivery_person)
    }

    pub async fn update_location(&self, delivery_person_id: Uuid, user_id: Uuid, request: UpdateLocationRequest) -> Result<()> {
        // Verify ownership
        self.verify_delivery_person_ownership(delivery_person_id, user_id).await?;

        sqlx::query(
            "UPDATE delivery_persons SET current_latitude = $1, current_longitude = $2, updated_at = $3 WHERE id = $4 AND user_id = $5"
        )
        .bind(request.latitude)
        .bind(request.longitude)
        .bind(Utc::now())
        .bind(delivery_person_id)
        .bind(user_id)
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // Order Assignment Logic
    pub async fn assign_order(&self, request: OrderAssignmentRequest) -> Result<DeliveryAssignment> {
        let assignment_id = Uuid::new_v4();
        let now = Utc::now();

        // Get order details first
        let order_details = self.get_order_details(request.order_id).await?;

        // Find best delivery person
        let delivery_person_id = if let Some(preferred_id) = request.preferred_delivery_person_id {
            // Check if preferred delivery person is available
            if self.is_delivery_person_available(preferred_id).await? {
                preferred_id
            } else {
                self.find_best_delivery_person(&order_details, request.max_distance_km).await?
            }
        } else {
            self.find_best_delivery_person(&order_details, request.max_distance_km).await?
        };

        // Calculate estimated times
        let estimated_pickup_time = now + chrono::Duration::minutes(10); // 10 min to reach restaurant
        let estimated_delivery_time = estimated_pickup_time + chrono::Duration::minutes(25); // 25 min to deliver

        let assignment = sqlx::query_as::<_, DeliveryAssignment>(
            r#"
            INSERT INTO delivery_assignments (
                id, order_id, delivery_person_id, restaurant_id, customer_id,
                pickup_address, delivery_address, status, assigned_at,
                estimated_pickup_time, estimated_delivery_time, delivery_fee,
                created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
            ) RETURNING id, order_id, delivery_person_id, restaurant_id, customer_id,
                     pickup_address, delivery_address, status as status_str, assigned_at,
                     accepted_at, picked_up_at, delivered_at, estimated_pickup_time,
                     estimated_delivery_time, actual_distance_km, delivery_fee,
                     tip_amount, delivery_notes, proof_of_delivery, created_at, updated_at
            "#,
        )
        .bind(assignment_id)
        .bind(request.order_id)
        .bind(delivery_person_id)
        .bind(order_details.restaurant_id)
        .bind(order_details.customer_id)
        .bind(&order_details.pickup_address)
        .bind(&order_details.delivery_address)
        .bind(DeliveryStatus::Assigned.as_str())
        .bind(now)
        .bind(estimated_pickup_time)
        .bind(estimated_delivery_time)
        .bind(order_details.delivery_fee)
        .bind(now)
        .bind(now)
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Update delivery person availability
        self.update_delivery_person_availability(delivery_person_id, false).await?;

        Ok(assignment)
    }

    pub async fn update_delivery_status(&self, assignment_id: Uuid, delivery_person_id: Uuid, request: UpdateDeliveryStatusRequest) -> Result<DeliveryAssignment> {
        let now = Utc::now();
        
        // Build update query based on status
        let (status_updates, time_field) = match request.status {
            DeliveryStatus::Accepted => ("accepted_at = $3", Some(now)),
            DeliveryStatus::PickedUp => ("picked_up_at = $3", Some(now)),
            DeliveryStatus::Delivered => ("delivered_at = $3", Some(now)),
            _ => ("", None),
        };

        let mut query = format!(
            "UPDATE delivery_assignments SET status = $1, updated_at = $2"
        );

        if !status_updates.is_empty() {
            query.push_str(&format!(", {}", status_updates));
        }

        if request.notes.is_some() {
            query.push_str(", delivery_notes = $4");
        }

        if request.proof_of_delivery.is_some() {
            query.push_str(", proof_of_delivery = $5");
        }

        query.push_str(" WHERE id = $6 AND delivery_person_id = $7 RETURNING id, order_id, delivery_person_id, restaurant_id, customer_id,
                        pickup_address, delivery_address, status as status_str, assigned_at,
                        accepted_at, picked_up_at, delivered_at, estimated_pickup_time,
                        estimated_delivery_time, actual_distance_km, delivery_fee,
                        tip_amount, delivery_notes, proof_of_delivery, created_at, updated_at");

        let mut query_builder = sqlx::query_as::<_, DeliveryAssignment>(&query);
        query_builder = query_builder
            .bind(request.status.as_str())
            .bind(now);

        if let Some(time) = time_field {
            query_builder = query_builder.bind(time);
        }

        if let Some(notes) = &request.notes {
            query_builder = query_builder.bind(notes);
        }

        if let Some(proof) = &request.proof_of_delivery {
            query_builder = query_builder.bind(proof);
        }

        let assignment = query_builder
            .bind(assignment_id)
            .bind(delivery_person_id)
            .fetch_optional(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFound("Delivery assignment not found".to_string()))?;

        // If delivered, update delivery person stats and make them available
        if matches!(request.status, DeliveryStatus::Delivered) {
            self.update_delivery_stats(delivery_person_id, true).await?;
            self.update_delivery_person_availability(delivery_person_id, true).await?;
        } else if matches!(request.status, DeliveryStatus::Cancelled | DeliveryStatus::Failed) {
            self.update_delivery_stats(delivery_person_id, false).await?;
            self.update_delivery_person_availability(delivery_person_id, true).await?;
        }

        Ok(assignment)
    }

    pub async fn get_nearby_delivery_persons(&self, latitude: f64, longitude: f64, radius_km: f64) -> Result<Vec<NearbyDeliveryPersonResponse>> {
        // Using Haversine formula for distance calculation
        let query = r#"
            SELECT id, user_id, name, phone, email, vehicle_type as vehicle_type_str, 
                   vehicle_number, license_number, aadhar_number, pan_number, 
                   bank_account_number, ifsc_code, current_latitude, current_longitude,
                   is_available, is_verified, is_active, rating, total_deliveries,
                   successful_deliveries, average_delivery_time, earnings_today,
                   earnings_this_month, created_at, updated_at,
                   (6371 * acos(cos(radians($1)) * cos(radians(current_latitude)) * 
                   cos(radians(current_longitude) - radians($2)) + 
                   sin(radians($1)) * sin(radians(current_latitude)))) AS distance_km
            FROM delivery_persons 
            WHERE is_available = true 
              AND is_verified = true 
              AND is_active = true
              AND current_latitude IS NOT NULL 
              AND current_longitude IS NOT NULL
            HAVING distance_km <= $3
            ORDER BY distance_km ASC
            LIMIT 10
        "#;

        let rows = sqlx::query(query)
            .bind(latitude)
            .bind(longitude)
            .bind(radius_km)
            .fetch_all(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut nearby_persons = Vec::new();
        for row in rows {
            let distance_km: f64 = row.get("distance_km");
            let estimated_arrival_minutes = self.calculate_arrival_time(distance_km);

            // Convert row to DeliveryPerson manually
            let delivery_person = DeliveryPerson {
                id: row.get("id"),
                user_id: row.get("user_id"),
                name: row.get("name"),
                phone: row.get("phone"),
                email: row.get("email"),
                vehicle_type_str: row.get("vehicle_type_str"),
                vehicle_number: row.get("vehicle_number"),
                license_number: row.get("license_number"),
                aadhar_number: row.get("aadhar_number"),
                pan_number: row.get("pan_number"),
                bank_account_number: row.get("bank_account_number"),
                ifsc_code: row.get("ifsc_code"),
                current_latitude: row.get("current_latitude"),
                current_longitude: row.get("current_longitude"),
                is_available: row.get("is_available"),
                is_verified: row.get("is_verified"),
                is_active: row.get("is_active"),
                rating: row.get("rating"),
                total_deliveries: row.get("total_deliveries"),
                successful_deliveries: row.get("successful_deliveries"),
                average_delivery_time: row.get("average_delivery_time"),
                earnings_today: row.get("earnings_today"),
                earnings_this_month: row.get("earnings_this_month"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            };

            nearby_persons.push(NearbyDeliveryPersonResponse {
                delivery_person: DeliveryPersonResponse::from(delivery_person),
                distance_km,
                estimated_arrival_minutes,
            });
        }

        Ok(nearby_persons)
    }

    pub async fn get_delivery_person_stats(&self, delivery_person_id: Uuid, user_id: Uuid) -> Result<DeliveryStatsResponse> {
        // Verify ownership
        self.verify_delivery_person_ownership(delivery_person_id, user_id).await?;

        let delivery_person = self.get_delivery_person(delivery_person_id).await?;

        // Calculate additional stats
        let success_rate = if delivery_person.total_deliveries > 0 {
            (delivery_person.successful_deliveries as f64 / delivery_person.total_deliveries as f64) * 100.0
        } else {
            0.0
        };

        // Get weekly earnings (mock calculation for now)
        let earnings_this_week = delivery_person.earnings_today * 7.0; // Simplified

        Ok(DeliveryStatsResponse {
            total_deliveries: delivery_person.total_deliveries,
            successful_deliveries: delivery_person.successful_deliveries,
            success_rate,
            average_delivery_time: delivery_person.average_delivery_time,
            earnings_today: delivery_person.earnings_today,
            earnings_this_week,
            earnings_this_month: delivery_person.earnings_this_month,
            rating: delivery_person.rating,
            total_ratings: delivery_person.total_deliveries, // Simplified
        })
    }

    pub async fn get_delivery_assignments(&self, delivery_person_id: Uuid, user_id: Uuid, status: Option<DeliveryStatus>) -> Result<Vec<DeliveryAssignment>> {
        // Verify ownership
        self.verify_delivery_person_ownership(delivery_person_id, user_id).await?;

        let mut query = "SELECT id, order_id, delivery_person_id, restaurant_id, customer_id,
                                pickup_address, delivery_address, status as status_str, assigned_at,
                                accepted_at, picked_up_at, delivered_at, estimated_pickup_time,
                                estimated_delivery_time, actual_distance_km, delivery_fee,
                                tip_amount, delivery_notes, proof_of_delivery, created_at, updated_at
                         FROM delivery_assignments WHERE delivery_person_id = $1".to_string();
        let mut bind_count = 2;

        if status.is_some() {
            query.push_str(&format!(" AND status = ${}", bind_count));
            #[allow(unused_assignments)]
            {
                bind_count += 1;
            }
        }

        query.push_str(" ORDER BY created_at DESC");

        let mut query_builder = sqlx::query_as::<_, DeliveryAssignment>(&query);
        query_builder = query_builder.bind(delivery_person_id);

        if let Some(ref status_val) = status {
            query_builder = query_builder.bind(status_val.as_str());
        }

        let assignments = query_builder
            .fetch_all(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(assignments)
    }

    // India-specific delivery features
    pub async fn get_india_delivery_zones(&self) -> Result<Vec<IndiaDeliveryZone>> {
        Ok(vec![
            IndiaDeliveryZone {
                zone_name: "Metro Cities".to_string(),
                cities: vec![
                    "Mumbai".to_string(), "Delhi".to_string(), "Bangalore".to_string(),
                    "Chennai".to_string(), "Kolkata".to_string(), "Hyderabad".to_string()
                ],
                base_delivery_time_minutes: 30,
                peak_hour_surcharge: 15.0,
                weekend_surcharge: 10.0,
                festival_surcharge: 25.0,
                minimum_order_amount: 99.0,
            },
            IndiaDeliveryZone {
                zone_name: "Tier 1 Cities".to_string(),
                cities: vec![
                    "Pune".to_string(), "Ahmedabad".to_string(), "Jaipur".to_string(),
                    "Lucknow".to_string(), "Kanpur".to_string(), "Nagpur".to_string()
                ],
                base_delivery_time_minutes: 35,
                peak_hour_surcharge: 12.0,
                weekend_surcharge: 8.0,
                festival_surcharge: 20.0,
                minimum_order_amount: 79.0,
            },
            IndiaDeliveryZone {
                zone_name: "Tier 2 Cities".to_string(),
                cities: vec![
                    "Indore".to_string(), "Bhopal".to_string(), "Coimbatore".to_string(),
                    "Kochi".to_string(), "Chandigarh".to_string()
                ],
                base_delivery_time_minutes: 40,
                peak_hour_surcharge: 10.0,
                weekend_surcharge: 5.0,
                festival_surcharge: 15.0,
                minimum_order_amount: 59.0,
            },
        ])
    }

    pub async fn calculate_delivery_time_estimate(&self, pickup_lat: f64, pickup_lng: f64, delivery_lat: f64, delivery_lng: f64, city: &str) -> Result<DeliveryTimeEstimate> {
        // Get base time for city
        let zones = self.get_india_delivery_zones().await?;
        let base_time = zones.iter()
            .find(|zone| zone.cities.iter().any(|c| c.eq_ignore_ascii_case(city)))
            .map(|zone| zone.base_delivery_time_minutes)
            .unwrap_or(45); // Default for unknown cities

        // Calculate distance
        let distance_km = self.calculate_distance(pickup_lat, pickup_lng, delivery_lat, delivery_lng);
        
        // Add time based on distance (assuming 20 km/h average speed)
        let distance_time = (distance_km / 20.0 * 60.0) as i32;

        // Add various delays (simplified logic)
        let traffic_delay = self.get_traffic_delay(city).await;
        let weather_delay = 0; // Could integrate with weather API
        let peak_hour_delay = self.get_peak_hour_delay().await;

        let total_time = base_time + distance_time + traffic_delay + weather_delay + peak_hour_delay;
        let confidence = if distance_km < 5.0 { 0.9 } else { 0.7 };

        Ok(DeliveryTimeEstimate {
            base_time_minutes: base_time,
            traffic_delay_minutes: traffic_delay,
            weather_delay_minutes: weather_delay,
            peak_hour_delay_minutes: peak_hour_delay,
            total_estimated_minutes: total_time,
            confidence_level: confidence,
        })
    }

    // Helper methods
    async fn verify_delivery_person_ownership(&self, delivery_person_id: Uuid, user_id: Uuid) -> Result<()> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM delivery_persons WHERE id = $1 AND user_id = $2 AND is_active = true)"
        )
        .bind(delivery_person_id)
        .bind(user_id)
        .fetch_one(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !exists {
            return Err(AppError::Forbidden("Delivery person not found or not owned by user".to_string()));
        }

        Ok(())
    }

    async fn get_order_details(&self, order_id: Uuid) -> Result<OrderDetails> {
        // Mock implementation - in production, this would query the orders table
        Ok(OrderDetails {
            order_id,
            restaurant_id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            pickup_address: serde_json::json!({
                "street": "123 Restaurant Street",
                "city": "Mumbai",
                "latitude": 19.0760,
                "longitude": 72.8777
            }),
            delivery_address: serde_json::json!({
                "street": "456 Customer Street",
                "city": "Mumbai", 
                "latitude": 19.0825,
                "longitude": 72.8811
            }),
            delivery_fee: 29.0,
        })
    }

    async fn is_delivery_person_available(&self, delivery_person_id: Uuid) -> Result<bool> {
        let is_available = sqlx::query_scalar::<_, bool>(
            "SELECT is_available FROM delivery_persons WHERE id = $1 AND is_verified = true AND is_active = true"
        )
        .bind(delivery_person_id)
        .fetch_optional(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .unwrap_or(false);

        Ok(is_available)
    }

    async fn find_best_delivery_person(&self, order_details: &OrderDetails, max_distance_km: Option<f64>) -> Result<Uuid> {
        let pickup_lat: f64 = order_details.pickup_address["latitude"].as_f64().unwrap_or(0.0);
        let pickup_lng: f64 = order_details.pickup_address["longitude"].as_f64().unwrap_or(0.0);
        let max_dist = max_distance_km.unwrap_or(10.0);

        tracing::info!(
            "Finding delivery person for order {} at location ({}, {}) within {} km",
            order_details.order_id,
            pickup_lat,
            pickup_lng,
            max_dist
        );

        let nearby_persons = self.get_nearby_delivery_persons(pickup_lat, pickup_lng, max_dist).await?;
        
        if nearby_persons.is_empty() {
            tracing::warn!("No available delivery persons found for order {}", order_details.order_id);
            return Err(AppError::NotFound("No available delivery persons found".to_string()));
        }

        let selected_person_id = nearby_persons[0].delivery_person.id;
        tracing::info!(
            "Selected delivery person {} for order {}",
            selected_person_id,
            order_details.order_id
        );

        // Return the closest available delivery person
        Ok(selected_person_id)
    }

    async fn update_delivery_person_availability(&self, delivery_person_id: Uuid, is_available: bool) -> Result<()> {
        sqlx::query(
            "UPDATE delivery_persons SET is_available = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(is_available)
        .bind(Utc::now())
        .bind(delivery_person_id)
        .execute(self.db.pool())
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update_delivery_stats(&self, delivery_person_id: Uuid, successful: bool) -> Result<()> {
        let update_query = if successful {
            "UPDATE delivery_persons SET total_deliveries = total_deliveries + 1, successful_deliveries = successful_deliveries + 1, earnings_today = earnings_today + 50.0, earnings_this_month = earnings_this_month + 50.0, updated_at = $2 WHERE id = $1"
        } else {
            "UPDATE delivery_persons SET total_deliveries = total_deliveries + 1, updated_at = $2 WHERE id = $1"
        };

        sqlx::query(update_query)
            .bind(delivery_person_id)
            .bind(Utc::now())
            .execute(self.db.pool())
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    fn calculate_distance(&self, lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        // Haversine formula for calculating distance between two points
        let r = 6371.0; // Earth's radius in kilometers
        let d_lat = (lat2 - lat1).to_radians();
        let d_lng = (lng2 - lng1).to_radians();
        let a = (d_lat / 2.0).sin().powi(2) + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        r * c
    }

    fn calculate_arrival_time(&self, distance_km: f64) -> i32 {
        // Estimate arrival time based on distance and vehicle type
        // Assuming average speed of 25 km/h in Indian cities
        let time_hours = distance_km / 25.0;
        (time_hours * 60.0) as i32 // Convert to minutes
    }

    async fn get_traffic_delay(&self, city: &str) -> i32 {
        // Mock traffic delay calculation - in production, integrate with traffic APIs
        match city.to_lowercase().as_str() {
            "mumbai" | "delhi" | "bangalore" => 10, // High traffic cities
            "chennai" | "kolkata" | "hyderabad" => 8,
            "pune" | "ahmedabad" => 5,
            _ => 3,
        }
    }

    async fn get_peak_hour_delay(&self) -> i32 {
        let now = Utc::now();
        let hour = now.hour();
        
        // Peak hours: 12-2 PM and 7-9 PM IST
        if (hour >= 12 && hour <= 14) || (hour >= 19 && hour <= 21) {
            15 // 15 minutes extra during peak hours
        } else {
            0
        }
    }
}

// Helper struct for order details
#[derive(Debug)]
struct OrderDetails {
    order_id: Uuid,
    restaurant_id: Uuid,
    customer_id: Uuid,
    pickup_address: serde_json::Value,
    delivery_address: serde_json::Value,
    delivery_fee: f64,
}