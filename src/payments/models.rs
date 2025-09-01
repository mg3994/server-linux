use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub order_id: Uuid,
    pub customer_id: Uuid,
    pub amount: f64,
    pub currency: String,
    pub status: PaymentStatus,
    pub payment_method: PaymentMethod,
    pub transaction_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    CreditCard,
    DebitCard,
    DigitalWallet,
    BankTransfer,
    Cash,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaymentRequest {
    pub order_id: Uuid,
    pub payment_method: PaymentMethod,
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentResponse {
    pub payment: Payment,
    pub message: String,
}