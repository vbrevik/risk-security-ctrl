use sqlx::SqlitePool;
use uuid::Uuid;

use crate::error::{AppError, FieldError};
use crate::features::auth::models::UserProfile;
use crate::features::auth::password::{generate_session_token, hash_session_token};

/// Internal struct for reading full user rows from the database.
#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub is_active: i32,
}

/// Internal struct for session rows.
#[derive(sqlx::FromRow)]
pub struct SessionRow {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Insert a new user with role=viewer. Returns UserProfile on success.
/// On duplicate email (UNIQUE violation), returns a generic validation error
/// to prevent account enumeration.
pub async fn create_user(
    pool: &SqlitePool,
    email: &str,
    password_hash: &str,
    name: &str,
) -> Result<UserProfile, AppError> {
    let id = Uuid::new_v4().to_string();
    let result = sqlx::query(
        "INSERT INTO users (id, email, password_hash, name, role, is_active) VALUES (?, ?, ?, ?, 'viewer', 1)",
    )
    .bind(&id)
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .execute(pool)
    .await;

    match result {
        Ok(_) => Ok(UserProfile {
            id,
            email: email.to_string(),
            name: name.to_string(),
            role: "viewer".to_string(),
        }),
        Err(sqlx::Error::Database(e)) if e.message().contains("UNIQUE constraint") => {
            Err(AppError::ValidationError(vec![FieldError {
                field: "email".to_string(),
                message: "Registration failed".to_string(),
            }]))
        }
        Err(e) => Err(AppError::Database(e)),
    }
}

/// Look up a user by email. Returns the full UserRow (including password_hash)
/// for credential verification.
pub async fn find_user_by_email(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<UserRow>, AppError> {
    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id, email, password_hash, name, role, is_active FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

/// Look up a user by ID. Used by the auth extractor to load user from session.
pub async fn find_user_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<UserRow>, AppError> {
    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id, email, password_hash, name, role, is_active FROM users WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

/// Create a new session for the given user. Enforces single-session by deleting
/// any existing sessions first. Returns (SessionRow, raw_token).
pub async fn create_session(
    pool: &SqlitePool,
    user_id: &str,
    ip_address: &str,
    user_agent: &str,
) -> Result<(SessionRow, String), AppError> {
    let mut tx = pool.begin().await?;

    // Single-session enforcement: delete all existing sessions for this user
    sqlx::query("DELETE FROM sessions WHERE user_id = ?")
        .bind(user_id)
        .execute(&mut *tx)
        .await?;

    let raw_token = generate_session_token();
    let token_hash = hash_session_token(&raw_token);
    let session_id = Uuid::new_v4().to_string();

    // Expire in 8 hours
    let expires_at = sqlx::query_scalar::<_, String>(
        "SELECT datetime('now', '+8 hours')",
    )
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO sessions (id, user_id, token, expires_at, ip_address, user_agent) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&session_id)
    .bind(user_id)
    .bind(&token_hash)
    .bind(&expires_at)
    .bind(ip_address)
    .bind(user_agent)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok((
        SessionRow {
            id: session_id,
            user_id: user_id.to_string(),
            token: token_hash,
            expires_at,
            ip_address: Some(ip_address.to_string()),
            user_agent: Some(user_agent.to_string()),
        },
        raw_token,
    ))
}

/// Validate a raw session token. Hashes it first, then looks up in DB.
/// Returns None if token not found or session expired.
pub async fn validate_session(
    pool: &SqlitePool,
    raw_token: &str,
) -> Result<Option<SessionRow>, AppError> {
    let token_hash = hash_session_token(raw_token);

    // Look for valid (non-expired) session
    let session = sqlx::query_as::<_, SessionRow>(
        "SELECT id, user_id, token, expires_at, ip_address, user_agent FROM sessions WHERE token = ? AND expires_at > datetime('now')",
    )
    .bind(&token_hash)
    .fetch_optional(pool)
    .await?;

    if session.is_some() {
        return Ok(session);
    }

    // Clean up expired session if it exists
    sqlx::query("DELETE FROM sessions WHERE token = ? AND expires_at <= datetime('now')")
        .bind(&token_hash)
        .execute(pool)
        .await?;

    Ok(None)
}

/// Delete a specific session by ID.
pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(session_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Delete all sessions for a user.
pub async fn delete_user_sessions(pool: &SqlitePool, user_id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sessions WHERE user_id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Update the last_login_at timestamp for a user.
pub async fn update_last_login(pool: &SqlitePool, user_id: &str) -> Result<(), AppError> {
    sqlx::query("UPDATE users SET last_login_at = datetime('now') WHERE id = ?")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Write an entry to the audit log.
pub async fn log_audit(
    pool: &SqlitePool,
    user_id: &str,
    action: &str,
    entity_type: &str,
    entity_id: &str,
    ip_address: &str,
) -> Result<(), AppError> {
    let id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, ip_address) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(user_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(ip_address)
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqliteConnectOptions;
    use std::str::FromStr;

    async fn setup_test_db() -> SqlitePool {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);
        let pool = SqlitePool::connect_with(options).await.unwrap();

        // Enable foreign keys
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_user_inserts_with_viewer_role() {
        let pool = setup_test_db().await;
        let profile = create_user(&pool, "test@example.com", "hashed_pw", "Test User")
            .await
            .unwrap();

        assert_eq!(profile.email, "test@example.com");
        assert_eq!(profile.name, "Test User");
        assert_eq!(profile.role, "viewer");
        assert!(!profile.id.is_empty());

        // Verify in DB
        let row = sqlx::query_as::<_, UserRow>(
            "SELECT id, email, password_hash, name, role, is_active FROM users WHERE email = ?",
        )
        .bind("test@example.com")
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(row.role, "viewer");
        assert_eq!(row.is_active, 1);
    }

    #[tokio::test]
    async fn test_create_user_rejects_duplicate_email() {
        let pool = setup_test_db().await;
        create_user(&pool, "dupe@test.com", "hash1", "User1")
            .await
            .unwrap();
        let result = create_user(&pool, "dupe@test.com", "hash2", "User2").await;
        assert!(result.is_err());
        // Should be a validation error, not a raw DB error
        match result.unwrap_err() {
            AppError::ValidationError(fields) => {
                assert_eq!(fields[0].message, "Registration failed");
            }
            other => panic!("Expected ValidationError, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_find_user_by_email_returns_none_for_unknown() {
        let pool = setup_test_db().await;
        let result = find_user_by_email(&pool, "nonexistent@test.com")
            .await
            .unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_create_session_enforces_single_session() {
        let pool = setup_test_db().await;
        create_user(&pool, "user@test.com", "hash", "User")
            .await
            .unwrap();
        let user = find_user_by_email(&pool, "user@test.com")
            .await
            .unwrap()
            .unwrap();

        // Create two sessions
        create_session(&pool, &user.id, "127.0.0.1", "Agent1")
            .await
            .unwrap();
        create_session(&pool, &user.id, "127.0.0.1", "Agent2")
            .await
            .unwrap();

        // Only one should remain
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM sessions WHERE user_id = ?")
                .bind(&user.id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(count.0, 1);
    }

    #[tokio::test]
    async fn test_validate_session_returns_none_for_expired() {
        let pool = setup_test_db().await;
        create_user(&pool, "user@test.com", "hash", "User")
            .await
            .unwrap();
        let user = find_user_by_email(&pool, "user@test.com")
            .await
            .unwrap()
            .unwrap();

        let raw_token = generate_session_token();
        let token_hash = hash_session_token(&raw_token);

        // Insert expired session directly
        sqlx::query(
            "INSERT INTO sessions (id, user_id, token, expires_at) VALUES (?, ?, ?, datetime('now', '-1 hour'))",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(&user.id)
        .bind(&token_hash)
        .execute(&pool)
        .await
        .unwrap();

        let result = validate_session(&pool, &raw_token).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_validate_session_returns_session_for_valid_token() {
        let pool = setup_test_db().await;
        create_user(&pool, "user@test.com", "hash", "User")
            .await
            .unwrap();
        let user = find_user_by_email(&pool, "user@test.com")
            .await
            .unwrap()
            .unwrap();

        let (session, raw_token) =
            create_session(&pool, &user.id, "127.0.0.1", "TestAgent")
                .await
                .unwrap();

        let result = validate_session(&pool, &raw_token).await.unwrap();
        assert!(result.is_some());
        let found = result.unwrap();
        assert_eq!(found.user_id, user.id);
        assert_eq!(found.id, session.id);
    }

    #[tokio::test]
    async fn test_validate_session_uses_token_hashing() {
        let pool = setup_test_db().await;
        create_user(&pool, "user@test.com", "hash", "User")
            .await
            .unwrap();
        let user = find_user_by_email(&pool, "user@test.com")
            .await
            .unwrap()
            .unwrap();

        let (session, raw_token) =
            create_session(&pool, &user.id, "127.0.0.1", "TestAgent")
                .await
                .unwrap();

        // The stored token should be the hash, not the raw token
        let stored: (String,) =
            sqlx::query_as("SELECT token FROM sessions WHERE id = ?")
                .bind(&session.id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_ne!(stored.0, raw_token, "Stored token must be hash, not raw");
        assert_eq!(stored.0, hash_session_token(&raw_token));

        // Validation should still work with raw token
        let result = validate_session(&pool, &raw_token).await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_log_audit_writes_entry() {
        let pool = setup_test_db().await;
        create_user(&pool, "user@test.com", "hash", "User")
            .await
            .unwrap();
        let user = find_user_by_email(&pool, "user@test.com")
            .await
            .unwrap()
            .unwrap();

        log_audit(&pool, &user.id, "login", "session", "sess-456", "192.168.1.1")
            .await
            .unwrap();

        let entry: (String, String, Option<String>) = sqlx::query_as(
            "SELECT action, entity_type, ip_address FROM audit_log WHERE user_id = ?",
        )
        .bind(&user.id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(entry.0, "login");
        assert_eq!(entry.1, "session");
        assert_eq!(entry.2.as_deref(), Some("192.168.1.1"));
    }
}
