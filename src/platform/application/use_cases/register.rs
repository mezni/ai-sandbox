//! User registration use case.

use serde_json::json;
use uuid::Uuid;

use crate::platform::application::ports::IssuedTokens;
use crate::platform::application::service::AuthService;
use crate::platform::domain::audit::AuditEntry;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::role::DEFAULT_REGISTRATION_ROLE;
use crate::platform::domain::user::{is_valid_email, is_valid_password, normalize_email, User};

pub async fn execute(
    service: &AuthService,
    email: &str,
    password: &str,
    request_id: Option<Uuid>,
) -> Result<IssuedTokens, PlatformError> {
    let email = normalize_email(email);

    if !is_valid_email(&email) {
        return Err(PlatformError::Validation(
            "email must be a valid email address".to_string(),
        ));
    }

    if !is_valid_password(password, service.policy.password_min_length) {
        return Err(PlatformError::Validation(format!(
            "password must be at least {} characters and include a letter and a digit",
            service.policy.password_min_length
        )));
    }

    if service.users.find_by_email(&email).await?.is_some() {
        return Err(PlatformError::EmailAlreadyExists);
    }

    let password_hash = service.hasher.hash(password)?;
    let user = User::new(email.clone(), password_hash);
    let user = service.users.create(&user).await?;

    service
        .roles
        .assign_role(user.id, DEFAULT_REGISTRATION_ROLE)
        .await?;

    service
        .audit(
            AuditEntry::new("role.assigned", "user_role")
                .actor(user.id)
                .resource(user.id)
                .correlation(request_id)
                .metadata(json!({ "role": DEFAULT_REGISTRATION_ROLE })),
        )
        .await;

    let roles = service.roles.codes_for_user(user.id).await?;
    let issued = super::common::issue_pair(service, &user, &roles, false).await?;

    service
        .audit(
            AuditEntry::new("user.registered", "user")
                .actor(user.id)
                .resource(user.id)
                .correlation(request_id)
                .metadata(json!({ "email": email })),
        )
        .await;

    Ok(issued)
}
