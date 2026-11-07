//! Domain errors for the Platform bounded context.

#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("an account with this email already exists")]
    EmailAlreadyExists,
    #[error("invalid email or password")]
    InvalidCredentials,
    #[error("authentication is required")]
    Unauthorized,
    #[error("you do not have permission to perform this action")]
    Forbidden,
    #[error("resource not found")]
    NotFound,
    #[error("account is not active")]
    AccountInactive,
    #[error("token is invalid, revoked, or expired")]
    InvalidToken,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl PlatformError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::EmailAlreadyExists => "EMAIL_ALREADY_EXISTS",
            Self::InvalidCredentials => "INVALID_CREDENTIALS",
            Self::Unauthorized | Self::InvalidToken => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "RESOURCE_NOT_FOUND",
            Self::AccountInactive => "ACCOUNT_INACTIVE",
            Self::Conflict(_) => "CONFLICT",
            Self::Internal(_) => "INTERNAL_ERROR",
        }
    }
}
