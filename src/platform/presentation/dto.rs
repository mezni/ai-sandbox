//! Request and response DTOs (separate from domain/persistence models).

use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::platform::application::ports::{IssuedTokens, UserView};

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
}

impl From<IssuedTokens> for Tokens {
    fn from(tokens: IssuedTokens) -> Self {
        Self {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            token_type: tokens.token_type,
            expires_in: tokens.expires_in,
            refresh_expires_in: tokens.refresh_expires_in,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
    pub email_verified_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<UserView> for UserResponse {
    fn from(view: UserView) -> Self {
        Self {
            id: view.user.id,
            email: view.user.email,
            status: view.user.status.as_str().to_string(),
            roles: view.roles,
            email_verified_at: view.user.email_verified_at.and_then(format_timestamp),
            created_at: format_timestamp(view.user.created_at).unwrap_or_default(),
            updated_at: format_timestamp(view.user.updated_at).unwrap_or_default(),
        }
    }
}

fn format_timestamp(value: OffsetDateTime) -> Option<String> {
    value.format(&Rfc3339).ok()
}

#[derive(Debug, Serialize)]
pub struct Meta {
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct Success<T> {
    pub data: T,
    pub meta: Meta,
}

impl<T> Success<T> {
    pub fn new(data: T, request_id: Uuid) -> Self {
        Self {
            data,
            meta: Meta { request_id },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub details: Vec<serde_json::Value>,
    pub request_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ErrorEnvelope {
    pub error: ErrorBody,
}
