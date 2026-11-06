// User model per data-model.md Platform Domain entity
// Fields: id, email, password_hash, status, email_verified_at, created_at, updated_at

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;
use time::OffsetDateTime;

/// User platform entity
/// Owned by: Platform Domain (constitution.md §99-116)
/// Relationships: M:N with Role through user_roles junction
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Unique identifier
    pub id: Uuid,
    /// Normalized email address (unique, case-insensitive)
    pub email: String,
    /// Hashed password (never store plaintext per constitution.md §262-269)
    pub password_hash: String,
    /// User status lifecycle: pending → active → suspended → closed
    pub status: UserStatus,
    /// When email was verified (if applicable)
    pub email_verified_at: Option<OffsetDateTime>,
    /// When the user account was created
    pub created_at: OffsetDateTime,
    /// When the user account was last updated
    pub updated_at: OffsetDateTime,
}

/// User status lifecycle per constitution.md §100-116
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    /// Initial state after registration
    Pending,
    /// Active account, fully operational
    Active,
    /// Account suspended by administrator
    Suspended,
    /// Account closed/deleted
    Closed,
}

/// Create a new user with pending status
impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        let now = OffsetDateTime::now_local()
            .expect("Failed to get current time");
        User {
            id: Uuid::new_v4(),
            email,
            password_hash,
            status: UserStatus::Pending,
            email_verified_at: None,
            created_at: now,
            updated_at: now,
        }
    }

/// Activate a user account (transition from Pending → Active)
    pub fn activate(&mut self) {
        self.status = UserStatus::Active;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
    }

/// Suspend a user account (administrator action)
    pub fn suspend(&mut self) {
        self.status = UserStatus::Suspended;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
    }

/// Close a user account (administrator or user action)
    pub fn close(&mut self) {
        self.status = UserStatus::Closed;
        self.updated_at = OffsetDateTime::now_local()
            .expect("Failed to get current time");
    }
}

/// Check if user can perform authentication actions
    pub fn can_authenticate(&self) -> bool {
        matches!(self.status, UserStatus::Pending | UserStatus::Active)
    }

/// Check if user account is active and operational
    pub fn is_active(&self) -> bool {
        matches!(self.status, UserStatus::Active)
    }

/// Check if account is closed/deleted
    pub fn is_closed(&self) -> bool {
        matches!(self.status, UserStatus::Closed)
    }
}

/// Convert User to authentication response DTO (no sensitive data)
    pub fn to_auth_dto(&self) -> AuthUserDto {
        AuthUserDto {
            id: self.id,
            email: self.email.clone(),
            status: self.status,
            email_verified_at: self.email_verified_at,
        }
    }
}

/// DTO for authentication responses (no password hash)
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct AuthUserDto {
        /// Unique identifier
        pub id: Uuid,
        /// Email address
        pub email: String,
        /// User status
        pub status: UserStatus,
        /// When email was verified (if applicable)
        pub email_verified_at: Option<OffsetDateTime>,
    }
}