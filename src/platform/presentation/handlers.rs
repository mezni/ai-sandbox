//! HTTP handlers for auth and user endpoints.

use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use uuid::Uuid;

use crate::platform::application::use_cases::{
    get_current_user, get_user, login, logout, refresh, register, update_user,
};
use crate::platform::presentation::dto::{
    LoginRequest, LogoutRequest, RefreshRequest, RegisterRequest, Success, Tokens,
    UpdateUserRequest, UserResponse,
};
use crate::platform::presentation::errors::ApiError;
use crate::platform::presentation::middleware::auth::AuthUser;
use crate::platform::presentation::middleware::request_id::RequestId;
use crate::AppState;

pub async fn register(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response, ApiError> {
    let issued = register::execute(
        &state.auth,
        &payload.email,
        &payload.password,
        Some(request_id.0),
    )
    .await
    .map_err(|error| ApiError::from_platform(error, request_id.0))?;

    Ok((
        StatusCode::CREATED,
        Json(Success::new(Tokens::from(issued), request_id.0)),
    )
        .into_response())
}

pub async fn login(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, ApiError> {
    let issued = login::execute(
        &state.auth,
        &payload.email,
        &payload.password,
        payload.remember_me,
        Some(request_id.0),
    )
    .await
    .map_err(|error| ApiError::from_platform(error, request_id.0))?;

    Ok(Json(Success::new(Tokens::from(issued), request_id.0)).into_response())
}

pub async fn refresh(
    State(state): State<AppState>,
    Extension(request_id): Extension<RequestId>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Response, ApiError> {
    let issued = refresh::execute(&state.auth, &payload.refresh_token, Some(request_id.0))
        .await
        .map_err(|error| ApiError::from_platform(error, request_id.0))?;

    Ok(Json(Success::new(Tokens::from(issued), request_id.0)).into_response())
}

pub async fn logout(
    State(state): State<AppState>,
    caller: AuthUser,
    Extension(request_id): Extension<RequestId>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Response, ApiError> {
    logout::execute(
        &state.auth,
        caller.user_id,
        &payload.refresh_token,
        Some(request_id.0),
    )
    .await
    .map_err(|error| ApiError::from_platform(error, request_id.0))?;

    Ok(StatusCode::NO_CONTENT.into_response())
}

pub async fn me(
    State(state): State<AppState>,
    caller: AuthUser,
    Extension(request_id): Extension<RequestId>,
) -> Result<Response, ApiError> {
    let view = get_current_user::execute(&state.auth, caller.user_id)
        .await
        .map_err(|error| ApiError::from_platform(error, request_id.0))?;

    Ok(Json(Success::new(UserResponse::from(view), request_id.0)).into_response())
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    caller: AuthUser,
    Extension(request_id): Extension<RequestId>,
) -> Result<Response, ApiError> {
    let view = get_user::execute(&state.auth, caller.user_id, &caller.roles, user_id)
        .await
        .map_err(|error| ApiError::from_platform(error, request_id.0))?;

    Ok(Json(Success::new(UserResponse::from(view), request_id.0)).into_response())
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    caller: AuthUser,
    Extension(request_id): Extension<RequestId>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Response, ApiError> {
    let view = update_user::execute(
        &state.auth,
        caller.user_id,
        &caller.roles,
        user_id,
        &payload.email,
        Some(request_id.0),
    )
    .await
    .map_err(|error| ApiError::from_platform(error, request_id.0))?;

    Ok(Json(Success::new(UserResponse::from(view), request_id.0)).into_response())
}
