use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt;

mod common;
use common::create_test_app;

// ============================================================================
// Assessment CRUD Tests
// ============================================================================

#[tokio::test]
async fn test_create_assessment() {
    let app = create_test_app().await;

    let body = json!({
        "framework_id": "iso31000",
        "name": "Test Assessment",
        "description": "A test compliance assessment"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json["id"].is_string());
    assert_eq!(json["framework_id"], "iso31000");
    assert_eq!(json["name"], "Test Assessment");
    assert_eq!(json["status"], "draft");
}

#[tokio::test]
async fn test_create_assessment_invalid_framework() {
    let app = create_test_app().await;

    let body = json!({
        "framework_id": "nonexistent",
        "name": "Test Assessment"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_assessments() {
    let app = create_test_app().await;

    // Create an assessment first
    let create_body = json!({
        "framework_id": "iso31000",
        "name": "List Test Assessment"
    });

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // List assessments
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/compliance/assessments?page=1&limit=10")
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
    assert!(json["total"].as_i64().unwrap() >= 1);
    assert!(json["page"].is_number());
    assert!(json["limit"].is_number());
}

#[tokio::test]
async fn test_get_assessment() {
    let app = create_test_app().await;

    // Create an assessment
    let create_body = json!({
        "framework_id": "iso31000",
        "name": "Get Test Assessment"
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let created: Value = serde_json::from_slice(&create_body).unwrap();
    let assessment_id = created["id"].as_str().unwrap();

    // Get the assessment
    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/compliance/assessments/{}", assessment_id))
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

    assert_eq!(json["id"], assessment_id);
    assert_eq!(json["name"], "Get Test Assessment");
}

#[tokio::test]
async fn test_get_assessment_not_found() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/compliance/assessments/nonexistent-id")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_assessment() {
    let app = create_test_app().await;

    // Create an assessment
    let create_body = json!({
        "framework_id": "iso31000",
        "name": "Update Test Assessment"
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let created: Value = serde_json::from_slice(&create_body).unwrap();
    let assessment_id = created["id"].as_str().unwrap();

    // Update the assessment
    let update_body = json!({
        "name": "Updated Assessment Name",
        "status": "in_progress"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(&format!("/api/compliance/assessments/{}", assessment_id))
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&update_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["name"], "Updated Assessment Name");
    assert_eq!(json["status"], "in_progress");
}

#[tokio::test]
async fn test_delete_assessment() {
    let app = create_test_app().await;

    // Create an assessment
    let create_body = json!({
        "framework_id": "iso31000",
        "name": "Delete Test Assessment"
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let created: Value = serde_json::from_slice(&create_body).unwrap();
    let assessment_id = created["id"].as_str().unwrap();

    // Delete the assessment
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/api/compliance/assessments/{}", assessment_id))
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let get_response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/api/compliance/assessments/{}", assessment_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
}

// ============================================================================
// Compliance Items Tests
// ============================================================================

/// Helper to create an assessment and return its ID
async fn create_test_assessment(app: &axum::Router) -> String {
    let body = json!({
        "framework_id": "iso31000",
        "name": "Items Test Assessment"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    json["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_get_compliance_items() {
    let app = create_test_app().await;
    let assessment_id = create_test_assessment(&app).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/items?page=1&limit=10",
                    assessment_id
                ))
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
    // Auto-generated items from iso31000 framework concepts
    assert!(json["total"].as_i64().unwrap() > 0);

    // Check item structure
    let items = json["data"].as_array().unwrap();
    if let Some(first) = items.first() {
        assert!(first["id"].is_string());
        assert_eq!(first["assessment_id"], assessment_id);
        assert!(first["concept_id"].is_string());
        assert!(first["concept_name_en"].is_string());
        assert!(first["concept_type"].is_string());
    }
}

#[tokio::test]
async fn test_update_compliance_item() {
    let app = create_test_app().await;
    let assessment_id = create_test_assessment(&app).await;

    // Get items to find an item ID
    let items_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/items?limit=1",
                    assessment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let items_body = axum::body::to_bytes(items_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items_json: Value = serde_json::from_slice(&items_body).unwrap();
    let item_id = items_json["data"][0]["id"].as_str().unwrap();

    // Update the item
    let update_body = json!({
        "status": "compliant",
        "notes": "Verified during audit"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(&format!(
                    "/api/compliance/assessments/{}/items/{}",
                    assessment_id, item_id
                ))
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&update_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["status"], "compliant");
    assert_eq!(json["notes"], "Verified during audit");
}

#[tokio::test]
async fn test_add_item_note() {
    let app = create_test_app().await;
    let assessment_id = create_test_assessment(&app).await;

    // Get items to find an item ID
    let items_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/items?limit=1",
                    assessment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let items_body = axum::body::to_bytes(items_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items_json: Value = serde_json::from_slice(&items_body).unwrap();
    let item_id = items_json["data"][0]["id"].as_str().unwrap();

    // Add a note
    let note_body = json!({
        "note": "Initial assessment completed"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!(
                    "/api/compliance/assessments/{}/items/{}/notes",
                    assessment_id, item_id
                ))
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&note_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    // Notes should contain the added note text
    let notes = json["notes"].as_str().unwrap();
    assert!(notes.contains("Initial assessment completed"));
}

// ============================================================================
// Evidence Tests
// ============================================================================

#[tokio::test]
async fn test_add_and_get_evidence() {
    let app = create_test_app().await;
    let assessment_id = create_test_assessment(&app).await;

    // Get an item ID
    let items_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/items?limit=1",
                    assessment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let items_body = axum::body::to_bytes(items_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items_json: Value = serde_json::from_slice(&items_body).unwrap();
    let item_id = items_json["data"][0]["id"].as_str().unwrap();

    // Add evidence
    let evidence_body = json!({
        "evidence_type": "link",
        "title": "Compliance Policy Document",
        "description": "Internal policy document",
        "url": "https://example.com/policy.pdf"
    });

    let add_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!(
                    "/api/compliance/assessments/{}/items/{}/evidence",
                    assessment_id, item_id
                ))
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&evidence_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(add_response.status(), StatusCode::CREATED);

    let add_body = axum::body::to_bytes(add_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let evidence: Value = serde_json::from_slice(&add_body).unwrap();

    assert!(evidence["id"].is_string());
    assert_eq!(evidence["title"], "Compliance Policy Document");
    assert_eq!(evidence["evidence_type"], "link");
    assert_eq!(evidence["url"], "https://example.com/policy.pdf");

    let evidence_id = evidence["id"].as_str().unwrap();

    // Get evidence for the item
    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/items/{}/evidence",
                    assessment_id, item_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(get_response.status(), StatusCode::OK);

    let get_body = axum::body::to_bytes(get_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let evidence_list: Value = serde_json::from_slice(&get_body).unwrap();

    assert!(evidence_list.is_array());
    let list = evidence_list.as_array().unwrap();
    assert!(list.iter().any(|e| e["id"] == evidence_id));

    // Delete evidence
    let delete_response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/api/compliance/evidence/{}", evidence_id))
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_add_evidence_requires_url_or_path() {
    let app = create_test_app().await;
    let assessment_id = create_test_assessment(&app).await;

    // Get an item ID
    let items_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/items?limit=1",
                    assessment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let items_body = axum::body::to_bytes(items_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items_json: Value = serde_json::from_slice(&items_body).unwrap();
    let item_id = items_json["data"][0]["id"].as_str().unwrap();

    // Try to add evidence without url or file_path
    let evidence_body = json!({
        "evidence_type": "document",
        "title": "Missing reference"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!(
                    "/api/compliance/assessments/{}/items/{}/evidence",
                    assessment_id, item_id
                ))
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&evidence_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

// ============================================================================
// Scoring Tests
// ============================================================================

#[tokio::test]
async fn test_compliance_score() {
    let app = create_test_app().await;
    let assessment_id = create_test_assessment(&app).await;

    // Get items and update some statuses
    let items_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/items?limit=5",
                    assessment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let items_body = axum::body::to_bytes(items_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let items_json: Value = serde_json::from_slice(&items_body).unwrap();
    let items = items_json["data"].as_array().unwrap();

    // Mark first item as compliant
    if let Some(first) = items.first() {
        let item_id = first["id"].as_str().unwrap();
        let update_body = json!({ "status": "compliant" });

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&format!(
                        "/api/compliance/assessments/{}/items/{}",
                        assessment_id, item_id
                    ))
                    .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                    .body(Body::from(serde_json::to_string(&update_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    // Mark second item as not_applicable
    if items.len() > 1 {
        let item_id = items[1]["id"].as_str().unwrap();
        let update_body = json!({ "status": "not_applicable" });

        let _ = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&format!(
                        "/api/compliance/assessments/{}/items/{}",
                        assessment_id, item_id
                    ))
                    .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                    .body(Body::from(serde_json::to_string(&update_body).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
    }

    // Get compliance score
    let score_response = app
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/score",
                    assessment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(score_response.status(), StatusCode::OK);

    let score_body = axum::body::to_bytes(score_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let score: Value = serde_json::from_slice(&score_body).unwrap();

    assert_eq!(score["assessment_id"], assessment_id);
    assert!(score["total_items"].as_i64().unwrap() > 0);
    assert_eq!(score["compliant"].as_i64().unwrap(), 1);
    assert_eq!(score["not_applicable"].as_i64().unwrap(), 1);
    assert!(score["overall_compliance_percentage"].is_f64());
    assert!(score["sections"].is_array());
}

#[tokio::test]
async fn test_compliance_score_not_found() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/compliance/assessments/nonexistent-id/score")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// ============================================================================
// Audit Trail Tests
// ============================================================================

#[tokio::test]
async fn test_assessment_history() {
    let app = create_test_app().await;

    // Create assessment (generates audit entry)
    let create_body = json!({
        "framework_id": "iso31000",
        "name": "Audit Trail Test"
    });

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/compliance/assessments")
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let created: Value = serde_json::from_slice(&create_body).unwrap();
    let assessment_id = created["id"].as_str().unwrap();

    // Update assessment (generates another audit entry)
    let update_body = json!({ "name": "Updated Audit Trail Test" });

    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(&format!("/api/compliance/assessments/{}", assessment_id))
                .header("Content-Type", "application/json")
                .header("X-Requested-With", "XMLHttpRequest")
                .body(Body::from(serde_json::to_string(&update_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Get history
    let history_response = app
        .oneshot(
            Request::builder()
                .uri(&format!(
                    "/api/compliance/assessments/{}/history",
                    assessment_id
                ))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(history_response.status(), StatusCode::OK);

    let history_body = axum::body::to_bytes(history_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let history: Value = serde_json::from_slice(&history_body).unwrap();

    assert!(history.is_array());
    let entries = history.as_array().unwrap();

    // Should have at least 2 entries: create + update
    assert!(
        entries.len() >= 2,
        "Expected at least 2 audit entries, got {}",
        entries.len()
    );

    // Check entry structure
    let first = &entries[0];
    assert!(first["id"].is_string());
    assert!(first["action"].is_string());
    assert_eq!(first["entity_type"], "assessment");
    assert!(first["created_at"].is_string());
}
