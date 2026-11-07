//! User retrieval with ownership/admin authorization.

use uuid::Uuid;

use crate::platform::application::ports::UserView;
use crate::platform::application::service::AuthService;
use crate::platform::domain::errors::PlatformError;

pub async fn execute(
    service: &AuthService,
    caller_id: Uuid,
    caller_roles: &[String],
    target_id: Uuid,
) -> Result<UserView, PlatformError> {
    authorize(caller_id, caller_roles, target_id)?;

    let Some(user) = service.users.find_by_id(target_id).await? else {
        return Err(PlatformError::NotFound);
    };
    let roles = service.roles.codes_for_user(user.id).await?;
    Ok(UserView { user, roles })
}

pub(crate) fn authorize(
    caller_id: Uuid,
    caller_roles: &[String],
    target_id: Uuid,
) -> Result<(), PlatformError> {
    if caller_id == target_id || caller_roles.iter().any(|role| role == "admin") {
        Ok(())
    } else {
        Err(PlatformError::Forbidden)
    }
}
