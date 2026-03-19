Now I have all the context needed to write section 08.

# Section 8: Wiring and Integration

## Overview

This section assembles all middleware layers from sections 02-07 into the final application router, updates the server binding for IP-based rate limiting, gates Swagger UI behind a feature flag, and adds integration tests that verify the full middleware stack works end-to-end.

**Dependencies:** This section depends on all prior sections being complete:
- Section 01: `Config` has `behind_proxy` and `enable_https` fields
- Section 02: `permissions.rs` with `Feature`, `Action`, `Role` enums and `has_permission()`
- Section 03: `require_permission()` middleware and restructured feature routers
- Section 04: `GovernorConfig` instances for auth and API rate limiting
- Section 05: `security_headers_layer()` and `sensitive_headers_layer()` functions
- Section 06: `csrf_check()` middleware
- Section 07: `cleanup_expired_sessions()` function

## Files Modified

- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/lib.rs` -- restructure `create_router()` to apply middleware in correct order
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs` -- update serve call, gate Swagger, add session cleanup on startup
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/mod.rs` -- export new submodules
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/common/mod.rs` -- extend `create_test_app()` with new config fields
- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/wiring_tests.rs` -- new integration test file

---

## Tests

Write these tests in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/wiring_tests.rs`.

```rust
// File: backend/tests/wiring_tests.rs

mod common;

/// Test: Security headers appear on every response, including rate-limited (429) responses.
/// This validates that the security headers layer runs outermost (before rate limiting).
/// Send enough rapid requests to trigger a 429, then assert the 429 response still
/// contains X-Frame-Options: DENY and X-Content-Type-Options: nosniff.
#[tokio::test]
async fn middleware_order_headers_visible_on_rate_limited_response() {
    // Build the test app, send burst requests to /api/auth/login to trigger 429,
    // assert the 429 response includes security headers.
}

/// Test: Full end-to-end flow: register a user, log in, access a protected POST
/// endpoint, verify RBAC and CSRF both pass when proper headers and session are present.
#[tokio::test]
async fn full_flow_register_login_access_protected_route() {
    // 1. POST /api/auth/register with valid body + X-Requested-With
    // 2. POST /api/auth/login with credentials + X-Requested-With
    // 3. Extract session cookie from login response
    // 4. POST to a write-protected endpoint (e.g., create compliance assessment)
    //    with session cookie and X-Requested-With
    // 5. Assert 200 (or 201) if user role has permission
}

/// Test: CORS, credentials, and CSRF all work together on a POST request.
/// A POST with Origin header, session cookie, and X-Requested-With should succeed.
/// A POST missing X-Requested-With should get 403 even with valid session.
#[tokio::test]
async fn cors_credentials_csrf_work_together() {
    // Send POST with all three: Origin, cookie, X-Requested-With -> success
    // Send POST with Origin, cookie, but no X-Requested-With -> 403
}

/// Test: OpenAPI spec includes security scheme annotations on protected endpoints.
/// This is a compile-time / structural check — fetch /api-docs/openapi.json and
/// verify that protected endpoints have security requirements listed.
#[tokio::test]
#[cfg(feature = "swagger")]
async fn openapi_spec_reflects_security_requirements() {
    // GET /api-docs/openapi.json
    // Parse JSON, check that write endpoints have security annotations
}
```

---

## Implementation Details

### 1. Module Exports

Update `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/mod.rs` to export new submodules created by prior sections:

```rust
pub mod routes;
pub mod permissions;  // Section 02
pub mod middleware;    // Sections 03, 05, 06
```

### 2. Router Assembly in `create_router()`

Restructure the `create_router()` function in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/lib.rs`. The critical requirement is **middleware ordering**. In Axum, layers added last execute first (outermost). The desired execution order from outermost to innermost is:

```
Security Headers -> CORS -> Rate Limit -> CSRF Check -> Auth -> RBAC -> Handler
```

In Axum `.layer()` calls, this means adding them in reverse order (bottom-up):

```rust
pub fn create_router(state: AppState) -> Router {
    let enable_https = state.config.enable_https;

    // CORS layer (already exists, update to include X-Requested-With in allowed headers)
    let cors = CorsLayer::new()
        .allow_origin(/* ... existing ... */)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(vec![
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::HeaderName::from_static("x-requested-with"),
        ])
        .allow_credentials(true);

    // Build API rate limiter (from section 04)
    let api_governor_conf = /* GovernorConfig from section 04 */;

    // Auth routes with strict rate limiter (from section 04)
    let auth_governor_conf = /* strict GovernorConfig from section 04 */;
    let auth_router = features::auth::routes::router()
        .layer(GovernorLayer { config: &auth_governor_conf });

    // API routes with RBAC guards (applied per-feature in section 03)
    // and moderate rate limiter
    let api_router = Router::new()
        .route("/health", get(features::ontology::routes::health_check))
        .nest("/ontology", features::ontology::routes::router())
        .nest("/compliance", features::compliance::routes::router())
        .nest("/reports", features::reports::routes::router())
        .nest("/analyses", features::analysis::routes::router())
        .layer(GovernorLayer { config: &api_governor_conf });

    Router::new()
        .nest("/api/auth", auth_router)
        .nest("/api", api_router)
        // Layers: added bottom-up, so first .layer() is innermost
        .layer(csrf_middleware())             // innermost: CSRF check on mutations
        .layer(cors)                          // CORS
        .layer(sensitive_headers_layer())     // mask sensitive headers in logs
        .layer(security_headers_layer(enable_https)) // outermost: security response headers
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

Key points about the restructured router:
- Auth routes are nested at `/api/auth` with their own strict rate limiter
- All other API routes are nested at `/api` with a moderate rate limiter
- RBAC guards are applied **inside** each feature router via `.route_layer()` (done in section 03), not at this level
- CSRF middleware wraps everything so mutations are checked before reaching any handler
- Security headers layer is outermost so headers appear on ALL responses including 429 and 403

### 3. Update `main.rs`

Three changes to `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs`:

**a) Gate Swagger UI behind feature flag:**

```rust
#[cfg(feature = "swagger")]
use utoipa_swagger_ui::SwaggerUi;

// In main(), after create_router():
#[cfg(feature = "swagger")]
let app = app.merge(
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
);
```

The `#[derive(OpenApi)]` struct and its attribute can remain unconditionally since `utoipa` itself is not optional -- only the Swagger UI serving is gated.

**b) Use `into_make_service_with_connect_info`:**

Replace the current serve call:

```rust
// Before:
axum::serve(listener, app).await?;

// After:
axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>()
).await?;
```

This is **required** for rate limiting IP extraction. Without it, `ConnectInfo<SocketAddr>` is not available and the `PeerIpKeyExtractor` / `SmartIpKeyExtractor` silently fail, making rate limiting ineffective.

**c) Run session cleanup on startup:**

After migrations and before building the router, call the startup cleanup function from section 07:

```rust
// After migrations:
let deleted = ontology_backend::features::auth::cleanup_expired_sessions(&db).await?;
if deleted > 0 {
    tracing::info!("Cleaned up {} expired sessions on startup", deleted);
}
```

### 4. Update CORS Allowed Headers

The existing CORS configuration uses `tower_http::cors::Any` for `allow_headers`. This must be changed to an explicit list that includes the `X-Requested-With` header needed by CSRF protection. Using `Any` for allowed headers is incompatible with `allow_credentials(true)` in some browsers. The explicit list should include:
- `Content-Type`
- `Authorization`
- `x-requested-with`

### 5. Update `create_test_app()` in Tests

The test helper at `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/common/mod.rs` must be updated to include the new `Config` fields (`behind_proxy`, `enable_https`) so integration tests compile. Set both to `false` for tests. The test app should also include the full middleware stack (security headers, CSRF, rate limiting) so integration tests exercise the real middleware pipeline.

### 6. OpenAPI Security Annotations

Add `security(("session_cookie" = []))` to the `#[utoipa::path(...)]` attributes on protected endpoints. In the top-level `#[derive(OpenApi)]` block, add:

```rust
#[openapi(
    // ... existing ...
    components(schemas(...)),
    modifiers(&SecurityAddon),
)]
```

Where `SecurityAddon` is a struct implementing `utoipa::Modify` that adds a `SecurityScheme::Http` of type `Bearer` or a cookie-based scheme. This ensures the generated OpenAPI spec documents which endpoints require authentication and what permissions they need.

### 7. Compilation Verification

After wiring, verify the project compiles in both feature configurations:

- `cargo build` (default features, includes swagger) -- should compile and serve Swagger UI
- `cargo build --no-default-features` -- should compile without `utoipa-swagger-ui`, Swagger UI routes return 404

Run `cargo test` to confirm all existing tests still pass alongside the new wiring integration tests.