Now I have enough context. Let me produce the section content.

# Section 3: RBAC Middleware

## Overview

This section implements the `require_permission()` middleware function that checks whether the currently authenticated user has the necessary role-based permission for a given route. It also restructures existing feature routers to separate read (GET) routes from write (POST/PUT/DELETE) routes so that `route_layer()` can apply RBAC guards to write groups only.

**Dependencies:** Section 01 (dependencies and config), Section 02 (permission matrix providing `Feature`, `Action`, `Role` enums and `HasPermission` trait on `AuthUser`).

**Blocks:** Section 08 (wiring and integration).

---

## Tests First

All tests live in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/` or as inline `#[cfg(test)]` modules in the middleware file.

```rust
// File: backend/src/features/auth/middleware.rs (inline #[cfg(test)] module)
// OR: backend/tests/rbac_middleware.rs (integration tests)

// Test: GET request to protected route passes without RBAC (read is universal)
//   Build a test app with require_permission(Feature::Ontology, Action::Create) on a POST route.
//   Send a GET to an unguarded route. Expect 200.

// Test: POST to ontology returns 403 for viewer
//   Create AuthUser with role="viewer". Insert into request extensions.
//   Apply require_permission(Feature::Ontology, Action::Create) middleware.
//   Expect 403 with JSON body {"error":"forbidden","message":"Insufficient permissions"}.

// Test: POST to ontology returns 200 for admin
//   AuthUser with role="admin". Same middleware. Expect request to pass through to handler.

// Test: POST to ontology returns 200 for risk_manager
//   AuthUser with role="risk_manager". Same middleware. Expect pass-through.

// Test: DELETE on compliance returns 403 for specialist
//   AuthUser with role="specialist".
//   require_permission(Feature::Compliance, Action::Delete). Expect 403.

// Test: DELETE on compliance returns 200 for risk_manager
//   AuthUser with role="risk_manager".
//   require_permission(Feature::Compliance, Action::Delete). Expect 200.

// Test: User management endpoint returns 403 for risk_manager
//   require_permission(Feature::Auth, Action::ManageUsers). role="risk_manager". Expect 403.

// Test: User management endpoint returns 200 for admin
//   require_permission(Feature::Auth, Action::ManageUsers). role="admin". Expect 200.

// Test: Unauthenticated request returns 401 (not 403)
//   No AuthUser in request extensions. Middleware should return 401 Unauthorized.
```

### Test Helper Pattern

For integration tests, build a minimal Axum router with the middleware applied and use `axum::body::Body` + `tower::ServiceExt::oneshot` to send requests:

```rust
/// Helper: build a test router with a single POST route guarded by require_permission.
/// Insert an AuthUser into request extensions to simulate authentication.
fn test_app_with_permission(feature: Feature, action: Action) -> Router {
    // Router with a single POST "/" handler that returns 200
    // Apply .route_layer(middleware::from_fn(require_permission(feature, action)))
}

/// Helper: build a request with AuthUser in extensions
fn request_with_role(method: Method, uri: &str, role: &str) -> Request<Body> {
    // Construct Request, insert AuthUser { role: role.to_string(), ... } into extensions
}
```

---

## Implementation Details

### File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/middleware.rs` (new file)

Create a new module for RBAC middleware.

#### `require_permission` Function

The middleware uses `axum::middleware::from_fn` with a closure that captures the required `Feature` and `Action`:

```rust
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use super::permissions::{Action, Feature, HasPermission};
// AuthUser is assumed to be in request extensions, placed by the auth extractor from split 01

/// Returns an async handler suitable for axum::middleware::from_fn.
///
/// Checks AuthUser from request extensions against the permission matrix.
/// Returns 401 if no AuthUser present, 403 if permission denied.
pub fn require_permission(
    feature: Feature,
    action: Action,
) -> impl Fn(Request, Next) -> /* impl Future<Output = Response> */ + Clone + Send
```

Key implementation points:

1. **Extract AuthUser from extensions** -- call `req.extensions().get::<AuthUser>()`. The `AuthUser` struct was placed into extensions by the authentication extractor (from split 01-backend-core-auth). If `AuthUser` is not present, return 401 Unauthorized immediately (the user is not authenticated at all).

2. **Check permission** -- call `auth_user.has_permission(feature, action)` using the `HasPermission` trait from section 02.

3. **Deny with 403** -- if permission check returns false, respond with:
   ```json
   { "error": "forbidden", "message": "Insufficient permissions" }
   ```
   Status code: 403 Forbidden.

4. **Allow** -- if permission check returns true, call `next.run(req).await` to proceed to the handler.

Because `middleware::from_fn` expects an async function (not a closure returning an async fn), the idiomatic pattern is to return a boxed future or use a helper that wraps the closure. The simplest approach:

```rust
pub async fn require_permission_inner(
    feature: Feature,
    action: Action,
    req: Request,
    next: Next,
) -> Response {
    // Extract AuthUser, check permission, return 401/403/pass-through
}
```

Then at route-building time, use a closure with `from_fn`:

```rust
use axum::middleware;

// Example usage in a router:
my_router.route_layer(middleware::from_fn(move |req, next| {
    require_permission_inner(Feature::Compliance, Action::Delete, req, next)
}))
```

Alternatively, use `from_fn` with a state-carrying extractor. Choose whichever compiles cleanly with Axum's type system.

### File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/mod.rs`

Update to declare the new modules:

```rust
pub mod middleware;
pub mod permissions; // from section 02
pub mod routes;
```

---

## Applying RBAC to Existing Feature Routers

Each feature's `router()` function must be restructured to separate read routes from write routes, so RBAC can be applied via `route_layer()` only to the write group.

### Ontology Routes

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/ontology/routes.rs`

Currently all routes are GET (list_frameworks, get_framework, list_concepts, etc.). No RBAC needed today. If POST/PUT/DELETE ontology management routes are added later, they should be in a separate sub-router with `require_permission(Feature::Ontology, Action::Create)` or similar.

No changes required at this time.

### Compliance Routes

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/compliance/routes.rs`

The current `router()` mixes GET and POST/PUT/DELETE routes. Restructure into sub-groups:

- **Read routes (no RBAC):** All GET routes -- `list_assessments`, `get_assessment`, `get_compliance_items`, `get_compliance_score`, `get_assessment_history`, `get_evidence`
- **Create/Update routes:** POST and PUT routes -- `create_assessment`, `update_assessment`, `update_compliance_item`, `add_item_note`, `add_evidence`, `upload_evidence`. Guard with `require_permission(Feature::Compliance, Action::Create)` (or `Action::Update` where appropriate; since specialist can both create and update, using `Action::Create` for POST and `Action::Update` for PUT is the cleanest split).
- **Delete routes:** DELETE routes -- `delete_assessment`, `delete_evidence`. Guard with `require_permission(Feature::Compliance, Action::Delete)`.

Pattern for restructuring:

```rust
pub fn router() -> Router<AppState> {
    let read_routes = Router::new()
        .route("/assessments", get(list_assessments))
        .route("/assessments/:id", get(get_assessment))
        // ... all other GET routes

    let create_update_routes = Router::new()
        .route("/assessments", post(create_assessment))
        .route("/assessments/:id", put(update_assessment))
        // ... all other POST/PUT routes
        .route_layer(/* require_permission(Compliance, Create) */);

    let delete_routes = Router::new()
        .route("/assessments/:id", delete(delete_assessment))
        .route("/evidence/:id", delete(delete_evidence))
        .route_layer(/* require_permission(Compliance, Delete) */);

    read_routes.merge(create_update_routes).merge(delete_routes)
}
```

Note: Axum does not allow two `.route()` calls for the same path in the same `Router` (e.g., `/assessments/:id` for both GET and PUT). The `.merge()` approach handles this because each sub-router has distinct method handlers on the same path. If merge causes conflicts, use method routing within a single `.route()` call and apply the RBAC check at handler level instead (calling `has_permission` directly on the `AuthUser` inside the handler).

### Analysis Routes

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/routes.rs`

Separate similarly:

- **Read routes:** GET `/`, GET `/{id}`, GET `/{id}/findings`, GET `/{id}/export/{format}`, GET `/prompt-template`
- **Create routes:** POST `/` (`create_analysis`), POST `/upload` (`upload_analysis`), PUT `/prompt-template`. Guard with `require_permission(Feature::Analysis, Action::Create)`.
- **Delete routes:** DELETE `/{id}`. Guard with `require_permission(Feature::Analysis, Action::Delete)`.

The export endpoint (GET `/{id}/export/{format}`) is a read-like operation but the permission matrix specifies that export requires `Reports::Export`. Apply `require_permission(Feature::Reports, Action::Export)` to the export route specifically if the permission matrix requires it. Otherwise leave it as a read route.

### Reports Routes

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/reports/routes.rs`

Currently only has a GET placeholder. When export endpoints are added, guard them with `require_permission(Feature::Reports, Action::Export)`.

No changes required at this time.

### Auth Routes

**File:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/auth/routes.rs`

The `/me` GET endpoint needs no RBAC. Future user management endpoints (POST/PUT/DELETE users) should be guarded with `require_permission(Feature::Auth, Action::ManageUsers)`.

No changes required at this time beyond ensuring the module structure supports future additions.

---

## Error Response Format

Both 401 and 403 responses should use the same JSON error format for consistency with the rest of the API:

```json
// 401 - No authenticated user
{ "error": "unauthorized", "message": "Authentication required" }

// 403 - Authenticated but lacks permission
{ "error": "forbidden", "message": "Insufficient permissions" }
```

---

## Key Design Decisions

1. **Read access is universal.** All authenticated users (including viewer) can access all GET endpoints. RBAC is only applied to mutation routes.

2. **Static permission checks.** No database queries in the middleware -- the permission matrix from section 02 is a compile-time constant. This keeps the middleware fast and allocation-free.

3. **Middleware vs handler-level checks.** Use `route_layer()` where route grouping is clean. For routes where the same path has both GET and mutation methods (e.g., `/assessments/:id` for GET and DELETE), either split into sub-routers and merge, or perform the permission check inside the handler. The sub-router approach is preferred for consistency.

4. **Default to Viewer on unknown role.** If `Role::from_str()` fails, the middleware treats the user as a Viewer (most restrictive). A warning should be logged via `tracing::warn!`.