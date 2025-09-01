use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub firebase_uid: String,
    pub email: String,
    pub phone: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
    pub provider: AuthProvider,
    pub role: String, // customer, restaurant, delivery_person, admin
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProvider {
    Google,
    Apple,
    Email,
    Phone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseToken {
    pub uid: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub email_verified: bool,
    pub firebase: FirebaseClaims,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseClaims {
    pub sign_in_provider: String,
    pub identities: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub id_token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub access_token: String,
}