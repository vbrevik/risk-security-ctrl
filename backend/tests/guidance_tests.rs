//! Tests for the guidance data schema (migration 004).

use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;

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
