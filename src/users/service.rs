use crate::database::Database;
use crate::error::{AppError, Result};
use crate::users::models::*;
use uuid::Uuid;
use chrono::Utc;
use sqlx::Row;

pub struct UserService {
    db: Database,
}

impl UserService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (
                id, firebase_uid, email, phone, name, role, 
                address, city, state, postal_code, country,
                is_active, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&request.firebase_uid)
        .bind(&request.email)
        .bind(&request.phone)
        .bind(&request.name)
        .bind(&request.role)
        .bind(&request.address)
        .bind(&request.city)
        .bind(&request.state)
        .bind(&request.postal_code)
        .bind("India") // Default country for India-focused platform
        .bind(true)
        .bind(now)
        .bind(now)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn get_user_by_firebase_uid(&self, firebase_uid: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE firebase_uid = $1")
            .bind(firebase_uid)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn get_user_by_phone(&self, phone: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE phone = $1")
            .bind(phone)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn update_user(&self, user_id: Uuid, request: UpdateUserRequest) -> Result<User> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users SET
                name = COALESCE($2, name),
                email = COALESCE($3, email),
                phone = COALESCE($4, phone),
                address = COALESCE($5, address),
                city = COALESCE($6, city),
                state = COALESCE($7, state),
                postal_code = COALESCE($8, postal_code),
                profile_image_url = COALESCE($9, profile_image_url),
                updated_at = $10
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(&request.name)
        .bind(&request.email)
        .bind(&request.phone)
        .bind(&request.address)
        .bind(&request.city)
        .bind(&request.state)
        .bind(&request.postal_code)
        .bind(&request.profile_image_url)
        .bind(now)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn verify_email(&self, user_id: Uuid) -> Result<User> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET is_email_verified = true, updated_at = $2 WHERE id = $1 RETURNING *"
        )
        .bind(user_id)
        .bind(now)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn verify_phone(&self, user_id: Uuid) -> Result<User> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET is_phone_verified = true, updated_at = $2 WHERE id = $1 RETURNING *"
        )
        .bind(user_id)
        .bind(now)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn deactivate_user(&self, user_id: Uuid) -> Result<User> {
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET is_active = false, updated_at = $2 WHERE id = $1 RETURNING *"
        )
        .bind(user_id)
        .bind(now)
        .fetch_one(&self.db.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(user)
    }

    pub async fn list_users(&self, page: i32, per_page: i32, role: Option<UserRole>) -> Result<UserListResponse> {
        let offset = (page - 1) * per_page;

        let (users, total) = if let Some(role) = role {
            let users = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE role = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(&role)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            let total: i64 = sqlx::query("SELECT COUNT(*) FROM users WHERE role = $1")
                .bind(&role)
                .fetch_one(&self.db.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
                .get(0);

            (users, total)
        } else {
            let users = sqlx::query_as::<_, User>(
                "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(&self.db.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            let total: i64 = sqlx::query("SELECT COUNT(*) FROM users")
                .fetch_one(&self.db.pool)
                .await
                .map_err(|e| AppError::DatabaseError(e.to_string()))?
                .get(0);

            (users, total)
        };

        Ok(UserListResponse {
            users: users.into_iter().map(UserResponse::from).collect(),
            total,
            page,
            per_page,
        })
    }
}