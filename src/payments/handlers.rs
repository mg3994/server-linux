use crate::auth::models::User;
use crate::error::Result;
use crate::payments::models::{CreatePaymentRequest, Payment, PaymentResponse, PaymentStatus};
use axum::{
    extract::Path,
    Extension, Json,
};
use uuid::Uuid;

pub async fn create_payment(
    Extension(user): Extension<User>,
    Json(payload): Json<CreatePaymentRequest>,
) -> Result<Json<PaymentResponse>> {

    // In a real application, integrate with payment processors like Stripe, PayPal, etc.
    let now = chrono::Utc::now();
    let payment = Payment {
        id: Uuid::new_v4(),
        order_id: payload.order_id,
        customer_id: user.id,
        amount: payload.amount,
        currency: payload.currency,
        status: PaymentStatus::Processing,
        payment_method: payload.payment_method,
        transaction_id: Some(format!("txn_{}", Uuid::new_v4())),
        created_at: now,
        updated_at: now,
    };

    tracing::info!(
        "Payment created: {} for order: {} by user: {}",
        payment.id,
        payment.order_id,
        user.id
    );

    // Simulate payment processing
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    Ok(Json(PaymentResponse {
        payment,
        message: "Payment initiated successfully".to_string(),
    }))
}

pub async fn get_payment(
    Extension(user): Extension<User>,
    Path(payment_id): Path<Uuid>,
) -> Result<Json<Payment>> {

    // Mock payment retrieval - in real app, fetch from database
    let now = chrono::Utc::now();
    let payment = Payment {
        id: payment_id,
        order_id: Uuid::new_v4(),
        customer_id: user.id,
        amount: 25.99,
        currency: "USD".to_string(),
        status: PaymentStatus::Completed,
        payment_method: crate::payments::models::PaymentMethod::CreditCard,
        transaction_id: Some(format!("txn_{}", Uuid::new_v4())),
        created_at: now - chrono::Duration::minutes(5),
        updated_at: now,
    };

    Ok(Json(payment))
}