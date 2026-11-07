mod common;

use axum::http::{Method, StatusCode};
use common::{login, register, set_status, setup, unique_email, VALID_PASSWORD};

#[tokio::test]
async fn login_succeeds_with_valid_credentials() {
    let app = setup().await;
    let email = unique_email("login-success");
    register(&app, &email).await;

    let (status, body) = login(&app, &email, VALID_PASSWORD).await;

    assert_eq!(status, StatusCode::OK, "login failed: {body}");
    assert!(body["data"]["access_token"].is_string());
    assert!(body["data"]["refresh_token"].is_string());
}

#[tokio::test]
async fn wrong_password_and_unknown_email_are_indistinguishable() {
    let app = setup().await;
    let email = unique_email("login-invalid");
    register(&app, &email).await;

    let (wrong_status, wrong_body) = login(&app, &email, "definitely-wrong-1").await;
    assert_eq!(wrong_status, StatusCode::UNAUTHORIZED);
    assert_eq!(wrong_body["error"]["code"], "INVALID_CREDENTIALS");

    let (unknown_status, unknown_body) =
        login(&app, &unique_email("login-unknown"), VALID_PASSWORD).await;
    assert_eq!(unknown_status, StatusCode::UNAUTHORIZED);
    assert_eq!(unknown_body["error"]["code"], "INVALID_CREDENTIALS");
    assert_eq!(
        wrong_body["error"]["message"],
        unknown_body["error"]["message"]
    );
}

#[tokio::test]
async fn suspended_user_cannot_authenticate() {
    let app = setup().await;
    let email = unique_email("login-suspended");
    register(&app, &email).await;
    set_status(&email, "suspended").await;

    let (status, _) = login(&app, &email, VALID_PASSWORD).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn remember_me_extends_refresh_lifetime() {
    let app = setup().await;
    let email = unique_email("login-remember");
    register(&app, &email).await;

    let (_, normal) = login(&app, &email, VALID_PASSWORD).await;

    let (status, remembered) = common::call(
        &app,
        Method::POST,
        "/api/v1/auth/login",
        None,
        Some(serde_json::json!({
            "email": email,
            "password": VALID_PASSWORD,
            "remember_me": true
        })),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let normal_ttl = normal["data"]["refresh_expires_in"].as_u64().unwrap();
    let remembered_ttl = remembered["data"]["refresh_expires_in"].as_u64().unwrap();
    assert!(remembered_ttl > normal_ttl);
}
