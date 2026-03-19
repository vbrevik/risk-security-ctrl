Now I have all the context needed. Let me produce the section content.

# Section 6: Route Handlers

## Overview

This section implements the four HTTP handler functions for the auth API: `register_handler`, `login_handler`, `logout_handler`, and `me_handler`. These are mounted at `/api/auth` and compose the service layer (Section 4), extractor (Section 5), password utilities (Section 3), and models (Section 2) into user-facing endpoints.

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/routes.rs`

**Dependencies (must be completed first):**
- Section 1: `AppState` with `cookie_key`, `Config` with `session_duration_hours`
- Section 2: `RegisterRequest`, `LoginRequest`, `AuthResponse`, `UserProfile`, `AuthUser`, `AppError` variants
- Section 3: `hash_password`, `verify_password` functions
- Section 4: `create_user`, `find_user_by_email`, `create_session`, `delete_session`, `log_audit` service functions
- Section 5: `AuthUser` extractor (used by `logout_handler` and `me_handler`)

## Tests First

All tests are integration tests that exercise the full HTTP stack. They belong in a new test file at `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/auth_tests.rs`.

The tests rely on the `create_test_app()` helper from `backend/tests/common/mod.rs`, which will need the `cookie_key` field on `AppState` (added in Section 1). Tests use `tower::ServiceExt` to send requests to the router without a running server.

### Test: POST /api/auth/register creates user

- Send POST `/api/auth/register` with JSON body `{ "email": "test@example.com", "name": "Test User", "password": "password123" }`
- Assert status 201
- Assert response body contains `id`, `email`, `name`, `role` fields
- Assert `role` equals `"viewer"`
- Assert no `password_hash` field in the response

### Test: POST /api/auth/register rejects duplicate email

- Register `"dup@example.com"` once (assert 201)
- Register `"dup@example.com"` again
- Assert second response is 422
- Assert response body has `"error": "validation_error"` -- generic message, must NOT reveal that the email already exists

### Test: POST /api/auth/login returns token and sets cookie

- Register a user, then POST `/api/auth/login` with correct credentials
- Assert status 200
- Assert response body contains `token` (string) and `user` (object with profile fields)
- Assert response includes a `Set-Cookie` header containing `session_id`

### Test: POST /api/auth/login rejects wrong password

- Register a user, then login with the wrong password
- Assert 401
- Assert response body has `"error": "invalid_credentials"` with message `"Invalid credentials"`

### Test: POST /api/auth/login rejects unknown email

- Login with `"nobody@example.com"` (never registered)
- Assert 401 with the same error shape as wrong password -- no account enumeration

### Test: POST /api/auth/logout invalidates session

- Register, login (capture the token), then POST `/api/auth/logout` with `Authorization: Bearer {token}`
- Then GET `/api/auth/me` with the same token
- Assert the `/me` call returns 401

### Test: POST /api/auth/logout writes audit log

- Register, login, logout
- Query `audit_log` table directly for action `"logout"`
- Assert an entry exists with the correct `user_id`

### Test: GET /api/auth/me returns current user

- Register, login (capture token)
- GET `/api/auth/me` with `Authorization: Bearer {token}`
- Assert 200
- Assert response body contains the correct `email`, `name`, and `role`

### Test: Full lifecycle register, login, me, logout, me-fails

- POST `/api/auth/register` -- assert 201
- POST `/api/auth/login` -- assert 200, capture token
- GET `/api/auth/me` with token -- assert 200
- POST `/api/auth/logout` with token -- assert 204
- GET `/api/auth/me` with same token -- assert 401

Each test should use a unique email address (e.g., include a UUID or test-name prefix) to avoid cross-test interference.

## Implementation Details

### Router Setup

Replace the entire contents of `routes.rs`. The placeholder `me()` handler and `PlaceholderResponse` struct are removed. The new `router()` function returns a `Router<AppState>` with four routes:

```rust
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/me", get(me_handler))
}
```

All handlers take `State<AppState>` as the first parameter. The `logout_handler` and `me_handler` additionally take `AuthUser` as an extractor parameter (which triggers authentication).

### register_handler

Signature: `async fn register_handler(State(state): State<AppState>, Json(body): Json<RegisterRequest>) -> Result<impl IntoResponse, AppError>`

Steps:
1. Call `body.validate()` (from the `validator` crate). On failure, convert field errors into `AppError::ValidationError`.
2. Call `hash_password(&body.password)` to produce the Argon2id PHC string.
3. Call `service::create_user(&state.db, &body.email, &hash, &body.name)`. On UNIQUE constraint violation from SQLx, return `AppError::ValidationError` with a generic `"Registration failed"` message -- never reveal the email is taken.
4. Return `(StatusCode::CREATED, Json(user_profile))`.

### login_handler

Signature: `async fn login_handler(State(state): State<AppState>, jar: PrivateCookieJar, headers: HeaderMap, Json(body): Json<LoginRequest>) -> Result<impl IntoResponse, AppError>`

The handler receives `PrivateCookieJar` so it can set the encrypted session cookie on the response. The `PrivateCookieJar` extractor automatically reads the `Key` from `AppState` via the `FromRef` impl (set up in Section 1).

Steps:
1. Call `service::find_user_by_email(&state.db, &body.email)`. If `None`, return `AppError::InvalidCredentials`.
2. Call `verify_password(&body.password, &user.password_hash)`. If `false`, return `AppError::InvalidCredentials`.
3. If `user.is_active == 0`, return `AppError::Forbidden`.
4. Extract client IP from `x-forwarded-for` header or `ConnectInfo`, falling back to `"unknown"`. Extract `User-Agent` header value.
5. Call `service::create_session(&state.db, &user.id, &ip, &user_agent)`. This returns `(session, raw_token)`. The raw token is what goes in the cookie and response; the database stores only the SHA-256 hash.
6. Call `service::log_audit(&state.db, &user.id, "login", "session", &session.id, &ip)`.
7. Update `last_login_at` on the user record: `UPDATE users SET last_login_at = datetime('now') WHERE id = ?`.
8. Build the cookie:
   - Name: `"session_id"`
   - Value: the raw token
   - Path: `"/"`
   - HttpOnly: `true`
   - Secure: `!cfg!(debug_assertions)` (false in dev, true in release)
   - SameSite: `Lax`
   - MaxAge: session duration from config (default 8 hours), using `time::Duration::hours()`
9. Return `(jar.add(cookie), Json(AuthResponse { token: raw_token, user: user_profile }))`.

The response tuple includes the updated `PrivateCookieJar` which Axum automatically converts into `Set-Cookie` headers.

### logout_handler

Signature: `async fn logout_handler(State(state): State<AppState>, jar: PrivateCookieJar, auth_user: AuthUser) -> Result<impl IntoResponse, AppError>`

The `AuthUser` extractor validates the session and provides `session_id`.

Steps:
1. Call `service::delete_session(&state.db, &auth_user.session_id)`.
2. Call `service::log_audit(&state.db, &auth_user.id, "logout", "session", &auth_user.session_id, "")`. IP can be extracted from headers if desired, or left empty for logout.
3. Remove the cookie: `jar.remove(Cookie::from("session_id"))`.
4. Return `(jar, StatusCode::NO_CONTENT)`.

### me_handler

Signature: `async fn me_handler(auth_user: AuthUser) -> Json<UserProfile>`

This is the simplest handler. The `AuthUser` extractor does all the work. Convert `AuthUser` fields into a `UserProfile` and return as JSON. No database call needed since the extractor already loaded the user.

### OpenAPI Annotations

Each handler function gets a `#[utoipa::path(...)]` attribute. Key elements for each:

- **register**: `post`, path `/api/auth/register`, request_body `RegisterRequest`, responses `(status = 201, body = UserProfile)` and `(status = 422)`
- **login**: `post`, path `/api/auth/login`, request_body `LoginRequest`, responses `(status = 200, body = AuthResponse)` and `(status = 401)`
- **logout**: `post`, path `/api/auth/logout`, responses `(status = 204)` and `(status = 401)`, security tag `("bearer" = [])`
- **me**: `get`, path `/api/auth/me`, responses `(status = 200, body = UserProfile)` and `(status = 401)`, security tag `("bearer" = [])`

The security tags are used by the Swagger UI to show which endpoints require authentication. The actual auth paths are registered in the OpenAPI doc builder in Section 8.

### Validation Error Handling

When the `validator` crate returns errors, they need to be converted into the `AppError::ValidationError` variant. A helper function or `From` impl should map `validator::ValidationErrors` into the `Vec<FieldError>` structure expected by the error type. Each field error should include the field name and the validation code (e.g., `"email"`, `"length"`).

This conversion can be a standalone function in `routes.rs` or as part of the error module. The key requirement is that the 422 response for validation errors and the 422 response for duplicate emails must have the **same JSON shape** so the client cannot distinguish between them.

### Cookie and Header Imports

The handler file needs these key imports from `axum_extra`:
- `axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite}`

And from `time`:
- `time::Duration` for the cookie `max_age`

### IP Address Extraction

For audit logging, extract the client IP with a utility function. Check headers in order: `x-forwarded-for` (first value before comma), `x-real-ip`, then fall back to `"unknown"`. This is a simple helper within `routes.rs` that takes `&HeaderMap` and returns a `String`.