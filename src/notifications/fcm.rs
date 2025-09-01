use crate::config::Config;
use crate::error::{AppError, Result};
use crate::notifications::models::{NotificationPayload, OrderNotification};
use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct FCMService {
    client: Client,
    project_id: String,
    service_account_key: ServiceAccountKey,
    access_token: Option<String>,
    token_expires_at: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Clone, Deserialize)]
struct ServiceAccountKey {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    key_type: String,
    #[allow(dead_code)]
    project_id: String,
    #[allow(dead_code)]
    private_key_id: String,
    private_key: String,
    client_email: String,
    #[allow(dead_code)]
    client_id: String,
    #[allow(dead_code)]
    auth_uri: String,
    token_uri: String,
}

#[derive(Debug, Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64,
}

#[derive(Debug, Serialize)]
struct AccessTokenRequest {
    grant_type: String,
    assertion: String,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u32,
    #[allow(dead_code)]
    token_type: String,
}

#[derive(Debug, Serialize)]
struct FcmMessage {
    message: FcmMessagePayload,
}

#[derive(Debug, Serialize)]
struct FcmMessagePayload {
    token: String,
    notification: FcmNotification,
    data: HashMap<String, String>,
    android: Option<AndroidConfig>,
    apns: Option<ApnsConfig>,
}

#[derive(Debug, Serialize)]
struct FcmNotification {
    title: String,
    body: String,
}

#[derive(Debug, Serialize)]
struct AndroidConfig {
    priority: String,
    notification: AndroidNotification,
}

#[derive(Debug, Serialize)]
struct AndroidNotification {
    sound: String,
    click_action: String,
}

#[derive(Debug, Serialize)]
struct ApnsConfig {
    payload: ApnsPayload,
}

#[derive(Debug, Serialize)]
struct ApnsPayload {
    aps: ApnsAps,
}

#[derive(Debug, Serialize)]
struct ApnsAps {
    sound: String,
    category: String,
}

#[derive(Debug, Deserialize)]
struct FcmResponse {
    #[allow(dead_code)]
    name: Option<String>,
    error: Option<FcmError>,
}

#[derive(Debug, Deserialize)]
struct FcmError {
    #[allow(dead_code)]
    code: u32,
    message: String,
    status: String,
}

impl FCMService {
    pub fn new(config: &Config) -> Result<Self> {
        let service_account_content = fs::read_to_string(&config.firebase_service_account_key)
            .context("Failed to read service account key file")?;
        
        let service_account_key: ServiceAccountKey = serde_json::from_str(&service_account_content)
            .context("Failed to parse service account key JSON")?;

        Ok(Self {
            client: Client::new(),
            project_id: config.firebase_project_id.clone(),
            service_account_key,
            access_token: None,
            token_expires_at: None,
        })
    }

    async fn get_access_token(&mut self) -> Result<String> {
        // Check if we have a valid token
        if let (Some(token), Some(expires_at)) = (&self.access_token, &self.token_expires_at) {
            if Utc::now() < *expires_at - Duration::minutes(5) {
                return Ok(token.clone());
            }
        }

        // Generate new JWT
        let now = Utc::now();
        let claims = JwtClaims {
            iss: self.service_account_key.client_email.clone(),
            scope: "https://www.googleapis.com/auth/firebase.messaging".to_string(),
            aud: self.service_account_key.token_uri.clone(),
            exp: (now + Duration::hours(1)).timestamp(),
            iat: now.timestamp(),
        };

        let private_key = self.service_account_key.private_key
            .replace("\\n", "\n");
        
        let encoding_key = EncodingKey::from_rsa_pem(private_key.as_bytes())
            .context("Failed to create encoding key from private key")?;

        let jwt = encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)
            .context("Failed to encode JWT")?;

        // Exchange JWT for access token
        let token_request = AccessTokenRequest {
            grant_type: "urn:ietf:params:oauth:grant-type:jwt-bearer".to_string(),
            assertion: jwt,
        };

        let response = self
            .client
            .post(&self.service_account_key.token_uri)
            .form(&token_request)
            .send()
            .await
            .context("Failed to request access token")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(anyhow::anyhow!(
                "Failed to get access token: {}",
                error_text
            )));
        }

        let token_response: AccessTokenResponse = response
            .json()
            .await
            .context("Failed to parse access token response")?;

        // Cache the token
        self.access_token = Some(token_response.access_token.clone());
        self.token_expires_at = Some(now + Duration::seconds(token_response.expires_in as i64));

        Ok(token_response.access_token)
    }

    pub async fn send_notification(
        &mut self,
        device_token: &str,
        payload: &NotificationPayload,
    ) -> Result<()> {
        let access_token = self.get_access_token().await?;

        let mut data = HashMap::new();
        if let Some(payload_data) = &payload.data {
            if let Some(obj) = payload_data.as_object() {
                for (key, value) in obj {
                    data.insert(key.clone(), value.to_string());
                }
            }
        }

        let message = FcmMessage {
            message: FcmMessagePayload {
                token: device_token.to_string(),
                notification: FcmNotification {
                    title: payload.title.clone(),
                    body: payload.body.clone(),
                },
                data,
                android: Some(AndroidConfig {
                    priority: "high".to_string(),
                    notification: AndroidNotification {
                        sound: "default".to_string(),
                        click_action: "FLUTTER_NOTIFICATION_CLICK".to_string(),
                    },
                }),
                apns: Some(ApnsConfig {
                    payload: ApnsPayload {
                        aps: ApnsAps {
                            sound: "default".to_string(),
                            category: "ORDER_UPDATE".to_string(),
                        },
                    },
                }),
            },
        };

        let url = format!(
            "https://fcm.googleapis.com/v1/projects/{}/messages:send",
            self.project_id
        );

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Content-Type", "application/json")
            .json(&message)
            .send()
            .await
            .context("Failed to send FCM notification")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Internal(anyhow::anyhow!(
                "FCM request failed: {}",
                error_text
            )));
        }

        let fcm_response: FcmResponse = response
            .json()
            .await
            .context("Failed to parse FCM response")?;

        if let Some(error) = fcm_response.error {
            return Err(AppError::Internal(anyhow::anyhow!(
                "FCM error: {} - {}",
                error.status,
                error.message
            )));
        }

        tracing::info!("FCM notification sent successfully to {}", device_token);
        Ok(())
    }

    pub async fn send_order_notifications(
        &mut self,
        order_notification: &OrderNotification,
        device_tokens: &HashMap<String, String>, // role -> device_token
    ) -> Result<()> {
        let mut tasks = Vec::new();

        for (role, token) in device_tokens {
            let payload = self.customize_payload_for_role(
                &order_notification.payload,
                role,
                &order_notification.order_id,
            );

            // Clone self for async task
            let mut service_clone = self.clone();
            let token_clone = token.clone();
            
            let task = tokio::spawn(async move {
                service_clone.send_notification(&token_clone, &payload).await
            });
            
            tasks.push(task);
        }

        // Wait for all notifications to complete
        let results = futures::future::join_all(tasks).await;

        // Check for any failures
        for result in results {
            match result {
                Ok(Ok(())) => {
                    // Success
                }
                Ok(Err(e)) => {
                    tracing::error!("Failed to send notification: {:?}", e);
                    // Continue sending other notifications even if one fails
                }
                Err(e) => {
                    tracing::error!("Task failed: {:?}", e);
                }
            }
        }

        Ok(())
    }

    fn customize_payload_for_role(
        &self,
        base_payload: &NotificationPayload,
        role: &str,
        order_id: &Uuid,
    ) -> NotificationPayload {
        let (title, body) = match role {
            "customer" => (
                format!("Order Update - {}", base_payload.title),
                format!("Your order #{} - {}", order_id, base_payload.body),
            ),
            "restaurant" => (
                format!("New Order - {}", base_payload.title),
                format!("Order #{} - {}", order_id, base_payload.body),
            ),
            "delivery" => (
                format!("Delivery Assignment - {}", base_payload.title),
                format!("Delivery #{} - {}", order_id, base_payload.body),
            ),
            _ => (base_payload.title.clone(), base_payload.body.clone()),
        };

        NotificationPayload {
            title,
            body,
            data: Some(json!({
                "order_id": order_id,
                "role": role,
                "original_data": base_payload.data
            })),
        }
    }

    // Helper methods for different order states
    pub async fn notify_order_placed(
        &mut self,
        order_id: Uuid,
        customer_token: &str,
        restaurant_token: &str,
    ) -> Result<()> {
        let payload = NotificationPayload {
            title: "Order Placed".to_string(),
            body: "Your order has been placed successfully".to_string(),
            data: Some(json!({"order_id": order_id})),
        };

        let mut tokens = HashMap::new();
        tokens.insert("customer".to_string(), customer_token.to_string());
        tokens.insert("restaurant".to_string(), restaurant_token.to_string());

        let notification = OrderNotification {
            order_id,
            notification_type: crate::notifications::models::NotificationType::OrderPlaced,
            recipient_type: crate::notifications::models::RecipientType::Customer,
            payload,
        };

        self.send_order_notifications(&notification, &tokens).await
    }

    pub async fn notify_order_ready(
        &mut self,
        order_id: Uuid,
        customer_token: &str,
        delivery_token: &str,
    ) -> Result<()> {
        let payload = NotificationPayload {
            title: "Order Ready".to_string(),
            body: "Your order is ready for pickup".to_string(),
            data: Some(json!({"order_id": order_id})),
        };

        let mut tokens = HashMap::new();
        tokens.insert("customer".to_string(), customer_token.to_string());
        tokens.insert("delivery".to_string(), delivery_token.to_string());

        let notification = OrderNotification {
            order_id,
            notification_type: crate::notifications::models::NotificationType::OrderReady,
            recipient_type: crate::notifications::models::RecipientType::Customer,
            payload,
        };

        self.send_order_notifications(&notification, &tokens).await
    }

    pub async fn notify_order_delivered(
        &mut self,
        order_id: Uuid,
        customer_token: &str,
    ) -> Result<()> {
        let payload = NotificationPayload {
            title: "Order Delivered".to_string(),
            body: "Your order has been delivered successfully".to_string(),
            data: Some(json!({"order_id": order_id})),
        };

        let mut tokens = HashMap::new();
        tokens.insert("customer".to_string(), customer_token.to_string());

        let notification = OrderNotification {
            order_id,
            notification_type: crate::notifications::models::NotificationType::OrderDelivered,
            recipient_type: crate::notifications::models::RecipientType::Customer,
            payload,
        };

        self.send_order_notifications(&notification, &tokens).await
    }
}