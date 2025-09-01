use axum::extract::ws::{Message, WebSocket};
use chrono::{DateTime, Utc};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::delivery::models::{DeliveryStatus, LocationUpdate};
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DeliveryWebSocketMessage {
    #[serde(rename = "location_update")]
    LocationUpdate {
        delivery_person_id: Uuid,
        latitude: f64,
        longitude: f64,
        speed: Option<f64>,
        heading: Option<f64>,
        timestamp: DateTime<Utc>,
    },
    #[serde(rename = "status_update")]
    StatusUpdate {
        assignment_id: Uuid,
        delivery_person_id: Uuid,
        status: String,
        estimated_arrival: Option<DateTime<Utc>>,
        notes: Option<String>,
    },
    #[serde(rename = "order_assigned")]
    OrderAssigned {
        assignment_id: Uuid,
        delivery_person_id: Uuid,
        order_id: Uuid,
        pickup_address: serde_json::Value,
        delivery_address: serde_json::Value,
        estimated_pickup_time: Option<DateTime<Utc>>,
        estimated_delivery_time: Option<DateTime<Utc>>,
    },
    #[serde(rename = "delivery_person_online")]
    DeliveryPersonOnline {
        delivery_person_id: Uuid,
        latitude: f64,
        longitude: f64,
    },
    #[serde(rename = "delivery_person_offline")]
    DeliveryPersonOffline { delivery_person_id: Uuid },
    #[serde(rename = "emergency_alert")]
    EmergencyAlert {
        delivery_person_id: Uuid,
        latitude: f64,
        longitude: f64,
        message: String,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone)]
pub struct DeliveryWebSocketConnection {
    pub user_id: Uuid,
    pub role: String,
    pub delivery_person_id: Option<Uuid>,
    pub restaurant_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub connected_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct DeliveryWebSocketManager {
    connections: Arc<RwLock<HashMap<Uuid, DeliveryWebSocketConnection>>>,
    broadcast_tx: broadcast::Sender<DeliveryWebSocketMessage>,
}

impl DeliveryWebSocketManager {
    pub fn new() -> Self {
        let (broadcast_tx, _) = broadcast::channel(1000);

        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx,
        }
    }

    pub async fn add_connection(
        &self,
        connection_id: Uuid,
        connection: DeliveryWebSocketConnection,
    ) {
        let mut connections = self.connections.write().await;
        connections.insert(connection_id, connection);
    }

    pub async fn remove_connection(&self, connection_id: &Uuid) {
        let mut connections = self.connections.write().await;
        if let Some(connection) = connections.remove(connection_id) {
            // Send offline notification if it's a delivery person
            if let Some(delivery_person_id) = connection.delivery_person_id {
                let _ = self
                    .broadcast_tx
                    .send(DeliveryWebSocketMessage::DeliveryPersonOffline { delivery_person_id });
            }
        }
    }

    pub async fn broadcast_location_update(&self, location_update: LocationUpdate) -> Result<()> {
        let message = DeliveryWebSocketMessage::LocationUpdate {
            delivery_person_id: location_update.delivery_person_id,
            latitude: location_update.latitude,
            longitude: location_update.longitude,
            speed: location_update.speed,
            heading: location_update.heading,
            timestamp: location_update.timestamp,
        };

        self.broadcast_tx
            .send(message)
            .map_err(|e| crate::error::AppError::WebSocketError(e.to_string()))?;

        Ok(())
    }

    pub async fn broadcast_status_update(
        &self,
        assignment_id: Uuid,
        delivery_person_id: Uuid,
        status: DeliveryStatus,
        estimated_arrival: Option<DateTime<Utc>>,
        notes: Option<String>,
    ) -> Result<()> {
        let message = DeliveryWebSocketMessage::StatusUpdate {
            assignment_id,
            delivery_person_id,
            status: status.as_str().to_string(),
            estimated_arrival,
            notes,
        };

        self.broadcast_tx
            .send(message)
            .map_err(|e| crate::error::AppError::WebSocketError(e.to_string()))?;

        Ok(())
    }

    pub async fn broadcast_order_assignment(
        &self,
        assignment_id: Uuid,
        delivery_person_id: Uuid,
        order_id: Uuid,
        pickup_address: serde_json::Value,
        delivery_address: serde_json::Value,
        estimated_pickup_time: Option<DateTime<Utc>>,
        estimated_delivery_time: Option<DateTime<Utc>>,
    ) -> Result<()> {
        let message = DeliveryWebSocketMessage::OrderAssigned {
            assignment_id,
            delivery_person_id,
            order_id,
            pickup_address,
            delivery_address,
            estimated_pickup_time,
            estimated_delivery_time,
        };

        self.broadcast_tx
            .send(message)
            .map_err(|e| crate::error::AppError::WebSocketError(e.to_string()))?;

        Ok(())
    }

    pub async fn broadcast_emergency_alert(
        &self,
        delivery_person_id: Uuid,
        latitude: f64,
        longitude: f64,
        message: String,
    ) -> Result<()> {
        let alert = DeliveryWebSocketMessage::EmergencyAlert {
            delivery_person_id,
            latitude,
            longitude,
            message,
            timestamp: Utc::now(),
        };

        self.broadcast_tx
            .send(alert)
            .map_err(|e| crate::error::AppError::WebSocketError(e.to_string()))?;

        Ok(())
    }

    pub async fn handle_websocket_connection(
        &self,
        socket: WebSocket,
        connection_id: Uuid,
        connection: DeliveryWebSocketConnection,
    ) {
        self.add_connection(connection_id, connection.clone()).await;

        let (mut sender, mut receiver) = socket.split();
        let mut broadcast_rx = self.broadcast_tx.subscribe();

        // Send online notification if it's a delivery person
        if let Some(delivery_person_id) = connection.delivery_person_id {
            // In a real implementation, you'd get the current location from the database
            let _ = self
                .broadcast_tx
                .send(DeliveryWebSocketMessage::DeliveryPersonOnline {
                    delivery_person_id,
                    latitude: 0.0, // Would be fetched from database
                    longitude: 0.0,
                });
        }

        // Create a channel for sending messages to the client
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

        // Handle incoming messages
        let broadcast_tx_clone = self.broadcast_tx.clone();
        let tx_clone = tx.clone();
        let incoming_task = tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(parsed_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                            // Handle different message types
                            match parsed_msg.get("type").and_then(|t| t.as_str()) {
                                Some("ping") => {
                                    // Respond with pong
                                    let pong = serde_json::json!({"type": "pong", "timestamp": Utc::now()});
                                    if let Ok(pong_text) = serde_json::to_string(&pong) {
                                        let _ = tx_clone.send(pong_text);
                                    }
                                }
                                Some("emergency") => {
                                    // Handle emergency alert
                                    if let (Some(lat), Some(lng), Some(msg)) = (
                                        parsed_msg.get("latitude").and_then(|v| v.as_f64()),
                                        parsed_msg.get("longitude").and_then(|v| v.as_f64()),
                                        parsed_msg.get("message").and_then(|v| v.as_str()),
                                    ) {
                                        if let Some(delivery_person_id) =
                                            connection.delivery_person_id
                                        {
                                            let alert = DeliveryWebSocketMessage::EmergencyAlert {
                                                delivery_person_id,
                                                latitude: lat,
                                                longitude: lng,
                                                message: msg.to_string(),
                                                timestamp: Utc::now(),
                                            };
                                            let _ = broadcast_tx_clone.send(alert);
                                        }
                                    }
                                }
                                _ => {
                                    // Unknown message type, ignore
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        break;
                    }
                    Err(_) => {
                        break;
                    }
                    _ => {}
                }
            }
        });

        // Handle outgoing messages from broadcast
        let tx_clone2 = tx.clone();
        let connection_clone = connection.clone();
        let broadcast_task = tokio::spawn(async move {
            while let Ok(msg) = broadcast_rx.recv().await {
                // Filter messages based on connection type
                let should_send = match (&msg, &connection_clone) {
                    // Send location updates to customers and restaurants for their orders
                    (
                        DeliveryWebSocketMessage::LocationUpdate {
                            delivery_person_id, ..
                        },
                        conn,
                    ) => {
                        conn.role == "admin"
                            || conn.delivery_person_id == Some(*delivery_person_id)
                            || conn.role == "customer"
                            || conn.role == "restaurant"
                    }
                    // Send status updates to relevant parties
                    (
                        DeliveryWebSocketMessage::StatusUpdate {
                            delivery_person_id, ..
                        },
                        conn,
                    ) => {
                        conn.role == "admin"
                            || conn.delivery_person_id == Some(*delivery_person_id)
                            || conn.role == "customer"
                            || conn.role == "restaurant"
                    }
                    // Send order assignments to the specific delivery person
                    (
                        DeliveryWebSocketMessage::OrderAssigned {
                            delivery_person_id, ..
                        },
                        conn,
                    ) => {
                        conn.delivery_person_id == Some(*delivery_person_id) || conn.role == "admin"
                    }
                    // Send online/offline status to admins and nearby delivery persons
                    (DeliveryWebSocketMessage::DeliveryPersonOnline { .. }, conn) => {
                        conn.role == "admin" || conn.role == "delivery_person"
                    }
                    (DeliveryWebSocketMessage::DeliveryPersonOffline { .. }, conn) => {
                        conn.role == "admin" || conn.role == "delivery_person"
                    }
                    // Send emergency alerts to admins
                    (DeliveryWebSocketMessage::EmergencyAlert { .. }, conn) => conn.role == "admin",
                };

                if should_send {
                    if let Ok(json_msg) = serde_json::to_string(&msg) {
                        let _ = tx_clone2.send(json_msg);
                    }
                }
            }
        });

        // Handle sending messages to the WebSocket
        let outgoing_task = tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if sender.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
        });

        // Wait for any task to complete
        tokio::select! {
            _ = incoming_task => {},
            _ = outgoing_task => {},
            _ = broadcast_task => {},
        }

        // Clean up connection
        self.remove_connection(&connection_id).await;
    }

    pub async fn get_active_connections(&self) -> usize {
        let connections = self.connections.read().await;
        connections.len()
    }

    pub async fn get_delivery_person_connections(&self) -> usize {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter(|conn| conn.role == "delivery_person")
            .count()
    }

    pub async fn get_connections_by_role(&self, role: &str) -> Vec<DeliveryWebSocketConnection> {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter(|conn| conn.role == role)
            .cloned()
            .collect()
    }
}
