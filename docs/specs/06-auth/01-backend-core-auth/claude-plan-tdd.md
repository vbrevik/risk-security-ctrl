# TDD Plan: Backend Core Auth

## Testing Infrastructure

- **Framework:** Rust built-in test framework + `#[tokio::test]` for async
- **Integration tests:** `backend/tests/` directory with shared `common/mod.rs` providing `create_test_app()`
- **Unit tests:** Inline `#[cfg(test)] mod tests` within source files
- **Test DB:** In-memory SQLite via `SqliteConnectOptions` with migrations
- **HTTP testing:** `axum::test` or `tower::ServiceExt` for integration

The existing `create_test_app()` needs to be extended with a `cookie_key` field on `AppState`.

## Section 1: Dependencies and AppState — Tests

### Unit test: Config parses COOKIE_KEY from env
- Set `COOKIE_KEY` env var to a valid 64-char hex string
- Assert `Config::from_env()` captures it
- Assert decoded bytes length >= 32

### Unit test: Config rejects short COOKIE_KEY
- Set `COOKIE_KEY` to a 16-char hex string (8 bytes)
- Assert startup validation fails with clear error

### Unit test: CORS layer includes credentials
- Build the app router
- Assert CORS headers include `Access-Control-Allow-Credentials: true`

## Section 2: Auth Models — Tests

### Unit test: RegisterRequest validation rejects invalid email
- Construct `RegisterRequest` with email "notanemail"
- Call `.validate()`, assert error on `email` field

### Unit test: RegisterRequest validation rejects short password
- Construct with password "short"
- Assert validation error on `password` field (min 8)

### Unit test: LoginRequest validation rejects empty password
- Construct with empty password
- Assert validation error on `password` field

### Unit test: UserProfile serializes correctly
- Construct `UserProfile`, serialize to JSON
- Assert all fields present, no `password_hash` field

## Section 3: Password and Session Utilities — Tests

### Unit test: hash_password produces valid PHC string
- Hash "testpassword123"
- Assert result starts with `$argon2id$`
- Assert result contains salt and hash segments

### Unit test: verify_password returns true for correct password
- Hash a password, then verify with same plaintext
- Assert `Ok(true)`

### Unit test: verify_password returns false for wrong password
- Hash "password1", verify with "password2"
- Assert `Ok(false)`

### Unit test: generate_session_token produces 64-char hex
- Generate token
- Assert length == 64
- Assert all chars are hex digits

### Unit test: generate_session_token is unique
- Generate 100 tokens
- Assert all unique (HashSet size == 100)

### Unit test: hash_session_token is deterministic
- Hash same token twice
- Assert results are equal

### Unit test: hash_session_token produces different output for different input
- Hash two different tokens
- Assert results differ

## Section 4: Auth Service — Tests

### Integration test: create_user inserts user with viewer role
- Call `create_user(pool, email, hash, name)`
- Query DB, assert user exists with role="viewer", is_active=1

### Integration test: create_user rejects duplicate email
- Create user with email "a@b.com"
- Try again with same email
- Assert returns validation error (not 409)

### Integration test: find_user_by_email returns None for unknown email
- Query for "nonexistent@test.com"
- Assert `Ok(None)`

### Integration test: create_session deletes previous sessions (single-session)
- Create session for user_id "u1"
- Create another session for same user_id
- Assert only 1 session exists in DB for "u1"

### Integration test: validate_session returns None for expired token
- Insert session with expires_at in the past
- Call validate_session with the raw token
- Assert `Ok(None)`

### Integration test: validate_session returns session for valid token
- Create session, get raw token
- Call validate_session with raw token
- Assert returns the session

### Integration test: validate_session uses token hashing
- Create session (stores hash)
- Query DB directly, assert stored token != raw token (it's the hash)
- Call validate_session with raw token, assert it works

### Integration test: log_audit writes entry with ip_address
- Call `log_audit(pool, user_id, "login", "session", session_id, "127.0.0.1")`
- Query audit_log, assert entry exists with correct ip_address

## Section 5: AuthUser Extractor — Tests

### Integration test: extractor returns 401 when no auth provided
- Send GET /api/auth/me with no cookie and no Bearer header
- Assert 401

### Integration test: extractor works with Bearer header
- Create user + session, get raw token
- Send GET /api/auth/me with `Authorization: Bearer {token}`
- Assert 200 with user profile

### Integration test: extractor works with cookie
- Create user + session
- Send GET /api/auth/me with encrypted session cookie
- Assert 200

### Integration test: extractor returns 401 for expired session
- Create session with past expiry
- Send GET /api/auth/me with that token
- Assert 401

### Integration test: extractor returns 401 for inactive user
- Create user, set is_active=0
- Create session
- Send request with valid token
- Assert 401

## Section 6: Route Handlers — Tests

### Integration test: POST /api/auth/register creates user
- POST with valid email/name/password
- Assert 201 with UserProfile (no password_hash in response)

### Integration test: POST /api/auth/register rejects duplicate email
- Register "a@b.com" twice
- Assert second returns 422 (generic, no email leak)

### Integration test: POST /api/auth/login returns token and sets cookie
- Register user, then login
- Assert 200 with AuthResponse containing token
- Assert response has Set-Cookie header

### Integration test: POST /api/auth/login rejects wrong password
- Register user, login with wrong password
- Assert 401 InvalidCredentials

### Integration test: POST /api/auth/login rejects unknown email
- Login with non-existent email
- Assert 401 (same error as wrong password — no enumeration)

### Integration test: POST /api/auth/logout invalidates session
- Register, login, logout
- Try GET /api/auth/me with same token
- Assert 401

### Integration test: POST /api/auth/logout writes audit log
- Register, login, logout
- Query audit_log for "logout" action
- Assert entry exists

### Integration test: GET /api/auth/me returns current user
- Register, login
- GET /api/auth/me with token
- Assert 200 with correct email/name/role

### Integration test: full flow register → login → me → logout → me fails
- Execute full lifecycle
- Assert each step returns expected status

## Section 7: Seed-Admin Binary — Tests

### Integration test: seed-admin creates admin user
- Run seed-admin with test email/password
- Query DB, assert user with role="admin" exists

### Integration test: seed-admin is idempotent
- Run seed-admin twice with same email
- Assert no error, still one admin user

## Section 8: Wiring — Tests

### Integration test: auth routes are mounted at /api/auth
- GET /api/auth/me (unauthenticated) returns 401 (not 404)
- POST /api/auth/register returns 422 on empty body (not 404)

### Integration test: OpenAPI spec includes auth endpoints
- GET /swagger-ui or parse OpenAPI JSON
- Assert /api/auth/register, /login, /logout, /me are documented
