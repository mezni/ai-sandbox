// JWT authentication framework
// Per docs/06-api-specification.md §4.1-4.3 and constitution.md §226-231

use axum::{
    extract::{FromRequestParts, Request},
    headers::{Authorization, AuthorizationHeader},
    http::{Request as HttpRequest, StatusCode},
    Json,
};
use serde::{Deserialize, Serialize};
use time::Duration;
use uuid::Uuid;

/// JWT access token claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Issued at timestamp
    pub iat: usize,
    /// Expiration timestamp
    pub exp: usize,
    /// Additional claims
    pub extra: serde_json::Value,
}

/// JWT refresh token claims
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    /// Subject (user ID)
    pub sub: Uuid,
    /// Issuer
    pub iss: String,
    /// Audience
    pub aud: String,
    /// Issued at timestamp
    pub iat: usize,
    /// Expiration timestamp (longer than access token)
    pub exp: usize,
}

/// JWT token pair (access + refresh)
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Token type
    pub token_type: String,
    /// Expires in seconds for access token
    pub expires_in: u64,
    /// Expires in seconds for refresh token
    pub refresh_expires_in: u64,
}

/// JWT configuration per constitution.md §226-231
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// Secret key for signing tokens
    pub secret: String,
    /// Access token duration (in minutes)
    pub access_token_duration_minutes: u64,
    /// Refresh token duration (in minutes)
    pub refresh_token_duration_minutes: u64,
    /// Issuer identifier
    pub issuer: String,
    /// Audience identifier
    pub audience: String,
}

impl JwtConfig {
    /// Create a new JWT config with default durations
    pub fn default_config() -> Self {
        Self {
            secret: "secret".to_string(), // In production, use a secure random key
            access_token_duration_minutes: 15,
            refresh_token_duration_minutes: 30 * 24 * 60, // 30 days
            issuer: "biosouk".to_string(),
            audience: "biosouk-users".to_string(),
        }
    }

    /// Access token expires in seconds
    pub fn access_token_expires_in(&self) -> u64 {
        self.access_token_duration_minutes * 60
    }

    /// Refresh token expires in seconds
    pub fn refresh_token_expires_in(&self) -> u64 {
        self.refresh_token_duration_minutes * 60
    }
}

/// Create a JWT access token from claims
pub fn create_access_token(config: &JwtConfig, claims: &Claims) -> Result<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| anyhow::anyhow!("Failed to encode JWT: {}", e))?;

    Ok(token)
}

/// Create a JWT refresh token from claims
pub fn create_refresh_token(config: &JwtConfig, claims: &RefreshClaims) -> Result<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};

    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .map_err(|e| anyhow::anyhow!("Failed to encode refresh JWT: {}", e))?;

    Ok(token)
}

/// Verify and decode a JWT access token
pub fn decode_access_token(config: &JwtConfig, token: &str) -> Result<Claims> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    let mut validation = Validation::default();
    validation.set_issuer(&config.issuer);
    validation.set_audience(&config.audience);

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(config.secret.as_bytes()), &validation)
        .map_err(|e| anyhow::anyhow!("Failed to decode JWT: {}", e))?;

    Ok(token_data.claims)
}

/// Verify and decode a JWT refresh token
pub fn decode_refresh_token(config: &JwtConfig, token: &str) -> Result<RefreshClaims> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    let mut validation = Validation::default();
    validation.set_issuer(&config.issuer);
    validation.set_audience(&config.audience);

    let token_data = decode::<RefreshClaims>(token, &DecodingKey::from_secret(config.secret.as_bytes()), &validation)
        .map_err(|e| anyhow::anyhow!("Failed to decode refresh JWT: {}", e))?;

    Ok(token_data.claims)
}

/// Authenticate user from JWT token in Authorization header
pub async fn authenticate_from_header(
    config: &JwtConfig,
    headers: &axum::http::HeaderMap,
) -> Result<Claims> {
    let auth_header = Authorization::extract(headers)
        .ok_or_else(|| anyhow::anyhow!("Authorization header missing"))?;

    let token = auth_header
        .token()
        .ok_or_else(|| anyhow::anyhow!("Bearer token missing in Authorization header"))?;

    decode_access_token(config, token)
}

/// Password hashing using Argon2id per constitution.md §229
// TODO: Implement Argon2id password hashing using the argon2 or argon2rs crate

/// Verify a password hash against a plain text password
// TODO: Implement password verification

/// Password hasher trait for abstraction
pub trait PasswordHasher {
    /// Hash a plain text password
    fn hash_password(&self, password: &str) -> Result<String>;

    /// Verify a password against its hash
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool>;
}

/// Argon2id implementation of PasswordHasher (TODO: implement when argon2 crate is added)
// #[derive(Debug)]
// struct Argon2idHasher {
//     memory_cost: u32,
//     time_cost: u32,
//     parallelism: u32,
// }

// impl PasswordHasher for Argon2idHasher {
//     fn hash_password(&self, password: &str) -> Result<String> {
//         // TODO: Implement Argon2id hashing
//         // use argon2::Argon2::new(argon2::Version::V0_19_0, memory_cost, time_cost, parallelism);
//         unimplemented!()
//     }
//
//     fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
//         // TODO: Implement password verification
//         unimplemented!()
//     }
// }