use crate::auth::models::{FirebaseToken, User, AuthProvider};
use crate::config::Config;
use crate::error::{AppError, Result};
use anyhow::Context;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use std::collections::HashMap;
use uuid::Uuid;

pub struct FirebaseAuth {
    client: Client,
    project_id: String,
    public_keys: HashMap<String, String>,
}

impl FirebaseAuth {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            project_id: config.firebase_project_id.clone(),
            public_keys: HashMap::new(),
        }
    }

    pub async fn verify_token(&mut self, id_token: &str) -> Result<FirebaseToken> {
        // Decode header to get key ID
        let header = decode_header(id_token)
            .map_err(|e| AppError::BadRequest(format!("Invalid token header: {}", e)))?;

        let kid = header.kid
            .ok_or_else(|| AppError::BadRequest("Missing key ID in token".to_string()))?;

        // Get public key
        let public_key = self.get_public_key(&kid).await?;
        
        // Verify token
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.project_id]);
        validation.set_issuer(&[&format!("https://securetoken.google.com/{}", self.project_id)]);

        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Failed to create decoding key: {}", e)))?;

        let token_data = decode::<FirebaseToken>(id_token, &decoding_key, &validation)
            .map_err(|_e| AppError::Unauthorized)?;

        Ok(token_data.claims)
    }

    async fn get_public_key(&mut self, kid: &str) -> Result<String> {
        if let Some(key) = self.public_keys.get(kid) {
            return Ok(key.clone());
        }

        // Fetch public keys from Google
        let response = self
            .client
            .get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
            .send()
            .await
            .context("Failed to fetch Firebase public keys")?;

        let keys: HashMap<String, String> = response
            .json()
            .await
            .context("Failed to parse Firebase public keys")?;

        self.public_keys = keys;

        self.public_keys
            .get(kid)
            .cloned()
            .ok_or_else(|| AppError::BadRequest("Invalid key ID".to_string()))
    }

    pub fn validate_user_requirements(&self, token: &FirebaseToken) -> Result<()> {
        // Check if both email and phone are present and verified
        let email_verified = token.email_verified;
        let has_phone = token.phone_number.is_some();

        if !email_verified {
            return Err(AppError::ValidationError(
                "Email must be verified".to_string(),
            ));
        }

        if !has_phone {
            return Err(AppError::ValidationError(
                "Phone number must be linked to account".to_string(),
            ));
        }

        Ok(())
    }

    pub fn token_to_user(&self, token: FirebaseToken) -> User {
        let provider = match token.firebase.sign_in_provider.as_str() {
            "google.com" => AuthProvider::Google,
            "apple.com" => AuthProvider::Apple,
            "phone" => AuthProvider::Phone,
            _ => AuthProvider::Email,
        };

        let now = chrono::Utc::now();

        User {
            id: Uuid::new_v4(),
            firebase_uid: token.uid,
            email: token.email.unwrap_or_default(),
            phone: token.phone_number.unwrap_or_default(),
            email_verified: token.email_verified,
            phone_verified: true, // Assume verified if present in Firebase token
            display_name: None,
            photo_url: None,
            provider,
            role: "customer".to_string(), // Default role
            created_at: now,
            updated_at: now,
        }
    }
}