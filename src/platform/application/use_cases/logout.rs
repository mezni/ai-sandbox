//! Logout use case: revoke the presented refresh session.

use uuid::Uuid;

use crate::platform::application::service::AuthService;
use crate::platform::domain::audit::AuditEntry;
use crate::platform::domain::errors::PlatformError;

use super::common::hash_refresh_token;

pub async fn execute(
    service: &AuthService,
    user_id: Uuid,
    presented_token: &str,
    request_id: Option<Uuid>,
) -> Result<(), PlatformError> {
    let token_hash = hash_refresh_token(presented_token);
    service
        .refresh_tokens
        .revoke_by_hash_for_user(&token_hash, user_id)
        .await?;

    service
        .audit(
            AuditEntry::new("user.logged_out", "user")
                .actor(user_id)
                .resource(user_id)
                .correlation(request_id),
        )
        .await;

    Ok(())
}
