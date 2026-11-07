//! User login use case.

use serde_json::json;
use uuid::Uuid;

use crate::platform::application::ports::IssuedTokens;
use crate::platform::application::service::AuthService;
use crate::platform::domain::audit::AuditEntry;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::user::{is_valid_email, normalize_email};

pub async fn execute(
    service: &AuthService,
    email: &str,
    password: &str,
    remember_me: bool,
    request_id: Option<Uuid>,
) -> Result<IssuedTokens, PlatformError> {
    let email = normalize_email(email);

    if !is_valid_email(&email) {
        record_failure(service, &email, request_id).await;
        return Err(PlatformError::InvalidCredentials);
    }

    let Some(user) = service.users.find_by_email(&email).await? else {
        record_failure(service, &email, request_id).await;
        return Err(PlatformError::InvalidCredentials);
    };

    if !service.hasher.verify(password, &user.password_hash)? {
        record_failure(service, &email, request_id).await;
        return Err(PlatformError::InvalidCredentials);
    }

    if !user.can_authenticate() {
        record_failure(service, &email, request_id).await;
        return Err(PlatformError::AccountInactive);
    }

    let roles = service.roles.codes_for_user(user.id).await?;
    let issued = super::common::issue_pair(service, &user, &roles, remember_me).await?;

    service
        .audit(
            AuditEntry::new("user.login", "user")
                .actor(user.id)
                .resource(user.id)
                .correlation(request_id)
                .metadata(json!({ "email": email })),
        )
        .await;

    Ok(issued)
}

async fn record_failure(service: &AuthService, email: &str, request_id: Option<Uuid>) {
    service
        .audit(
            AuditEntry::new("user.login_failed", "user")
                .correlation(request_id)
                .metadata(json!({ "email": email })),
        )
        .await;
}
