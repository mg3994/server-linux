use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeliveryPerson {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    #[serde(skip)]
    pub vehicle_type_str: String, // Database field
    pub vehicle_number: String,
    pub license_number: String,
    pub aadhar_number: Option<String>, // Optional for privacy
    pub pan_number: Option<String>,    // Optional for tax purposes
    pub bank_account_number: Option<String>,
    pub ifsc_code: Option<String>,
    pub current_latitude: Option<f64>,
    pub current_longitude: Option<f64>,
    pub is_available: bool,
    pub is_verified: bool,
    pub is_active: bool,
    pub rating: f64,
    pub total_deliveries: i32,
    pub successful_deliveries: i32,
    pub average_delivery_time: Option<i32>, // in minutes
    pub earnings_today: f64,
    pub earnings_this_month: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DeliveryPerson {
    pub fn vehicle_type(&self) -> Result<VehicleType, String> {
        VehicleType::from_str(&self.vehicle_type_str)
    }
    
    pub fn set_vehicle_type(&mut self, vehicle_type: VehicleType) {
        self.vehicle_type_str = vehicle_type.as_str().to_string();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VehicleType {
    #[serde(rename = "bicycle")]
    Bicycle,
    #[serde(rename = "motorcycle")]
    Motorcycle,
    #[serde(rename = "scooter")]
    Scooter,
    #[serde(rename = "car")]
    Car,
    #[serde(rename = "van")]
    Van,
}

impl VehicleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            VehicleType::Bicycle => "bicycle",
            VehicleType::Motorcycle => "motorcycle",
            VehicleType::Scooter => "scooter",
            VehicleType::Car => "car",
            VehicleType::Van => "van",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "bicycle" => Ok(VehicleType::Bicycle),
            "motorcycle" => Ok(VehicleType::Motorcycle),
            "scooter" => Ok(VehicleType::Scooter),
            "car" => Ok(VehicleType::Car),
            "van" => Ok(VehicleType::Van),
            _ => Err(format!("Invalid vehicle type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeliveryAssignment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub delivery_person_id: Uuid,
    pub restaurant_id: Uuid,
    pub customer_id: Uuid,
    pub pickup_address: serde_json::Value,
    pub delivery_address: serde_json::Value,
    #[serde(skip)]
    pub status_str: String, // Database field
    pub assigned_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub picked_up_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub estimated_pickup_time: Option<DateTime<Utc>>,
    pub estimated_delivery_time: Option<DateTime<Utc>>,
    pub actual_distance_km: Option<f64>,
    pub delivery_fee: f64,
    pub tip_amount: Option<f64>,
    pub delivery_notes: Option<String>,
    pub proof_of_delivery: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DeliveryAssignment {
    pub fn status(&self) -> Result<DeliveryStatus, String> {
        DeliveryStatus::from_str(&self.status_str)
    }
    
    pub fn set_status(&mut self, status: DeliveryStatus) {
        self.status_str = status.as_str().to_string();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryStatus {
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "enroutetorestaurant")]
    EnRouteToRestaurant,
    #[serde(rename = "arrivedatrestaurant")]
    ArrivedAtRestaurant,
    #[serde(rename = "pickedup")]
    PickedUp,
    #[serde(rename = "enroutetocustomer")]
    EnRouteToCustomer,
    #[serde(rename = "arrivedatcustomer")]
    ArrivedAtCustomer,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
}

impl DeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeliveryStatus::Assigned => "assigned",
            DeliveryStatus::Accepted => "accepted",
            DeliveryStatus::EnRouteToRestaurant => "enroutetorestaurant",
            DeliveryStatus::ArrivedAtRestaurant => "arrivedatrestaurant",
            DeliveryStatus::PickedUp => "pickedup",
            DeliveryStatus::EnRouteToCustomer => "enroutetocustomer",
            DeliveryStatus::ArrivedAtCustomer => "arrivedatcustomer",
            DeliveryStatus::Delivered => "delivered",
            DeliveryStatus::Cancelled => "cancelled",
            DeliveryStatus::Failed => "failed",
        }
    }
    
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "assigned" => Ok(DeliveryStatus::Assigned),
            "accepted" => Ok(DeliveryStatus::Accepted),
            "enroutetorestaurant" => Ok(DeliveryStatus::EnRouteToRestaurant),
            "arrivedatrestaurant" => Ok(DeliveryStatus::ArrivedAtRestaurant),
            "pickedup" => Ok(DeliveryStatus::PickedUp),
            "enroutetocustomer" => Ok(DeliveryStatus::EnRouteToCustomer),
            "arrivedatcustomer" => Ok(DeliveryStatus::ArrivedAtCustomer),
            "delivered" => Ok(DeliveryStatus::Delivered),
            "cancelled" => Ok(DeliveryStatus::Cancelled),
            "failed" => Ok(DeliveryStatus::Failed),
            _ => Err(format!("Invalid delivery status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LocationUpdate {
    pub delivery_person_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: DateTime<Utc>,
    pub speed: Option<f64>, // km/h
    pub heading: Option<f64>, // degrees
}

// Request/Response Models
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterDeliveryPersonRequest {
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub vehicle_type: VehicleType,
    pub vehicle_number: String,
    pub license_number: String,
    pub aadhar_number: Option<String>,
    pub pan_number: Option<String>,
    pub bank_account_number: Option<String>,
    pub ifsc_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDeliveryPersonRequest {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub vehicle_type: Option<VehicleType>,
    pub vehicle_number: Option<String>,
    pub license_number: Option<String>,
    pub bank_account_number: Option<String>,
    pub ifsc_code: Option<String>,
    pub is_available: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLocationRequest {
    pub latitude: f64,
    pub longitude: f64,
    pub speed: Option<f64>,
    pub heading: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeliveryStatusRequest {
    pub status: DeliveryStatus,
    pub notes: Option<String>,
    pub proof_of_delivery: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct OrderAssignmentRequest {
    pub order_id: Uuid,
    pub preferred_delivery_person_id: Option<Uuid>,
    pub max_distance_km: Option<f64>,
}

// Response Models
#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryPersonResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub vehicle_type: VehicleType,
    pub vehicle_number: String,
    pub current_latitude: Option<f64>,
    pub current_longitude: Option<f64>,
    pub is_available: bool,
    pub is_verified: bool,
    pub rating: f64,
    pub total_deliveries: i32,
    pub successful_deliveries: i32,
    pub average_delivery_time: Option<i32>,
    pub earnings_today: f64,
    pub created_at: DateTime<Utc>,
}

impl From<DeliveryPerson> for DeliveryPersonResponse {
    fn from(person: DeliveryPerson) -> Self {
        let vehicle_type = person.vehicle_type().unwrap_or(VehicleType::Bicycle);
        Self {
            id: person.id,
            user_id: person.user_id,
            name: person.name,
            phone: person.phone,
            email: person.email,
            vehicle_type,
            vehicle_number: person.vehicle_number,
            current_latitude: person.current_latitude,
            current_longitude: person.current_longitude,
            is_available: person.is_available,
            is_verified: person.is_verified,
            rating: person.rating,
            total_deliveries: person.total_deliveries,
            successful_deliveries: person.successful_deliveries,
            average_delivery_time: person.average_delivery_time,
            earnings_today: person.earnings_today,
            created_at: person.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct DeliveryAssignmentResponse {
    pub id: Uuid,
    pub order_id: Uuid,
    pub delivery_person: DeliveryPersonResponse,
    pub pickup_address: serde_json::Value,
    pub delivery_address: serde_json::Value,
    pub status: DeliveryStatus,
    pub assigned_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub picked_up_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub estimated_pickup_time: Option<DateTime<Utc>>,
    pub estimated_delivery_time: Option<DateTime<Utc>>,
    pub actual_distance_km: Option<f64>,
    pub delivery_fee: f64,
    pub tip_amount: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct DeliveryPersonListResponse {
    pub delivery_persons: Vec<DeliveryPersonResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

#[derive(Debug, Serialize)]
pub struct DeliveryStatsResponse {
    pub total_deliveries: i32,
    pub successful_deliveries: i32,
    pub success_rate: f64,
    pub average_delivery_time: Option<i32>,
    pub earnings_today: f64,
    pub earnings_this_week: f64,
    pub earnings_this_month: f64,
    pub rating: f64,
    pub total_ratings: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearbyDeliveryPersonResponse {
    pub delivery_person: DeliveryPersonResponse,
    pub distance_km: f64,
    pub estimated_arrival_minutes: i32,
}

// India-specific delivery models
#[derive(Debug, Serialize, Deserialize)]
pub struct IndiaDeliveryZone {
    pub zone_name: String,
    pub cities: Vec<String>,
    pub base_delivery_time_minutes: i32,
    pub peak_hour_surcharge: f64,
    pub weekend_surcharge: f64,
    pub festival_surcharge: f64,
    pub minimum_order_amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryTimeEstimate {
    pub base_time_minutes: i32,
    pub traffic_delay_minutes: i32,
    pub weather_delay_minutes: i32,
    pub peak_hour_delay_minutes: i32,
    pub total_estimated_minutes: i32,
    pub confidence_level: f64, // 0.0 to 1.0
}