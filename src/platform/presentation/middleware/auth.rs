//! Bearer-token authentication extractor.

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use uuid::Uuid;

use crate::platform::application::ports::AccessClaims;
use crate::platform::presentation::errors::ApiError;
use crate::AppState;

use super::request_id::RequestId;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub roles: Vec<String>,
    pub claims: AccessClaims,
}

impl AuthUser {
    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|candidate| candidate == role)
    }

    pub fn is_admin(&self) -> bool {
        self.has_role("admin")
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let request_id = parts
            .extensions
            .get::<RequestId>()
            .map(|request_id| request_id.0)
            .unwrap_or_else(Uuid::new_v4);

        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| ApiError::unauthorized(request_id))?;

        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .ok_or_else(|| ApiError::unauthorized(request_id))?;

        let claims = state
            .auth
            .authenticate_access_token(token)
            .map_err(|_| ApiError::unauthorized(request_id))?;

        Ok(Self {
            user_id: claims.sub,
            roles: claims.roles.clone(),
            claims,
        })
    }
}
