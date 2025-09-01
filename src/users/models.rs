use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub firebase_uid: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: String,
    pub role: UserRole,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub profile_image_url: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Customer,
    Restaurant,
    DeliveryPerson,
    Admin,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub firebase_uid: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: String,
    pub role: UserRole,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub profile_image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub name: String,
    pub role: UserRole,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub profile_image_url: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            phone: user.phone,
            name: user.name,
            role: user.role,
            is_email_verified: user.is_email_verified,
            is_phone_verified: user.is_phone_verified,
            profile_image_url: user.profile_image_url,
            address: user.address,
            city: user.city,
            state: user.state,
            postal_code: user.postal_code,
            country: user.country,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub verification_code: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyPhoneRequest {
    pub verification_code: String,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}