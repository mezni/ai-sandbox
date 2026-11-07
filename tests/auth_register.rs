mod common;

use axum::http::{Method, StatusCode};
use common::{call, register, setup, unique_email, VALID_PASSWORD};

#[tokio::test]
async fn register_succeeds_and_returns_tokens() {
    let app = setup().await;
    let email = unique_email("register-success");

    let body = register(&app, &email).await;

    assert!(body["data"]["access_token"]
        .as_str()
        .is_some_and(|token| !token.is_empty()));
    assert!(body["data"]["refresh_token"]
        .as_str()
        .is_some_and(|token| !token.is_empty()));
    assert_eq!(body["data"]["token_type"], "Bearer");
    assert!(body["meta"]["request_id"].is_string());
}

#[tokio::test]
async fn duplicate_email_returns_409() {
    let app = setup().await;
    let email = unique_email("register-duplicate");
    register(&app, &email).await;

    let (status, body) = call(
        &app,
        Method::POST,
        "/api/v1/auth/register",
        None,
        Some(serde_json::json!({ "email": email, "password": VALID_PASSWORD })),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "EMAIL_ALREADY_EXISTS");
}

#[tokio::test]
async fn invalid_input_returns_400() {
    let app = setup().await;

    let (status, body) = call(
        &app,
        Method::POST,
        "/api/v1/auth/register",
        None,
        Some(serde_json::json!({ "email": "not-an-email", "password": "short" })),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "VALIDATION_ERROR");
    assert!(body["error"]["request_id"].is_string());
}

#[tokio::test]
async fn registration_records_audit_entries() {
    let app = setup().await;
    let email = unique_email("register-audit");
    let registration = register(&app, &email).await;
    let access = registration["data"]["access_token"].as_str().unwrap();

    let (_, me) = call(&app, Method::GET, "/api/v1/auth/me", Some(access), None).await;
    let user_id = me["data"]["id"].as_str().unwrap().to_string();

    let pool = common::pool().await;
    let actions: Vec<(String,)> = sqlx::query_as(
        "SELECT action FROM platform.audit_logs WHERE resource_id = $1::uuid ORDER BY action",
    )
    .bind(&user_id)
    .fetch_all(&pool)
    .await
    .expect("query audit logs");

    let actions: Vec<&str> = actions.iter().map(|(action,)| action.as_str()).collect();
    assert!(actions.contains(&"user.registered"), "{actions:?}");
    assert!(actions.contains(&"role.assigned"), "{actions:?}");
}
