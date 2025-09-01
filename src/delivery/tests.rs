#[cfg(test)]
mod tests {
    use crate::database::Database;
    use crate::delivery::models::*;
    use uuid::Uuid;
    use chrono::Utc;

    async fn setup_test_db() -> Database {
        // This would be a test database setup
        // For now, we'll mock it
        todo!("Setup test database")
    }

    #[tokio::test]
    async fn test_vehicle_type_conversion() {
        let vehicle_type = VehicleType::Motorcycle;
        assert_eq!(vehicle_type.as_str(), "motorcycle");
        
        let parsed = VehicleType::from_str("motorcycle").unwrap();
        assert!(matches!(parsed, VehicleType::Motorcycle));
        
        let invalid = VehicleType::from_str("invalid");
        assert!(invalid.is_err());
    }

    #[tokio::test]
    async fn test_delivery_status_conversion() {
        let status = DeliveryStatus::EnRouteToRestaurant;
        assert_eq!(status.as_str(), "enroutetorestaurant");
        
        let parsed = DeliveryStatus::from_str("enroutetorestaurant").unwrap();
        assert!(matches!(parsed, DeliveryStatus::EnRouteToRestaurant));
        
        let invalid = DeliveryStatus::from_str("invalid");
        assert!(invalid.is_err());
    }

    #[tokio::test]
    async fn test_delivery_person_vehicle_type_methods() {
        let mut delivery_person = DeliveryPerson {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Test Driver".to_string(),
            phone: "+91-9876543210".to_string(),
            email: Some("test@example.com".to_string()),
            vehicle_type_str: "motorcycle".to_string(),
            vehicle_number: "MH01AB1234".to_string(),
            license_number: "MH0120230001234".to_string(),
            aadhar_number: None,
            pan_number: None,
            bank_account_number: None,
            ifsc_code: None,
            current_latitude: Some(19.0760),
            current_longitude: Some(72.8777),
            is_available: true,
            is_verified: true,
            is_active: true,
            rating: 4.5,
            total_deliveries: 100,
            successful_deliveries: 95,
            average_delivery_time: Some(25),
            earnings_today: 500.0,
            earnings_this_month: 15000.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Test getting vehicle type
        let vehicle_type = delivery_person.vehicle_type().unwrap();
        assert!(matches!(vehicle_type, VehicleType::Motorcycle));

        // Test setting vehicle type
        delivery_person.set_vehicle_type(VehicleType::Car);
        assert_eq!(delivery_person.vehicle_type_str, "car");
    }

    #[tokio::test]
    async fn test_delivery_assignment_status_methods() {
        let mut assignment = DeliveryAssignment {
            id: Uuid::new_v4(),
            order_id: Uuid::new_v4(),
            delivery_person_id: Uuid::new_v4(),
            restaurant_id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            pickup_address: serde_json::json!({"address": "Restaurant Address"}),
            delivery_address: serde_json::json!({"address": "Customer Address"}),
            status_str: "assigned".to_string(),
            assigned_at: Utc::now(),
            accepted_at: None,
            picked_up_at: None,
            delivered_at: None,
            estimated_pickup_time: None,
            estimated_delivery_time: None,
            actual_distance_km: None,
            delivery_fee: 30.0,
            tip_amount: None,
            delivery_notes: None,
            proof_of_delivery: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Test getting status
        let status = assignment.status().unwrap();
        assert!(matches!(status, DeliveryStatus::Assigned));

        // Test setting status
        assignment.set_status(DeliveryStatus::Accepted);
        assert_eq!(assignment.status_str, "accepted");
    }

    #[tokio::test]
    async fn test_delivery_person_response_conversion() {
        let delivery_person = DeliveryPerson {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Test Driver".to_string(),
            phone: "+91-9876543210".to_string(),
            email: Some("test@example.com".to_string()),
            vehicle_type_str: "motorcycle".to_string(),
            vehicle_number: "MH01AB1234".to_string(),
            license_number: "MH0120230001234".to_string(),
            aadhar_number: None,
            pan_number: None,
            bank_account_number: None,
            ifsc_code: None,
            current_latitude: Some(19.0760),
            current_longitude: Some(72.8777),
            is_available: true,
            is_verified: true,
            is_active: true,
            rating: 4.5,
            total_deliveries: 100,
            successful_deliveries: 95,
            average_delivery_time: Some(25),
            earnings_today: 500.0,
            earnings_this_month: 15000.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let response = DeliveryPersonResponse::from(delivery_person.clone());
        assert_eq!(response.id, delivery_person.id);
        assert_eq!(response.name, delivery_person.name);
        assert!(matches!(response.vehicle_type, VehicleType::Motorcycle));
    }

    #[tokio::test]
    async fn test_register_delivery_person_request_validation() {
        let request = RegisterDeliveryPersonRequest {
            name: "Test Driver".to_string(),
            phone: "+91-9876543210".to_string(),
            email: Some("test@example.com".to_string()),
            vehicle_type: VehicleType::Motorcycle,
            vehicle_number: "MH01AB1234".to_string(),
            license_number: "MH0120230001234".to_string(),
            aadhar_number: Some("123456789012".to_string()),
            pan_number: Some("ABCDE1234F".to_string()),
            bank_account_number: Some("1234567890123456".to_string()),
            ifsc_code: Some("SBIN0001234".to_string()),
        };

        // Test serialization
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("motorcycle"));
        assert!(json.contains("Test Driver"));

        // Test deserialization
        let deserialized: RegisterDeliveryPersonRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.name, request.name);
        assert!(matches!(deserialized.vehicle_type, VehicleType::Motorcycle));
    }

    #[tokio::test]
    async fn test_update_delivery_status_request() {
        let request = UpdateDeliveryStatusRequest {
            status: DeliveryStatus::PickedUp,
            notes: Some("Food picked up successfully".to_string()),
            proof_of_delivery: Some(serde_json::json!({"photo": "base64_image_data"})),
        };

        // Test serialization
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("pickedup"));

        // Test deserialization
        let deserialized: UpdateDeliveryStatusRequest = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized.status, DeliveryStatus::PickedUp));
        assert_eq!(deserialized.notes, request.notes);
    }

    #[tokio::test]
    async fn test_india_delivery_zone_structure() {
        let zone = IndiaDeliveryZone {
            zone_name: "Mumbai Metro".to_string(),
            cities: vec!["Mumbai".to_string(), "Navi Mumbai".to_string(), "Thane".to_string()],
            base_delivery_time_minutes: 30,
            peak_hour_surcharge: 15.0,
            weekend_surcharge: 10.0,
            festival_surcharge: 25.0,
            minimum_order_amount: 150.0,
        };

        // Test serialization
        let json = serde_json::to_string(&zone).unwrap();
        assert!(json.contains("Mumbai Metro"));
        assert!(json.contains("Mumbai"));

        // Test deserialization
        let deserialized: IndiaDeliveryZone = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.zone_name, zone.zone_name);
        assert_eq!(deserialized.cities.len(), 3);
        assert_eq!(deserialized.base_delivery_time_minutes, 30);
    }

    #[tokio::test]
    async fn test_delivery_time_estimate() {
        let estimate = DeliveryTimeEstimate {
            base_time_minutes: 25,
            traffic_delay_minutes: 5,
            weather_delay_minutes: 0,
            peak_hour_delay_minutes: 10,
            total_estimated_minutes: 40,
            confidence_level: 0.85,
        };

        // Test serialization
        let json = serde_json::to_string(&estimate).unwrap();
        assert!(json.contains("40"));
        assert!(json.contains("0.85"));

        // Test deserialization
        let deserialized: DeliveryTimeEstimate = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.total_estimated_minutes, 40);
        assert_eq!(deserialized.confidence_level, 0.85);
    }

    #[tokio::test]
    async fn test_nearby_delivery_person_response() {
        let delivery_person = DeliveryPerson {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Test Driver".to_string(),
            phone: "+91-9876543210".to_string(),
            email: Some("test@example.com".to_string()),
            vehicle_type_str: "motorcycle".to_string(),
            vehicle_number: "MH01AB1234".to_string(),
            license_number: "MH0120230001234".to_string(),
            aadhar_number: None,
            pan_number: None,
            bank_account_number: None,
            ifsc_code: None,
            current_latitude: Some(19.0760),
            current_longitude: Some(72.8777),
            is_available: true,
            is_verified: true,
            is_active: true,
            rating: 4.5,
            total_deliveries: 100,
            successful_deliveries: 95,
            average_delivery_time: Some(25),
            earnings_today: 500.0,
            earnings_this_month: 15000.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let nearby_response = NearbyDeliveryPersonResponse {
            delivery_person: DeliveryPersonResponse::from(delivery_person),
            distance_km: 2.5,
            estimated_arrival_minutes: 8,
        };

        // Test serialization
        let json = serde_json::to_string(&nearby_response).unwrap();
        assert!(json.contains("2.5"));
        assert!(json.contains("Test Driver"));

        // Test deserialization
        let deserialized: NearbyDeliveryPersonResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.distance_km, 2.5);
        assert_eq!(deserialized.estimated_arrival_minutes, 8);
    }
}