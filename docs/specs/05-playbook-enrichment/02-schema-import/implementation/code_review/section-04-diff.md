diff --git a/backend/tests/guidance_tests.rs b/backend/tests/guidance_tests.rs
index ba51d8b..a2ed1a2 100644
--- a/backend/tests/guidance_tests.rs
+++ b/backend/tests/guidance_tests.rs
@@ -834,3 +834,267 @@ async fn test_fts5_join_back_to_source_tables() {
     assert_eq!(rows[0].get::<String, _>("name_en"), "Test Concept");
     assert_eq!(rows[0].get::<String, _>("about_en"), "About this concept");
 }
+
+// ============================================================
+// Integration Tests (real ontology data, real concept IDs)
+// ============================================================
+
+mod common;
+
+use sqlx::sqlite::SqliteConnectOptions;
+use std::str::FromStr;
+
+/// Setup a pool with full ontology data imported (mirrors create_test_app but returns pool)
+async fn setup_integration_pool() -> sqlx::SqlitePool {
+    let config = ontology_backend::Config::from_env();
+    let options = SqliteConnectOptions::from_str(&config.database_url)
+        .expect("Invalid database URL")
+        .create_if_missing(true);
+
+    let pool = sqlx::SqlitePool::connect_with(options)
+        .await
+        .expect("Failed to connect to test database");
+
+    sqlx::migrate!("./migrations")
+        .run(&pool)
+        .await
+        .expect("Failed to run migrations");
+
+    // Import ontology data if needed
+    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concepts")
+        .fetch_one(&pool)
+        .await
+        .expect("Failed to count concepts");
+
+    if count.0 == 0 {
+        let data_dir = std::path::Path::new("../ontology-data");
+        if data_dir.exists() {
+            ontology_backend::import::import_all_ontologies(&pool, data_dir)
+                .await
+                .expect("Failed to import test data");
+        }
+    }
+
+    pool
+}
+
+/// Cleanup guidance data for specific concept IDs after test
+async fn cleanup_guidance(pool: &sqlx::SqlitePool, concept_ids: &[&str]) {
+    for id in concept_ids {
+        sqlx::query("DELETE FROM concept_guidance WHERE concept_id = ?")
+            .bind(id)
+            .execute(pool)
+            .await
+            .ok();
+    }
+}
+
+fn integration_guidance_json() -> String {
+    r#"{
+        "framework_id": "nist-ai-rmf",
+        "source_pdf": "NIST-AI-100-1.pdf",
+        "guidance": [
+            {
+                "concept_id": "nist-ai-gv-1-1",
+                "source_page": 35,
+                "about_en": "INTEGRATIONTESTKEYWORD governance policies for AI risk",
+                "about_nb": "Styringspolicyer for AI-risiko",
+                "suggested_actions_en": ["Establish governance board", "Define risk appetite"],
+                "suggested_actions_nb": ["Etabler styringsråd", "Definer risikoappetitt"],
+                "transparency_questions_en": ["What governance structure exists?"],
+                "resources": [{"title": "NIST AI 100-1", "url": "https://example.com", "type": "standard"}],
+                "references": [{"title": "AI Governance Paper", "authors": "Smith et al.", "year": 2023, "venue": "NeurIPS"}]
+            },
+            {
+                "concept_id": "nist-ai-gv-1-2",
+                "source_page": 38,
+                "about_en": "INTEGRATIONTESTKEYWORD regulatory compliance for AI systems",
+                "suggested_actions_en": ["Map applicable regulations", "Assign compliance ownership"],
+                "transparency_questions_en": ["Which regulations apply?", "Who owns compliance?"]
+            }
+        ]
+    }"#
+    .to_string()
+}
+
+#[tokio::test]
+async fn integration_import_with_real_concept_ids() {
+    let pool = setup_integration_pool().await;
+    let test_ids = ["nist-ai-gv-1-1", "nist-ai-gv-1-2"];
+    cleanup_guidance(&pool, &test_ids).await;
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", integration_guidance_json()).unwrap();
+
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
+        .await
+        .unwrap();
+
+    // Verify both concepts got guidance rows
+    let count: (i64,) = sqlx::query_as(
+        "SELECT COUNT(*) FROM concept_guidance WHERE concept_id IN ('nist-ai-gv-1-1', 'nist-ai-gv-1-2')",
+    )
+    .fetch_one(&pool)
+    .await
+    .unwrap();
+    assert_eq!(count.0, 2);
+
+    // Verify actions for gv-1-1
+    let actions = sqlx::query(
+        "SELECT action_text_en, action_text_nb, sort_order FROM concept_actions \
+         WHERE concept_id = 'nist-ai-gv-1-1' ORDER BY sort_order",
+    )
+    .fetch_all(&pool)
+    .await
+    .unwrap();
+    assert_eq!(actions.len(), 2);
+    assert_eq!(actions[0].get::<String, _>("action_text_en"), "Establish governance board");
+    assert_eq!(actions[0].get::<String, _>("action_text_nb"), "Etabler styringsråd");
+    assert_eq!(actions[0].get::<i64, _>("sort_order"), 1);
+
+    // Verify transparency questions for gv-1-2
+    let questions = sqlx::query(
+        "SELECT question_text_en, sort_order FROM concept_transparency_questions \
+         WHERE concept_id = 'nist-ai-gv-1-2' ORDER BY sort_order",
+    )
+    .fetch_all(&pool)
+    .await
+    .unwrap();
+    assert_eq!(questions.len(), 2);
+    assert_eq!(questions[1].get::<String, _>("question_text_en"), "Who owns compliance?");
+
+    // Verify references for gv-1-1 (1 resource + 1 academic)
+    let refs = sqlx::query(
+        "SELECT reference_type, title, sort_order FROM concept_references \
+         WHERE concept_id = 'nist-ai-gv-1-1' ORDER BY sort_order",
+    )
+    .fetch_all(&pool)
+    .await
+    .unwrap();
+    assert_eq!(refs.len(), 2);
+    assert_eq!(refs[0].get::<String, _>("reference_type"), "transparency_resource");
+    assert_eq!(refs[1].get::<String, _>("reference_type"), "academic");
+
+    cleanup_guidance(&pool, &test_ids).await;
+}
+
+#[tokio::test]
+async fn integration_reimport_idempotent_with_real_ids() {
+    let pool = setup_integration_pool().await;
+    let test_ids = ["nist-ai-gv-1-1", "nist-ai-gv-1-2"];
+    cleanup_guidance(&pool, &test_ids).await;
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", integration_guidance_json()).unwrap();
+
+    // Import twice
+    ontology_backend::import::import_guidance_file(&pool, tmp.path()).await.unwrap();
+    ontology_backend::import::import_guidance_file(&pool, tmp.path()).await.unwrap();
+
+    let count: (i64,) = sqlx::query_as(
+        "SELECT COUNT(*) FROM concept_guidance WHERE concept_id IN ('nist-ai-gv-1-1', 'nist-ai-gv-1-2')",
+    )
+    .fetch_one(&pool)
+    .await
+    .unwrap();
+    assert_eq!(count.0, 2, "Upsert should not duplicate guidance rows");
+
+    let action_count: (i64,) = sqlx::query_as(
+        "SELECT COUNT(*) FROM concept_actions WHERE concept_id = 'nist-ai-gv-1-1'",
+    )
+    .fetch_one(&pool)
+    .await
+    .unwrap();
+    assert_eq!(action_count.0, 2, "Delete-reinsert should not duplicate actions");
+
+    cleanup_guidance(&pool, &test_ids).await;
+}
+
+#[tokio::test]
+async fn integration_fts5_search_with_real_data() {
+    let pool = setup_integration_pool().await;
+    let test_ids = ["nist-ai-gv-1-1", "nist-ai-gv-1-2"];
+    cleanup_guidance(&pool, &test_ids).await;
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", integration_guidance_json()).unwrap();
+
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
+        .await
+        .unwrap();
+
+    // FTS5 search using our distinctive test keyword
+    let rows = sqlx::query(
+        "SELECT cg.concept_id, c.name_en, c.code, cg.about_en \
+         FROM concept_guidance_fts \
+         JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid \
+         JOIN concepts c ON c.id = cg.concept_id \
+         WHERE concept_guidance_fts MATCH 'INTEGRATIONTESTKEYWORD' \
+         ORDER BY rank",
+    )
+    .fetch_all(&pool)
+    .await
+    .unwrap();
+
+    assert_eq!(rows.len(), 2, "Both guidance entries contain the test keyword");
+
+    // Verify joined data is correct
+    let concept_ids: Vec<String> = rows.iter().map(|r| r.get("concept_id")).collect();
+    assert!(concept_ids.contains(&"nist-ai-gv-1-1".to_string()));
+    assert!(concept_ids.contains(&"nist-ai-gv-1-2".to_string()));
+
+    // Verify concept name_en comes through the join
+    for row in &rows {
+        let name: String = row.get("name_en");
+        assert!(!name.is_empty(), "Concept name should be populated via join");
+    }
+
+    cleanup_guidance(&pool, &test_ids).await;
+}
+
+#[tokio::test]
+async fn integration_api_health_still_works_after_migration() {
+    use axum::{body::Body, http::{Request, StatusCode}};
+    use tower::ServiceExt;
+
+    let app = common::create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/health")
+                .body(Body::empty())
+                .unwrap(),
+        )
+        .await
+        .unwrap();
+
+    assert_eq!(response.status(), StatusCode::OK);
+}
+
+#[tokio::test]
+async fn integration_api_frameworks_still_works_after_migration() {
+    use axum::{body::Body, http::{Request, StatusCode}};
+    use tower::ServiceExt;
+
+    let app = common::create_test_app().await;
+
+    let response = app
+        .oneshot(
+            Request::builder()
+                .uri("/api/ontology/frameworks")
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
+    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
+    assert!(json.is_array(), "Frameworks endpoint should return an array");
+    assert!(!json.as_array().unwrap().is_empty(), "Should have frameworks");
+}
diff --git a/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-04-contract.md b/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-04-contract.md
new file mode 100644
index 0000000..a2e26f0
--- /dev/null
+++ b/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-04-contract.md
@@ -0,0 +1,21 @@
+# Section 04 Contract: Integration Tests
+
+## GOAL
+Add integration tests that exercise the full guidance pipeline with real NIST AI RMF concept IDs and verify no regressions on existing API endpoints.
+
+## CONTEXT
+Sections 01-03 already have 29 unit tests with in-memory SQLite. This section adds integration-level tests using the real application setup (`create_test_app()` / `setup_pool()`) with the full ontology imported, plus API regression checks.
+
+## CONSTRAINTS
+- Use real concept IDs from NIST AI RMF ontology (nist-ai-gv-1-1, nist-ai-gv-1-2)
+- Reuse `common::create_test_app()` pattern for API tests
+- Tests must tolerate pre-existing data (no total row counts, use specific-value assertions)
+- Clean up test guidance data after import tests
+
+## FORMAT
+- Modify: `backend/tests/guidance_tests.rs` (add integration tests)
+
+## FAILURE CONDITIONS
+- SHALL NOT duplicate tests already covered in sections 01-03
+- SHALL NOT break existing tests
+- SHALL NOT leave test data that pollutes other test runs
