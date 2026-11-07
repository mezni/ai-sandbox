//! Refresh-session entity. Only the hash of the opaque token is persisted.

use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl RefreshToken {
    pub fn new(user_id: Uuid, token_hash: String, expires_at: OffsetDateTime) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            expires_at,
            revoked_at: None,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn is_active(&self, now: OffsetDateTime) -> bool {
        self.revoked_at.is_none() && self.expires_at > now
    }
}
