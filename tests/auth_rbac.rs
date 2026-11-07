mod common;

use axum::http::{Method, StatusCode};
use common::{assign_role, call, login, register, setup, unique_email, VALID_PASSWORD};

async fn user_id(app: &common::TestApp, access_token: &str) -> String {
    let (status, body) = call(
        app,
        Method::GET,
        "/api/v1/auth/me",
        Some(access_token),
        None,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    body["data"]["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn ownership_and_admin_override_are_enforced() {
    let app = setup().await;

    let email_a = unique_email("rbac-a");
    let email_b = unique_email("rbac-b");
    let registration_a = register(&app, &email_a).await;
    let registration_b = register(&app, &email_b).await;

    let access_a = registration_a["data"]["access_token"].as_str().unwrap();
    let access_b = registration_b["data"]["access_token"].as_str().unwrap();

    let id_a = user_id(&app, access_a).await;
    let id_b = user_id(&app, access_b).await;

    let (forbidden, body) = call(
        &app,
        Method::GET,
        &format!("/api/v1/users/{id_b}"),
        Some(access_a),
        None,
    )
    .await;
    assert_eq!(forbidden, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");

    let (own_status, own_body) = call(
        &app,
        Method::GET,
        &format!("/api/v1/users/{id_a}"),
        Some(access_a),
        None,
    )
    .await;
    assert_eq!(own_status, StatusCode::OK);
    assert_eq!(own_body["data"]["email"], email_a);

    let (unauthenticated, _) = call(
        &app,
        Method::GET,
        &format!("/api/v1/users/{id_a}"),
        None,
        None,
    )
    .await;
    assert_eq!(unauthenticated, StatusCode::UNAUTHORIZED);

    let (patch_forbidden, _) = call(
        &app,
        Method::PATCH,
        &format!("/api/v1/users/{id_b}"),
        Some(access_a),
        Some(serde_json::json!({ "email": unique_email("rbac-hijack") })),
    )
    .await;
    assert_eq!(patch_forbidden, StatusCode::FORBIDDEN);

    let new_email_a = unique_email("rbac-a-updated");
    let (patch_own, patched) = call(
        &app,
        Method::PATCH,
        &format!("/api/v1/users/{id_a}"),
        Some(access_a),
        Some(serde_json::json!({ "email": new_email_a })),
    )
    .await;
    assert_eq!(patch_own, StatusCode::OK);
    assert_eq!(patched["data"]["email"], new_email_a);

    // Promote A to admin and re-authenticate so the token carries the new role.
    assign_role(&new_email_a, "admin").await;
    let (login_status, login_body) = login(&app, &new_email_a, VALID_PASSWORD).await;
    assert_eq!(login_status, StatusCode::OK);
    let admin_access = login_body["data"]["access_token"].as_str().unwrap();

    let (admin_get, _) = call(
        &app,
        Method::GET,
        &format!("/api/v1/users/{id_b}"),
        Some(admin_access),
        None,
    )
    .await;
    assert_eq!(admin_get, StatusCode::OK);

    let new_email_b = unique_email("rbac-b-updated");
    let (admin_patch, admin_patched) = call(
        &app,
        Method::PATCH,
        &format!("/api/v1/users/{id_b}"),
        Some(admin_access),
        Some(serde_json::json!({ "email": new_email_b })),
    )
    .await;
    assert_eq!(admin_patch, StatusCode::OK);
    assert_eq!(admin_patched["data"]["email"], new_email_b);
}

#[tokio::test]
async fn duplicate_email_on_update_returns_409() {
    let app = setup().await;

    let email_a = unique_email("rbac-dup-a");
    let email_b = unique_email("rbac-dup-b");
    let registration_a = register(&app, &email_a).await;
    register(&app, &email_b).await;

    let access_a = registration_a["data"]["access_token"].as_str().unwrap();
    let id_a = user_id(&app, access_a).await;

    let (status, body) = call(
        &app,
        Method::PATCH,
        &format!("/api/v1/users/{id_a}"),
        Some(access_a),
        Some(serde_json::json!({ "email": email_b })),
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], "EMAIL_ALREADY_EXISTS");
}
