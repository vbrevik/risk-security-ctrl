# Implementation Plan: Backend Core Auth

## Context

Risk-security-ctrl is a Rust/Axum web application for governmental IT security compliance. It manages ontology frameworks (ISO 31000, NIST CSF, etc.), compliance assessments, and document analysis. The application runs in an air-gapped environment.

The database already has `users`, `sessions`, and `audit_log` tables with full indexes. The backend has a placeholder auth module at `src/features/auth/`. This plan implements the foundational auth system that all other features will depend on.

## Architecture Overview

The auth system has five components:

1. **Auth models** — request/response types, password hashing, session token generation
2. **Auth service layer** — user CRUD, session management, credential verification
3. **AuthUser extractor** — Axum `FromRequestParts` that extracts authenticated user from cookie or Bearer header
4. **Auth routes** — HTTP handlers for register, login, logout, and current user
5. **Seed-admin CLI** — separate binary for bootstrapping the first admin user

Data flows through: HTTP request → route handler → service function → SQLx query → SQLite.

Authentication state flows: Login → session created in DB + cookie set → subsequent requests extract session from cookie/header → validate against DB → load user.

## Section 1: Dependencies and AppState Updates

### New Cargo Dependencies

Add to `backend/Cargo.toml`:
- `argon2 = "0.5"` — RustCrypto Argon2id password hashing. OWASP-recommended defaults (19 MiB memory, 2 iterations). Pure Rust, no network calls.
- `axum-extra = { version = "0.10", features = ["cookie-private"] }` — AES-GCM encrypted cookies. Maintained by Tokio team.
- `validator = { version = "0.20", features = ["derive"] }` — Input validation with derive macros.
- `rand = "0.9"` — CSPRNG for session token generation via OS entropy.
- `sha2 = "0.10"` — SHA-256 for session token hashing before DB storage. Explicit dep (do not rely on argon2 transitive).
- `hex = "0.4"` — Decode COOKIE_KEY from hex-encoded environment variable.
- `time = "0.3"` — Required by cookie crate for `max_age` duration type. Only used for cookie durations; all other datetime work uses chrono.

**Version compatibility:** Verify that `axum-extra` version is compatible with `axum 0.7`. These version numbers do not move in lockstep — pin the correct compatible version.

### AppState Changes

The existing `AppState` struct needs two new fields:

- `cookie_key: axum_extra::extract::cookie::Key` — encryption key for `PrivateCookieJar`
- Config values for session duration (default 8 hours)

Implement `FromRef<AppState> for Key` so that `PrivateCookieJar` can access the encryption key from any handler.

### Cookie Key Initialization

On server startup:

1. Check `COOKIE_KEY` env var (hex-encoded, minimum 32 bytes / 64 hex chars)
2. **Validate minimum length** — decoded bytes must be >= 32. Reject shorter keys with a clear startup error.
3. If not set, read from `.cookie_key` file in working directory
4. If file doesn't exist, generate 32 random bytes, save as hex to `.cookie_key`, **set file permissions to 0600 (Unix)**, log a warning
5. If file exists, **warn if permissions are more open than 0600**
6. Call `Key::derive_from(&master_key_bytes)` to produce signing + encryption keys via HKDF

Add `.cookie_key` to `.gitignore`.

### Config Updates

Add to `Config` struct:
- `COOKIE_KEY` — optional, hex string
- `SESSION_DURATION_HOURS` — optional, defaults to 8

### CORS Configuration Update

**Critical for cookie auth:** The existing CORS config uses `allow_origin(Any)`, which prevents browsers from sending cookies cross-origin. Update `src/lib.rs`:

- Replace `allow_origin(Any)` with `allow_origin(frontend_url)` from config (default `http://localhost:5173`)
- Add `allow_credentials(true)` to enable cookie transmission
- Keep `allow_methods` and `allow_headers` as needed for the API

Without this, cookie-based authentication will silently fail in any cross-origin deployment.

## Section 2: Auth Models and Types

### Request Types

```rust
struct RegisterRequest {
    email: String,      // #[validate(email)]
    name: String,       // #[validate(length(min = 1, max = 255))]
    password: String,   // #[validate(length(min = 8))]
}

struct LoginRequest {
    email: String,      // #[validate(email)]
    password: String,   // #[validate(length(min = 1))]
}
```

Both derive `Deserialize`, `Validate`, and `ToSchema` (utoipa).

### Response Types

```rust
struct AuthResponse {
    token: String,
    user: UserProfile,
}

struct UserProfile {
    id: String,
    email: String,
    name: String,
    role: String,
}
```

Derive `Serialize` and `ToSchema`.

### AuthUser (Extractor Output)

```rust
struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub session_id: String,
}
```

This is what handlers receive when authentication succeeds. It's populated from the database after session validation. The `session_id` is needed by the logout handler to delete the correct session and write the audit log entry.

### Error Variants

Extend `AppError` in `src/error.rs`:
- `InvalidCredentials` → 401 with message "Invalid credentials" (intentionally vague)
- `ValidationError(Vec<FieldError>)` → 422 with field-level errors
- `SessionExpired` → 401 with message "Session expired"

The generic 422 response for duplicate emails is important: the register endpoint must NOT reveal whether an email is already registered. Return the same validation error shape regardless of whether the email exists.

**Validator error conversion:** Implement `From<validator::ValidationErrors> for AppError` to convert the nested field-error structure into the flat `Vec<FieldError>` format used by the API's error response.

## Section 3: Password and Session Utilities

### Password Hashing

Create a `password` module (or functions within the auth service):

- `hash_password(plain: &str) -> Result<String>` — generates a salt via `SaltString::generate(&mut OsRng)`, hashes with `Argon2::default()` (Argon2id, OWASP defaults), returns the PHC-encoded string.
- `verify_password(plain: &str, phc_hash: &str) -> Result<bool>` — parses the stored PHC string, verifies with `Argon2::default().verify_password()`. Returns `Ok(true)` on match, `Ok(false)` on mismatch. Never panics.

The PHC string format (`$argon2id$v=19$m=19456,t=2,p=1$<salt>$<hash>`) is self-describing — salt and parameters are embedded, so no separate storage is needed.

### Session Token Generation

- `generate_session_token() -> String` — generates 32 random bytes via `rand::rngs::OsRng`, returns as 64-char hex string. This gives 256-bit entropy.

### Session Token Hashing

- `hash_session_token(token: &str) -> String` — computes SHA-256 of the raw token, returns as hex string. The database stores the **hash**, not the raw token. On lookup, the presented token is hashed before the `WHERE` clause. This prevents session hijacking from database file exposure.

Use `sha2` crate (already a transitive dependency of `argon2`).

## Section 4: Auth Service (Database Operations)

Create `src/features/auth/service.rs` with functions that take `&SqlitePool` and perform database operations.

**Transaction boundaries:** Multi-step operations (especially `create_session` which deletes + inserts, and the login flow which spans multiple tables) must use `sqlx::Transaction` or `pool.begin()` to ensure atomicity. If the app crashes between DELETE and INSERT, the user would be left with no valid session.

**DateTime format:** Use `datetime('now')` in SQL for all timestamp operations. This matches the existing migration convention and ensures consistent ISO 8601 comparison in SQLite.

### User Operations

- `create_user(pool, email, password_hash, name) -> Result<UserProfile>` — INSERT into `users` table with generated UUID, role=viewer, is_active=1. On UNIQUE constraint violation (email exists), return a generic validation error (not 409) to prevent account enumeration.
- `find_user_by_email(pool, email) -> Result<Option<User>>` — SELECT from users. Returns the full user row including password_hash for credential verification.
- `find_user_by_id(pool, id) -> Result<Option<User>>` — SELECT from users. Used by the auth extractor to load user from session.

### Session Operations

- `create_session(pool, user_id, ip, user_agent) -> Result<(Session, String)>` — First DELETE any existing sessions for this user_id (single-session enforcement). Generate CSPRNG token, hash it with SHA-256, INSERT new session with generated UUID, **hashed** token, expires_at = now + 8h, ip_address, and user_agent. Returns the session and the **raw** token (for cookie/response). The raw token is never stored.
- `validate_session(pool, raw_token) -> Result<Option<Session>>` — Hash the raw token with SHA-256, then SELECT from sessions WHERE token = hashed_token AND expires_at > now. Returns None if not found or expired. If found but expired, DELETE the expired session.
- `delete_session(pool, session_id) -> Result<()>` — DELETE from sessions WHERE id = ?.
- `delete_user_sessions(pool, user_id) -> Result<()>` — DELETE all sessions for a user. Used during login to enforce single-session.

### Audit Operations

- `log_audit(pool, user_id, action, entity_type, entity_id, ip_address) -> Result<()>` — INSERT into `audit_log` including `ip_address` column. Actions: "login", "logout". Entity type: "session".

## Section 5: AuthUser Extractor

Implement `FromRequestParts<S>` for `AuthUser` where `AppState: FromRef<S>`:

1. Extract `AppState` via `FromRef`
2. Try `Authorization: Bearer <token>` header first (using `axum_extra::TypedHeader<Authorization<Bearer>>`)
3. If no header, try `PrivateCookieJar` → get "session" cookie value
4. If neither present: return `Err(AppError::Unauthorized)`
5. Call `validate_session(pool, token)` — if None, return 401
6. Call `find_user_by_id(pool, session.user_id)` — if None or is_active=0, return 401
7. Return `AuthUser { id, email, name, role, session_id: session.id }`

**Important:** The `PrivateCookieJar` uses `FromRef<S>` to get the `Key`. The extractor needs both the DB pool and the cookie key from state.

**Bearer header preference:** When both cookie and header are present, prefer the header. This allows API clients to override the cookie-based session.

## Section 6: Route Handlers

Replace the placeholder in `src/features/auth/routes.rs`.

### Router Setup

Create a router with four routes mounted at `/api/auth`:
- POST `/register` → `register_handler`
- POST `/login` → `login_handler`
- POST `/logout` → `logout_handler`
- GET `/me` → `me_handler`

All handlers receive `State<AppState>`.

### register_handler

1. Parse and validate `RegisterRequest` body (validator crate)
2. Call `hash_password(request.password)`
3. Call `create_user(pool, email, hash, name)`
4. Return 201 with `UserProfile`
5. On duplicate email: return generic 422 `{ "error": "validation_error", "message": "Registration failed" }` — same shape as other validation errors

### login_handler

1. Parse `LoginRequest` body
2. Call `find_user_by_email(pool, email)`
3. If None: return 401 InvalidCredentials
4. Call `verify_password(request.password, user.password_hash)`
5. If false: return 401 InvalidCredentials
6. If user.is_active == 0: return 403 Forbidden
7. Extract client IP via `ConnectInfo<SocketAddr>` and User-Agent header. Note: in reverse proxy setups, ConnectInfo gives the proxy IP — consider X-Forwarded-For if applicable
8. Call `create_session(pool, user.id, ip, user_agent)` (this deletes previous sessions)
9. Call `log_audit(pool, user.id, "login", "session", session.id)`
10. Update `last_login_at` on user record
11. Build encrypted cookie with session token
12. Return `(jar.add(cookie), Json(AuthResponse { token, user }))`

**Cookie settings:** path="/", http_only=true, secure=!cfg!(debug_assertions), same_site=Lax, max_age=8h.

**Token in response body tradeoff:** The raw session token is intentionally returned in the JSON `AuthResponse`. This enables Bearer-header API clients (scripts, integrations) that don't use cookies. For the SPA, the httpOnly cookie provides XSS protection regardless of the JSON body. This is a deliberate design decision accepting that API clients will handle token storage themselves.

### logout_handler

1. `AuthUser` extractor provides authenticated user
2. Look up current session by token (from cookie or header)
3. Call `delete_session(pool, session.id)`
4. Call `log_audit(pool, user.id, "logout", "session", session.id)`
5. Return `(jar.remove(Cookie::from("session")), StatusCode::NO_CONTENT)`

### me_handler

1. `AuthUser` extractor provides authenticated user
2. Return `Json(UserProfile { id, email, name, role })`

### OpenAPI Annotations

All handlers get `#[utoipa::path(...)]` annotations with:
- Request/response schemas
- Security requirement tags
- Appropriate status codes and error responses

Register the auth paths in the OpenAPI doc builder in `src/lib.rs`.

## Section 7: Seed-Admin Binary

Add to `Cargo.toml`:
```toml
[[bin]]
name = "seed-admin"
path = "src/bin/seed_admin.rs"
```

The binary:
1. Loads `DATABASE_URL` from env/dotenvy
2. Reads `ADMIN_EMAIL` (required), `ADMIN_PASSWORD` (required), `ADMIN_NAME` (optional, defaults to "Admin")
3. Connects to SQLite database
4. Checks if a user with that email already exists — if yes, print warning and exit 0
5. Hashes password with Argon2id
6. Inserts user with role=admin, is_active=1
7. Prints "Admin user created: {email} (id: {uuid})"

Uses the same password hashing and database utilities as the main application — import from the library crate.

## Section 8: Wiring and Integration

### Module Structure

```
src/features/auth/
  mod.rs          — pub mod routes, models, service, password
  models.rs       — request/response types, AuthUser struct
  routes.rs       — handler functions and router
  service.rs      — database operations (user, session, audit)
  password.rs     — hash_password, verify_password, generate_session_token
```

### Router Integration

In `src/lib.rs`, replace the existing auth route nesting:
- Remove old: `.nest("/auth", features::auth::routes::router())`
- Add new: `.nest("/auth", features::auth::routes::router())`

The router function signature remains the same but now returns a fully implemented router.

### OpenAPI Integration

Add auth path structs to the utoipa `OpenApi` derive in `src/lib.rs`:
- Register, Login, Logout, Me endpoints
- Request/response schemas
- Add security scheme definition (Bearer token + Cookie)

## Edge Cases and Error Handling

- **Concurrent registration with same email:** SQLx UNIQUE constraint handles this. Catch the constraint violation error and return generic 422.
- **Session token collision:** 256-bit CSPRNG tokens make collision probability negligible (~2^-128 for birthday bound). No retry logic needed.
- **Database connection failure during auth:** Let SQLx errors propagate through AppError. Existing 500 handling applies.
- **Cookie key file permissions:** The `.cookie_key` file should be readable only by the application user. Log a warning if permissions are too open (Unix only).
- **Missing COOKIE_KEY in prod:** If both env var and file are absent, auto-generate and warn. This is safe for single-node deploys but would break multi-node (not applicable for air-gapped single instance).

## Known Gaps (Deferred)

- **Rate limiting:** Login/register have no rate limiting. Deferred to 02-rbac-and-hardening or reverse proxy layer.
- **Session garbage collection:** Expired sessions cleaned on lookup only. No periodic cleanup task. Acceptable for single-instance air-gapped deploy.
- **Open registration:** Anyone can register as viewer. This is per spec. Admin-only user creation is a potential follow-up.
- **Single-session enforcement:** New login invalidates previous session. This is per spec but may surprise multi-tab users.
