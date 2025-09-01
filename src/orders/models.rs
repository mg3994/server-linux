use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub restaurant_id: Uuid,
    pub delivery_person_id: Option<Uuid>,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub delivery_address: Address,
    pub restaurant_address: Address,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub estimated_delivery_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub menu_item_id: Uuid,
    pub name: String,
    pub quantity: u32,
    pub unit_price: f64,
    pub total_price: f64,
    pub customizations: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Placed,
    Confirmed,
    Preparing,
    Ready,
    PickedUp,
    OnTheWay,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderRequest {
    pub restaurant_id: Uuid,
    pub items: Vec<CreateOrderItem>,
    pub delivery_address: Address,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderItem {
    pub menu_item_id: Uuid,
    pub quantity: u32,
    pub customizations: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrderStatusRequest {
    pub status: OrderStatus,
}

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub order: Order,
    pub message: String,
}