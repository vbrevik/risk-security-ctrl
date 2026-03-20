//! Integration tests for the guidance enrichment feature (Sections 1-5).
//! Tests the complete pipeline: API guidance responses, matcher scoring,
//! actionable recommendations, and OpenAPI schema registration.

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use tower::ServiceExt;

mod common;
use common::{create_test_app, create_test_pool};

use ontology_backend::features::analysis::engine::MatchingEngine;
use ontology_backend::features::analysis::matcher::DeterministicMatcher;

/// Verify guidance data exists in the DB (imported from nist-ai-rmf-guidance.json by create_test_pool).
/// No manual seeding needed — the real guidance file is auto-discovered by import_all_ontologies.
async fn verify_guidance_data_exists(pool: &sqlx::SqlitePool) {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(pool)
        .await
        .unwrap();
    assert!(
        count.0 > 0,
        "Guidance data should be imported from nist-ai-rmf-guidance.json"
    );
}

/// Helper: build app from pool
async fn app_from_pool(pool: sqlx::SqlitePool) -> axum::Router {
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

// === API Integration Tests ===

#[tokio::test]
async fn integration_concept_detail_returns_guidance_object() {
    let pool = create_test_pool().await;
    verify_guidance_data_exists(&pool).await;
    let app = app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-4-1/relationships")
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
        .expect("guidance field should be present");
    assert!(guidance["source_pdf"].is_string());
    assert!(guidance["suggested_actions"].is_array());
    assert!(guidance["transparency_questions"].is_array());
    assert!(guidance["references"].is_array());
}

#[tokio::test]
async fn integration_guidance_response_schema_shape() {
    let pool = create_test_pool().await;
    verify_guidance_data_exists(&pool).await;
    let app = app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-4-1/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    let guidance = &json["guidance"];

    // Verify action shape
    let actions = guidance["suggested_actions"].as_array().unwrap();
    assert!(!actions.is_empty());
    for action in actions {
        assert!(action["sort_order"].is_number());
        assert!(action["text_en"].is_string());
    }

    // Verify question shape
    let questions = guidance["transparency_questions"].as_array().unwrap();
    assert!(!questions.is_empty());
    for q in questions {
        assert!(q["sort_order"].is_number());
        assert!(q["text_en"].is_string());
    }

    // Verify reference shape (uses "type" not "reference_type")
    let refs = guidance["references"].as_array().unwrap();
    assert!(!refs.is_empty());
    for r in refs {
        assert!(r["type"].is_string());
        assert!(r.get("reference_type").is_none());
        assert!(r["title"].is_string());
    }
}

#[tokio::test]
async fn integration_non_guidance_concept_omits_field() {
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

    assert!(json.get("guidance").is_none());
    assert!(json["related_concepts"].is_array());
}

// === Matcher Integration Tests ===

#[tokio::test]
async fn integration_analysis_with_guidance_enriched_scoring() {
    let pool = create_test_pool().await;
    verify_guidance_data_exists(&pool).await;

    let topics =
        ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
    let matcher = DeterministicMatcher::new(topics);

    let text = "organizational risk tolerance thresholds for AI systems governance \
                and risk appetite statements for machine learning";

    let result = matcher.analyze(text, None, &pool).await;
    assert!(result.is_ok(), "Analysis should succeed: {:?}", result.err());

    let result = result.unwrap();
    assert!(
        !result.findings.is_empty(),
        "Should produce findings for AI governance text"
    );
}

#[tokio::test]
async fn integration_candidate_retrieval_populates_actions_text() {
    // Verify the full retrieve_candidates pipeline populates actions_text
    // from the real DB for FTS-matched concepts with guidance data.
    let pool = create_test_pool().await;
    verify_guidance_data_exists(&pool).await;

    use ontology_backend::features::analysis::matcher::retrieve_candidates;
    use ontology_backend::features::analysis::tokenizer::extract_keywords;

    // "governance" matches many NIST AI RMF concepts via FTS, including ones we seeded guidance for
    let doc_keywords = extract_keywords("governance accountability AI risk tolerance oversight");
    let framework_ids = vec!["nist-ai-rmf".to_string()];

    let candidates = retrieve_candidates(&doc_keywords, &framework_ids, &pool)
        .await
        .unwrap();

    // At least one FTS-matched candidate should have actions_text from guidance
    let enriched: Vec<_> = candidates
        .iter()
        .filter(|c| c.actions_text.is_some())
        .collect();
    assert!(
        !enriched.is_empty(),
        "At least one candidate should have actions_text from guidance data"
    );

    // Verify enriched candidate has meaningful content
    let first = enriched[0];
    let actions = first.actions_text.as_ref().unwrap();
    assert!(!actions.is_empty(), "actions_text should be non-empty");
    assert!(first.about_en.is_some(), "about_en should also be populated");
}

#[tokio::test]
async fn integration_non_guidance_framework_no_actions() {
    let pool = create_test_pool().await;

    let topics =
        ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
    let matcher = DeterministicMatcher::new(topics);

    let text = "risk assessment process treatment monitoring review evaluation \
                iso 31000 principles framework";

    let result = matcher.analyze(text, None, &pool).await.unwrap();

    for finding in &result.findings {
        if let Some(ref rec) = finding.recommendation {
            assert!(
                !rec.contains("Suggested Actions:"),
                "ISO 31000 findings should not have suggested actions: {}",
                finding.concept_id
            );
        }
    }
}

// === No-Regression Tests ===

#[tokio::test]
async fn integration_existing_relationship_fields_preserved() {
    let pool = create_test_pool().await;
    verify_guidance_data_exists(&pool).await;
    let app = app_from_pool(pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/ontology/concepts/nist-ai-gv-4-1/relationships")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["id"], "nist-ai-gv-4-1");
    assert!(json["name_en"].is_string());
    assert!(json["related_concepts"].is_array());
    assert!(json.get("guidance").is_some());
}

#[tokio::test]
async fn integration_unrelated_text_returns_no_frameworks_error() {
    let pool = create_test_pool().await;
    let topics =
        ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
    let matcher = DeterministicMatcher::new(topics);

    let text = "the weather today is sunny with clear skies and mild temperatures";
    let result = matcher.analyze(text, None, &pool).await;
    // Unrelated text should return NoFrameworksDetected error (not panic)
    assert!(result.is_err(), "Analysis of irrelevant text should return error");
}
