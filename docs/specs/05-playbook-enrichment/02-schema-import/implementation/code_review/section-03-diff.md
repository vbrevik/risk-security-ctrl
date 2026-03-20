diff --git a/backend/src/import.rs b/backend/src/import.rs
index 53d34f2..efb7312 100644
--- a/backend/src/import.rs
+++ b/backend/src/import.rs
@@ -483,6 +483,17 @@ pub async fn import_all_ontologies(
         }
     }
 
+    // Scan for *-guidance.json companion files (after frameworks + relationships)
+    let mut guidance_entries = tokio::fs::read_dir(data_dir).await?;
+    while let Some(dir_entry) = guidance_entries.next_entry().await? {
+        let name = dir_entry.file_name().to_string_lossy().to_string();
+        if name.ends_with("-guidance.json") {
+            if let Err(e) = import_guidance_file(db, &dir_entry.path()).await {
+                warn!("Failed to import guidance file {}: {}", name, e);
+            }
+        }
+    }
+
     info!("Full ontology import completed");
     Ok(())
 }
diff --git a/backend/tests/guidance_tests.rs b/backend/tests/guidance_tests.rs
index 796a038..0b62241 100644
--- a/backend/tests/guidance_tests.rs
+++ b/backend/tests/guidance_tests.rs
@@ -650,3 +650,146 @@ async fn test_mismatched_bilingual_array_lengths() {
     assert_eq!(rows[2].get::<String, _>("action_text_en"), "English 3");
     assert!(rows[2].get::<Option<String>, _>("action_text_nb").is_none());
 }
+
+// ============================================================
+// Wiring Tests (import_all_ontologies scans *-guidance.json)
+// ============================================================
+
+#[tokio::test]
+async fn test_import_all_ontologies_picks_up_guidance_files() {
+    let pool = setup_db().await;
+    seed_concept(&pool).await;
+
+    let dir = tempfile::tempdir().unwrap();
+    // Write a guidance file matching the *-guidance.json pattern
+    std::fs::write(
+        dir.path().join("test-guidance.json"),
+        sample_guidance_json(),
+    )
+    .unwrap();
+
+    ontology_backend::import::import_all_ontologies(&pool, dir.path())
+        .await
+        .unwrap();
+
+    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count.0, 1);
+}
+
+#[tokio::test]
+async fn test_import_all_ontologies_no_guidance_files_ok() {
+    let pool = setup_db().await;
+
+    let dir = tempfile::tempdir().unwrap();
+    // Empty dir — no framework files, no guidance files
+    ontology_backend::import::import_all_ontologies(&pool, dir.path())
+        .await
+        .unwrap();
+
+    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count.0, 0);
+}
+
+#[tokio::test]
+async fn test_non_guidance_json_files_are_ignored() {
+    let pool = setup_db().await;
+    seed_concept(&pool).await;
+
+    let dir = tempfile::tempdir().unwrap();
+    // Write a file that does NOT match *-guidance.json
+    std::fs::write(
+        dir.path().join("some-other-file.json"),
+        sample_guidance_json(),
+    )
+    .unwrap();
+
+    ontology_backend::import::import_all_ontologies(&pool, dir.path())
+        .await
+        .unwrap();
+
+    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(count.0, 0);
+}
+
+// ============================================================
+// FTS5 Tests
+// ============================================================
+
+#[tokio::test]
+async fn test_fts5_match_on_about_en() {
+    let pool = setup_db().await;
+    seed_concept(&pool).await;
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", sample_guidance_json()).unwrap();
+
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
+        .await
+        .unwrap();
+
+    let rows = sqlx::query("SELECT * FROM concept_guidance_fts WHERE concept_guidance_fts MATCH 'concept'")
+        .fetch_all(&pool)
+        .await
+        .unwrap();
+
+    assert!(!rows.is_empty(), "FTS5 should find 'concept' in about_en");
+}
+
+#[tokio::test]
+async fn test_fts5_match_on_concept_name() {
+    let pool = setup_db().await;
+    seed_concept(&pool).await; // seeds name_en = "Test Concept"
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", sample_guidance_json()).unwrap();
+
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
+        .await
+        .unwrap();
+
+    let rows = sqlx::query("SELECT * FROM concept_guidance_fts WHERE concept_guidance_fts MATCH '\"Test Concept\"'")
+        .fetch_all(&pool)
+        .await
+        .unwrap();
+
+    assert!(!rows.is_empty(), "FTS5 should find concept name_en via content view");
+}
+
+#[tokio::test]
+async fn test_fts5_join_back_to_source_tables() {
+    let pool = setup_db().await;
+    seed_concept(&pool).await;
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", sample_guidance_json()).unwrap();
+
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
+        .await
+        .unwrap();
+
+    let rows = sqlx::query(
+        "SELECT cg.concept_id, c.name_en, c.code, cg.about_en \
+         FROM concept_guidance_fts \
+         JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid \
+         JOIN concepts c ON c.id = cg.concept_id \
+         WHERE concept_guidance_fts MATCH 'concept' \
+         ORDER BY rank",
+    )
+    .fetch_all(&pool)
+    .await
+    .unwrap();
+
+    assert!(!rows.is_empty());
+    assert_eq!(rows[0].get::<String, _>("concept_id"), "test-concept-1");
+    assert_eq!(rows[0].get::<String, _>("name_en"), "Test Concept");
+    assert_eq!(rows[0].get::<String, _>("about_en"), "About this concept");
+}
diff --git a/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-03-contract.md b/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-03-contract.md
new file mode 100644
index 0000000..544e28f
--- /dev/null
+++ b/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-03-contract.md
@@ -0,0 +1,22 @@
+# Section 03 Contract: Wiring into import_all_ontologies() and FTS5
+
+## GOAL
+Wire `import_guidance_file()` into `import_all_ontologies()` via dynamic `*-guidance.json` scan. Verify FTS5 search works end-to-end after import.
+
+## CONTEXT
+Section 03 of 02-schema-import. Section 02 built the import function. This section integrates it into the orchestrator and validates FTS5 search.
+
+## CONSTRAINTS
+- Guidance scan must run AFTER frameworks and relationships are loaded (FK dependency)
+- Use `tokio::fs::read_dir` for async consistency
+- Dynamic scan pattern, no hardcoded filenames
+- Error from one guidance file should not abort the entire import
+
+## FORMAT
+- Modify: `backend/src/import.rs` (add scan to `import_all_ontologies()`)
+- Modify: `backend/tests/guidance_tests.rs` (add wiring + FTS5 tests)
+
+## FAILURE CONDITIONS
+- SHALL NOT scan guidance files before framework/relationship imports complete
+- SHALL NOT break existing tests
+- SHALL NOT hardcode guidance filenames
