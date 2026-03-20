diff --git a/backend/src/import.rs b/backend/src/import.rs
index 799d732..119bbfd 100644
--- a/backend/src/import.rs
+++ b/backend/src/import.rs
@@ -2,6 +2,7 @@ use serde::{Deserialize, Serialize};
 use sqlx::SqlitePool;
 use std::path::Path;
 use tracing::{info, warn};
+use uuid::Uuid;
 
 /// Framework definition from JSON
 #[derive(Debug, Deserialize, Serialize)]
@@ -52,6 +53,48 @@ pub struct RelationshipsFile {
     pub relationships: Vec<RelationshipData>,
 }
 
+/// Guidance file structure (*-guidance.json companion files)
+#[derive(Debug, Deserialize)]
+pub struct GuidanceFile {
+    pub framework_id: String,
+    pub source_pdf: String,
+    pub guidance: Vec<GuidanceEntry>,
+}
+
+/// One guidance entry per concept
+#[derive(Debug, Deserialize)]
+pub struct GuidanceEntry {
+    pub concept_id: String,
+    pub source_page: i64,
+    pub about_en: Option<String>,
+    pub about_nb: Option<String>,
+    pub suggested_actions_en: Option<Vec<String>>,
+    pub suggested_actions_nb: Option<Vec<String>>,
+    pub transparency_questions_en: Option<Vec<String>>,
+    pub transparency_questions_nb: Option<Vec<String>>,
+    pub resources: Option<Vec<ResourceEntry>>,
+    pub references: Option<Vec<ReferenceEntry>>,
+}
+
+/// Transparency resource entry
+#[derive(Debug, Deserialize)]
+pub struct ResourceEntry {
+    pub title: String,
+    pub url: Option<String>,
+    #[serde(rename = "type")]
+    pub resource_type: Option<String>,
+}
+
+/// Academic reference entry
+#[derive(Debug, Deserialize)]
+pub struct ReferenceEntry {
+    pub title: String,
+    pub authors: Option<String>,
+    pub year: Option<i64>,
+    pub venue: Option<String>,
+    pub url: Option<String>,
+}
+
 /// Import a framework and its concepts from a JSON file
 pub async fn import_ontology_file(
     db: &SqlitePool,
@@ -184,6 +227,169 @@ pub async fn import_relationships(
     Ok(())
 }
 
+/// Import guidance data from a *-guidance.json companion file
+pub async fn import_guidance_file(
+    db: &SqlitePool,
+    file_path: &Path,
+) -> Result<(), Box<dyn std::error::Error>> {
+    info!("Importing guidance from: {}", file_path.display());
+
+    let content = tokio::fs::read_to_string(file_path).await?;
+    let guidance_file: GuidanceFile = serde_json::from_str(&content)?;
+
+    info!(
+        "Processing {} guidance entries for framework {}",
+        guidance_file.guidance.len(),
+        guidance_file.framework_id
+    );
+
+    for entry in &guidance_file.guidance {
+        // Validate concept exists (STIG V-222606)
+        let exists: Option<(String,)> =
+            sqlx::query_as("SELECT id FROM concepts WHERE id = ?")
+                .bind(&entry.concept_id)
+                .fetch_optional(db)
+                .await?;
+
+        if exists.is_none() {
+            warn!(
+                "Concept {} not found, skipping guidance entry",
+                entry.concept_id
+            );
+            continue;
+        }
+
+        // Transaction per entry for atomicity
+        let mut tx = db.begin().await?;
+
+        // Upsert concept_guidance
+        let guidance_id = Uuid::new_v4().to_string();
+        sqlx::query(
+            "INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en, about_nb) \
+             VALUES (?, ?, ?, ?, ?, ?) \
+             ON CONFLICT(concept_id) DO UPDATE SET \
+             source_pdf = excluded.source_pdf, \
+             source_page = excluded.source_page, \
+             about_en = excluded.about_en, \
+             about_nb = excluded.about_nb, \
+             updated_at = datetime('now')",
+        )
+        .bind(&guidance_id)
+        .bind(&entry.concept_id)
+        .bind(&guidance_file.source_pdf)
+        .bind(entry.source_page)
+        .bind(&entry.about_en)
+        .bind(&entry.about_nb)
+        .execute(&mut *tx)
+        .await?;
+
+        // Delete existing child rows before reinserting
+        sqlx::query("DELETE FROM concept_actions WHERE concept_id = ?")
+            .bind(&entry.concept_id)
+            .execute(&mut *tx)
+            .await?;
+        sqlx::query("DELETE FROM concept_transparency_questions WHERE concept_id = ?")
+            .bind(&entry.concept_id)
+            .execute(&mut *tx)
+            .await?;
+        sqlx::query("DELETE FROM concept_references WHERE concept_id = ?")
+            .bind(&entry.concept_id)
+            .execute(&mut *tx)
+            .await?;
+
+        // Insert suggested actions
+        if let Some(actions_en) = &entry.suggested_actions_en {
+            let actions_nb = entry.suggested_actions_nb.as_deref().unwrap_or(&[]);
+            for (i, action_en) in actions_en.iter().enumerate() {
+                let action_nb = actions_nb.get(i).map(|s| s.as_str());
+                sqlx::query(
+                    "INSERT INTO concept_actions (id, concept_id, action_text_en, action_text_nb, sort_order) \
+                     VALUES (?, ?, ?, ?, ?)",
+                )
+                .bind(Uuid::new_v4().to_string())
+                .bind(&entry.concept_id)
+                .bind(action_en)
+                .bind(action_nb)
+                .bind((i + 1) as i64)
+                .execute(&mut *tx)
+                .await?;
+            }
+        }
+
+        // Insert transparency questions
+        if let Some(questions_en) = &entry.transparency_questions_en {
+            let questions_nb = entry.transparency_questions_nb.as_deref().unwrap_or(&[]);
+            for (i, question_en) in questions_en.iter().enumerate() {
+                let question_nb = questions_nb.get(i).map(|s| s.as_str());
+                sqlx::query(
+                    "INSERT INTO concept_transparency_questions (id, concept_id, question_text_en, question_text_nb, sort_order) \
+                     VALUES (?, ?, ?, ?, ?)",
+                )
+                .bind(Uuid::new_v4().to_string())
+                .bind(&entry.concept_id)
+                .bind(question_en)
+                .bind(question_nb)
+                .bind((i + 1) as i64)
+                .execute(&mut *tx)
+                .await?;
+            }
+        }
+
+        // Insert references (resources first, then academic)
+        let mut sort_order: i64 = 1;
+
+        if let Some(resources) = &entry.resources {
+            for resource in resources {
+                sqlx::query(
+                    "INSERT INTO concept_references (id, concept_id, reference_type, title, url, sort_order) \
+                     VALUES (?, ?, 'transparency_resource', ?, ?, ?)",
+                )
+                .bind(Uuid::new_v4().to_string())
+                .bind(&entry.concept_id)
+                .bind(&resource.title)
+                .bind(&resource.url)
+                .bind(sort_order)
+                .execute(&mut *tx)
+                .await?;
+                sort_order += 1;
+            }
+        }
+
+        if let Some(references) = &entry.references {
+            for reference in references {
+                sqlx::query(
+                    "INSERT INTO concept_references (id, concept_id, reference_type, title, authors, year, venue, url, sort_order) \
+                     VALUES (?, ?, 'academic', ?, ?, ?, ?, ?, ?)",
+                )
+                .bind(Uuid::new_v4().to_string())
+                .bind(&entry.concept_id)
+                .bind(&reference.title)
+                .bind(&reference.authors)
+                .bind(reference.year)
+                .bind(&reference.venue)
+                .bind(&reference.url)
+                .bind(sort_order)
+                .execute(&mut *tx)
+                .await?;
+                sort_order += 1;
+            }
+        }
+
+        tx.commit().await?;
+    }
+
+    // Rebuild FTS5 index
+    sqlx::query("INSERT INTO concept_guidance_fts(concept_guidance_fts) VALUES('rebuild')")
+        .execute(db)
+        .await?;
+
+    info!(
+        "Successfully imported guidance from {}",
+        file_path.display()
+    );
+    Ok(())
+}
+
 /// Import all ontology data from the ontology-data directory
 pub async fn import_all_ontologies(
     db: &SqlitePool,
diff --git a/backend/tests/guidance_tests.rs b/backend/tests/guidance_tests.rs
index 77d983e..a23152a 100644
--- a/backend/tests/guidance_tests.rs
+++ b/backend/tests/guidance_tests.rs
@@ -1,7 +1,10 @@
-//! Tests for the guidance data schema (migration 004).
+//! Tests for the guidance data schema (migration 004) and import function.
 
+use ontology_backend::import::{GuidanceEntry, GuidanceFile, ReferenceEntry, ResourceEntry};
 use sqlx::sqlite::SqlitePoolOptions;
 use sqlx::Row;
+use std::io::Write;
+use tempfile::NamedTempFile;
 
 async fn setup_db() -> sqlx::SqlitePool {
     let pool = SqlitePoolOptions::new()
@@ -252,3 +255,317 @@ async fn cascade_delete_removes_guidance_data() {
         .unwrap();
     assert_eq!(refs.0, 0);
 }
+
+// ============================================================
+// Deserialization Tests
+// ============================================================
+
+#[test]
+fn test_guidance_file_deserializes_from_valid_json() {
+    let json = r#"{
+        "framework_id": "nist-ai-rmf",
+        "source_pdf": "playbook.pdf",
+        "extracted_at": "2026-01-01T00:00:00Z",
+        "guidance": [
+            {
+                "concept_id": "gv-1.1-001",
+                "source_page": 10,
+                "about_en": "About text"
+            }
+        ]
+    }"#;
+    let file: GuidanceFile = serde_json::from_str(json).unwrap();
+    assert_eq!(file.framework_id, "nist-ai-rmf");
+    assert_eq!(file.source_pdf, "playbook.pdf");
+    assert_eq!(file.guidance.len(), 1);
+}
+
+#[test]
+fn test_guidance_entry_with_all_optional_fields_null() {
+    let json = r#"{
+        "concept_id": "gv-1.1-001",
+        "source_page": 5
+    }"#;
+    let entry: GuidanceEntry = serde_json::from_str(json).unwrap();
+    assert_eq!(entry.concept_id, "gv-1.1-001");
+    assert_eq!(entry.source_page, 5);
+    assert!(entry.about_en.is_none());
+    assert!(entry.about_nb.is_none());
+    assert!(entry.suggested_actions_en.is_none());
+    assert!(entry.suggested_actions_nb.is_none());
+    assert!(entry.transparency_questions_en.is_none());
+    assert!(entry.transparency_questions_nb.is_none());
+    assert!(entry.resources.is_none());
+    assert!(entry.references.is_none());
+}
+
+#[test]
+fn test_resource_entry_deserializes_type_field() {
+    let json = r#"{"title": "NIST AI 100-1", "url": "https://example.com", "type": "standard"}"#;
+    let entry: ResourceEntry = serde_json::from_str(json).unwrap();
+    assert_eq!(entry.title, "NIST AI 100-1");
+    assert_eq!(entry.url.as_deref(), Some("https://example.com"));
+    assert_eq!(entry.resource_type.as_deref(), Some("standard"));
+}
+
+#[test]
+fn test_reference_entry_with_partial_fields() {
+    let json = r#"{"title": "Some Paper"}"#;
+    let entry: ReferenceEntry = serde_json::from_str(json).unwrap();
+    assert_eq!(entry.title, "Some Paper");
+    assert!(entry.authors.is_none());
+    assert!(entry.year.is_none());
+    assert!(entry.venue.is_none());
+    assert!(entry.url.is_none());
+}
+
+#[test]
+fn test_unknown_json_fields_are_ignored() {
+    let json = r#"{
+        "framework_id": "test",
+        "source_pdf": "test.pdf",
+        "extracted_at": "2026-01-01",
+        "extra_field": true,
+        "guidance": [{
+            "concept_id": "x",
+            "source_page": 1,
+            "unknown_nested": 42
+        }]
+    }"#;
+    let file: GuidanceFile = serde_json::from_str(json).unwrap();
+    assert_eq!(file.guidance.len(), 1);
+}
+
+// ============================================================
+// Import Function Tests
+// ============================================================
+
+fn sample_guidance_json() -> String {
+    r#"{
+        "framework_id": "nist-ai-rmf",
+        "source_pdf": "playbook.pdf",
+        "guidance": [{
+            "concept_id": "test-concept-1",
+            "source_page": 42,
+            "about_en": "About this concept",
+            "suggested_actions_en": ["Action one", "Action two"],
+            "transparency_questions_en": ["Question one"],
+            "resources": [{"title": "NIST AI 100-1", "url": "https://example.com", "type": "standard"}],
+            "references": [{"title": "Paper A", "authors": "Smith et al.", "year": 2023, "venue": "AAAI"}]
+        }]
+    }"#
+    .to_string()
+}
+
+#[tokio::test]
+async fn test_import_guidance_populates_all_four_tables() {
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
+    let guidance: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(guidance.0, 1);
+
+    let actions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_actions")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(actions.0, 2);
+
+    let questions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_transparency_questions")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(questions.0, 1);
+
+    let refs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_references")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(refs.0, 2); // 1 resource + 1 academic
+}
+
+#[tokio::test]
+async fn test_concept_guidance_row_has_correct_fields() {
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
+    let row = sqlx::query("SELECT source_pdf, source_page, about_en FROM concept_guidance WHERE concept_id = 'test-concept-1'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+
+    assert_eq!(row.get::<String, _>("source_pdf"), "playbook.pdf");
+    assert_eq!(row.get::<i64, _>("source_page"), 42);
+    assert_eq!(row.get::<String, _>("about_en"), "About this concept");
+}
+
+#[tokio::test]
+async fn test_concept_actions_have_correct_sort_order() {
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
+    let rows = sqlx::query("SELECT action_text_en, sort_order FROM concept_actions WHERE concept_id = 'test-concept-1' ORDER BY sort_order")
+        .fetch_all(&pool)
+        .await
+        .unwrap();
+
+    assert_eq!(rows.len(), 2);
+    assert_eq!(rows[0].get::<String, _>("action_text_en"), "Action one");
+    assert_eq!(rows[0].get::<i64, _>("sort_order"), 1);
+    assert_eq!(rows[1].get::<String, _>("action_text_en"), "Action two");
+    assert_eq!(rows[1].get::<i64, _>("sort_order"), 2);
+}
+
+#[tokio::test]
+async fn test_references_split_into_correct_types() {
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
+    let rows = sqlx::query("SELECT reference_type, title, sort_order FROM concept_references WHERE concept_id = 'test-concept-1' ORDER BY sort_order")
+        .fetch_all(&pool)
+        .await
+        .unwrap();
+
+    assert_eq!(rows.len(), 2);
+    assert_eq!(rows[0].get::<String, _>("reference_type"), "transparency_resource");
+    assert_eq!(rows[0].get::<String, _>("title"), "NIST AI 100-1");
+    assert_eq!(rows[0].get::<i64, _>("sort_order"), 1);
+    assert_eq!(rows[1].get::<String, _>("reference_type"), "academic");
+    assert_eq!(rows[1].get::<String, _>("title"), "Paper A");
+    assert_eq!(rows[1].get::<i64, _>("sort_order"), 2);
+}
+
+#[tokio::test]
+async fn test_invalid_concept_id_is_skipped() {
+    let pool = setup_db().await;
+    seed_concept(&pool).await;
+
+    let json = r#"{
+        "framework_id": "test",
+        "source_pdf": "test.pdf",
+        "guidance": [
+            {"concept_id": "test-concept-1", "source_page": 1, "about_en": "Valid"},
+            {"concept_id": "nonexistent-concept", "source_page": 2, "about_en": "Invalid"}
+        ]
+    }"#;
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", json).unwrap();
+
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
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
+async fn test_reimport_produces_no_duplicates() {
+    let pool = setup_db().await;
+    seed_concept(&pool).await;
+
+    let mut tmp = NamedTempFile::new().unwrap();
+    write!(tmp, "{}", sample_guidance_json()).unwrap();
+
+    // Import twice
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
+        .await
+        .unwrap();
+    ontology_backend::import::import_guidance_file(&pool, tmp.path())
+        .await
+        .unwrap();
+
+    let guidance: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(guidance.0, 1);
+
+    let actions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_actions")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(actions.0, 2); // not 4
+}
+
+#[tokio::test]
+async fn test_child_rows_replaced_on_reimport() {
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
+    // Re-import with different actions
+    let json2 = r#"{
+        "framework_id": "nist-ai-rmf",
+        "source_pdf": "playbook.pdf",
+        "guidance": [{
+            "concept_id": "test-concept-1",
+            "source_page": 42,
+            "about_en": "Updated about",
+            "suggested_actions_en": ["New action"]
+        }]
+    }"#;
+
+    let mut tmp2 = NamedTempFile::new().unwrap();
+    write!(tmp2, "{}", json2).unwrap();
+
+    ontology_backend::import::import_guidance_file(&pool, tmp2.path())
+        .await
+        .unwrap();
+
+    let rows = sqlx::query("SELECT action_text_en FROM concept_actions WHERE concept_id = 'test-concept-1'")
+        .fetch_all(&pool)
+        .await
+        .unwrap();
+
+    assert_eq!(rows.len(), 1);
+    assert_eq!(rows[0].get::<String, _>("action_text_en"), "New action");
+
+    // about_en should be updated too
+    let row = sqlx::query("SELECT about_en FROM concept_guidance WHERE concept_id = 'test-concept-1'")
+        .fetch_one(&pool)
+        .await
+        .unwrap();
+    assert_eq!(row.get::<String, _>("about_en"), "Updated about");
+}
diff --git a/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-02-contract.md b/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-02-contract.md
new file mode 100644
index 0000000..386c4e1
--- /dev/null
+++ b/docs/specs/05-playbook-enrichment/02-schema-import/implementation/contracts/section-02-contract.md
@@ -0,0 +1,26 @@
+# Section 02 Contract: Import Types and `import_guidance_file()`
+
+## GOAL
+Add Rust deserialization types (`GuidanceFile`, `GuidanceEntry`, `ResourceEntry`, `ReferenceEntry`) and `import_guidance_file()` async function to `backend/src/import.rs`. The function loads `*-guidance.json` companion files into the four guidance tables (concept_guidance, concept_actions, concept_transparency_questions, concept_references) with upsert semantics and FTS5 rebuild.
+
+## CONTEXT
+Section 02 of 05-playbook-enrichment/02-schema-import. Section 01 created migration 004 with the guidance data tables. This section provides the import logic. Section 03 will wire it into `import_all_ontologies()`.
+
+## CONSTRAINTS
+- All SQL uses parameterized binds (STIG V-222607) — no string interpolation
+- Concept validation before insert (STIG V-222606) — SELECT before INSERT
+- Transaction per entry for atomicity of parent-child rows
+- Upsert for concept_guidance; delete-reinsert for child tables
+- UUID v4 for all row IDs
+- FTS5 rebuild after all entries processed
+- Types must be `pub` for test access
+
+## FORMAT
+- Modify: `backend/src/import.rs` (add types + function)
+- Modify: `backend/tests/guidance_tests.rs` (add deserialization + import tests)
+
+## FAILURE CONDITIONS
+- SHALL NOT use string interpolation in SQL
+- SHALL NOT skip concept existence validation
+- SHALL NOT leave child rows orphaned on reimport (must delete before reinsert)
+- SHALL NOT break existing tests
