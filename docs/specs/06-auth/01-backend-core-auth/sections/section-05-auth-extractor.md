Now I have all the context needed. Let me generate the section content.

# Section 5: AuthUser Extractor

## Overview

This section implements the `AuthUser` Axum extractor, which provides authenticated user information to any handler that declares it as a parameter. The extractor implements `FromRequestParts<S>` and checks for authentication credentials via Bearer token header or encrypted session cookie, validates the session against the database, loads the associated user, and returns an `AuthUser` struct.

## Dependencies on Previous Sections

- **Section 1 (Dependencies and AppState):** `AppState` must have `cookie_key: Key` and `FromRef<AppState> for Key` implemented. The `axum-extra` crate with `cookie-private` feature must be available.
- **Section 2 (Auth Models):** The `AuthUser` struct must be defined in `models.rs` with fields `id`, `email`, `name`, `role`, and `session_id`. The `AppError::Unauthorized` (or equivalent 401 variant) must exist.
- **Section 4 (Auth Service):** `validate_session(pool, raw_token)` and `find_user_by_id(pool, id)` must be implemented in `service.rs`.

## File to Create/Modify

**Primary file:** `backend/src/features/auth/extractors.rs` (new file)

**Also update:** `backend/src/features/auth/mod.rs` to add `pub mod extractors;`

## Tests First

All tests for this section are **integration tests** that exercise the extractor through HTTP requests. They belong in `backend/tests/auth_tests.rs` (or a dedicated extractor test file). These tests require a running test app with the full auth stack wired up, so some will only pass after Section 6 (route handlers) is also complete. However, the extractor can be tested directly against the `/api/auth/me` endpoint since that endpoint simply returns the `AuthUser` as a `UserProfile`.

### Test: extractor returns 401 when no auth provided

```rust
/// Send GET /api/auth/me with no cookie and no Bearer header.
/// Assert 401 status code.
#[tokio::test]
async fn extractor_returns_401_when_no_auth_provided() {
    // Build test app with auth routes mounted
    // Send GET /api/auth/me with no credentials
    // Assert response status is 401
}
```

### Test: extractor works with Bearer header

```rust
/// Create a user and session directly via service functions.
/// Send GET /api/auth/me with Authorization: Bearer {raw_token}.
/// Assert 200 with correct user profile fields.
#[tokio::test]
async fn extractor_works_with_bearer_header() {
    // Insert user into DB
    // Create session via create_session(), capture raw token
    // Send GET /api/auth/me with Authorization: Bearer {token}
    // Assert 200 and response body contains correct email/name/role
}
```

### Test: extractor works with cookie

```rust
/// Create a user and session. Build an encrypted cookie using PrivateCookieJar
/// with the test app's cookie key. Send GET /api/auth/me with that cookie.
/// Assert 200.
#[tokio::test]
async fn extractor_works_with_cookie() {
    // Insert user, create session
    // Encrypt the raw token into a "session_id" cookie using the app's Key
    // Send GET /api/auth/me with Cookie header
    // Assert 200
}
```

### Test: extractor returns 401 for expired session

```rust
/// Insert a session with expires_at in the past.
/// Send request with that token.
/// Assert 401.
#[tokio::test]
async fn extractor_returns_401_for_expired_session() {
    // Insert user, create session, then UPDATE session SET expires_at to past
    // Send GET /api/auth/me with the token
    // Assert 401
}
```

### Test: extractor returns 401 for inactive user

```rust
/// Create a user, set is_active=0, create a session.
/// Send request with valid token.
/// Assert 401.
#[tokio::test]
async fn extractor_returns_401_for_inactive_user() {
    // Insert user, create session
    // UPDATE users SET is_active = 0
    // Send GET /api/auth/me with the token
    // Assert 401
}
```

## Implementation Details

### The `FromRequestParts` Implementation

Create `backend/src/features/auth/extractors.rs` with a single `impl` block:

```rust
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::extract::cookie::PrivateCookieJar;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;

use crate::AppState;
use crate::error::AppError;
use super::models::AuthUser;
use super::service;
```

Implement `FromRequestParts<AppState>` for `AuthUser`. The extraction logic proceeds as follows:

1. **Extract state** -- Get a reference to `AppState` from the request parts. This provides the DB pool and cookie key.

2. **Try Bearer header first** -- Attempt to extract `TypedHeader<Authorization<Bearer>>` from the parts. Use the `from_request_parts` method with rejection handling (do not unwrap; a missing header is not an error at this stage). If present, the raw token is `bearer.token().to_string()`.

3. **Fall back to cookie** -- If no Bearer header, extract `PrivateCookieJar` from the parts (this uses `FromRef<AppState> for Key` to decrypt). Look for a cookie named `"session_id"`. If found, its value is the raw session token.

4. **No credentials** -- If neither Bearer header nor cookie yields a token, return `Err(AppError::Unauthorized)`.

5. **Validate session** -- Call `service::validate_session(&state.db, &token).await`. This hashes the raw token with SHA-256 and queries the sessions table. If `None` is returned (token not found or expired), return `Err(AppError::Unauthorized)`.

6. **Load user** -- Call `service::find_user_by_id(&state.db, &session.user_id).await`. If `None` is returned, or if the user's `is_active` field is `0` (or `false`), return `Err(AppError::Unauthorized)`.

7. **Return AuthUser** -- Construct and return `AuthUser { id: user.id, email: user.email, name: user.name, role: user.role, session_id: session.id }`.

### Bearer vs Cookie Priority

When both a Bearer header and a session cookie are present in the same request, the **Bearer header takes precedence**. This allows API clients (scripts, CLI tools) to override any browser cookie that might be lingering. The implementation achieves this naturally by checking the Bearer header first and only falling back to the cookie.

### Extracting the Bearer Token Without Hard Failure

The Bearer header extraction must not cause a rejection if the header is absent. Use `TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await` and match on the `Result`. An `Err` simply means the header is not present -- proceed to cookie extraction.

Alternatively, extract the raw `Authorization` header string manually and parse it, but the `TypedHeader` approach is cleaner and type-safe.

### PrivateCookieJar Extraction

`PrivateCookieJar` requires `Key: FromRef<S>`. Since Section 1 implements `FromRef<AppState> for Key`, the jar can be extracted in the same `FromRequestParts` impl. Call `PrivateCookieJar::from_request_parts(parts, state).await` and then `jar.get("session_id")` to retrieve the decrypted cookie value. The cookie name `"session_id"` must match what the login handler sets (Section 6).

### Error Type

The extractor's rejection type is `AppError`. Since `AppError` already implements `IntoResponse` (returning JSON error bodies with appropriate status codes), all 401 responses from the extractor will be well-formatted JSON.

The associated types for the `FromRequestParts` impl:

```rust
type Rejection = AppError;
```

### Module Registration

Update `backend/src/features/auth/mod.rs` to include:

```rust
pub mod extractors;
pub mod models;
pub mod routes;
pub mod service;
```

The `AuthUser` type is defined in `models.rs` (Section 2) and re-exported or imported as needed. The extractor in `extractors.rs` is what handlers use when they write `AuthUser` as a function parameter. Axum resolves it automatically via the `FromRequestParts` trait.

### Cookie Name Constant

Consider defining the cookie name as a constant shared between the extractor and the login handler (Section 6):

```rust
pub const SESSION_COOKIE_NAME: &str = "session_id";
```

This can live in `models.rs` or `extractors.rs` and be imported by `routes.rs`.

## Key Design Decisions

- **No caching:** Each request re-validates the session against the database. This is acceptable for an air-gapped single-instance deployment and ensures immediate session invalidation on logout.
- **No role checking in the extractor:** The extractor only confirms the user is authenticated. Role-based access control is deferred to a later spec (02-rbac-and-hardening).
- **session_id in AuthUser:** The logout handler needs `session_id` to delete the correct session row and write the audit log. Including it in `AuthUser` avoids a redundant DB lookup in the logout handler.