use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

mod common;
use common::{create_test_app, create_test_pool};

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
        assert!(
            has_term,
            "Search result should contain 'risk' in at least one field"
        );
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

// === Guidance enrichment tests (Section 02) ===
// These tests seed their own guidance data using INSERT OR IGNORE to be idempotent.

/// Helper: build a test app from an existing pool
async fn create_app_from_pool(pool: sqlx::SqlitePool) -> axum::Router {
    let config = ontology_backend::Config::from_env();
    let topics =
        ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
    let cookie_key = axum_extra::extract::cookie::Key::generate();
    let state = ontology_backend::AppState {
        db: pool,
        config: config.clone(),
        topics,
        cookie_key,
    };
    ontology_backend::create_router(state)
}

/// Helper: seed guidance data for nist-ai-gv-3-1 (idempotent via INSERT OR IGNORE)
async fn ensure_guidance_data(pool: &sqlx::SqlitePool) {
    sqlx::query(
        "INSERT OR IGNORE INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en, about_nb) \
         VALUES ('test-gv31-guidance', 'nist-ai-gv-3-1', 'nist-ai-rmf-playbook.pdf', 42, 'About GV 1.1', 'Om GV 1.1')",
    )
    .execute(pool)
    .await
    .unwrap();

    for i in 1..=3 {
        sqlx::query(
            "INSERT OR IGNORE INTO concept_actions (id, concept_id, action_text_en, action_text_nb, sort_order) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(format!("test-gv31-action-{i}"))
        .bind("nist-ai-gv-3-1")
        .bind(format!("Action {i}"))
        .bind(format!("Handling {i}"))
        .bind(i as i64)
        .execute(pool)
        .await
        .unwrap();
    }

    sqlx::query(
        "INSERT OR IGNORE INTO concept_transparency_questions (id, concept_id, question_text_en, sort_order) \
         VALUES ('test-gv31-q1', 'nist-ai-gv-3-1', 'How is this managed?', 1)",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT OR IGNORE INTO concept_references (id, concept_id, reference_type, title, authors, year, venue, url, sort_order) \
         VALUES ('test-gv31-ref1', 'nist-ai-gv-3-1', 'academic', 'AI Risk Paper', 'Smith', 2024, 'ICML', NULL, 1)",
    )
    .execute(pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT OR IGNORE INTO concept_references (id, concept_id, reference_type, title, sort_order) \
         VALUES ('test-gv31-ref2', 'nist-ai-gv-3-1', 'transparency_resource', 'Toolkit', 2)",
    )
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn test_concept_without_guidance_omits_guidance_field() {
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

    assert!(
        json.get("guidance").is_none(),
        "Concept without guidance data should not have a guidance field"
    );
    assert_eq!(json["id"], "iso31000-principles");
    assert!(json["related_concepts"].is_array());
}

#[tokio::test]
async fn test_concept_relationships_includes_guidance_when_present() {
    let pool = create_test_pool().await;
    ensure_guidance_data(&pool).await;
    let app = create_app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-3-1/relationships")
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

    let guidance = json
        .get("guidance")
        .expect("guidance field should be present for enriched concept");

    assert_eq!(guidance["source_pdf"], "nist-ai-rmf-playbook.pdf");
    assert_eq!(guidance["source_page"], 42);
    assert!(guidance.get("about_en").is_some());
    assert!(guidance.get("about_nb").is_some());
    assert!(guidance["suggested_actions"].is_array());
    assert!(guidance["transparency_questions"].is_array());
    assert!(guidance["references"].is_array());
}

#[tokio::test]
async fn test_guidance_actions_ordered_by_sort_order() {
    let pool = create_test_pool().await;
    ensure_guidance_data(&pool).await;
    let app = create_app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-3-1/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let actions = json["guidance"]["suggested_actions"]
        .as_array()
        .expect("suggested_actions should be an array");
    assert!(
        !actions.is_empty(),
        "nist-ai-gv-3-1 should have at least one action"
    );

    let orders: Vec<i64> = actions
        .iter()
        .map(|a| a["sort_order"].as_i64().unwrap())
        .collect();
    for window in orders.windows(2) {
        assert!(
            window[0] <= window[1],
            "Actions should be ordered by sort_order"
        );
    }
}

#[tokio::test]
async fn test_guidance_references_have_correct_types() {
    let pool = create_test_pool().await;
    ensure_guidance_data(&pool).await;
    let app = create_app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-3-1/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let references = json["guidance"]["references"]
        .as_array()
        .expect("references should be an array");

    for r in references {
        let ref_type = r["type"].as_str().expect("type field should be a string");
        assert!(
            ref_type == "academic" || ref_type == "transparency_resource",
            "reference type should be academic or transparency_resource, got: {ref_type}"
        );
        assert!(r.get("reference_type").is_none());
    }
}

#[tokio::test]
async fn test_guidance_with_empty_sub_items_returns_empty_arrays() {
    let pool = create_test_pool().await;

    // Insert guidance row with no actions/questions/references
    sqlx::query(
        "INSERT OR IGNORE INTO concept_guidance (id, concept_id, source_pdf, source_page) \
         VALUES ('test-gv21-guidance', 'nist-ai-gv-2-1', 'playbook.pdf', 99)",
    )
    .execute(&pool)
    .await
    .unwrap();

    // Ensure no sub-items exist
    sqlx::query("DELETE FROM concept_actions WHERE concept_id = 'nist-ai-gv-2-1'")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM concept_transparency_questions WHERE concept_id = 'nist-ai-gv-2-1'")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM concept_references WHERE concept_id = 'nist-ai-gv-2-1'")
        .execute(&pool)
        .await
        .unwrap();

    let app = create_app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-2-1/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    let guidance = json
        .get("guidance")
        .expect("guidance should be present even with empty sub-data");
    assert_eq!(guidance["suggested_actions"], serde_json::json!([]));
    assert_eq!(guidance["transparency_questions"], serde_json::json!([]));
    assert_eq!(guidance["references"], serde_json::json!([]));
}

#[tokio::test]
async fn test_existing_relationship_fields_preserved_with_guidance() {
    let pool = create_test_pool().await;
    ensure_guidance_data(&pool).await;
    let app = create_app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-3-1/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "nist-ai-gv-3-1");
    assert_eq!(json["framework_id"], "nist-ai-rmf");
    assert!(json["name_en"].is_string());
    assert!(json["related_concepts"].is_array());
    assert!(json.get("guidance").is_some());
}
