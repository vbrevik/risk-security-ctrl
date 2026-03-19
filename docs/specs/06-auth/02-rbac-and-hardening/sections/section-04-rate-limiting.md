Now I have all the context I need. Let me produce the section content.

# Section 4: Rate Limiting

## Overview

This section adds GCRA-based rate limiting to the application using `governor` and `tower_governor`. Two separate rate limiter instances are configured: a strict limiter for authentication endpoints (login/register) and a moderate limiter for general API traffic. The rate limiter extracts client IP addresses to apply per-client limits, and emits standard rate-limit response headers.

**Dependencies:** Section 01 (deps-and-config) must be completed first. Specifically, the `governor` and `tower_governor` crate dependencies must be in `Cargo.toml`, and the `behind_proxy` field must exist on the `Config` struct.

**Blocked by this section:** Section 08 (wiring) integrates the rate limiter layers into the final middleware stack.

## Tests

All tests live in `backend/tests/rate_limiting.rs` (integration tests) with some unit-level tests inline.

```rust
// backend/tests/rate_limiting.rs

/// Test: Login endpoint allows burst_size requests without being rate limited.
/// Send 5 rapid POST requests to /api/auth/login. All should return a non-429 status
/// (likely 400 or 401 due to missing/bad credentials, but NOT 429).
#[tokio::test]
async fn login_allows_burst_size_requests() { todo!() }

/// Test: Login endpoint returns 429 after exceeding the burst limit.
/// Send 6+ rapid POST requests to /api/auth/login. The request exceeding
/// burst_size(5) should receive HTTP 429 Too Many Requests.
#[tokio::test]
async fn login_returns_429_after_exceeding_burst() { todo!() }

/// Test: 429 response includes x-ratelimit-limit header.
/// After triggering a 429, inspect response headers for the presence of
/// the `x-ratelimit-limit` header.
#[tokio::test]
async fn rate_limited_response_includes_ratelimit_limit_header() { todo!() }

/// Test: 429 response includes retry-after header.
/// After triggering a 429, inspect response headers for `retry-after`.
#[tokio::test]
async fn rate_limited_response_includes_retry_after_header() { todo!() }

/// Test: API endpoints have higher burst tolerance than auth endpoints.
/// Send 10 rapid requests to a general API endpoint (e.g., GET /api/health).
/// All 10 should succeed (burst_size=30 for API). This proves the API limiter
/// is more permissive than the auth limiter (burst_size=5).
#[tokio::test]
async fn api_endpoints_have_higher_burst_tolerance() { todo!() }

/// Test: Rate limiter uses PeerIpKeyExtractor by default.
/// With BEHIND_PROXY unset or false, build the governor config and verify it
/// works without X-Forwarded-For headers. This is primarily a compilation/smoke test.
#[tokio::test]
async fn rate_limiter_uses_peer_ip_by_default() { todo!() }

/// Test: Rate limiter uses SmartIpKeyExtractor when BEHIND_PROXY=true.
/// With BEHIND_PROXY=true, build the governor config. Send a request with
/// X-Forwarded-For header and confirm rate limiting applies to the forwarded IP.
#[tokio::test]
async fn rate_limiter_uses_smart_ip_when_behind_proxy() { todo!() }

/// Test: into_make_service_with_connect_info is used so IP extraction works.
/// Build the app and verify that a request to a rate-limited endpoint
/// actually receives rate-limit headers (proving IP extraction is functional).
#[tokio::test]
async fn connect_info_enables_ip_extraction() { todo!() }
```

## Implementation Details

### File: `backend/src/features/auth/rate_limit.rs` (new)

Create a new module that builds and exposes the two governor configurations.

**Auth limiter (strict):**
- Replenish rate: 1 token every 4 seconds (`per_second(4)`)
- Burst size: 5 (`burst_size(5)`)
- Applied to: `/api/auth/login` and `/api/auth/register`

**API limiter (moderate):**
- Replenish rate: 1 token per second (`per_second(1)`)
- Burst size: 30 (`burst_size(30)`)
- Applied to: all `/api/*` routes (including auth routes, which hit both limiters)

**Public API of this module:**

```rust
// backend/src/features/auth/rate_limit.rs

use governor::Quota;
use std::num::NonZeroU32;
use tower_governor::GovernorConfigBuilder;

/// Build a GovernorConfig for auth endpoints (login/register).
/// Strict: burst_size=5, replenish 1 per 4 seconds.
pub fn auth_governor_config(behind_proxy: bool) -> tower_governor::GovernorConfig<...> {
    // ...
}

/// Build a GovernorConfig for general API endpoints.
/// Moderate: burst_size=30, replenish 1 per second.
pub fn api_governor_config(behind_proxy: bool) -> tower_governor::GovernorConfig<...> {
    // ...
}
```

### Governor Config Construction

Each function follows the same pattern:

1. Start with `GovernorConfigBuilder::default()`
2. Set the quota using `Quota::per_second(NonZeroU32::new(rate).unwrap()).allow_burst(NonZeroU32::new(burst).unwrap())`
3. Call `.use_headers()` to enable `x-ratelimit-limit`, `x-ratelimit-remaining`, `x-ratelimit-after`, and `retry-after` response headers
4. Conditionally set the key extractor based on `behind_proxy`:
   - If `behind_proxy` is `true`: use `SmartIpKeyExtractor` (reads `X-Forwarded-For` or `X-Real-IP`, falls back to peer IP)
   - If `behind_proxy` is `false`: use the default `PeerIpKeyExtractor` (direct socket address)
5. Call `.finish()` to produce the `GovernorConfig`

The `tower_governor` crate provides `GovernorLayer` which wraps the config into a tower Layer. The layer is applied to route groups via `.layer(GovernorLayer { config: &governor_config })`.

### IP Key Extraction

The `SmartIpKeyExtractor` from `tower_governor::key_extractor` handles proxy scenarios by checking:
1. `X-Forwarded-For` header (first IP in the chain)
2. `X-Real-IP` header
3. Falls back to peer socket address

When `behind_proxy` is false, the default `PeerIpKeyExtractor` reads the peer address directly from the connection, which requires `into_make_service_with_connect_info::<SocketAddr>()`.

### File: `backend/src/main.rs` (modify)

Update the `axum::serve` call to use `into_make_service_with_connect_info` so that the rate limiter can extract client IP addresses from the TCP connection:

```rust
// BEFORE:
axum::serve(listener, app).await?;

// AFTER:
use std::net::SocketAddr;
axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
```

Without this change, `PeerIpKeyExtractor` silently fails and rate limiting does not function. The `SocketAddr` is already imported in `main.rs`.

### File: `backend/src/features/auth/mod.rs` (modify)

Add `pub mod rate_limit;` to the auth feature module so the rate limiting functions are accessible from the wiring code.

### How the Layers Are Applied (Preview for Section 08)

The actual wiring of rate limiter layers into the router happens in Section 08. The pattern will be:

```rust
// Auth routes get the strict limiter
let auth_routes = features::auth::routes::router()
    .layer(GovernorLayer { config: &auth_limiter });

// All API routes get the moderate limiter
Router::new()
    .nest("/api", api_routes)
    .layer(GovernorLayer { config: &api_limiter });
```

The governor configs must be constructed in `main.rs` or `create_router()` and passed/referenced where the layers are applied. Since `GovernorConfig` contains an `Arc` internally, it can be cloned or referenced cheaply.

### Error Response Format

When rate limited, `tower_governor` returns HTTP 429 by default with a plain text body. To customize the error response to match the application's JSON error format, use `GovernorConfigBuilder::error_handler()`:

```rust
/// Custom 429 error handler returning JSON.
fn rate_limit_error_handler(err: GovernorError) -> axum::response::Response {
    // Return JSON: { "error": "too_many_requests", "message": "Rate limit exceeded. Try again later." }
}
```

This is optional but recommended for API consistency. If omitted, the default plain-text 429 response from tower_governor is still functional.

### Configuration Dependency

This section assumes Section 01 has already added the following to `Config`:

```rust
pub behind_proxy: bool,  // default: false, parsed from BEHIND_PROXY env var
```

This field controls which IP key extractor the governor uses.