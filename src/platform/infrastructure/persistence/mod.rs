pub mod audit_repository;
pub mod refresh_token_repository;
pub mod role_repository;
pub mod user_repository;

use crate::platform::domain::errors::PlatformError;

pub(crate) fn internal(err: sqlx::Error) -> PlatformError {
    PlatformError::Internal(format!("database error: {err}"))
}

pub(crate) fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db) if db.code().as_deref() == Some("23505"))
}
