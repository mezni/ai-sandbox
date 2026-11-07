//! HTTP error mapping to the standard BioSouk error envelope.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::Value;
use uuid::Uuid;

use crate::platform::domain::errors::PlatformError;

use super::dto::{ErrorBody, ErrorEnvelope};

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    pub message: String,
    pub details: Vec<Value>,
    pub request_id: Uuid,
}

impl ApiError {
    pub fn from_platform(error: PlatformError, request_id: Uuid) -> Self {
        let status = match &error {
            PlatformError::Validation(_) => StatusCode::BAD_REQUEST,
            PlatformError::EmailAlreadyExists | PlatformError::Conflict(_) => StatusCode::CONFLICT,
            PlatformError::InvalidCredentials
            | PlatformError::Unauthorized
            | PlatformError::InvalidToken
            | PlatformError::AccountInactive => StatusCode::UNAUTHORIZED,
            PlatformError::Forbidden => StatusCode::FORBIDDEN,
            PlatformError::NotFound => StatusCode::NOT_FOUND,
            PlatformError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match &error {
            PlatformError::Internal(_) => "An internal error occurred.".to_string(),
            other => other.to_string(),
        };

        Self {
            status,
            code: error.code(),
            message,
            details: Vec::new(),
            request_id,
        }
    }

    pub fn unauthorized(request_id: Uuid) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "UNAUTHORIZED",
            message: "Authentication is required.".to_string(),
            details: Vec::new(),
            request_id,
        }
    }

    pub fn forbidden(request_id: Uuid) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "FORBIDDEN",
            message: "You do not have permission to perform this action.".to_string(),
            details: Vec::new(),
            request_id,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorEnvelope {
            error: ErrorBody {
                code: self.code.to_string(),
                message: self.message,
                details: self.details,
                request_id: self.request_id,
            },
        };

        (self.status, Json(body)).into_response()
    }
}
