Now I have all the context. Let me generate the section content.

# Section 7: Session Cleanup

## Overview

This section adds two session cleanup mechanisms to prevent stale session rows from accumulating in the `sessions` table:

1. **Lazy cleanup** -- When the `AuthUser` extractor (from split 01) encounters an expired session during validation, it deletes that session row before returning `None`. This is already specified in split 01's `validate_session()` implementation.

2. **Bulk cleanup on startup** -- A standalone function `cleanup_expired_sessions()` that deletes all expired sessions when the application starts. This handles sessions that expire while the server is offline (e.g., between restarts in the air-gapped deployment).

No background tasks, timers, or cron jobs are needed. The combination of startup cleanup and lazy per-request cleanup is sufficient for a single-instance air-gapped deployment.

## Dependencies on Previous Sections

- **Section 1 (Dependencies and Configuration):** The `AppState` struct with `db: SqlitePool` must be available. No new dependencies are required for this section.
- **Split 01 (01-backend-core-auth):** The `validate_session()` function in `backend/src/features/auth/service.rs` must exist. The `sessions` table schema must be in place (from `001_initial_schema.sql`).

## Database Schema Reference

The `sessions` table has the following structure (from `backend/migrations/001_initial_schema.sql`):

```sql
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT UNIQUE NOT NULL,
    expires_at TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);
```

The `expires_at` column stores ISO 8601 datetime strings. SQLite's `datetime('now')` produces UTC timestamps in the same format, so lexicographic comparison works correctly for expiry checks.

## Tests First

All tests belong in the `#[cfg(test)]` module within `backend/src/features/auth/service.rs`, alongside the existing service tests. They require an in-memory SQLite database with migrations applied (using the same `setup_test_db()` helper from Section 4 of split 01).

### Test: Expired session is deleted when encountered during validation

```rust
/// Insert a user and a session with expires_at in the past.
/// Call validate_session() with the raw token.
/// Assert: returns None AND the session row no longer exists in the DB.
#[tokio::test]
async fn expired_session_deleted_on_validation() {
    // setup_test_db()
    // Insert user
    // Insert session with expires_at = datetime('now', '-1 hour')
    // Call validate_session(pool, &raw_token)
    // Assert returns Ok(None)
    // SELECT COUNT(*) FROM sessions WHERE user_id = ? -> assert 0
}
```

This test verifies that `validate_session()` performs lazy cleanup. If split 01's implementation already handles this (the plan specifies it should), this test simply confirms the behavior. If it does not yet perform the DELETE, this section modifies `validate_session()` to add it.

### Test: cleanup_expired_sessions deletes all expired sessions

```rust
/// Insert multiple sessions: some expired, some valid.
/// Call cleanup_expired_sessions().
/// Assert: only expired sessions are removed.
#[tokio::test]
async fn cleanup_deletes_all_expired_sessions() {
    // setup_test_db()
    // Insert user A with session expiring in the past
    // Insert user B with session expiring in the past
    // Insert user C with session expiring in the future
    // Call cleanup_expired_sessions(&pool)
    // SELECT COUNT(*) FROM sessions -> assert 1 (only user C's)
}
```

### Test: cleanup_expired_sessions does not delete valid sessions

```rust
/// Insert only non-expired sessions.
/// Call cleanup_expired_sessions().
/// Assert: all sessions still exist, count returned is 0.
#[tokio::test]
async fn cleanup_preserves_valid_sessions() {
    // setup_test_db()
    // Insert user with session expiring 8 hours in the future
    // let count = cleanup_expired_sessions(&pool).await.unwrap()
    // assert_eq!(count, 0)
    // SELECT COUNT(*) FROM sessions -> assert 1
}
```

### Test: cleanup_expired_sessions returns count of deleted sessions

```rust
/// Insert 3 expired sessions and 1 valid session.
/// Call cleanup_expired_sessions().
/// Assert: returns 3.
#[tokio::test]
async fn cleanup_returns_deleted_count() {
    // setup_test_db()
    // Insert 3 users each with an expired session
    // Insert 1 user with a valid session
    // let count = cleanup_expired_sessions(&pool).await.unwrap()
    // assert_eq!(count, 3)
}
```

## Implementation Details

### File to Modify

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/service.rs`

### Part 1: Verify Lazy Cleanup in validate_session()

The existing `validate_session()` function (from split 01, Section 4) should already implement lazy deletion of expired sessions. The specified behavior is:

1. Hash the raw token with SHA-256
2. Query: `SELECT * FROM sessions WHERE token = ? AND expires_at > datetime('now')`
3. If no valid row found, check if an expired row exists for this token hash
4. If an expired row exists, DELETE it
5. Return `None`

**If this is already implemented:** No changes needed to `validate_session()`. The test above simply confirms the behavior.

**If the lazy DELETE is not yet implemented:** Add the expired-session deletion step. After the initial query returns no results, execute:

```rust
/// Inside validate_session(), after the primary query returns None:
/// Check for and delete the expired session matching this token hash.
// DELETE FROM sessions WHERE token = ? AND expires_at <= datetime('now')
```

The key SQL for the lazy cleanup path:

```sql
DELETE FROM sessions WHERE token = ? AND expires_at <= datetime('now')
```

This is a single statement that both finds and deletes the expired session, avoiding a race condition. No need for a separate SELECT followed by DELETE.

### Part 2: Bulk Cleanup Function

Add the following function to `backend/src/features/auth/service.rs`:

```rust
/// Delete all expired sessions from the database.
/// Returns the number of sessions deleted.
/// Called once during server startup to clean up sessions
/// that expired while the server was offline.
pub async fn cleanup_expired_sessions(pool: &SqlitePool) -> Result<u64, AppError>
```

The implementation executes a single SQL statement:

```sql
DELETE FROM sessions WHERE expires_at <= datetime('now')
```

Use `sqlx::query(...)` with `.execute(pool).await` and read `result.rows_affected()` to get the count. Log the count at `info` level if greater than zero, or at `debug` level if zero.

### Part 3: Call Cleanup on Startup

In `backend/src/main.rs`, after the database pool is created and migrations have run, but before the server starts listening, call the cleanup function:

```rust
// After pool creation and migrations, before server bind:
match auth::service::cleanup_expired_sessions(&pool).await {
    Ok(count) if count > 0 => {
        tracing::info!("Cleaned up {} expired sessions on startup", count);
    }
    Ok(_) => {
        tracing::debug!("No expired sessions to clean up");
    }
    Err(e) => {
        tracing::warn!("Failed to clean up expired sessions: {}", e);
        // Non-fatal: server continues starting even if cleanup fails
    }
}
```

The cleanup failure is non-fatal. The server should start regardless, since lazy cleanup will handle expired sessions as they are encountered.

### File to Modify for Startup Call

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs`

The `service` module must be accessible from `main.rs`. It is already exposed via `features::auth::service` since `mod.rs` declares `pub mod service;` (added in split 01, Section 4).

## Key Design Decisions

1. **No background task:** A periodic timer (e.g., `tokio::spawn` with `tokio::time::interval`) is unnecessary for this deployment model. The server is single-instance and air-gapped, so session volume is low. Startup cleanup handles the "server was offline" case, and lazy cleanup handles the "server is running" case.

2. **Non-fatal startup cleanup:** If the cleanup query fails (e.g., database locked during startup), the server still starts. Expired sessions will be cleaned up lazily as users attempt to use them.

3. **Single SQL statement for bulk cleanup:** Using `DELETE FROM sessions WHERE expires_at <= datetime('now')` is efficient and uses the existing `idx_sessions_expires` index. No cursor-based iteration is needed.

4. **Logging:** The cleanup count is logged at `info` level only when sessions were actually deleted. This avoids log noise on clean restarts while providing visibility when stale sessions are purged.