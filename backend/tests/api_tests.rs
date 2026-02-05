use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

mod common;
use common::create_test_app;

#[tokio::test]
async fn test_health_check() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
}

#[tokio::test]
async fn test_list_frameworks() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/frameworks")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.is_array());
    let frameworks = json.as_array().unwrap();
    assert!(frameworks.len() >= 3); // ISO 31000, ISO 31010, NIST CSF

    // Check structure of first framework
    let first = &frameworks[0];
    assert!(first["id"].is_string());
    assert!(first["name"].is_string());
}

#[tokio::test]
async fn test_get_framework_by_id() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/frameworks/iso31000")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "iso31000");
    assert_eq!(json["name"], "ISO 31000:2018");
    assert!(json["description"].is_string());
}

#[tokio::test]
async fn test_get_framework_not_found() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/frameworks/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_concepts() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts?page=1&limit=10")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_array());
    assert!(json["total"].is_number());
    assert!(json["page"].is_number());
    assert!(json["limit"].is_number());
    assert_eq!(json["page"], 1);
    assert_eq!(json["limit"], 10);

    let data = json["data"].as_array().unwrap();
    assert!(!data.is_empty());
    assert!(data.len() <= 10);

    // Check structure of first concept
    let first = &data[0];
    assert!(first["id"].is_string());
    assert!(first["framework_id"].is_string());
    assert!(first["name_en"].is_string());
}

#[tokio::test]
async fn test_list_concepts_with_framework_filter() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts?framework_id=iso31000&limit=50")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let data = json["data"].as_array().unwrap();

    // All concepts should be from iso31000 framework
    for concept in data {
        assert_eq!(concept["framework_id"], "iso31000");
    }
}

#[tokio::test]
async fn test_get_concept_by_id() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/iso31000-principles")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "iso31000-principles");
    assert_eq!(json["framework_id"], "iso31000");
    assert!(json["name_en"].is_string());
}

#[tokio::test]
async fn test_get_concept_not_found() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_get_concept_relationships() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/iso31000-principles/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "iso31000-principles");
    assert!(json["related_concepts"].is_array());
}

#[tokio::test]
async fn test_search_concepts() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/search?q=risk&limit=20")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["data"].is_array());
    assert!(json["total"].is_number());

    let data = json["data"].as_array().unwrap();

    // Verify search results contain the search term in any of the searchable fields
    for concept in data {
        let name_en = concept["name_en"].as_str().unwrap_or("");
        let name_nb = concept["name_nb"].as_str().unwrap_or("");
        let def_en = concept["definition_en"].as_str().unwrap_or("");
        let def_nb = concept["definition_nb"].as_str().unwrap_or("");
        let has_term = name_en.to_lowercase().contains("risk")
            || name_nb.to_lowercase().contains("risk")
            || def_en.to_lowercase().contains("risk")
            || def_nb.to_lowercase().contains("risk");
        assert!(has_term, "Search result should contain 'risk' in at least one field");
    }
}

#[tokio::test]
async fn test_list_relationships() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.is_array());
    let relationships = json.as_array().unwrap();
    assert!(!relationships.is_empty());

    // Check structure of first relationship
    if let Some(first) = relationships.first() {
        assert!(first["id"].is_string());
        assert!(first["source_concept_id"].is_string());
        assert!(first["target_concept_id"].is_string());
        assert!(first["relationship_type"].is_string());
    }
}

#[tokio::test]
async fn test_pagination() {
    let app = create_test_app().await;

    // Get first page
    let response1 = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts?page=1&limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body1 = axum::body::to_bytes(response1.into_body(), usize::MAX)
        .await
        .unwrap();
    let json1: Value = serde_json::from_slice(&body1).unwrap();

    // Get second page
    let response2 = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts?page=2&limit=5")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body2 = axum::body::to_bytes(response2.into_body(), usize::MAX)
        .await
        .unwrap();
    let json2: Value = serde_json::from_slice(&body2).unwrap();

    // Pages should have different data
    let data1 = &json1["data"][0]["id"];
    let data2 = &json2["data"][0]["id"];
    assert_ne!(data1, data2, "Different pages should have different data");
}
