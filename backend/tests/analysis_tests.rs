use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

mod common;
use common::create_test_app;

/// Proves the analysis router is registered under /api/analyses.
/// A GET to the list endpoint should return 200 (not 404).
#[tokio::test]
async fn test_router_registration() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/analyses")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Must not be 404 — that would mean the route is not registered.
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
    // The stub handler should return 200 with an empty list or similar.
    assert_eq!(response.status(), StatusCode::OK);
}
