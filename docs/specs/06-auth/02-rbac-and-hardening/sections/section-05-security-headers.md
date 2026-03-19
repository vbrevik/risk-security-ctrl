Now I have all the context needed. Let me generate the section content.

# Section 5: Security Headers Middleware

## Overview

This section adds a tower middleware layer that sets security response headers on every HTTP response. It also marks sensitive request headers (`Authorization`, `Cookie`) so they are redacted in tracing/logging output. The HSTS header is conditionally included based on the `ENABLE_HTTPS` config flag added in section 01.

**Depends on:** section-01-deps-and-config (for `set-header` and `sensitive-headers` features on `tower-http`, and the `ENABLE_HTTPS` config field).

**Blocks:** section-08-wiring (which assembles all middleware layers in the correct order).

## Background

Security headers instruct browsers to enforce protections against clickjacking, MIME-sniffing, and other common attacks. Since this application is a governmental IT security tool, defense-in-depth headers are essential even when deployed air-gapped.

The headers to set on every response are:

| Header | Value | Purpose |
|--------|-------|---------|
| `X-Frame-Options` | `DENY` | Prevent clickjacking via iframes |
| `X-Content-Type-Options` | `nosniff` | Prevent MIME-type sniffing |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Limit referrer leakage |
| `Content-Security-Policy` | `default-src 'self'` | Restrict resource loading to same origin |
| `Permissions-Policy` | `camera=(), microphone=(), geolocation=()` | Disable unused browser features |
| `X-XSS-Protection` | `0` | Disabled per modern guidance; CSP is preferred |
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | **Only when `ENABLE_HTTPS=true`** |

Additionally, the `Authorization` and `Cookie` headers must be marked as sensitive so `tower_http::trace::TraceLayer` redacts their values from log output.

## Tests

Write these tests in `backend/tests/security_headers.rs` (integration tests) or as unit tests within the middleware module. They require building a test app with the security headers layer applied.

```rust
// File: backend/tests/security_headers.rs

// Test: Response includes X-Frame-Options: DENY
//   Build a test app with security_headers_layer applied.
//   Send any GET request. Assert response header "x-frame-options" == "DENY".

// Test: Response includes X-Content-Type-Options: nosniff
//   Same setup. Assert "x-content-type-options" == "nosniff".

// Test: Response includes Referrer-Policy header
//   Assert "referrer-policy" == "strict-origin-when-cross-origin".

// Test: Response includes Content-Security-Policy header
//   Assert "content-security-policy" == "default-src 'self'".

// Test: Response includes Permissions-Policy header
//   Assert "permissions-policy" == "camera=(), microphone=(), geolocation=()".

// Test: Response includes X-XSS-Protection: 0
//   Assert "x-xss-protection" == "0".

// Test: HSTS header present when ENABLE_HTTPS=true
//   Build test app with config.enable_https = true.
//   Assert "strict-transport-security" == "max-age=31536000; includeSubDomains".

// Test: HSTS header absent when ENABLE_HTTPS=false
//   Build test app with config.enable_https = false.
//   Assert "strict-transport-security" header is NOT present on the response.

// Test: Authorization and Cookie headers marked as sensitive (not in logs)
//   This is harder to test directly. Verify that SetSensitiveHeadersLayer
//   is constructed with the correct header names. A unit test can check
//   that the layer is included in the middleware stack by inspecting the
//   builder function's return type or by sending a request with an
//   Authorization header and confirming tracing output redacts it.
```

### Test Helper Pattern

Each test should use `axum::Router` with a simple handler, apply the security headers layer, and use `tower::ServiceExt::oneshot` to send a request without starting a server:

```rust
use axum::{Router, routing::get, body::Body};
use http::Request;
use tower::ServiceExt;

async fn test_handler() -> &'static str { "ok" }

// Example structure (do not copy verbatim — adapt to actual function signatures)
#[tokio::test]
async fn response_includes_x_frame_options() {
    let app = Router::new()
        .route("/test", get(test_handler))
        .layer(security_headers_layer(/* enable_https = */ false));

    let response = app
        .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(
        response.headers().get("x-frame-options").unwrap(),
        "DENY"
    );
}
```

## Implementation

### File: `backend/src/features/auth/middleware.rs`

This module may already exist or be created alongside CSRF and RBAC middleware (sections 03 and 06). The security headers code lives here as a public function that returns a composed layer stack.

#### Approach A: Multiple `SetResponseHeaderLayer` instances (simpler)

Use `tower_http::set_header::SetResponseHeaderLayer::overriding` for each header. Stack them with `tower::ServiceBuilder`:

```rust
use axum::http::{header, HeaderName, HeaderValue};
use tower::ServiceBuilder;
use tower_http::set_header::SetResponseHeaderLayer;
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;

/// Build the security headers middleware stack.
///
/// When `enable_https` is true, includes the HSTS header.
/// Returns a tower layer that can be applied to a Router via `.layer()`.
pub fn security_headers_layer(enable_https: bool) -> impl tower::Layer<...> + Clone {
    // Build a ServiceBuilder stacking SetResponseHeaderLayer for each header.
    // Use HeaderName::from_static / HeaderValue::from_static for zero-cost construction.
    //
    // Headers to set:
    //   X-Frame-Options: DENY
    //   X-Content-Type-Options: nosniff
    //   Referrer-Policy: strict-origin-when-cross-origin
    //   Content-Security-Policy: default-src 'self'
    //   Permissions-Policy: camera=(), microphone=(), geolocation=()
    //   X-XSS-Protection: 0
    //   Strict-Transport-Security: max-age=31536000; includeSubDomains  (conditional)
    //
    // ...
}
```

Note: The exact return type of `ServiceBuilder` with many layers is complex. You may need to return `impl Layer<S, Service = ...>` or use `tower::util::Either` / a boxed approach. An alternative is **Approach B**.

#### Approach B: Custom middleware function (recommended)

Write a single `axum::middleware::from_fn` middleware that inserts all headers in one pass. This avoids complex generic stacking:

```rust
use axum::{extract::State, middleware::Next, response::Response};
use http::{header, HeaderName, HeaderValue, Request};

/// Middleware that sets security response headers on every response.
pub async fn security_headers(
    // If you need access to config for ENABLE_HTTPS, accept State<AppState>
    // or capture `enable_https: bool` via a closure / from_fn_with_state.
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    // Insert each security header...
    // headers.insert(...)
    response
}
```

The `enable_https` flag determines whether to insert the HSTS header. Two options for passing this config:

1. **`from_fn_with_state`**: Use `axum::middleware::from_fn_with_state(state, security_headers)` and accept `State<AppState>` in the function signature.
2. **Closure capture**: Create a factory function `pub fn make_security_headers_layer(enable_https: bool)` that returns `middleware::from_fn(move |req, next| { ... })`.

### Sensitive Headers

Separately from the response header middleware, apply `SetSensitiveHeadersLayer` to redact `Authorization` and `Cookie` from tracing output. This is a request-side concern and should be layered alongside the trace layer:

```rust
use tower_http::sensitive_headers::SetSensitiveHeadersLayer;
use http::header;

// In the router builder (lib.rs create_router), add:
// .layer(SetSensitiveHeadersLayer::new([header::AUTHORIZATION, header::COOKIE]))
```

This layer wraps the request headers so that when `TraceLayer` logs them, sensitive values appear as `[redacted]`.

### Module Registration

Ensure `backend/src/features/auth/mod.rs` exports the middleware module:

```rust
pub mod middleware; // add if not already present
```

The actual wiring of the layer into the router stack is handled in section 08 (wiring). This section only creates the middleware function/layer and its tests.

## Summary Checklist

1. Add `security_headers` middleware function to `backend/src/features/auth/middleware.rs`
2. The function inserts 6 constant headers plus conditionally HSTS based on `enable_https`
3. Add `SetSensitiveHeadersLayer` construction for `Authorization` and `Cookie` (to be wired in section 08)
4. Write integration tests in `backend/tests/security_headers.rs` covering all 8 test cases listed above
5. Verify tests pass with `cargo test security_headers` from `backend/`