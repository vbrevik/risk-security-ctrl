diff --git a/backend/src/features/ontology/routes.rs b/backend/src/features/ontology/routes.rs
index 76c815c..97cc5d8 100644
--- a/backend/src/features/ontology/routes.rs
+++ b/backend/src/features/ontology/routes.rs
@@ -10,7 +10,8 @@ use utoipa::ToSchema;
 use crate::AppState;
 
 use super::models::{
-    Concept, ConceptListQuery, ConceptWithRelationships, Framework, PaginatedResponse,
+    ActionResponse, Concept, ConceptGuidanceResponse, ConceptListQuery,
+    ConceptWithRelationships, Framework, PaginatedResponse, QuestionResponse, ReferenceResponse,
     RelatedConcept, Relationship, SearchQuery, Topic, TopicTagsFile,
 };
 
@@ -291,10 +292,84 @@ pub async fn get_concept_relationships(
     let mut related_concepts = outgoing;
     related_concepts.extend(incoming);
 
+    // Query guidance data concurrently (4 queries in parallel)
+    let guidance_query = sqlx::query(
+        "SELECT source_pdf, source_page, about_en, about_nb FROM concept_guidance WHERE concept_id = ?",
+    )
+    .bind(&id)
+    .fetch_optional(&state.db);
+
+    let actions_query = sqlx::query(
+        "SELECT action_text_en, action_text_nb, sort_order FROM concept_actions WHERE concept_id = ? ORDER BY sort_order",
+    )
+    .bind(&id)
+    .fetch_all(&state.db);
+
+    let questions_query = sqlx::query(
+        "SELECT question_text_en, question_text_nb, sort_order FROM concept_transparency_questions WHERE concept_id = ? ORDER BY sort_order",
+    )
+    .bind(&id)
+    .fetch_all(&state.db);
+
+    let references_query = sqlx::query(
+        "SELECT reference_type, title, authors, year, venue, url, sort_order FROM concept_references WHERE concept_id = ? ORDER BY sort_order",
+    )
+    .bind(&id)
+    .fetch_all(&state.db);
+
+    let (guidance_row, actions_rows, questions_rows, references_rows) = tokio::try_join!(
+        guidance_query,
+        actions_query,
+        questions_query,
+        references_query,
+    )
+    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
+
+    // Assemble guidance if present
+    let guidance = guidance_row.map(|row| {
+        use sqlx::Row;
+        let suggested_actions = actions_rows
+            .iter()
+            .map(|r| ActionResponse {
+                sort_order: r.get("sort_order"),
+                text_en: r.get("action_text_en"),
+                text_nb: r.get("action_text_nb"),
+            })
+            .collect();
+        let transparency_questions = questions_rows
+            .iter()
+            .map(|r| QuestionResponse {
+                sort_order: r.get("sort_order"),
+                text_en: r.get("question_text_en"),
+                text_nb: r.get("question_text_nb"),
+            })
+            .collect();
+        let references = references_rows
+            .iter()
+            .map(|r| ReferenceResponse {
+                reference_type: r.get("reference_type"),
+                title: r.get("title"),
+                authors: r.get("authors"),
+                year: r.get("year"),
+                venue: r.get("venue"),
+                url: r.get("url"),
+            })
+            .collect();
+        ConceptGuidanceResponse {
+            source_pdf: row.get("source_pdf"),
+            source_page: row.get("source_page"),
+            about_en: row.get("about_en"),
+            about_nb: row.get("about_nb"),
+            suggested_actions,
+            transparency_questions,
+            references,
+        }
+    });
+
     Ok(Json(ConceptWithRelationships {
         concept,
         related_concepts,
-        guidance: None,
+        guidance,
     }))
 }
 
diff --git a/backend/src/main.rs b/backend/src/main.rs
index 8f30a16..8fb6145 100644
--- a/backend/src/main.rs
+++ b/backend/src/main.rs
@@ -41,6 +41,10 @@ use utoipa_swagger_ui::SwaggerUi;
             ontology_backend::features::auth::models::LoginRequest,
             ontology_backend::features::auth::models::AuthResponse,
             ontology_backend::features::auth::models::UserProfile,
+            ontology_backend::features::ontology::models::ConceptGuidanceResponse,
+            ontology_backend::features::ontology::models::ActionResponse,
+            ontology_backend::features::ontology::models::QuestionResponse,
+            ontology_backend::features::ontology::models::ReferenceResponse,
         )
     ),
     tags(
diff --git a/backend/tests/api_tests.rs b/backend/tests/api_tests.rs
index fbe7ffa..087a768 100644
--- a/backend/tests/api_tests.rs
+++ b/backend/tests/api_tests.rs
@@ -6,7 +6,7 @@ use serde_json::Value;
 use tower::ServiceExt;
 
 mod common;
-use common::create_test_app;
+use common::{create_test_app, create_test_pool};
 
 #[tokio::test]
 async fn test_health_check() {
@@ -362,3 +362,237 @@ async fn test_pagination() {
     let data2 = &json2["data"][0]["id"];
     assert_ne!(data1, data2, "Different pages should have different data");
 }
+
+// === Guidance enrichment tests (Section 02) ===
+// These tests rely on guidance data already imported into the persistent test DB
+// (nist-ai-gv-1-1 has 2 actions, 1 question, 2 references)
+
+#[tokio::test]
+async fn test_concept_without_guidance_omits_guidance_field() {
+    let app = create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/ontology/concepts/iso31000-principles/relationships")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    assert_eq!(response.status(), StatusCode::OK);
+
+    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
+        .await
+        .unwrap();
+    let json: Value = serde_json::from_slice(&body).unwrap();
+
+    // guidance key should be absent (skip_serializing_if)
+    assert!(
+        json.get("guidance").is_none(),
+        "Concept without guidance data should not have a guidance field"
+    );
+    // Existing fields still present
+    assert_eq!(json["id"], "iso31000-principles");
+    assert!(json["related_concepts"].is_array());
+}
+
+#[tokio::test]
+async fn test_concept_relationships_includes_guidance_when_present() {
+    let app = create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/ontology/concepts/nist-ai-gv-1-1/relationships")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    assert_eq!(response.status(), StatusCode::OK);
+
+    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
+        .await
+        .unwrap();
+    let json: Value = serde_json::from_slice(&body).unwrap();
+
+    let guidance = json
+        .get("guidance")
+        .expect("guidance field should be present for enriched concept");
+
+    assert_eq!(guidance["source_pdf"], "nist-ai-rmf-playbook.pdf");
+    assert_eq!(guidance["source_page"], 42);
+    assert!(guidance["suggested_actions"].is_array());
+    assert!(guidance["transparency_questions"].is_array());
+    assert!(guidance["references"].is_array());
+}
+
+#[tokio::test]
+async fn test_guidance_actions_ordered_by_sort_order() {
+    let app = create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/ontology/concepts/nist-ai-gv-1-1/relationships")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
+        .await
+        .unwrap();
+    let json: Value = serde_json::from_slice(&body).unwrap();
+
+    let actions = json["guidance"]["suggested_actions"]
+        .as_array()
+        .expect("suggested_actions should be an array");
+    assert!(
+        !actions.is_empty(),
+        "nist-ai-gv-1-1 should have at least one action"
+    );
+
+    // Verify sort_order is monotonically increasing
+    let orders: Vec<i64> = actions
+        .iter()
+        .map(|a| a["sort_order"].as_i64().unwrap())
+        .collect();
+    for window in orders.windows(2) {
+        assert!(
+            window[0] <= window[1],
+            "Actions should be ordered by sort_order"
+        );
+    }
+}
+
+#[tokio::test]
+async fn test_guidance_references_have_correct_types() {
+    let app = create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/ontology/concepts/nist-ai-gv-1-1/relationships")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
+        .await
+        .unwrap();
+    let json: Value = serde_json::from_slice(&body).unwrap();
+
+    let references = json["guidance"]["references"]
+        .as_array()
+        .expect("references should be an array");
+
+    // Verify "type" key (not "reference_type") with valid values
+    for r in references {
+        let ref_type = r["type"].as_str().expect("type field should be a string");
+        assert!(
+            ref_type == "academic" || ref_type == "transparency_resource",
+            "reference type should be academic or transparency_resource, got: {}",
+            ref_type
+        );
+        // Ensure "reference_type" key does NOT appear
+        assert!(r.get("reference_type").is_none());
+    }
+}
+
+#[tokio::test]
+async fn test_guidance_with_empty_sub_items_returns_empty_arrays() {
+    // Use create_test_pool to insert a guidance row for a concept that has no actions/refs
+    let pool = create_test_pool().await;
+
+    // Use a concept that doesn't already have guidance (nist-ai-gv-2-1)
+    let has_guidance: Option<(i64,)> = sqlx::query_as(
+        "SELECT COUNT(*) FROM concept_guidance WHERE concept_id = 'nist-ai-gv-2-1'",
+    )
+    .fetch_optional(&pool)
+    .await
+    .unwrap();
+
+    if has_guidance.map_or(true, |r| r.0 == 0) {
+        sqlx::query(
+            "INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page) VALUES (?, ?, ?, ?)",
+        )
+        .bind("test-guidance-empty-sub")
+        .bind("nist-ai-gv-2-1")
+        .bind("playbook.pdf")
+        .bind(99)
+        .execute(&pool)
+        .await
+        .unwrap();
+    }
+
+    let config = ontology_backend::Config::from_env();
+    let topics = ontology_backend::load_topics(std::path::Path::new("../ontology-data/topic-tags.json"));
+    let cookie_key = axum_extra::extract::cookie::Key::generate();
+    let state = ontology_backend::AppState {
+        db: pool,
+        config: config.clone(),
+        topics,
+        cookie_key,
+    };
+    let app = ontology_backend::create_router(state);
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/ontology/concepts/nist-ai-gv-2-1/relationships")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
+        .await
+        .unwrap();
+    let json: Value = serde_json::from_slice(&body).unwrap();
+
+    let guidance = json
+        .get("guidance")
+        .expect("guidance should be present even with empty sub-data");
+    assert_eq!(guidance["suggested_actions"], serde_json::json!([]));
+    assert_eq!(guidance["transparency_questions"], serde_json::json!([]));
+    assert_eq!(guidance["references"], serde_json::json!([]));
+}
+
+#[tokio::test]
+async fn test_existing_relationship_fields_preserved_with_guidance() {
+    let app = create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/ontology/concepts/nist-ai-gv-1-1/relationships")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
+        .await
+        .unwrap();
+    let json: Value = serde_json::from_slice(&body).unwrap();
+
+    // Concept fields still present via serde(flatten)
+    assert_eq!(json["id"], "nist-ai-gv-1-1");
+    assert_eq!(json["framework_id"], "nist-ai-rmf");
+    assert!(json["name_en"].is_string());
+
+    // related_concepts still present
+    assert!(json["related_concepts"].is_array());
+
+    // guidance also present
+    assert!(json.get("guidance").is_some());
+}
