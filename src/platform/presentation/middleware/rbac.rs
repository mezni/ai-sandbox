//! Role and ownership authorization guards.

use uuid::Uuid;

use crate::platform::domain::errors::PlatformError;

use super::auth::AuthUser;

/// Require at least one of the allowed role codes.
pub fn require_role(caller: &AuthUser, allowed: &[&str]) -> Result<(), PlatformError> {
    if allowed.iter().any(|role| caller.has_role(role)) {
        Ok(())
    } else {
        Err(PlatformError::Forbidden)
    }
}

/// Allow access only to the resource owner or an administrator.
pub fn require_owner_or_admin(caller: &AuthUser, owner_id: Uuid) -> Result<(), PlatformError> {
    if caller.user_id == owner_id || caller.is_admin() {
        Ok(())
    } else {
        Err(PlatformError::Forbidden)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::application::ports::AccessClaims;

    fn caller(id: Uuid, roles: &[&str]) -> AuthUser {
        let roles: Vec<String> = roles.iter().map(|role| role.to_string()).collect();
        AuthUser {
            user_id: id,
            roles: roles.clone(),
            claims: AccessClaims {
                sub: id,
                roles,
                iat: 0,
                exp: 0,
            },
        }
    }

    #[test]
    fn role_guard_allows_and_denies() {
        let customer = caller(Uuid::new_v4(), &["customer"]);
        let admin = caller(Uuid::new_v4(), &["admin"]);

        assert!(require_role(&customer, &["admin"]).is_err());
        assert!(require_role(&admin, &["admin"]).is_ok());
        assert!(require_role(&customer, &["customer", "producer"]).is_ok());
    }

    #[test]
    fn ownership_guard_respects_admin_override() {
        let owner = Uuid::new_v4();
        let other = Uuid::new_v4();

        let customer = caller(owner, &["customer"]);
        assert!(require_owner_or_admin(&customer, owner).is_ok());
        assert!(require_owner_or_admin(&customer, other).is_err());

        let admin = caller(Uuid::new_v4(), &["admin"]);
        assert!(require_owner_or_admin(&admin, other).is_ok());
    }

    #[test]
    fn caller_without_roles_is_denied() {
        let anonymous = caller(Uuid::new_v4(), &[]);
        assert!(require_role(&anonymous, &["customer"]).is_err());
    }
}
