//! Shared helpers for identity use cases.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::rngs::OsRng;
use rand::RngCore;
use sha2::{Digest, Sha256};
use time::{Duration, OffsetDateTime};

use crate::platform::application::ports::IssuedTokens;
use crate::platform::application::service::AuthService;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::refresh_token::RefreshToken;
use crate::platform::domain::user::User;

/// Generate a 256-bit opaque refresh token.
pub fn generate_refresh_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Hash a refresh token before persistence.
pub fn hash_refresh_token(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    hex::encode(digest)
}

/// Issue an access token and persist a new refresh session.
pub async fn issue_pair(
    service: &AuthService,
    user: &User,
    roles: &[String],
    remember_me: bool,
) -> Result<IssuedTokens, PlatformError> {
    let (access_token, expires_in) = service.issuer.issue_access(user, roles)?;

    let refresh_token = generate_refresh_token();
    let refresh_ttl = if remember_me {
        service.policy.remember_me_ttl
    } else {
        service.policy.refresh_ttl
    };
    let expires_at = OffsetDateTime::now_utc() + Duration::seconds(refresh_ttl);

    let session = RefreshToken::new(user.id, hash_refresh_token(&refresh_token), expires_at);
    service.refresh_tokens.insert(&session).await?;

    Ok(IssuedTokens {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in,
        refresh_expires_in: refresh_ttl as u64,
    })
}
