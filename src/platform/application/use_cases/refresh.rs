//! Refresh-token rotation use case.

use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::platform::application::ports::IssuedTokens;
use crate::platform::application::service::AuthService;
use crate::platform::domain::audit::AuditEntry;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::refresh_token::RefreshToken;

use super::common::{generate_refresh_token, hash_refresh_token};

pub async fn execute(
    service: &AuthService,
    presented_token: &str,
    request_id: Option<Uuid>,
) -> Result<IssuedTokens, PlatformError> {
    if presented_token.trim().is_empty() {
        return Err(PlatformError::InvalidToken);
    }

    let token_hash = hash_refresh_token(presented_token);
    let Some(existing) = service
        .refresh_tokens
        .find_active_by_hash(&token_hash)
        .await?
    else {
        return Err(PlatformError::InvalidToken);
    };

    let Some(user) = service.users.find_by_id(existing.user_id).await? else {
        return Err(PlatformError::InvalidToken);
    };

    if !user.can_authenticate() {
        return Err(PlatformError::AccountInactive);
    }

    let roles = service.roles.codes_for_user(user.id).await?;

    let (access_token, expires_in) = service.issuer.issue_access(&user, &roles)?;
    let refresh_token = generate_refresh_token();
    let expires_at = OffsetDateTime::now_utc() + Duration::seconds(service.policy.refresh_ttl);
    let replacement = RefreshToken::new(user.id, hash_refresh_token(&refresh_token), expires_at);

    service
        .refresh_tokens
        .rotate(existing.id, &replacement)
        .await?;

    service
        .audit(
            AuditEntry::new("token.refreshed", "refresh_token")
                .actor(user.id)
                .resource(existing.id)
                .correlation(request_id),
        )
        .await;

    Ok(IssuedTokens {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in,
        refresh_expires_in: service.policy.refresh_ttl as u64,
    })
}
