mod common;

use axum::http::{Method, StatusCode};
use common::{call, login, register, setup, unique_email, TEST_JWT_SECRET, VALID_PASSWORD};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

fn expired_access_token(user_id: &str) -> String {
    let now = time::OffsetDateTime::now_utc().unix_timestamp();
    let claims = serde_json::json!({
        "sub": user_id,
        "roles": ["customer"],
        "iss": "biosouk",
        "aud": "biosouk-users",
        "iat": now - 3600,
        "exp": now - 3600
    });

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(TEST_JWT_SECRET.as_bytes()),
    )
    .expect("encode expired token")
}

#[tokio::test]
async fn me_returns_current_user_without_secrets() {
    let app = setup().await;
    let email = unique_email("session-me");
    let registration = register(&app, &email).await;
    let access = registration["data"]["access_token"].as_str().unwrap();

    let (status, body) = call(&app, Method::GET, "/api/v1/auth/me", Some(access), None).await;

    assert_eq!(status, StatusCode::OK, "me failed: {body}");
    assert_eq!(body["data"]["email"], email);
    assert_eq!(body["data"]["status"], "pending");
    assert!(body["data"]["roles"]
        .as_array()
        .unwrap()
        .iter()
        .any(|role| role == "customer"));

    let payload = body["data"].as_object().unwrap();
    assert!(!payload.contains_key("password_hash"));
    assert!(!payload.contains_key("password"));
}

#[tokio::test]
async fn protected_route_requires_token() {
    let app = setup().await;
    let (status, body) = call(&app, Method::GET, "/api/v1/auth/me", None, None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHORIZED");
}

#[tokio::test]
async fn expired_access_token_is_rejected() {
    let app = setup().await;
    let email = unique_email("session-expired");
    let registration = register(&app, &email).await;
    let access = registration["data"]["access_token"].as_str().unwrap();

    let (_, me) = call(&app, Method::GET, "/api/v1/auth/me", Some(access), None).await;
    let user_id = me["data"]["id"].as_str().unwrap().to_string();

    let expired = expired_access_token(&user_id);
    let (status, _) = call(&app, Method::GET, "/api/v1/auth/me", Some(&expired), None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn refresh_rotates_and_old_token_is_rejected() {
    let app = setup().await;
    let email = unique_email("session-refresh");
    let registration = register(&app, &email).await;
    let refresh = registration["data"]["refresh_token"].as_str().unwrap();

    let (status, rotated) = call(
        &app,
        Method::POST,
        "/api/v1/auth/refresh",
        None,
        Some(serde_json::json!({ "refresh_token": refresh })),
    )
    .await;

    assert_eq!(status, StatusCode::OK, "refresh failed: {rotated}");
    assert!(rotated["data"]["access_token"].is_string());
    assert!(rotated["data"]["refresh_token"].is_string());
    assert_ne!(rotated["data"]["refresh_token"], refresh);

    let (reuse_status, _) = call(
        &app,
        Method::POST,
        "/api/v1/auth/refresh",
        None,
        Some(serde_json::json!({ "refresh_token": refresh })),
    )
    .await;

    assert_eq!(reuse_status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn logout_revokes_the_refresh_session() {
    let app = setup().await;
    let email = unique_email("session-logout");
    let registration = register(&app, &email).await;
    let access = registration["data"]["access_token"].as_str().unwrap();
    let refresh = registration["data"]["refresh_token"].as_str().unwrap();

    let (status, _) = call(
        &app,
        Method::POST,
        "/api/v1/auth/logout",
        Some(access),
        Some(serde_json::json!({ "refresh_token": refresh })),
    )
    .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    let (refresh_status, _) = call(
        &app,
        Method::POST,
        "/api/v1/auth/refresh",
        None,
        Some(serde_json::json!({ "refresh_token": refresh })),
    )
    .await;
    assert_eq!(refresh_status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn concurrent_refresh_with_same_token_succeeds_once() {
    let app = setup().await;
    let email = unique_email("session-concurrent");
    let registration = register(&app, &email).await;
    let refresh = registration["data"]["refresh_token"].as_str().unwrap();

    let body = serde_json::json!({ "refresh_token": refresh });
    let (first, second) = tokio::join!(
        call(
            &app,
            Method::POST,
            "/api/v1/auth/refresh",
            None,
            Some(body.clone())
        ),
        call(
            &app,
            Method::POST,
            "/api/v1/auth/refresh",
            None,
            Some(body.clone())
        )
    );

    let successes = [first.0, second.0]
        .iter()
        .filter(|status| **status == StatusCode::OK)
        .count();
    assert_eq!(successes, 1, "exactly one refresh should succeed");
}

#[tokio::test]
async fn login_after_logout_still_works() {
    let app = setup().await;
    let email = unique_email("session-relogin");
    register(&app, &email).await;

    let (status, body) = login(&app, &email, VALID_PASSWORD).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["data"]["access_token"].is_string());
}
