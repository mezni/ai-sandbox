//! Application service that wires ports together for identity use cases.

use std::sync::Arc;

use crate::platform::domain::errors::PlatformError;

use super::policy::AuthPolicy;
use super::ports::{
    AccessClaims, AuditRepository, PasswordHasher, RefreshTokenRepository, RoleRepository,
    TokenIssuer, UserRepository,
};

pub struct AuthService {
    pub(crate) users: Arc<dyn UserRepository>,
    pub(crate) roles: Arc<dyn RoleRepository>,
    pub(crate) refresh_tokens: Arc<dyn RefreshTokenRepository>,
    pub(crate) audit: Arc<dyn AuditRepository>,
    pub(crate) hasher: Arc<dyn PasswordHasher>,
    pub(crate) issuer: Arc<dyn TokenIssuer>,
    pub(crate) policy: AuthPolicy,
}

impl AuthService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        users: Arc<dyn UserRepository>,
        roles: Arc<dyn RoleRepository>,
        refresh_tokens: Arc<dyn RefreshTokenRepository>,
        audit: Arc<dyn AuditRepository>,
        hasher: Arc<dyn PasswordHasher>,
        issuer: Arc<dyn TokenIssuer>,
        policy: AuthPolicy,
    ) -> Self {
        Self {
            users,
            roles,
            refresh_tokens,
            audit,
            hasher,
            issuer,
            policy,
        }
    }

    pub fn authenticate_access_token(&self, token: &str) -> Result<AccessClaims, PlatformError> {
        self.issuer.decode_access(token)
    }

    /// Audit is best-effort: authentication must not fail because of audit storage.
    pub(crate) async fn audit(&self, entry: crate::platform::domain::audit::AuditEntry) {
        if let Err(error) = self.audit.record(&entry).await {
            tracing::warn!(%error, action = %entry.action, "failed to record audit entry");
        }
    }
}
