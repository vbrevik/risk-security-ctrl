Now I have all the context needed. Let me produce the section content.

# Section 6: CSRF Protection Middleware

## Overview

This section implements a custom Axum/Tower middleware that validates the presence of an `X-Requested-With: XMLHttpRequest` header on all state-changing HTTP requests (POST, PUT, DELETE). Safe methods (GET, HEAD, OPTIONS) pass through without validation. This provides defense-in-depth against Cross-Site Request Forgery attacks alongside the SameSite=Lax session cookie already set by split 01.

**Why this works:** Browsers do not send custom headers in "simple" cross-origin requests. A CSRF attack page cannot add `X-Requested-With` to a form submission or image load. Combined with the SameSite=Lax cookie (which already prevents cross-site cookie transmission on POST), this eliminates CSRF without needing a token-based system.

**Dependencies:** Section 01 (deps-and-config) must be complete. No other section dependencies.

**Blocked by this section:** Section 08 (wiring) integrates this middleware into the router stack.

---

## Tests First

All tests live in the middleware module or in integration tests. The CSRF middleware is tested in isolation using a minimal Axum app.

**File:** `backend/src/features/auth/middleware.rs` (unit tests at bottom) or `backend/tests/csrf.rs` (integration)

```rust
// --- CSRF Protection Tests ---

// Test: GET request passes without X-Requested-With header
//   Build a test app with the CSRF layer applied, send a GET request with no
//   X-Requested-With header, assert 200 (or whatever the handler returns).

// Test: OPTIONS request passes without X-Requested-With header
//   Same setup, send OPTIONS, assert it is not blocked.

// Test: POST without X-Requested-With returns 403
//   Send a POST request with no X-Requested-With header.
//   Assert status 403 and body contains "CSRF validation failed".

// Test: PUT without X-Requested-With returns 403
//   Send a PUT request with no X-Requested-With header.
//   Assert status 403.

// Test: DELETE without X-Requested-With returns 403
//   Send a DELETE request with no X-Requested-With header.
//   Assert status 403.

// Test: POST with X-Requested-With: XMLHttpRequest passes
//   Send a POST request with header X-Requested-With: XMLHttpRequest.
//   Assert 200 (request reaches the handler).

// Test: POST with wrong X-Requested-With value returns 403
//   Send a POST with X-Requested-With: SomeOtherValue.
//   Assert status 403 and body contains "CSRF validation failed".
```

### Test Helper Pattern

Each test should construct a minimal router with the CSRF middleware applied and a simple echo handler, then use `axum::body::Body` and `tower::ServiceExt::oneshot` to send requests without starting a real server:

```rust
/// Scaffold for test setup (not full implementation):
async fn build_csrf_test_app() -> Router {
    Router::new()
        .route("/test", post(|| async { "ok" }))
        .route("/test", get(|| async { "ok" }))
        .route("/test", put(|| async { "ok" }))
        .route("/test", delete(|| async { "ok" }))
        .layer(axum::middleware::from_fn(csrf_check))
}
```

---

## Implementation

### File: `backend/src/features/auth/middleware.rs`

Create this new file (or add to it if section 03/05 have already created it). The module will contain the `csrf_check` middleware function.

### Middleware Function Signature

```rust
/// CSRF protection middleware.
///
/// Rejects POST, PUT, and DELETE requests that lack a valid
/// `X-Requested-With: XMLHttpRequest` header. GET, HEAD, and OPTIONS
/// requests pass through unconditionally.
pub async fn csrf_check(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response
```

### Logic

1. **Check the HTTP method.** If the method is GET, HEAD, or OPTIONS, call `next.run(req).await` immediately and return the response.

2. **For POST, PUT, DELETE:** inspect `req.headers().get("x-requested-with")`.
   - If the header is present and its value (case-insensitive comparison) equals `"XMLHttpRequest"`, call `next.run(req).await`.
   - Otherwise, return a 403 Forbidden JSON response:
     ```json
     { "error": "forbidden", "message": "CSRF validation failed" }
     ```
     Use `axum::http::StatusCode::FORBIDDEN` and `axum::Json` to construct the response. The response type should be `axum::response::Response` (use `.into_response()`).

### Error Response Construction

```rust
/// Returns a 403 JSON error for CSRF failures.
fn csrf_forbidden() -> axum::response::Response {
    // Return (StatusCode::FORBIDDEN, Json(serde_json::json!({...}))).into_response()
}
```

This keeps the error format consistent with the RBAC middleware from section 03, which also returns `{ "error": "forbidden", "message": "..." }`.

### Case Sensitivity Note

The header name lookup via `req.headers().get("x-requested-with")` is already case-insensitive in `http::HeaderMap`. For the header **value**, compare case-insensitively using `.to_str()` and `.eq_ignore_ascii_case("XMLHttpRequest")`.

---

## Module Registration

### File: `backend/src/features/auth/mod.rs`

Add `pub mod middleware;` to the module declaration. If other sections (03, 05) also add items to `middleware.rs`, they share this single module file. The `mod.rs` only needs the declaration once.

```rust
pub mod middleware;
pub mod routes;
```

---

## Frontend Impact

### File: `frontend/src/lib/api.ts`

The Axios instance must send `X-Requested-With: XMLHttpRequest` on every request. Add it to the default headers in the `axios.create` call:

```typescript
export const api = axios.create({
  baseURL: "/api",
  headers: {
    "Content-Type": "application/json",
    "X-Requested-With": "XMLHttpRequest",  // Required for CSRF protection
  },
});
```

This is a one-line change. All requests made through the `api` instance will automatically include the header. No other frontend changes are needed.

---

## CORS Interaction

The `X-Requested-With` header is a non-standard header, which means cross-origin requests including it will trigger a CORS preflight (OPTIONS). The existing CORS configuration in `backend/src/lib.rs` already uses `allow_headers(tower_http::cors::Any)`, which permits all headers including `X-Requested-With`. No CORS changes are needed.

However, when section 08 (wiring) assembles the middleware stack, the CSRF middleware must be placed **after** the CORS layer so that preflight OPTIONS requests are handled by CORS before reaching the CSRF check. The CSRF middleware itself also passes OPTIONS through, providing a second safety net.

---

## Edge Case: API-Only Clients

For future API clients using Bearer token authentication (no cookies), CSRF is technically not a concern since there is no ambient credential. However, the current design always requires the header regardless of auth method. This is the simpler approach -- API clients can trivially add the one-line header. If this becomes a pain point in the future, the middleware could be updated to skip the check when `Authorization: Bearer ...` is present, but that is out of scope for this section.