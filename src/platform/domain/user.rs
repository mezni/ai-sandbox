//! User aggregate and its invariants.

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use super::errors::PlatformError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Pending,
    Active,
    Suspended,
    Closed,
}

impl UserStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Suspended => "suspended",
            Self::Closed => "closed",
        }
    }

    pub fn from_db(value: &str) -> Result<Self, PlatformError> {
        match value {
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "suspended" => Ok(Self::Suspended),
            "closed" => Ok(Self::Closed),
            other => Err(PlatformError::Internal(format!(
                "unknown user status: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub status: UserStatus,
    pub email_verified_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            email: normalize_email(&email),
            password_hash,
            status: UserStatus::Pending,
            email_verified_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn can_authenticate(&self) -> bool {
        matches!(self.status, UserStatus::Pending | UserStatus::Active)
    }
}

/// Normalize an email address for case-insensitive uniqueness.
pub fn normalize_email(email: &str) -> String {
    email.trim().to_lowercase()
}

/// Minimal, dependency-free email format validation.
pub fn is_valid_email(email: &str) -> bool {
    let email = email.trim();
    if email.is_empty() || email.len() > 320 || email.chars().any(char::is_whitespace) {
        return false;
    }

    let mut parts = email.split('@');
    let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };

    !local.is_empty()
        && !domain.is_empty()
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !domain.contains("..")
}

/// Password policy: length plus a mix of letters and digits.
pub fn is_valid_password(password: &str, min_length: usize) -> bool {
    let length = password.chars().count();
    length >= min_length
        && length <= 128
        && password.chars().any(char::is_alphabetic)
        && password.chars().any(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_and_validates_emails() {
        assert_eq!(normalize_email("  User@Example.COM "), "user@example.com");
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("test+tag@example.co.uk"));
        assert!(!is_valid_email("not-an-email"));
        assert!(!is_valid_email("user@localhost"));
        assert!(!is_valid_email("user@@example.com"));
    }

    #[test]
    fn enforces_password_policy() {
        assert!(is_valid_password("S3cure-passphrase!", 12));
        assert!(!is_valid_password("short", 12));
        assert!(!is_valid_password("onlylettershere", 12));
        assert!(!is_valid_password("123456789012", 12));
    }

    #[test]
    fn registration_starts_pending_and_can_authenticate() {
        let user = User::new("User@Example.com".into(), "$argon2id$hash".into());
        assert_eq!(user.email, "user@example.com");
        assert_eq!(user.status, UserStatus::Pending);
        assert!(user.can_authenticate());
    }
}
