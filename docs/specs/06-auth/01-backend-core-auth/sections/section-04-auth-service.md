Now I have all the context needed. Let me generate the section content.

# Section 4: Auth Service (Database Operations)

## Overview

This section implements the auth service layer at `backend/src/features/auth/service.rs`. The service contains all database operations for the auth system: user CRUD, session management with SHA-256 token hashing, and audit logging. Every function takes a `&SqlitePool` parameter and returns `Result<T, AppError>`.

## Dependencies on Prior Sections

- **Section 1 (Dependencies and AppState):** The `sqlx`, `uuid`, `sha2`, and `time` crates must be available in `Cargo.toml`.
- **Section 2 (Auth Models):** The `UserProfile` response type, `AuthUser` struct, and `AppError` variants (`InvalidCredentials`, `ValidationError`, `SessionExpired`) must be defined.
- **Section 3 (Password and Session Utilities):** The `generate_session_token()` and `hash_session_token()` functions must exist in `backend/src/features/auth/password.rs`.

## Database Schema Reference

The service operates against three existing tables (from `backend/migrations/001_initial_schema.sql`):

```sql
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'viewer',
    is_active INTEGER DEFAULT 1,
    last_login_at TEXT,
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token TEXT UNIQUE NOT NULL,
    expires_at TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS audit_log (
    id TEXT PRIMARY KEY,
    user_id TEXT REFERENCES users(id),
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    old_value TEXT,
    new_value TEXT,
    ip_address TEXT,
    created_at TEXT DEFAULT (datetime('now'))
);
```

Indexes exist on `users(email)`, `sessions(user_id)`, `sessions(token)`, and `sessions(expires_at)`.

## Tests First

All tests are integration tests that use an in-memory SQLite database with migrations applied. Place these in inline `#[cfg(test)] mod tests` within `backend/src/features/auth/service.rs`.

Each test must set up its own `SqlitePool` with in-memory SQLite and run migrations before executing. A helper function within the test module should handle this.

### Test: create_user inserts user with viewer role

- Call `create_user(pool, "test@example.com", "hashed_pw", "Test User")`
- Query the `users` table directly with `sqlx::query_as`
- Assert: the user exists, `role == "viewer"`, `is_active == 1`, and a UUID-format `id` is set

### Test: create_user rejects duplicate email

- Call `create_user` with email `"dupe@test.com"` once (should succeed)
- Call `create_user` with the same email again
- Assert: returns an `AppError` that maps to a validation error (not a 409 conflict). This prevents account enumeration.

### Test: find_user_by_email returns None for unknown email

- Call `find_user_by_email(pool, "nonexistent@test.com")`
- Assert: returns `Ok(None)`

### Test: create_session deletes previous sessions (single-session enforcement)

- Insert a user manually
- Call `create_session(pool, user_id, "127.0.0.1", "TestAgent")` to create session 1
- Call `create_session(pool, user_id, "127.0.0.1", "TestAgent")` to create session 2
- Query `SELECT COUNT(*) FROM sessions WHERE user_id = ?`
- Assert: count is 1 (the old session was deleted)

### Test: validate_session returns None for expired token

- Insert a user, then INSERT a session directly with `expires_at` set to a datetime in the past
- Call `validate_session(pool, raw_token)` with the token that corresponds to the stored hash
- Assert: returns `Ok(None)`

### Test: validate_session returns session for valid token

- Insert a user, call `create_session` to get `(session, raw_token)`
- Call `validate_session(pool, &raw_token)`
- Assert: returns `Ok(Some(session))` with the correct `user_id`

### Test: validate_session uses token hashing (stored token differs from raw)

- Insert a user, call `create_session` to get `(session, raw_token)`
- Query the `sessions` table directly: `SELECT token FROM sessions WHERE id = ?`
- Assert: the stored token value does NOT equal the raw token (it is the SHA-256 hash)
- Also assert that `validate_session(pool, &raw_token)` still succeeds (proving it hashes before lookup)

### Test: log_audit writes entry with ip_address

- Call `log_audit(pool, "user-123", "login", "session", "sess-456", "192.168.1.1")`
- Query `audit_log` table directly
- Assert: entry exists with `action == "login"`, `entity_type == "session"`, `ip_address == "192.168.1.1"`

## Implementation Details

### File to Create

`/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/service.rs`

### Module Registration

In `backend/src/features/auth/mod.rs`, add:

```rust
pub mod service;
```

(This file currently only contains `pub mod routes;`.)

### Internal Row Type

Define a private `UserRow` struct for reading full user rows from the database (including `password_hash`, which must never appear in API responses):

```rust
/// Internal struct for reading full user rows. Not exposed in API.
pub(crate) struct UserRow {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub name: String,
    pub role: String,
    pub is_active: bool,   // SQLite INTEGER maps to bool via i32 check
}
```

Derive `sqlx::FromRow` on this struct. Note that SQLite stores booleans as integers, so `is_active` may need to be `i32` or `i64` rather than `bool`, then converted when used.

### Session Row Type

```rust
/// Internal struct for session rows.
pub(crate) struct SessionRow {
    pub id: String,
    pub user_id: String,
    pub token: String,        // This is the SHA-256 hash, never the raw token
    pub expires_at: String,   // ISO 8601 datetime string
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
```

Derive `sqlx::FromRow`.

### Function Signatures and Behavior

#### create_user

```rust
/// Insert a new user with role=viewer. Returns UserProfile on success.
/// On duplicate email (UNIQUE violation), returns a generic validation error
/// to prevent account enumeration.
pub async fn create_user(
    pool: &SqlitePool,
    email: &str,
    password_hash: &str,
    name: &str,
) -> Result<UserProfile, AppError>
```

- Generate a UUID v4 for the `id` field
- INSERT into `users` with `role = "viewer"` and `is_active = 1`
- Catch `sqlx::Error::Database` where the error message contains "UNIQUE constraint" and map it to a generic `AppError::ValidationError` (or `AppError::BadRequest` with a vague message like "Registration failed")
- On success, return a `UserProfile` with `id`, `email`, `name`, `role`

#### find_user_by_email

```rust
/// Look up a user by email. Returns the full UserRow (including password_hash)
/// for credential verification. Returns None if no user matches.
pub async fn find_user_by_email(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<UserRow>, AppError>
```

- `SELECT id, email, password_hash, name, role, is_active FROM users WHERE email = ?`
- Use `query_as::<_, UserRow>` with `.fetch_optional(pool)`

#### find_user_by_id

```rust
/// Look up a user by ID. Used by the auth extractor to load user from session.
/// Returns None if user does not exist.
pub async fn find_user_by_id(
    pool: &SqlitePool,
    id: &str,
) -> Result<Option<UserRow>, AppError>
```

- Same as `find_user_by_email` but with `WHERE id = ?`

#### create_session

```rust
/// Create a new session for the given user. Enforces single-session by deleting
/// any existing sessions for this user_id first. Returns (SessionRow, raw_token)
/// where raw_token is the unhashed token for the cookie/response.
pub async fn create_session(
    pool: &SqlitePool,
    user_id: &str,
    ip_address: &str,
    user_agent: &str,
) -> Result<(SessionRow, String), AppError>
```

Steps:
1. `DELETE FROM sessions WHERE user_id = ?` (single-session enforcement)
2. Call `generate_session_token()` to get a raw 64-char hex token
3. Call `hash_session_token(&raw_token)` to get the SHA-256 hash
4. Generate a UUID v4 for the session `id`
5. Compute `expires_at` as current UTC time + 8 hours (or the configured `SESSION_DURATION_HOURS`), formatted as ISO 8601 string
6. INSERT into `sessions` with the **hashed** token
7. Return `(session_row, raw_token)` -- the raw token is what gets sent to the client

**Critical:** The raw token is returned to the caller but never stored. Only the hash goes into the database.

#### validate_session

```rust
/// Validate a raw session token. Hashes it first, then looks up in DB.
/// Returns None if token not found or session expired.
/// Deletes expired sessions on lookup.
pub async fn validate_session(
    pool: &SqlitePool,
    raw_token: &str,
) -> Result<Option<SessionRow>, AppError>
```

Steps:
1. Call `hash_session_token(raw_token)` to compute the hash
2. `SELECT * FROM sessions WHERE token = ? AND expires_at > datetime('now')`
3. If no row found, also check if an expired row exists (`SELECT ... WHERE token = ?` without the expiry check) and DELETE it if found
4. Return `Some(session)` or `None`

The expiry comparison uses SQLite's `datetime('now')` to compare against the stored ISO 8601 string.

#### delete_session

```rust
/// Delete a specific session by ID.
pub async fn delete_session(pool: &SqlitePool, session_id: &str) -> Result<(), AppError>
```

- `DELETE FROM sessions WHERE id = ?`

#### delete_user_sessions

```rust
/// Delete all sessions for a user. Used during login for single-session enforcement.
pub async fn delete_user_sessions(pool: &SqlitePool, user_id: &str) -> Result<(), AppError>
```

- `DELETE FROM sessions WHERE user_id = ?`
- This is called by `create_session` internally, but is also exposed for use by other parts of the system (e.g., admin force-logout)

#### update_last_login

```rust
/// Update the last_login_at timestamp for a user.
pub async fn update_last_login(pool: &SqlitePool, user_id: &str) -> Result<(), AppError>
```

- `UPDATE users SET last_login_at = datetime('now') WHERE id = ?`
- Called by the login handler after successful authentication

#### log_audit

```rust
/// Write an entry to the audit log.
pub async fn log_audit(
    pool: &SqlitePool,
    user_id: &str,
    action: &str,
    entity_type: &str,
    entity_id: &str,
    ip_address: &str,
) -> Result<(), AppError>
```

- Generate a UUID v4 for the audit log entry `id`
- INSERT into `audit_log` with `user_id`, `action`, `entity_type`, `entity_id`, `ip_address`
- `old_value` and `new_value` are NULL for login/logout actions

## Key Design Decisions

1. **Token hashing with SHA-256:** The database never stores raw session tokens. If the SQLite file is exposed (e.g., backup leak), sessions cannot be hijacked. SHA-256 is sufficient here because the tokens have 256 bits of entropy (not low-entropy passwords).

2. **Single-session enforcement:** Each call to `create_session` first deletes all existing sessions for the user. This means logging in from a new browser invalidates the old session. This is a deliberate security choice for an air-gapped government environment.

3. **Generic error on duplicate email:** `create_user` catches the UNIQUE constraint violation and returns a validation error that is indistinguishable from other validation errors. This prevents attackers from enumerating registered email addresses.

4. **Datetime handling:** All timestamps are stored as ISO 8601 text strings (SQLite's `datetime('now')` format). Expiry comparisons use SQLite's built-in `datetime('now')` function in the WHERE clause, which correctly compares these strings lexicographically.

5. **No transactions for create_session:** The DELETE + INSERT in `create_session` should ideally be wrapped in a transaction to prevent a race condition where two concurrent logins both delete and both insert. For the air-gapped single-instance deployment this is extremely unlikely, but wrapping in a transaction is still recommended as good practice. Use `pool.begin()` / `tx.commit()`.

## Test Helper Setup

Within the `#[cfg(test)]` module, create a helper to set up an in-memory database:

```rust
/// Creates an in-memory SQLite pool with migrations applied.
async fn setup_test_db() -> SqlitePool {
    // SqliteConnectOptions for in-memory DB, then run migrations
}
```

Use `SqliteConnectOptions::from_str("sqlite::memory:")` and apply migrations with `sqlx::migrate!("./migrations").run(&pool).await`. This is the same pattern used in `backend/tests/common/mod.rs` but scoped to the unit test module.