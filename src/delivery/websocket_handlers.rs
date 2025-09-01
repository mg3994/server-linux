use axum::{
    extract::{Path, Query, State},
    response::Response,
    extract::ws::WebSocketUpgrade,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::Utc;

use crate::auth::models::User;
use crate::delivery::websocket::{DeliveryWebSocketManager, DeliveryWebSocketConnection};
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
    pub role: Option<String>,
    pub delivery_person_id: Option<Uuid>,
    pub restaurant_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
}

pub async fn delivery_websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<WebSocketQuery>,
    State(ws_manager): State<DeliveryWebSocketManager>,
    auth_user: User,
) -> Response {
    let connection_id = Uuid::new_v4();
    
    // Determine role from query params or user context
    let role = params.role.unwrap_or_else(|| {
        // Default role determination logic
        if params.delivery_person_id.is_some() {
            "delivery_person".to_string()
        } else if params.restaurant_id.is_some() {
            "restaurant".to_string()
        } else if params.customer_id.is_some() {
            "customer".to_string()
        } else {
            "user".to_string()
        }
    });

    let connection = DeliveryWebSocketConnection {
        user_id: auth_user.id,
        role,
        delivery_person_id: params.delivery_person_id,
        restaurant_id: params.restaurant_id,
        customer_id: params.customer_id,
        connected_at: Utc::now(),
    };

    let ws_manager_clone = ws_manager.clone();
    ws.on_upgrade(move |socket| async move {
        ws_manager_clone.handle_websocket_connection(socket, connection_id, connection).await
    })
}

pub async fn admin_websocket_handler(
    ws: WebSocketUpgrade,
    State(ws_manager): State<DeliveryWebSocketManager>,
    auth_user: User,
) -> Response {
    let connection_id = Uuid::new_v4();
    
    let connection = DeliveryWebSocketConnection {
        user_id: auth_user.id,
        role: "admin".to_string(),
        delivery_person_id: None,
        restaurant_id: None,
        customer_id: None,
        connected_at: Utc::now(),
    };

    let ws_manager_clone = ws_manager.clone();
    ws.on_upgrade(move |socket| async move {
        ws_manager_clone.handle_websocket_connection(socket, connection_id, connection).await
    })
}

pub async fn delivery_person_websocket_handler(
    ws: WebSocketUpgrade,
    Path(delivery_person_id): Path<Uuid>,
    State(ws_manager): State<DeliveryWebSocketManager>,
    auth_user: User,
) -> Response {
    let connection_id = Uuid::new_v4();
    
    let connection = DeliveryWebSocketConnection {
        user_id: auth_user.id,
        role: "delivery_person".to_string(),
        delivery_person_id: Some(delivery_person_id),
        restaurant_id: None,
        customer_id: None,
        connected_at: Utc::now(),
    };

    let ws_manager_clone = ws_manager.clone();
    ws.on_upgrade(move |socket| async move {
        ws_manager_clone.handle_websocket_connection(socket, connection_id, connection).await
    })
}

pub async fn restaurant_websocket_handler(
    ws: WebSocketUpgrade,
    Path(restaurant_id): Path<Uuid>,
    State(ws_manager): State<DeliveryWebSocketManager>,
    auth_user: User,
) -> Response {
    let connection_id = Uuid::new_v4();
    
    let connection = DeliveryWebSocketConnection {
        user_id: auth_user.id,
        role: "restaurant".to_string(),
        delivery_person_id: None,
        restaurant_id: Some(restaurant_id),
        customer_id: None,
        connected_at: Utc::now(),
    };

    let ws_manager_clone = ws_manager.clone();
    ws.on_upgrade(move |socket| async move {
        ws_manager_clone.handle_websocket_connection(socket, connection_id, connection).await
    })
}

pub async fn customer_websocket_handler(
    ws: WebSocketUpgrade,
    Path(customer_id): Path<Uuid>,
    State(ws_manager): State<DeliveryWebSocketManager>,
    auth_user: User,
) -> Response {
    let connection_id = Uuid::new_v4();
    
    let connection = DeliveryWebSocketConnection {
        user_id: auth_user.id,
        role: "customer".to_string(),
        delivery_person_id: None,
        restaurant_id: None,
        customer_id: Some(customer_id),
        connected_at: Utc::now(),
    };

    let ws_manager_clone = ws_manager.clone();
    ws.on_upgrade(move |socket| async move {
        ws_manager_clone.handle_websocket_connection(socket, connection_id, connection).await
    })
}

pub async fn get_websocket_stats(
    State(ws_manager): State<DeliveryWebSocketManager>,
    _auth_user: User,
) -> Result<axum::Json<serde_json::Value>> {
    let total_connections = ws_manager.get_active_connections().await;
    
    Ok(axum::Json(serde_json::json!({
        "total_connections": total_connections,
        "status": "active",
        "timestamp": Utc::now()
    })))
}

pub async fn broadcast_test_message(
    State(ws_manager): State<DeliveryWebSocketManager>,
    _auth_user: User,
) -> Result<axum::Json<serde_json::Value>> {
    // Broadcast a test emergency alert
    let test_delivery_person_id = Uuid::new_v4();
    let _ = ws_manager
        .broadcast_emergency_alert(
            test_delivery_person_id,
            12.9716, // Bangalore latitude
            77.5946, // Bangalore longitude
            "Test emergency alert from WebSocket API".to_string(),
        )
        .await;

    Ok(axum::Json(serde_json::json!({
        "message": "Test emergency alert broadcasted",
        "delivery_person_id": test_delivery_person_id,
        "timestamp": Utc::now()
    })))
}