//! Current-user retrieval use case.

use uuid::Uuid;

use crate::platform::application::ports::UserView;
use crate::platform::application::service::AuthService;
use crate::platform::domain::errors::PlatformError;

pub async fn execute(service: &AuthService, user_id: Uuid) -> Result<UserView, PlatformError> {
    let Some(user) = service.users.find_by_id(user_id).await? else {
        return Err(PlatformError::Unauthorized);
    };
    let roles = service.roles.codes_for_user(user.id).await?;
    Ok(UserView { user, roles })
}
