use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderNotification {
    pub order_id: Uuid,
    pub notification_type: NotificationType,
    pub recipient_type: RecipientType,
    pub payload: NotificationPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    OrderPlaced,
    OrderConfirmed,
    OrderPreparing,
    OrderReady,
    OrderPickedUp,
    OrderDelivered,
    OrderCancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecipientType {
    Customer,
    Restaurant,
    DeliveryPerson,
}

// FCM message structures are now internal to the FCM service