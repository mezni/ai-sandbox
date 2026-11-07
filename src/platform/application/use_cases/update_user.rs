//! User profile update with ownership/admin authorization.

use serde_json::json;
use uuid::Uuid;

use crate::platform::application::ports::UserView;
use crate::platform::application::service::AuthService;
use crate::platform::domain::audit::AuditEntry;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::user::{is_valid_email, normalize_email};

pub async fn execute(
    service: &AuthService,
    caller_id: Uuid,
    caller_roles: &[String],
    target_id: Uuid,
    email: &str,
    request_id: Option<Uuid>,
) -> Result<UserView, PlatformError> {
    super::get_user::authorize(caller_id, caller_roles, target_id)?;

    let email = normalize_email(email);
    if !is_valid_email(&email) {
        return Err(PlatformError::Validation(
            "email must be a valid email address".to_string(),
        ));
    }

    let user = service.users.update_email(target_id, &email).await?;
    let roles = service.roles.codes_for_user(user.id).await?;

    service
        .audit(
            AuditEntry::new("user.updated", "user")
                .actor(caller_id)
                .resource(user.id)
                .correlation(request_id)
                .metadata(json!({ "email": email })),
        )
        .await;

    Ok(UserView { user, roles })
}
