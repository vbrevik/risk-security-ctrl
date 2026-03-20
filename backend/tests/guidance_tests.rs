//! Tests for the guidance data schema (migration 004) and import function.

use ontology_backend::import::{GuidanceEntry, GuidanceFile, ReferenceEntry, ResourceEntry};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;
use std::io::Write;
use tempfile::NamedTempFile;

async fn setup_db() -> sqlx::SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to create in-memory pool");

    // Enable foreign keys
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .unwrap();

    // Run all migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool
}

async fn seed_concept(pool: &sqlx::SqlitePool) -> String {
    let framework_id = "test-framework";
    let concept_id = "test-concept-1";

    sqlx::query("INSERT INTO frameworks (id, name, version) VALUES (?, ?, ?)")
        .bind(framework_id)
        .bind("Test Framework")
        .bind("1.0")
        .execute(pool)
        .await
        .unwrap();

    sqlx::query(
        "INSERT INTO concepts (id, framework_id, concept_type, code, name_en, definition_en) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(concept_id)
    .bind(framework_id)
    .bind("action")
    .bind("TEST 1.1")
    .bind("Test Concept")
    .bind("A test concept definition")
    .execute(pool)
    .await
    .unwrap();

    concept_id.to_string()
}

#[tokio::test]
async fn migration_creates_concept_guidance_table() {
    let pool = setup_db().await;
    let row = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='concept_guidance'")
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert!(row.is_some());
}

#[tokio::test]
async fn migration_creates_concept_actions_with_unique_constraint() {
    let pool = setup_db().await;
    let concept_id = seed_concept(&pool).await;

    // First insert succeeds
    sqlx::query("INSERT INTO concept_actions (id, concept_id, action_text_en, sort_order) VALUES (?, ?, ?, ?)")
        .bind("a1")
        .bind(&concept_id)
        .bind("Action 1")
        .bind(1)
        .execute(&pool)
        .await
        .unwrap();

    // Duplicate (concept_id, sort_order) fails
    let result = sqlx::query("INSERT INTO concept_actions (id, concept_id, action_text_en, sort_order) VALUES (?, ?, ?, ?)")
        .bind("a2")
        .bind(&concept_id)
        .bind("Action 2")
        .bind(1) // same sort_order
        .execute(&pool)
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn migration_creates_concept_transparency_questions_table() {
    let pool = setup_db().await;
    let row = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='concept_transparency_questions'")
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert!(row.is_some());
}

#[tokio::test]
async fn migration_creates_concept_references_with_check_constraint() {
    let pool = setup_db().await;
    let concept_id = seed_concept(&pool).await;

    // Invalid type fails
    let result = sqlx::query("INSERT INTO concept_references (id, concept_id, reference_type, title, sort_order) VALUES (?, ?, ?, ?, ?)")
        .bind("r1")
        .bind(&concept_id)
        .bind("invalid_type")
        .bind("Some Title")
        .bind(1)
        .execute(&pool)
        .await;
    assert!(result.is_err());

    // Valid types succeed
    sqlx::query("INSERT INTO concept_references (id, concept_id, reference_type, title, sort_order) VALUES (?, ?, ?, ?, ?)")
        .bind("r2")
        .bind(&concept_id)
        .bind("academic")
        .bind("Academic Paper")
        .bind(1)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO concept_references (id, concept_id, reference_type, title, sort_order) VALUES (?, ?, ?, ?, ?)")
        .bind("r3")
        .bind(&concept_id)
        .bind("transparency_resource")
        .bind("Transparency Resource")
        .bind(2)
        .execute(&pool)
        .await
        .unwrap();
}

#[tokio::test]
async fn migration_creates_search_view() {
    let pool = setup_db().await;
    let row = sqlx::query("SELECT name FROM sqlite_master WHERE type='view' AND name='concept_guidance_search_v'")
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert!(row.is_some());
}

#[tokio::test]
async fn migration_creates_fts5_table() {
    let pool = setup_db().await;
    let row = sqlx::query("SELECT name FROM sqlite_master WHERE type='table' AND name='concept_guidance_fts'")
        .fetch_optional(&pool)
        .await
        .unwrap();
    assert!(row.is_some());
}

#[tokio::test]
async fn indexes_exist() {
    let pool = setup_db().await;
    let rows = sqlx::query("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_concept_%'")
        .fetch_all(&pool)
        .await
        .unwrap();

    let names: Vec<String> = rows.iter().map(|r| r.get("name")).collect();
    assert!(names.contains(&"idx_concept_actions_concept".to_string()));
    assert!(names.contains(&"idx_concept_questions_concept".to_string()));
    assert!(names.contains(&"idx_concept_references_concept".to_string()));
    assert!(names.contains(&"idx_concept_references_type".to_string()));
}

#[tokio::test]
async fn cascade_delete_removes_guidance_data() {
    let pool = setup_db().await;
    let concept_id = seed_concept(&pool).await;

    // Insert guidance data
    sqlx::query("INSERT INTO concept_guidance (id, concept_id, source_pdf, source_page, about_en) VALUES (?, ?, ?, ?, ?)")
        .bind("g1")
        .bind(&concept_id)
        .bind("test.pdf")
        .bind(5)
        .bind("About text")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO concept_actions (id, concept_id, action_text_en, sort_order) VALUES (?, ?, ?, ?)")
        .bind("a1")
        .bind(&concept_id)
        .bind("Action 1")
        .bind(1)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO concept_transparency_questions (id, concept_id, question_text_en, sort_order) VALUES (?, ?, ?, ?)")
        .bind("q1")
        .bind(&concept_id)
        .bind("Question 1")
        .bind(1)
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO concept_references (id, concept_id, reference_type, title, sort_order) VALUES (?, ?, ?, ?, ?)")
        .bind("r1")
        .bind(&concept_id)
        .bind("academic")
        .bind("Ref 1")
        .bind(1)
        .execute(&pool)
        .await
        .unwrap();

    // Delete the concept
    sqlx::query("DELETE FROM concepts WHERE id = ?")
        .bind(&concept_id)
        .execute(&pool)
        .await
        .unwrap();

    // Verify cascade
    let guidance: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance WHERE concept_id = ?")
        .bind(&concept_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(guidance.0, 0);

    let actions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_actions WHERE concept_id = ?")
        .bind(&concept_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(actions.0, 0);

    let questions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_transparency_questions WHERE concept_id = ?")
        .bind(&concept_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(questions.0, 0);

    let refs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_references WHERE concept_id = ?")
        .bind(&concept_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(refs.0, 0);
}

// ============================================================
// Deserialization Tests
// ============================================================

#[test]
fn test_guidance_file_deserializes_from_valid_json() {
    let json = r#"{
        "framework_id": "nist-ai-rmf",
        "source_pdf": "playbook.pdf",
        "extracted_at": "2026-01-01T00:00:00Z",
        "guidance": [
            {
                "concept_id": "gv-1.1-001",
                "source_page": 10,
                "about_en": "About text"
            }
        ]
    }"#;
    let file: GuidanceFile = serde_json::from_str(json).unwrap();
    assert_eq!(file.framework_id, "nist-ai-rmf");
    assert_eq!(file.source_pdf, "playbook.pdf");
    assert_eq!(file.guidance.len(), 1);
}

#[test]
fn test_guidance_entry_with_all_optional_fields_null() {
    let json = r#"{
        "concept_id": "gv-1.1-001",
        "source_page": 5
    }"#;
    let entry: GuidanceEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.concept_id, "gv-1.1-001");
    assert_eq!(entry.source_page, 5);
    assert!(entry.about_en.is_none());
    assert!(entry.about_nb.is_none());
    assert!(entry.suggested_actions_en.is_none());
    assert!(entry.suggested_actions_nb.is_none());
    assert!(entry.transparency_questions_en.is_none());
    assert!(entry.transparency_questions_nb.is_none());
    assert!(entry.resources.is_none());
    assert!(entry.references.is_none());
}

#[test]
fn test_resource_entry_deserializes_type_field() {
    let json = r#"{"title": "NIST AI 100-1", "url": "https://example.com", "type": "standard"}"#;
    let entry: ResourceEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.title, "NIST AI 100-1");
    assert_eq!(entry.url.as_deref(), Some("https://example.com"));
    assert_eq!(entry.resource_type.as_deref(), Some("standard"));
}

#[test]
fn test_reference_entry_with_partial_fields() {
    let json = r#"{"title": "Some Paper"}"#;
    let entry: ReferenceEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.title, "Some Paper");
    assert!(entry.authors.is_none());
    assert!(entry.year.is_none());
    assert!(entry.venue.is_none());
    assert!(entry.url.is_none());
}

#[test]
fn test_unknown_json_fields_are_ignored() {
    let json = r#"{
        "framework_id": "test",
        "source_pdf": "test.pdf",
        "extracted_at": "2026-01-01",
        "extra_field": true,
        "guidance": [{
            "concept_id": "x",
            "source_page": 1,
            "unknown_nested": 42
        }]
    }"#;
    let file: GuidanceFile = serde_json::from_str(json).unwrap();
    assert_eq!(file.guidance.len(), 1);
}

// ============================================================
// Import Function Tests
// ============================================================

fn sample_guidance_json() -> String {
    r#"{
        "framework_id": "nist-ai-rmf",
        "source_pdf": "playbook.pdf",
        "guidance": [{
            "concept_id": "test-concept-1",
            "source_page": 42,
            "about_en": "About this concept",
            "suggested_actions_en": ["Action one", "Action two"],
            "transparency_questions_en": ["Question one"],
            "resources": [{"title": "NIST AI 100-1", "url": "https://example.com", "type": "standard"}],
            "references": [{"title": "Paper A", "authors": "Smith et al.", "year": 2023, "venue": "AAAI"}]
        }]
    }"#
    .to_string()
}

#[tokio::test]
async fn test_import_guidance_populates_all_four_tables() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let guidance: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(guidance.0, 1);

    let actions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_actions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(actions.0, 2);

    let questions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_transparency_questions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(questions.0, 1);

    let refs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_references")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(refs.0, 2); // 1 resource + 1 academic
}

#[tokio::test]
async fn test_concept_guidance_row_has_correct_fields() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let row = sqlx::query("SELECT source_pdf, source_page, about_en FROM concept_guidance WHERE concept_id = 'test-concept-1'")
        .fetch_one(&pool)
        .await
        .unwrap();

    assert_eq!(row.get::<String, _>("source_pdf"), "playbook.pdf");
    assert_eq!(row.get::<i64, _>("source_page"), 42);
    assert_eq!(row.get::<String, _>("about_en"), "About this concept");
}

#[tokio::test]
async fn test_concept_actions_have_correct_sort_order() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let rows = sqlx::query("SELECT action_text_en, sort_order FROM concept_actions WHERE concept_id = 'test-concept-1' ORDER BY sort_order")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get::<String, _>("action_text_en"), "Action one");
    assert_eq!(rows[0].get::<i64, _>("sort_order"), 1);
    assert_eq!(rows[1].get::<String, _>("action_text_en"), "Action two");
    assert_eq!(rows[1].get::<i64, _>("sort_order"), 2);
}

#[tokio::test]
async fn test_references_split_into_correct_types() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let rows = sqlx::query("SELECT reference_type, title, sort_order FROM concept_references WHERE concept_id = 'test-concept-1' ORDER BY sort_order")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].get::<String, _>("reference_type"), "transparency_resource");
    assert_eq!(rows[0].get::<String, _>("title"), "NIST AI 100-1");
    assert_eq!(rows[0].get::<i64, _>("sort_order"), 1);
    assert_eq!(rows[1].get::<String, _>("reference_type"), "academic");
    assert_eq!(rows[1].get::<String, _>("title"), "Paper A");
    assert_eq!(rows[1].get::<i64, _>("sort_order"), 2);
}

#[tokio::test]
async fn test_invalid_concept_id_is_skipped() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let json = r#"{
        "framework_id": "test",
        "source_pdf": "test.pdf",
        "guidance": [
            {"concept_id": "test-concept-1", "source_page": 1, "about_en": "Valid"},
            {"concept_id": "nonexistent-concept", "source_page": 2, "about_en": "Invalid"}
        ]
    }"#;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", json).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 1);
}

#[tokio::test]
async fn test_reimport_produces_no_duplicates() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    // Import twice
    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();
    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let guidance: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(guidance.0, 1);

    let actions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_actions")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(actions.0, 2); // not 4
}

#[tokio::test]
async fn test_child_rows_replaced_on_reimport() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    // Re-import with different actions
    let json2 = r#"{
        "framework_id": "nist-ai-rmf",
        "source_pdf": "playbook.pdf",
        "guidance": [{
            "concept_id": "test-concept-1",
            "source_page": 42,
            "about_en": "Updated about",
            "suggested_actions_en": ["New action"]
        }]
    }"#;

    let mut tmp2 = NamedTempFile::new().unwrap();
    write!(tmp2, "{}", json2).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp2.path())
        .await
        .unwrap();

    let rows = sqlx::query("SELECT action_text_en FROM concept_actions WHERE concept_id = 'test-concept-1'")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].get::<String, _>("action_text_en"), "New action");

    // about_en should be updated too
    let row = sqlx::query("SELECT about_en FROM concept_guidance WHERE concept_id = 'test-concept-1'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.get::<String, _>("about_en"), "Updated about");
}

#[tokio::test]
async fn test_norwegian_only_actions_are_preserved() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let json = r#"{
        "framework_id": "test",
        "source_pdf": "test.pdf",
        "guidance": [{
            "concept_id": "test-concept-1",
            "source_page": 1,
            "suggested_actions_nb": ["Norsk handling 1", "Norsk handling 2"],
            "transparency_questions_nb": ["Norsk spørsmål"]
        }]
    }"#;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", json).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let actions = sqlx::query("SELECT action_text_en, action_text_nb, sort_order FROM concept_actions ORDER BY sort_order")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(actions.len(), 2);
    // en is empty string (NOT NULL column), nb has the content
    assert_eq!(actions[0].get::<String, _>("action_text_en"), "");
    assert_eq!(actions[0].get::<String, _>("action_text_nb"), "Norsk handling 1");
    assert_eq!(actions[1].get::<String, _>("action_text_nb"), "Norsk handling 2");

    let questions = sqlx::query("SELECT question_text_en, question_text_nb FROM concept_transparency_questions")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(questions.len(), 1);
    assert_eq!(questions[0].get::<String, _>("question_text_en"), "");
    assert_eq!(questions[0].get::<String, _>("question_text_nb"), "Norsk spørsmål");
}

#[tokio::test]
async fn test_mismatched_bilingual_array_lengths() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let json = r#"{
        "framework_id": "test",
        "source_pdf": "test.pdf",
        "guidance": [{
            "concept_id": "test-concept-1",
            "source_page": 1,
            "suggested_actions_en": ["English 1", "English 2", "English 3"],
            "suggested_actions_nb": ["Norsk 1"]
        }]
    }"#;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", json).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let rows = sqlx::query("SELECT action_text_en, action_text_nb, sort_order FROM concept_actions ORDER BY sort_order")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].get::<String, _>("action_text_en"), "English 1");
    assert_eq!(rows[0].get::<String, _>("action_text_nb"), "Norsk 1");
    assert_eq!(rows[1].get::<String, _>("action_text_en"), "English 2");
    assert!(rows[1].get::<Option<String>, _>("action_text_nb").is_none());
    assert_eq!(rows[2].get::<String, _>("action_text_en"), "English 3");
    assert!(rows[2].get::<Option<String>, _>("action_text_nb").is_none());
}

// ============================================================
// Wiring Tests (import_all_ontologies scans *-guidance.json)
// ============================================================

#[tokio::test]
async fn test_import_all_ontologies_picks_up_guidance_files() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let dir = tempfile::tempdir().unwrap();
    // Write a guidance file matching the *-guidance.json pattern
    std::fs::write(
        dir.path().join("test-guidance.json"),
        sample_guidance_json(),
    )
    .unwrap();

    ontology_backend::import::import_all_ontologies(&pool, dir.path())
        .await
        .unwrap();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 1);
}

#[tokio::test]
async fn test_import_all_ontologies_no_guidance_files_ok() {
    let pool = setup_db().await;

    let dir = tempfile::tempdir().unwrap();
    // Empty dir — no framework files, no guidance files
    ontology_backend::import::import_all_ontologies(&pool, dir.path())
        .await
        .unwrap();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 0);
}

#[tokio::test]
async fn test_non_guidance_json_files_are_ignored() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let dir = tempfile::tempdir().unwrap();
    // Write a file that does NOT match *-guidance.json
    std::fs::write(
        dir.path().join("some-other-file.json"),
        sample_guidance_json(),
    )
    .unwrap();

    ontology_backend::import::import_all_ontologies(&pool, dir.path())
        .await
        .unwrap();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 0);
}

#[tokio::test]
async fn test_guidance_with_unknown_framework_id_imports() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    // framework_id is metadata only, not a FK — should still import fine
    let json = r#"{
        "framework_id": "nonexistent-framework",
        "source_pdf": "test.pdf",
        "guidance": [{
            "concept_id": "test-concept-1",
            "source_page": 1,
            "about_en": "Works despite unknown framework"
        }]
    }"#;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", json).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM concept_guidance")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count.0, 1);
}

// ============================================================
// FTS5 Tests
// ============================================================

#[tokio::test]
async fn test_fts5_match_on_about_en() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    // Use a distinctive word only in about_en, not in concept name_en
    let json = r#"{
        "framework_id": "test",
        "source_pdf": "test.pdf",
        "guidance": [{
            "concept_id": "test-concept-1",
            "source_page": 1,
            "about_en": "Governance orchestration methodology"
        }]
    }"#;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", json).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let rows = sqlx::query("SELECT * FROM concept_guidance_fts WHERE concept_guidance_fts MATCH 'orchestration'")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(!rows.is_empty(), "FTS5 should find 'orchestration' in about_en");
}

#[tokio::test]
async fn test_fts5_match_on_concept_name() {
    let pool = setup_db().await;
    seed_concept(&pool).await; // seeds name_en = "Test Concept"

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let rows = sqlx::query("SELECT * FROM concept_guidance_fts WHERE concept_guidance_fts MATCH '\"Test Concept\"'")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert!(!rows.is_empty(), "FTS5 should find concept name_en via content view");
}

#[tokio::test]
async fn test_fts5_join_back_to_source_tables() {
    let pool = setup_db().await;
    seed_concept(&pool).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", sample_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    let rows = sqlx::query(
        "SELECT cg.concept_id, c.name_en, c.code, cg.about_en \
         FROM concept_guidance_fts \
         JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid \
         JOIN concepts c ON c.id = cg.concept_id \
         WHERE concept_guidance_fts MATCH 'concept' \
         ORDER BY rank",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert!(!rows.is_empty());
    assert_eq!(rows[0].get::<String, _>("concept_id"), "test-concept-1");
    assert_eq!(rows[0].get::<String, _>("name_en"), "Test Concept");
    assert_eq!(rows[0].get::<String, _>("about_en"), "About this concept");
}

// ============================================================
// Integration Tests (real ontology data, real concept IDs)
// ============================================================

mod common;
use common::create_test_pool;

/// Cleanup guidance data for specific concept IDs after test
async fn cleanup_guidance(pool: &sqlx::SqlitePool, concept_ids: &[&str]) {
    for id in concept_ids {
        sqlx::query("DELETE FROM concept_guidance WHERE concept_id = ?")
            .bind(id)
            .execute(pool)
            .await
            .ok();
    }
}

fn integration_guidance_json() -> String {
    r#"{
        "framework_id": "nist-ai-rmf",
        "source_pdf": "NIST-AI-100-1.pdf",
        "guidance": [
            {
                "concept_id": "nist-ai-gv-1-1",
                "source_page": 35,
                "about_en": "INTEGRATIONTESTKEYWORD governance policies for AI risk",
                "about_nb": "Styringspolicyer for AI-risiko",
                "suggested_actions_en": ["Establish governance board", "Define risk appetite"],
                "suggested_actions_nb": ["Etabler styringsråd", "Definer risikoappetitt"],
                "transparency_questions_en": ["What governance structure exists?"],
                "resources": [{"title": "NIST AI 100-1", "url": "https://example.com", "type": "standard"}],
                "references": [{"title": "AI Governance Paper", "authors": "Smith et al.", "year": 2023, "venue": "NeurIPS"}]
            },
            {
                "concept_id": "nist-ai-gv-1-2",
                "source_page": 38,
                "about_en": "INTEGRATIONTESTKEYWORD regulatory compliance for AI systems",
                "suggested_actions_en": ["Map applicable regulations", "Assign compliance ownership"],
                "transparency_questions_en": ["Which regulations apply?", "Who owns compliance?"]
            }
        ]
    }"#
    .to_string()
}

#[tokio::test]
async fn integration_import_with_real_concept_ids() {
    let pool = create_test_pool().await;
    let test_ids = ["nist-ai-gv-1-1", "nist-ai-gv-1-2"];
    cleanup_guidance(&pool, &test_ids).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", integration_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    // Verify both concepts got guidance rows
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM concept_guidance WHERE concept_id IN ('nist-ai-gv-1-1', 'nist-ai-gv-1-2')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count.0, 2);

    // Verify actions for gv-1-1
    let actions = sqlx::query(
        "SELECT action_text_en, action_text_nb, sort_order FROM concept_actions \
         WHERE concept_id = 'nist-ai-gv-1-1' ORDER BY sort_order",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(actions.len(), 2);
    assert_eq!(actions[0].get::<String, _>("action_text_en"), "Establish governance board");
    assert_eq!(actions[0].get::<String, _>("action_text_nb"), "Etabler styringsråd");
    assert_eq!(actions[0].get::<i64, _>("sort_order"), 1);

    // Verify transparency questions for gv-1-2
    let questions = sqlx::query(
        "SELECT question_text_en, sort_order FROM concept_transparency_questions \
         WHERE concept_id = 'nist-ai-gv-1-2' ORDER BY sort_order",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(questions.len(), 2);
    assert_eq!(questions[1].get::<String, _>("question_text_en"), "Who owns compliance?");

    // Verify references for gv-1-1 (1 resource + 1 academic)
    let refs = sqlx::query(
        "SELECT reference_type, title, sort_order FROM concept_references \
         WHERE concept_id = 'nist-ai-gv-1-1' ORDER BY sort_order",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(refs.len(), 2);
    assert_eq!(refs[0].get::<String, _>("reference_type"), "transparency_resource");
    assert_eq!(refs[1].get::<String, _>("reference_type"), "academic");

    cleanup_guidance(&pool, &test_ids).await;
}

#[tokio::test]
async fn integration_reimport_idempotent_with_real_ids() {
    let pool = create_test_pool().await;
    let test_ids = ["nist-ai-gv-1-1", "nist-ai-gv-1-2"];
    cleanup_guidance(&pool, &test_ids).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", integration_guidance_json()).unwrap();

    // Import twice
    ontology_backend::import::import_guidance_file(&pool, tmp.path()).await.unwrap();
    ontology_backend::import::import_guidance_file(&pool, tmp.path()).await.unwrap();

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM concept_guidance WHERE concept_id IN ('nist-ai-gv-1-1', 'nist-ai-gv-1-2')",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count.0, 2, "Upsert should not duplicate guidance rows");

    let action_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM concept_actions WHERE concept_id = 'nist-ai-gv-1-1'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(action_count.0, 2, "Delete-reinsert should not duplicate actions");

    cleanup_guidance(&pool, &test_ids).await;
}

#[tokio::test]
async fn integration_fts5_search_with_real_data() {
    let pool = create_test_pool().await;
    let test_ids = ["nist-ai-gv-1-1", "nist-ai-gv-1-2"];
    cleanup_guidance(&pool, &test_ids).await;

    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", integration_guidance_json()).unwrap();

    ontology_backend::import::import_guidance_file(&pool, tmp.path())
        .await
        .unwrap();

    // FTS5 search using our distinctive test keyword
    let rows = sqlx::query(
        "SELECT cg.concept_id, c.name_en, c.code, cg.about_en \
         FROM concept_guidance_fts \
         JOIN concept_guidance cg ON cg.rowid = concept_guidance_fts.rowid \
         JOIN concepts c ON c.id = cg.concept_id \
         WHERE concept_guidance_fts MATCH 'INTEGRATIONTESTKEYWORD' \
         ORDER BY rank",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(rows.len(), 2, "Both guidance entries contain the test keyword");

    // Verify joined data is correct
    let concept_ids: Vec<String> = rows.iter().map(|r| r.get("concept_id")).collect();
    assert!(concept_ids.contains(&"nist-ai-gv-1-1".to_string()));
    assert!(concept_ids.contains(&"nist-ai-gv-1-2".to_string()));

    // Verify concept name_en comes through the join
    for row in &rows {
        let name: String = row.get("name_en");
        assert!(!name.is_empty(), "Concept name should be populated via join");
    }

    cleanup_guidance(&pool, &test_ids).await;
}

#[tokio::test]
async fn integration_api_health_still_works_after_migration() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    let app = common::create_test_app().await;

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
}

#[tokio::test]
async fn integration_api_frameworks_still_works_after_migration() {
    use axum::{body::Body, http::{Request, StatusCode}};
    use tower::ServiceExt;

    let app = common::create_test_app().await;

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
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json.is_array(), "Frameworks endpoint should return an array");
    assert!(!json.as_array().unwrap().is_empty(), "Should have frameworks");
}
