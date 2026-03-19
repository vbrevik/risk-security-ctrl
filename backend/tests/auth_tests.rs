use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Router;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::str::FromStr;
use tower::ServiceExt;

use ontology_backend::{AppState, Config};

async fn setup_auth_test_app() -> (Router, SqlitePool) {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.unwrap();
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let config = Config {
        database_url: "sqlite::memory:".to_string(),
        host: "127.0.0.1".to_string(),
        port: 3000,
        frontend_url: "http://localhost:5173".to_string(),
        cookie_key: None,
        session_duration_hours: 8,
        behind_proxy: false,
        enable_https: false,
    };

    let cookie_key = axum_extra::extract::cookie::Key::generate();
    let state = AppState {
        db: pool.clone(),
        config,
        topics: vec![],
        cookie_key,
    };

    let app = ontology_backend::create_router(state);
    (app, pool)
}

fn json_request(method: &str, uri: &str, body: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap()
}

fn bearer_request(method: &str, uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

async fn body_json(response: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

#[tokio::test]
async fn register_creates_user() {
    let (app, _pool) = setup_auth_test_app().await;
    let resp = app
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "test@example.com",
                "name": "Test User",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = body_json(resp).await;
    assert_eq!(body["email"], "test@example.com");
    assert_eq!(body["name"], "Test User");
    assert_eq!(body["role"], "viewer");
    assert!(body.get("id").is_some());
    assert!(body.get("password_hash").is_none());
}

#[tokio::test]
async fn register_rejects_duplicate_email() {
    let (app, _pool) = setup_auth_test_app().await;

    let resp1 = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "dup@example.com",
                "name": "User1",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(resp1.status(), StatusCode::CREATED);

    let resp2 = app
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "dup@example.com",
                "name": "User2",
                "password": "password456"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(resp2.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body = body_json(resp2).await;
    assert_eq!(body["error"], "validation_error");
}

#[tokio::test]
async fn login_returns_token() {
    let (app, _pool) = setup_auth_test_app().await;

    // Register
    app.clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "login@example.com",
                "name": "Login User",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    // Login
    let resp = app
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            serde_json::json!({
                "email": "login@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    // Check Set-Cookie header
    let has_session_cookie = resp
        .headers()
        .get_all("set-cookie")
        .iter()
        .any(|v| v.to_str().unwrap_or("").contains("session_id"));
    assert!(has_session_cookie, "Response should set session_id cookie");

    let body = body_json(resp).await;
    assert!(body.get("token").is_some());
    assert!(body["user"]["email"] == "login@example.com");
}

#[tokio::test]
async fn login_rejects_wrong_password() {
    let (app, _pool) = setup_auth_test_app().await;

    app.clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "wrong@example.com",
                "name": "User",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    let resp = app
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            serde_json::json!({
                "email": "wrong@example.com",
                "password": "wrongpassword"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    let body = body_json(resp).await;
    assert_eq!(body["error"], "invalid_credentials");
}

#[tokio::test]
async fn login_rejects_unknown_email() {
    let (app, _pool) = setup_auth_test_app().await;

    let resp = app
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            serde_json::json!({
                "email": "nobody@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    let body = body_json(resp).await;
    assert_eq!(body["error"], "invalid_credentials");
}

#[tokio::test]
async fn me_returns_current_user() {
    let (app, _pool) = setup_auth_test_app().await;

    // Register
    app.clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "me@example.com",
                "name": "Me User",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    // Login
    let login_resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            serde_json::json!({
                "email": "me@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();
    let login_body = body_json(login_resp).await;
    let token = login_body["token"].as_str().unwrap();

    // Me
    let me_resp = app
        .oneshot(bearer_request("GET", "/api/auth/me", token))
        .await
        .unwrap();

    assert_eq!(me_resp.status(), StatusCode::OK);
    let me_body = body_json(me_resp).await;
    assert_eq!(me_body["email"], "me@example.com");
    assert_eq!(me_body["name"], "Me User");
    assert_eq!(me_body["role"], "viewer");
}

#[tokio::test]
async fn logout_invalidates_session() {
    let (app, _pool) = setup_auth_test_app().await;

    // Register + Login
    app.clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "logout@example.com",
                "name": "Logout User",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    let login_resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            serde_json::json!({
                "email": "logout@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();
    let login_body = body_json(login_resp).await;
    let token = login_body["token"].as_str().unwrap();

    // Logout
    let logout_resp = app
        .clone()
        .oneshot(bearer_request("POST", "/api/auth/logout", token))
        .await
        .unwrap();
    assert_eq!(logout_resp.status(), StatusCode::NO_CONTENT);

    // Me should fail now
    let me_resp = app
        .oneshot(bearer_request("GET", "/api/auth/me", token))
        .await
        .unwrap();
    assert_eq!(me_resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn logout_writes_audit_log() {
    let (app, pool) = setup_auth_test_app().await;

    // Register + Login + Logout
    app.clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "audit@example.com",
                "name": "Audit User",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();

    let login_resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            serde_json::json!({
                "email": "audit@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();
    let login_body = body_json(login_resp).await;
    let token = login_body["token"].as_str().unwrap();

    app.oneshot(bearer_request("POST", "/api/auth/logout", token))
        .await
        .unwrap();

    // Check audit log
    let entry: (String,) =
        sqlx::query_as("SELECT action FROM audit_log WHERE action = 'logout'")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(entry.0, "logout");
}

#[tokio::test]
async fn full_lifecycle() {
    let (app, _pool) = setup_auth_test_app().await;

    // Register
    let reg_resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/register",
            serde_json::json!({
                "email": "lifecycle@example.com",
                "name": "Lifecycle User",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(reg_resp.status(), StatusCode::CREATED);

    // Login
    let login_resp = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/api/auth/login",
            serde_json::json!({
                "email": "lifecycle@example.com",
                "password": "password123"
            }),
        ))
        .await
        .unwrap();
    assert_eq!(login_resp.status(), StatusCode::OK);
    let login_body = body_json(login_resp).await;
    let token = login_body["token"].as_str().unwrap();

    // Me
    let me_resp = app
        .clone()
        .oneshot(bearer_request("GET", "/api/auth/me", token))
        .await
        .unwrap();
    assert_eq!(me_resp.status(), StatusCode::OK);

    // Logout
    let logout_resp = app
        .clone()
        .oneshot(bearer_request("POST", "/api/auth/logout", token))
        .await
        .unwrap();
    assert_eq!(logout_resp.status(), StatusCode::NO_CONTENT);

    // Me fails
    let me_fail = app
        .oneshot(bearer_request("GET", "/api/auth/me", token))
        .await
        .unwrap();
    assert_eq!(me_fail.status(), StatusCode::UNAUTHORIZED);
}
