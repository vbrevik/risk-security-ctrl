# Implementation Plan: RBAC & Security Hardening

## Context

Risk-security-ctrl is a Rust/Axum application for governmental IT security compliance, running air-gapped. Split 01-backend-core-auth provides an `AuthUser` extractor with `{ id, email, name, role, session_id }`. This split adds authorization, rate limiting, security headers, CSRF protection, and session cleanup.

All authenticated users can read all data. Write/delete operations are restricted by role via a permission matrix. The four roles are: admin, risk_manager, specialist, viewer.

## Architecture Overview

Five middleware/service layers added in this split:

1. **Permission matrix** — static mapping of role+feature+action to allow/deny
2. **RBAC guard** — Axum middleware checking AuthUser.role against required permission
3. **Rate limiter** — tower_governor on auth and API route groups
4. **Security headers** — tower-http response header middleware
5. **CSRF validation** — custom middleware requiring X-Requested-With on mutations

Middleware execution order (outermost first): Security Headers → CORS → Rate Limit → CSRF Check → Auth → RBAC → Handler.

## Section 1: Dependencies and Configuration

### New Cargo Dependencies

- `governor = "0.10"` — GCRA rate limiting algorithm, in-memory
- `tower_governor = "0.8"` — Tower/Axum middleware wrapper for governor

### tower-http Feature Update

Update existing `tower-http` dependency to add `set-header` feature for security headers:
```toml
tower-http = { version = "0.6", features = ["cors", "compression-gzip", "trace", "set-header", "sensitive-headers"] }
```

### Config Additions

Add to `Config` struct:
- `BEHIND_PROXY: bool` — default false. When true, rate limiter uses SmartIpKeyExtractor to read X-Forwarded-For
- `ENABLE_HTTPS: bool` — default false. When true, adds HSTS header and ensures Secure cookie flag

### Swagger Feature Flag

Gate `utoipa-swagger-ui` behind a cargo feature:
```toml
[features]
default = ["swagger"]
swagger = ["dep:utoipa-swagger-ui"]

[dependencies]
utoipa-swagger-ui = { version = "8", features = ["axum"], optional = true }
```

In `src/lib.rs`, wrap Swagger UI route registration with `#[cfg(feature = "swagger")]`. Production builds omit Swagger via `cargo build --release --no-default-features`.

## Section 2: Permission Matrix

### Permission Model

Define permissions as `"feature:action"` strings. The matrix is a static lookup:

```rust
enum Feature { Ontology, Compliance, Analysis, Reports, Auth }
enum Action { Read, Create, Update, Delete, Export, ManageUsers }
enum Role { Admin, RiskManager, Specialist, Viewer }

fn has_permission(role: &Role, feature: &Feature, action: &Action) -> bool
```

### Matrix Definition

Encode the permission table as a `const` or `static` — no database queries, no runtime allocation. Use a match expression or a `phf` (perfect hash function) map.

Read access is universal for all authenticated users. Write restrictions:
- **viewer:** read only, no create/update/delete/export on any feature
- **specialist:** can create/update compliance and analyses, can export reports, cannot delete
- **risk_manager:** can create/update/delete compliance and analyses, can export reports, can create/update ontology
- **admin:** all permissions including user management

### Permission Trait

```rust
trait HasPermission {
    fn has_permission(&self, feature: Feature, action: Action) -> bool;
}

impl HasPermission for AuthUser {
    fn has_permission(&self, feature: Feature, action: Action) -> bool {
        let role = Role::from_str(&self.role).unwrap_or(Role::Viewer);
        permission_matrix::has_permission(&role, &feature, &action)
    }
}
```

## Section 3: RBAC Middleware

### RequirePermission Guard

Use `middleware::from_fn` pattern with a closure that captures the required permission:

```rust
fn require_permission(feature: Feature, action: Action) -> impl Fn(Request, Next) -> ...
```

**Architectural note:** `AuthUser` is an Axum `FromRequestParts` extractor, not a middleware that inserts into extensions. The RBAC guard must therefore extract `AuthUser` itself from the request parts (calling the same `FromRequestParts` logic). Two approaches:

**Approach A (recommended):** Make the RBAC guard a handler-level check. Each write handler accepts `AuthUser` as a parameter and calls `auth_user.has_permission(feature, action)` at the start. Simple, no middleware complexity.

**Approach B:** Use `middleware::from_fn_with_state` and manually extract the session token + validate + load user inside the middleware (duplicating extractor logic). More DRY if many routes share the same permission, but harder to implement correctly.

For this plan, use **Approach A** for simplicity. Each write handler receives `AuthUser` (which already validates auth) and checks permission:

The permission check:
1. Handler receives `AuthUser` from Axum's extractor (auth validated)
2. Calls `auth_user.has_permission(feature, action)`
3. If denied: return 403 Forbidden with `{ "error": "forbidden", "message": "Insufficient permissions" }`
4. If allowed: proceed with handler logic

### Apply to Existing Routes

Group routes by permission level and apply `.route_layer()`:

**Ontology routes:**
- GET endpoints: no RBAC needed (all authenticated users can read)
- POST/PUT/DELETE endpoints: `require_permission(Ontology, Create) / require_permission(Ontology, Update)`

**Compliance routes:**
- GET endpoints: no RBAC
- POST/PUT: `require_permission(Compliance, Create)` or `Update`
- DELETE: `require_permission(Compliance, Delete)`

**Analysis routes:**
- GET endpoints: no RBAC
- POST: `require_permission(Analysis, Create)`
- DELETE: `require_permission(Analysis, Delete)`

**Reports routes:**
- GET endpoints: no RBAC
- Export endpoints: `require_permission(Reports, Export)`

**Auth routes:**
- User management (future): `require_permission(Auth, ManageUsers)`

### Route Restructuring

The existing router may need restructuring to separate read vs write routes into different groups so `route_layer()` applies correctly. Each feature's `router()` function should return sub-routers grouped by permission level.

## Section 4: Rate Limiting

### Governor Configuration

Two rate limiter instances with separate counters:

**Auth limiter (strict):**
- `per_second(4)` — 1 token replenished every 4 seconds
- `burst_size(5)` — max 5 rapid attempts
- Applied to: `/api/auth/login`, `/api/auth/register`

**API limiter (moderate):**
- `per_second(1)` — 1 token per second
- `burst_size(30)` — generous burst for normal usage
- Applied to: all `/api/*` routes

### IP Key Extraction

```rust
if config.behind_proxy {
    GovernorConfigBuilder::default().key_extractor(SmartIpKeyExtractor)
} else {
    GovernorConfigBuilder::default() // uses PeerIpKeyExtractor by default
}
```

### Response Headers

Enable `.use_headers()` on both configs. This adds:
- `x-ratelimit-limit`, `x-ratelimit-remaining`, `x-ratelimit-after`, `retry-after`

### Server Binding

Update `main.rs` to use `into_make_service_with_connect_info::<SocketAddr>()` instead of `into_make_service()`. Without this, IP extraction silently fails and rate limiting doesn't work.

## Section 5: Security Headers Middleware

### Header Layer

Add a tower middleware layer that sets security response headers on all responses:

```rust
// Always set:
X-Frame-Options: DENY
X-Content-Type-Options: nosniff
Referrer-Policy: strict-origin-when-cross-origin
Content-Security-Policy: default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self'
Permissions-Policy: camera=(), microphone=(), geolocation=()
X-XSS-Protection: 0  // disabled per modern guidance, CSP is preferred

// HTTPS only (when ENABLE_HTTPS=true):
Strict-Transport-Security: max-age=31536000; includeSubDomains
```

### Implementation

Use `tower_http::set_header::SetResponseHeaderLayer` for each header, or create a custom tower Layer/Service that sets all headers in one pass for efficiency.

### Sensitive Headers

Use `tower_http::sensitive_headers::SetSensitiveHeadersLayer` to mark `Authorization` and `Cookie` headers as sensitive — this prevents them from appearing in tracing/logging output.

## Section 6: CSRF Protection Middleware

### Custom Header Validation

Create a tower middleware that checks for `X-Requested-With: XMLHttpRequest` on all POST, PUT, and DELETE requests:

1. If method is GET, HEAD, or OPTIONS: pass through
2. If method is POST, PUT, or DELETE: check for `X-Requested-With` header
3. If header missing or wrong value: return 403 with `{ "error": "forbidden", "message": "CSRF validation failed" }`
4. If present: pass through

### Why This Works

The SameSite=Lax cookie from split 01 prevents cross-site cookie transmission on POST. The custom header adds defense-in-depth: browsers don't send custom headers in simple cross-origin requests. Together, they prevent CSRF without a token-based system.

### Frontend Impact

The frontend API client (`lib/api.ts`) must add `X-Requested-With: XMLHttpRequest` to all requests. This is typically one line in the Axios instance config.

## Section 7: Session Cleanup

### Lazy Cleanup Strategy

The AuthUser extractor (from split 01) already validates session expiry. Extend it to delete expired sessions when encountered:

In `validate_session()`:
- If session found but expired → DELETE it before returning None
- This is already specified in split 01's plan

### Bulk Cleanup (Optional)

For deployments running long without restarts, add a periodic cleanup that runs on application startup:

```rust
async fn cleanup_expired_sessions(pool: &SqlitePool) -> Result<u64>
```

Call this once during server initialization. Delete all sessions where `expires_at < datetime('now')`. Log the count of deleted sessions.

No background task or timer needed — startup cleanup + lazy per-request cleanup is sufficient for a single-instance air-gapped deploy.

## Section 8: Wiring and Integration

### Router Assembly

The main router in `src/lib.rs` needs restructuring to apply middleware in the correct order:

```
Router::new()
    // Auth routes with strict rate limiting
    .nest("/auth", auth_router)

    // API routes with RBAC and moderate rate limiting
    .nest("/api", api_router)

    // Global middleware (outermost = runs first)
    .layer(csrf_middleware)           // Check X-Requested-With on mutations
    .layer(security_headers_layer)    // Set security response headers
    .layer(cors_layer)                // CORS (already exists, update)
    .layer(trace_layer)               // Logging (already exists)
```

### into_make_service_with_connect_info

Update `src/main.rs` serve call:
```rust
axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
```

### Module Structure

```
src/features/auth/
  mod.rs           — add pub mod permissions, middleware
  permissions.rs   — Feature, Action, Role enums, has_permission(), HasPermission trait
  middleware.rs    — require_permission(), csrf_check(), security_headers()
```

### OpenAPI Updates

Add security scheme annotations for protected endpoints. Mark endpoints with their required permission in the OpenAPI description.

## Edge Cases and Error Handling

- **Role not recognized:** If `Role::from_str(&user.role)` fails, default to `Viewer` (most restrictive). Log a warning.
- **Rate limit on legitimate traffic:** 30 burst / 1 per second is generous. Monitor and adjust if needed.
- **CSRF on API-only clients:** Bearer-token auth does not use cookies, so CSRF is not applicable. The CSRF middleware should only check for the header when the auth method is cookie-based, OR always require it (simpler — API clients can easily add the header).
- **Feature flag compilation:** Ensure code compiles correctly with and without the `swagger` feature. Use `#[cfg(feature = "swagger")]` consistently.

## Opus Review Integration

The following issues were identified by Opus review and resolved:

- **CSP loosened** for React/Tailwind/shadcn: `style-src 'self' 'unsafe-inline'`, `img-src 'self' data:`, `font-src 'self'`
- **AuthUser extractor vs middleware:** RBAC uses handler-level checks (Approach A), not middleware extensions
- **Action::Write removed:** Use Create/Update/Delete individually per the enum
- **Specialist export permission:** Added per spec table
- **SameSite clarification:** Split 01 sets SameSite=Lax (correct). The spec's mention of Strict was aspirational — Lax is required to allow top-level navigation with cookies. Spec updated accordingly.
- **CSRF always requires header:** Simpler — API clients can trivially add X-Requested-With
- **Rate limiter dual-hit on auth routes:** Intentional defense-in-depth (both auth + API limiters apply)
- **Rate limiter resets on restart:** Acceptable for single-instance air-gapped deployment
- **Per-account rate limiting:** Deferred — per-IP is sufficient for air-gapped network
- **ConnectInfo in tests:** Rate limiting tests must either skip governor or provide mock ConnectInfo

## Known Gaps (Deferred)

- **BOLA protection:** Not implemented. All authenticated users can read all data. Write restrictions are role-based, not ownership-based. This is per spec decision.
- **Permission caching:** Permissions are computed from a static matrix — no caching needed. If permissions move to DB in the future, add an RwLock cache.
- **Audit logging for RBAC denials:** Not logging 403 events to audit_log. Could be added as a follow-up.
