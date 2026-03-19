Now I have all the context I need. Let me generate the section content.

# Section 8: Wiring and Integration

## Overview

This final section connects all auth components together. It updates the auth module structure (`mod.rs`), ensures the router is properly mounted at `/api/auth`, registers auth endpoints in the OpenAPI spec, and updates the test helper to support the new `AppState` fields. This section assumes all previous sections (01 through 07) are complete.

## Dependencies

- **Section 01** (Dependencies and AppState): `AppState` now has `cookie_key` field
- **Section 02** (Auth Models): All request/response types and `AuthUser` exist
- **Section 03** (Password and Session Utils): `password.rs` module exists
- **Section 04** (Auth Service): `service.rs` module exists
- **Section 05** (Auth Extractor): `AuthUser` extractor is implemented
- **Section 06** (Route Handlers): `routes.rs` has the full router with register/login/logout/me
- **Section 07** (Seed-Admin Binary): `src/bin/seed_admin.rs` exists

## Tests First

All tests go in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/auth_tests.rs`.

### Integration test: auth routes are mounted at /api/auth

Verify routes return auth-specific errors (401 or 422) rather than 404, which would indicate the routes are not mounted.

```rust
/// Test that GET /api/auth/me returns 401 (not 404) when unauthenticated.
/// A 404 would mean the route is not wired up at all.
#[tokio::test]
async fn auth_me_returns_401_not_404() {
    // Build test app via create_test_app()
    // Send GET /api/auth/me with no auth headers
    // Assert status == 401
}

/// Test that POST /api/auth/register returns 422 on empty body (not 404).
/// This confirms the register route is mounted and reachable.
#[tokio::test]
async fn auth_register_returns_422_on_empty_body() {
    // Build test app via create_test_app()
    // Send POST /api/auth/register with empty JSON body "{}"
    // Assert status == 422 (validation error)
}
```

### Integration test: OpenAPI spec includes auth endpoints

```rust
/// Test that the OpenAPI JSON includes all four auth endpoints.
#[tokio::test]
async fn openapi_includes_auth_endpoints() {
    // GET /api-docs/openapi.json
    // Parse response as JSON
    // Assert paths object contains:
    //   "/api/auth/register"
    //   "/api/auth/login"
    //   "/api/auth/logout"
    //   "/api/auth/me"
}
```

### Integration test: full lifecycle register -> login -> me -> logout -> me fails

```rust
/// End-to-end auth lifecycle: register, login, access protected route, logout, verify access revoked.
#[tokio::test]
async fn full_auth_lifecycle() {
    // 1. POST /api/auth/register with valid data -> assert 201
    // 2. POST /api/auth/login with same credentials -> assert 200, extract token from body
    // 3. GET /api/auth/me with Authorization: Bearer {token} -> assert 200, verify user fields
    // 4. POST /api/auth/logout with Authorization: Bearer {token} -> assert 204
    // 5. GET /api/auth/me with same token -> assert 401
}
```

## Implementation Details

### 1. Update Auth Module File: `backend/src/features/auth/mod.rs`

The current file only exposes `pub mod routes;`. It must be updated to expose all four submodules created in previous sections.

```rust
pub mod models;
pub mod password;
pub mod routes;
pub mod service;
```

This is the complete content. The module re-exports are not needed since consumers use the full path (`features::auth::service::create_user`, etc.).

### 2. Router Integration in `backend/src/lib.rs`

The existing `api_routes()` function already has `.nest("/auth", features::auth::routes::router())` at line 88. No change is needed to the route nesting itself because the `router()` function signature in `routes.rs` was kept the same (returns `Router<AppState>`), but its implementation was replaced in Section 06 to return the four-route auth router instead of the placeholder.

However, `create_router` must be updated:

- The `CorsLayer` currently uses `allow_origin(Any)`. This was addressed in Section 01 (CORS update), but verify it is wired: the CORS layer should read `state.config.frontend_url` and use `allow_credentials(true)`.
- The `create_router` function signature and its use of `with_state(state)` remain unchanged.

Confirm that `create_router` in `src/lib.rs` looks like this after Section 01's changes:

```rust
pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(state.config.frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(true);

    Router::new()
        .nest("/api", api_routes())
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

### 3. OpenAPI Registration in `backend/src/main.rs`

The `ApiDoc` struct at the top of `main.rs` uses a `#[derive(OpenApi)]` macro with a `paths(...)` list. Add the four auth handler paths to this list. The tag `"auth"` is already defined in the `tags` block (line 34).

Add to the `paths(...)` attribute:

```
ontology_backend::features::auth::routes::register_handler,
ontology_backend::features::auth::routes::login_handler,
ontology_backend::features::auth::routes::logout_handler,
ontology_backend::features::auth::routes::me_handler,
```

Also add the auth model schemas to a `components(schemas(...))` attribute if not already present:

```
components(
    schemas(
        ontology_backend::features::auth::models::RegisterRequest,
        ontology_backend::features::auth::models::LoginRequest,
        ontology_backend::features::auth::models::AuthResponse,
        ontology_backend::features::auth::models::UserProfile,
    )
)
```

Add a security scheme for Bearer auth:

```
modifiers(&SecurityAddon)
```

Where `SecurityAddon` is a small struct implementing `utoipa::Modify`:

```rust
struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Add Bearer token security scheme
        // Add Cookie security scheme
    }
}
```

This struct should be defined in `main.rs` right above or below the `ApiDoc` struct. It adds two security schemes: `bearer_auth` (HTTP Bearer) and `cookie_auth` (ApiKey in cookie named "session_id").

### 4. Update Test Helper: `backend/tests/common/mod.rs`

The existing `create_test_app()` constructs `AppState` with `{ db, config, topics }`. After Section 01, `AppState` has a `cookie_key` field. The test helper must be updated to include it.

Generate a deterministic test key:

```rust
use axum_extra::extract::cookie::Key;

let state = AppState {
    db: pool,
    config: config.clone(),
    topics,
    cookie_key: Key::generate(), // Random key, fine for tests
};
```

This is required for all integration tests to compile, not just auth tests. If this was already done in Section 01, no additional change is needed here, but verify it.

### 5. Add `.cookie_key` to `.gitignore`

Append to `/Users/vidarbrevik/projects/risk-security-ctrl/backend/.gitignore` (or the root `.gitignore`):

```
# Auth cookie encryption key (auto-generated)
.cookie_key
```

This prevents accidental commit of the generated key file.

### 6. Verify the SwaggerUI merge in `main.rs`

The existing `main.rs` line 160 already merges SwaggerUI:

```rust
.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
```

No change needed here. The auth endpoints will appear automatically once they are added to the `paths(...)` attribute.

## Files Modified

| File | Action |
|------|--------|
| `backend/src/features/auth/mod.rs` | Update to expose `models`, `password`, `service`, `routes` |
| `backend/src/main.rs` | Add auth paths and schemas to `ApiDoc`, add `SecurityAddon` modifier |
| `backend/tests/common/mod.rs` | Add `cookie_key` to test `AppState` (if not done in Section 01) |
| `backend/tests/auth_tests.rs` | New file with wiring integration tests |
| `.gitignore` | Add `.cookie_key` entry |

## Checklist

1. Update `backend/src/features/auth/mod.rs` to declare all four submodules
2. Add auth handler paths to the `ApiDoc` `paths(...)` list in `main.rs`
3. Add auth model schemas to `ApiDoc` `components(schemas(...))` in `main.rs`
4. Add `SecurityAddon` modifier struct to `main.rs` for Bearer and Cookie security schemes
5. Ensure `create_test_app()` in `tests/common/mod.rs` includes `cookie_key` on `AppState`
6. Add `.cookie_key` to `.gitignore`
7. Write and run the three integration test groups: route mounting, OpenAPI spec, full lifecycle
8. Run `cargo test` to confirm all tests pass
9. Run `cargo clippy` to confirm no warnings
10. Run the server and visit `/swagger-ui` to visually confirm auth endpoints appear