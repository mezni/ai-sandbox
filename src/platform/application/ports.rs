//! Application ports (interfaces implemented by infrastructure).

use async_trait::async_trait;
use uuid::Uuid;

use crate::platform::domain::audit::AuditEntry;
use crate::platform::domain::errors::PlatformError;
use crate::platform::domain::refresh_token::RefreshToken;
use crate::platform::domain::role::Role;
use crate::platform::domain::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, PlatformError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, PlatformError>;
    async fn create(&self, user: &User) -> Result<User, PlatformError>;
    async fn update_email(&self, id: Uuid, email: &str) -> Result<User, PlatformError>;
}

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_by_code(&self, code: &str) -> Result<Option<Role>, PlatformError>;
    async fn codes_for_user(&self, user_id: Uuid) -> Result<Vec<String>, PlatformError>;
    async fn assign_role(&self, user_id: Uuid, role_code: &str) -> Result<(), PlatformError>;
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn insert(&self, token: &RefreshToken) -> Result<(), PlatformError>;
    async fn find_active_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<RefreshToken>, PlatformError>;
    /// Revoke the old token and insert the replacement atomically.
    /// Returns `Unauthorized` when the old token was already rotated/revoked.
    async fn rotate(&self, old_id: Uuid, new_token: &RefreshToken) -> Result<(), PlatformError>;
    async fn revoke_by_hash_for_user(
        &self,
        token_hash: &str,
        user_id: Uuid,
    ) -> Result<(), PlatformError>;
}

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn record(&self, entry: &AuditEntry) -> Result<(), PlatformError>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, PlatformError>;
    fn verify(&self, password: &str, hash: &str) -> Result<bool, PlatformError>;
}

#[derive(Debug, Clone)]
pub struct AccessClaims {
    pub sub: Uuid,
    pub roles: Vec<String>,
    pub iat: i64,
    pub exp: i64,
}

pub trait TokenIssuer: Send + Sync {
    fn issue_access(&self, user: &User, roles: &[String]) -> Result<(String, u64), PlatformError>;
    fn decode_access(&self, token: &str) -> Result<AccessClaims, PlatformError>;
}

#[derive(Debug, Clone)]
pub struct IssuedTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct UserView {
    pub user: User,
    pub roles: Vec<String>,
}
