use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::str::FromStr;

// Import the seed_admin function from the binary crate
// Note: We re-implement the core logic test here since the binary's
// seed_admin function is in src/bin/ and not directly importable as a library.
// Instead, we test the same logic using the shared library functions.

async fn setup_test_db() -> SqlitePool {
    let options = SqliteConnectOptions::from_str("sqlite::memory:")
        .unwrap()
        .create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await.unwrap();
    sqlx::query("PRAGMA foreign_keys = ON")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    pool
}

/// Replicates the seed_admin core logic for testing.
async fn seed_admin_logic(
    pool: &SqlitePool,
    email: &str,
    password: &str,
    name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let existing: Option<(String,)> =
        sqlx::query_as("SELECT id FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await?;

    if existing.is_some() {
        return Ok(());
    }

    let password_hash = ontology_backend::features::auth::password::hash_password(password)
        .map_err(|e| format!("Failed to hash password: {e:?}"))?;

    let id = uuid::Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, is_active) VALUES (?, ?, ?, ?, 'admin', 1)",
    )
    .bind(&id)
    .bind(email)
    .bind(&password_hash)
    .bind(name)
    .execute(pool)
    .await?;

    Ok(())
}

#[tokio::test]
async fn seed_admin_creates_admin_user() {
    let pool = setup_test_db().await;
    seed_admin_logic(&pool, "admin@test.com", "securepass123", "Test Admin")
        .await
        .unwrap();

    let row: (String, String, String, i32) = sqlx::query_as(
        "SELECT email, role, password_hash, is_active FROM users WHERE email = ?",
    )
    .bind("admin@test.com")
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(row.0, "admin@test.com");
    assert_eq!(row.1, "admin");
    assert!(row.2.starts_with("$argon2id$"));
    assert_eq!(row.3, 1);
}

#[tokio::test]
async fn seed_admin_is_idempotent() {
    let pool = setup_test_db().await;

    // First call
    seed_admin_logic(&pool, "admin@test.com", "securepass123", "Test Admin")
        .await
        .unwrap();

    // Second call — should not error
    seed_admin_logic(&pool, "admin@test.com", "securepass123", "Test Admin")
        .await
        .unwrap();

    // Only one user should exist
    let count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = ?")
            .bind("admin@test.com")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count.0, 1);
}
