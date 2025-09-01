use crate::auth::models::User;
use crate::error::Result;
use crate::notifications::fcm::FCMService;
use crate::orders::models::{
    CreateOrderRequest, Order, OrderItem, OrderResponse, OrderStatus, UpdateOrderStatusRequest,
};
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub type SharedFCMService = Arc<Mutex<FCMService>>;

pub async fn create_order(
    State(fcm_service): State<SharedFCMService>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateOrderRequest>,
) -> Result<Json<OrderResponse>> {

    // Create order items with pricing (in real app, fetch from database)
    let mut total_amount = 0.0;
    let order_items: Vec<OrderItem> = payload
        .items
        .into_iter()
        .map(|item| {
            let unit_price = 15.99; // Mock price - fetch from menu in real app
            let total_price = unit_price * item.quantity as f64;
            total_amount += total_price;

            OrderItem {
                id: Uuid::new_v4(),
                menu_item_id: item.menu_item_id,
                name: format!("Menu Item {}", item.menu_item_id), // Mock name
                quantity: item.quantity,
                unit_price,
                total_price,
                customizations: item.customizations,
            }
        })
        .collect();

    let now = chrono::Utc::now();
    let order = Order {
        id: Uuid::new_v4(),
        customer_id: user.id,
        restaurant_id: payload.restaurant_id,
        delivery_person_id: None,
        items: order_items,
        status: OrderStatus::Placed,
        total_amount,
        delivery_address: payload.delivery_address,
        restaurant_address: crate::orders::models::Address {
            street: "123 Restaurant St".to_string(),
            city: "Food City".to_string(),
            state: "FC".to_string(),
            postal_code: "12345".to_string(),
            country: "US".to_string(),
            latitude: Some(40.7128),
            longitude: Some(-74.0060),
        },
        created_at: now,
        updated_at: now,
        estimated_delivery_time: Some(now + chrono::Duration::minutes(30)),
    };

    // Send notifications (mock tokens - in real app, fetch from database)
    let customer_token = "customer_device_token";
    let restaurant_token = "restaurant_device_token";

    if let Ok(mut fcm) = fcm_service.try_lock() {
        if let Err(e) = fcm
            .notify_order_placed(order.id, customer_token, restaurant_token)
            .await
        {
            tracing::error!("Failed to send order placed notifications: {:?}", e);
            // Don't fail the order creation if notifications fail
        }
    } else {
        tracing::warn!("FCM service is busy, skipping notifications");
    }

    tracing::info!("Order created: {} for user: {}", order.id, user.id);

    Ok(Json(OrderResponse {
        order,
        message: "Order created successfully".to_string(),
    }))
}

pub async fn get_order(
    Extension(user): Extension<User>,
    Path(order_id): Path<Uuid>,
) -> Result<Json<Order>> {

    // Mock order retrieval - in real app, fetch from database
    let now = chrono::Utc::now();
    let order = Order {
        id: order_id,
        customer_id: user.id,
        restaurant_id: Uuid::new_v4(),
        delivery_person_id: Some(Uuid::new_v4()),
        items: vec![],
        status: OrderStatus::Preparing,
        total_amount: 25.99,
        delivery_address: crate::orders::models::Address {
            street: "456 Customer Ave".to_string(),
            city: "User City".to_string(),
            state: "UC".to_string(),
            postal_code: "67890".to_string(),
            country: "US".to_string(),
            latitude: Some(40.7589),
            longitude: Some(-73.9851),
        },
        restaurant_address: crate::orders::models::Address {
            street: "123 Restaurant St".to_string(),
            city: "Food City".to_string(),
            state: "FC".to_string(),
            postal_code: "12345".to_string(),
            country: "US".to_string(),
            latitude: Some(40.7128),
            longitude: Some(-74.0060),
        },
        created_at: now - chrono::Duration::minutes(15),
        updated_at: now,
        estimated_delivery_time: Some(now + chrono::Duration::minutes(15)),
    };

    Ok(Json(order))
}

pub async fn update_order_status(
    State(fcm_service): State<SharedFCMService>,
    Extension(user): Extension<User>,
    Path(order_id): Path<Uuid>,
    Json(payload): Json<UpdateOrderStatusRequest>,
) -> Result<Json<OrderResponse>> {

    // In real app, update order in database and verify permissions
    tracing::info!(
        "Updating order {} status to {:?} by user {}",
        order_id,
        payload.status,
        user.id
    );

    // Send appropriate notifications based on status
    match payload.status {
        OrderStatus::Ready => {
            let customer_token = "customer_device_token";
            let delivery_token = "delivery_device_token";

            if let Ok(mut fcm) = fcm_service.try_lock() {
                if let Err(e) = fcm
                    .notify_order_ready(order_id, customer_token, delivery_token)
                    .await
                {
                    tracing::error!("Failed to send order ready notifications: {:?}", e);
                }
            } else {
                tracing::warn!("FCM service is busy, skipping notifications");
            }
        }
        _ => {
            // Handle other status updates with appropriate notifications
            tracing::info!("Status updated to {:?} for order {}", payload.status, order_id);
        }
    }

    // Mock updated order
    let now = chrono::Utc::now();
    let order = Order {
        id: order_id,
        customer_id: user.id,
        restaurant_id: Uuid::new_v4(),
        delivery_person_id: Some(Uuid::new_v4()),
        items: vec![],
        status: payload.status,
        total_amount: 25.99,
        delivery_address: crate::orders::models::Address {
            street: "456 Customer Ave".to_string(),
            city: "User City".to_string(),
            state: "UC".to_string(),
            postal_code: "67890".to_string(),
            country: "US".to_string(),
            latitude: Some(40.7589),
            longitude: Some(-73.9851),
        },
        restaurant_address: crate::orders::models::Address {
            street: "123 Restaurant St".to_string(),
            city: "Food City".to_string(),
            state: "FC".to_string(),
            postal_code: "12345".to_string(),
            country: "US".to_string(),
            latitude: Some(40.7128),
            longitude: Some(-74.0060),
        },
        created_at: now - chrono::Duration::minutes(20),
        updated_at: now,
        estimated_delivery_time: Some(now + chrono::Duration::minutes(10)),
    };

    Ok(Json(OrderResponse {
        order,
        message: "Order status updated successfully".to_string(),
    }))
}pub 
async fn get_customer_orders(
    Path(customer_id): Path<Uuid>,
    Extension(user): Extension<User>,
) -> Result<Json<Vec<Order>>> {
    // Ensure user can only access their own orders or is authorized
    if user.id != customer_id {
        return Err(crate::error::AppError::Unauthorized);
    }

    // Mock customer orders - in real app, fetch from database
    let now = chrono::Utc::now();
    let orders = vec![
        Order {
            id: Uuid::new_v4(),
            customer_id,
            restaurant_id: Uuid::new_v4(),
            delivery_person_id: Some(Uuid::new_v4()),
            items: vec![],
            status: OrderStatus::Delivered,
            total_amount: 25.99,
            delivery_address: crate::orders::models::Address {
                street: "456 Customer Ave".to_string(),
                city: "User City".to_string(),
                state: "UC".to_string(),
                postal_code: "67890".to_string(),
                country: "US".to_string(),
                latitude: Some(40.7589),
                longitude: Some(-73.9851),
            },
            restaurant_address: crate::orders::models::Address {
                street: "123 Restaurant St".to_string(),
                city: "Food City".to_string(),
                state: "FC".to_string(),
                postal_code: "12345".to_string(),
                country: "US".to_string(),
                latitude: Some(40.7128),
                longitude: Some(-74.0060),
            },
            created_at: now - chrono::Duration::hours(2),
            updated_at: now - chrono::Duration::hours(1),
            estimated_delivery_time: Some(now - chrono::Duration::hours(1)),
        },
        Order {
            id: Uuid::new_v4(),
            customer_id,
            restaurant_id: Uuid::new_v4(),
            delivery_person_id: None,
            items: vec![],
            status: OrderStatus::Preparing,
            total_amount: 18.50,
            delivery_address: crate::orders::models::Address {
                street: "456 Customer Ave".to_string(),
                city: "User City".to_string(),
                state: "UC".to_string(),
                postal_code: "67890".to_string(),
                country: "US".to_string(),
                latitude: Some(40.7589),
                longitude: Some(-73.9851),
            },
            restaurant_address: crate::orders::models::Address {
                street: "789 Food Plaza".to_string(),
                city: "Food City".to_string(),
                state: "FC".to_string(),
                postal_code: "12345".to_string(),
                country: "US".to_string(),
                latitude: Some(40.7505),
                longitude: Some(-73.9934),
            },
            created_at: now - chrono::Duration::minutes(30),
            updated_at: now - chrono::Duration::minutes(15),
            estimated_delivery_time: Some(now + chrono::Duration::minutes(20)),
        },
    ];

    Ok(Json(orders))
}