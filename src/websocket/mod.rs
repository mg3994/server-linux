use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State, Path,
    },
    response::Response,
    Extension,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;
use tracing::{info, warn, error};

use crate::auth::models::User;
use crate::orders::models::OrderStatus;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    #[serde(rename = "order_status_update")]
    OrderStatusUpdate {
        order_id: Uuid,
        status: OrderStatus,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    #[serde(rename = "delivery_location_update")]
    DeliveryLocationUpdate {
        order_id: Uuid,
        latitude: f64,
        longitude: f64,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    #[serde(rename = "notification")]
    Notification {
        title: String,
        message: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

#[derive(Clone)]
pub struct WebSocketManager {
    connections: Arc<RwLock<HashMap<Uuid, broadcast::Sender<WebSocketMessage>>>>,
    #[allow(dead_code)]
    global_sender: broadcast::Sender<WebSocketMessage>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (global_sender, _) = broadcast::channel(1000);
        
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            global_sender,
        }
    }

    pub async fn add_connection(&self, user_id: Uuid) -> broadcast::Receiver<WebSocketMessage> {
        let (sender, receiver) = broadcast::channel(100);
        
        {
            let mut connections = self.connections.write().await;
            connections.insert(user_id, sender);
        }
        
        info!("WebSocket connection added for user: {}", user_id);
        receiver
    }

    pub async fn remove_connection(&self, user_id: Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(&user_id);
        info!("WebSocket connection removed for user: {}", user_id);
    }

    pub async fn send_to_user(&self, user_id: Uuid, message: WebSocketMessage) -> Result<()> {
        let connections = self.connections.read().await;
        
        if let Some(sender) = connections.get(&user_id) {
            if let Err(e) = sender.send(message) {
                warn!("Failed to send message to user {}: {}", user_id, e);
            }
        }
        
        Ok(())
    }

    pub async fn broadcast(&self, message: WebSocketMessage) -> Result<()> {
        let connections = self.connections.read().await;
        
        for (user_id, sender) in connections.iter() {
            if let Err(e) = sender.send(message.clone()) {
                warn!("Failed to broadcast message to user {}: {}", user_id, e);
            }
        }
        
        Ok(())
    }

    pub async fn send_order_update(&self, user_id: Uuid, order_id: Uuid, status: OrderStatus) -> Result<()> {
        let message = WebSocketMessage::OrderStatusUpdate {
            order_id,
            status,
            timestamp: chrono::Utc::now(),
        };
        
        self.send_to_user(user_id, message).await
    }

    pub async fn send_location_update(&self, user_id: Uuid, order_id: Uuid, latitude: f64, longitude: f64) -> Result<()> {
        let message = WebSocketMessage::DeliveryLocationUpdate {
            order_id,
            latitude,
            longitude,
            timestamp: chrono::Utc::now(),
        };
        
        self.send_to_user(user_id, message).await
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(user): Extension<User>,
    State(ws_manager): State<WebSocketManager>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, user, ws_manager))
}

async fn handle_socket(socket: WebSocket, user: User, ws_manager: WebSocketManager) {
    let user_id = user.id;
    let mut receiver = ws_manager.add_connection(user_id).await;
    
    let (sender, mut recv) = socket.split();
    let sender = Arc::new(tokio::sync::Mutex::new(sender));
    
    // Spawn task to handle incoming messages from client
    let sender_clone = sender.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(msg) = recv.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    match serde_json::from_str::<WebSocketMessage>(&text) {
                        Ok(WebSocketMessage::Ping) => {
                            let pong = WebSocketMessage::Pong;
                            if let Ok(pong_text) = serde_json::to_string(&pong) {
                                let mut sender_guard = sender_clone.lock().await;
                                if let Err(e) = sender_guard.send(Message::Text(pong_text.into())).await {
                                    error!("Failed to send pong: {}", e);
                                    break;
                                }
                            }
                        }
                        Ok(msg) => {
                            info!("Received WebSocket message from user {}: {:?}", user_id, msg);
                        }
                        Err(e) => {
                            warn!("Failed to parse WebSocket message: {}", e);
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed by user: {}", user_id);
                    break;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Spawn task to handle outgoing messages to client
    let sender_clone = sender.clone();
    let outgoing_task = tokio::spawn(async move {
        while let Ok(message) = receiver.recv().await {
            match serde_json::to_string(&message) {
                Ok(text) => {
                    let mut sender_guard = sender_clone.lock().await;
                    if let Err(e) = sender_guard.send(Message::Text(text.into())).await {
                        error!("Failed to send WebSocket message: {}", e);
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to serialize WebSocket message: {}", e);
                }
            }
        }
    });
    
    // Wait for either task to complete
    tokio::select! {
        _ = incoming_task => {
            info!("Incoming WebSocket task completed for user: {}", user_id);
        }
        _ = outgoing_task => {
            info!("Outgoing WebSocket task completed for user: {}", user_id);
        }
    }
    
    // Clean up connection
    ws_manager.remove_connection(user_id).await;
}

pub async fn location_update_handler(
    Path(order_id): Path<Uuid>,
    Extension(_user): Extension<User>,
    State(ws_manager): State<WebSocketManager>,
    axum::Json(payload): axum::Json<LocationUpdateRequest>,
) -> Result<axum::Json<LocationUpdateResponse>> {
    // Send location update to customer
    ws_manager.send_location_update(
        payload.customer_id,
        order_id,
        payload.latitude,
        payload.longitude,
    ).await?;
    
    Ok(axum::Json(LocationUpdateResponse {
        success: true,
        message: "Location update sent".to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct LocationUpdateRequest {
    pub customer_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize)]
pub struct LocationUpdateResponse {
    pub success: bool,
    pub message: String,
}