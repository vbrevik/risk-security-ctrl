Now I have all the context needed. Let me produce the section content.

# Section 02: Route Scaffold and Wiring

## Overview

This section creates the route handler module skeleton at `backend/src/features/analysis/routes.rs`, registers it in the application router, and wires up the module declaration. The goal is to have all route paths defined with stub handlers that compile and return placeholder responses, proving the wiring works end-to-end before any real logic is implemented.

**Depends on:** Section 01 (dependencies and AppState changes must be in place)
**Blocks:** Sections 03, 04, 05, 06, 10, 11 (all handler implementations)

---

## Test First

Create the integration test file `backend/tests/analysis_tests.rs`. This section only needs one test to verify the router is registered and reachable.

### File: `backend/tests/analysis_tests.rs`

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

mod common;
use common::create_test_app;

/// Proves the analysis router is registered under /api/analyses.
/// A GET to the list endpoint should return 200 (not 404).
#[tokio::test]
async fn test_router_registration() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/analyses")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Must not be 404 — that would mean the route is not registered.
    assert_ne!(response.status(), StatusCode::NOT_FOUND);
    // The stub handler should return 200 with an empty list or similar.
    assert_eq!(response.status(), StatusCode::OK);
}
```

Note: The existing `create_test_app()` in `backend/tests/common/mod.rs` constructs an `AppState` with `db` and `config` fields. After Section 01 adds the `topics: Vec<Topic>` field to `AppState`, `create_test_app` must also populate that field. If Section 01 has not yet updated `create_test_app`, this section should add `topics: vec![]` to the `AppState` construction in `backend/tests/common/mod.rs` so that the test compiles:

```rust
let state = AppState {
    db: pool,
    config: config.clone(),
    topics: vec![],
};
```

---

## Implementation

### 1. Create the route handler module

**File: `backend/src/features/analysis/routes.rs`** (new file)

Define a `router()` function that registers all nine route paths, each pointing to a stub handler. The stub handlers must compile and return valid HTTP responses (they will be replaced by real implementations in later sections).

The router function signature and route table:

```rust
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_analysis))
        .route("/upload", post(upload_analysis))
        .route("/", get(list_analyses))
        .route("/{id}", get(get_analysis))
        .route("/{id}/findings", get(get_findings))
        .route("/{id}", delete(delete_analysis))
        .route("/{id}/export/{format}", get(export_analysis))
        .route("/prompt-template", get(get_prompt_template))
        .route("/prompt-template", put(update_prompt_template))
}
```

Note: Axum does not allow registering two separate `.route()` calls for the same path with different methods. Routes with the same path but different methods (e.g., `GET /` and `POST /`) must be combined using a `MethodRouter`:

```rust
.route("/", get(list_analyses).post(create_analysis))
.route("/{id}", get(get_analysis).delete(delete_analysis))
.route("/prompt-template", get(get_prompt_template).put(update_prompt_template))
```

The `upload` route needs a `DefaultBodyLimit` layer to allow 20MB uploads. Apply it as a route-specific layer:

```rust
use axum::extract::DefaultBodyLimit;

.route("/upload", post(upload_analysis))
    .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
```

However, because `.layer()` on a router applies to all routes above it, the 20MB body limit must be scoped. The cleanest approach is to nest the upload route in its own sub-router or use `route_layer`. An alternative is to apply `DefaultBodyLimit` only to the upload route by constructing it separately and merging. The important thing is that the 20MB limit applies only to the upload endpoint, not to all analysis endpoints. One clean pattern:

```rust
let upload_routes = Router::new()
    .route("/upload", post(upload_analysis))
    .layer(DefaultBodyLimit::max(20 * 1024 * 1024));

Router::new()
    .route("/", get(list_analyses).post(create_analysis))
    // ... other routes ...
    .merge(upload_routes)
```

#### Stub handlers

Each handler should accept the correct extractor types (so the signatures are ready for later sections) but return a placeholder response. Required imports: `axum::extract::{Json, Multipart, Path, Query, State}`, `axum::http::StatusCode`, `crate::error::AppResult`, `crate::AppState`, and the model types from `super::models`.

Stub signatures (docstrings describe what they will do in later sections):

- `create_analysis(State, Json<CreateAnalysisRequest>) -> AppResult<(StatusCode, Json<serde_json::Value>)>` -- return `(StatusCode::CREATED, Json(json!({})))`
- `upload_analysis(State, Multipart) -> AppResult<(StatusCode, Json<serde_json::Value>)>` -- return `(StatusCode::CREATED, Json(json!({})))`
- `list_analyses(State, Query<AnalysisListQuery>) -> AppResult<Json<serde_json::Value>>` -- return `Json(json!({"items":[], "total":0, "page":1, "limit":50, "total_pages":0}))`
- `get_analysis(State, Path<String>) -> AppResult<Json<serde_json::Value>>` -- return `Json(json!({}))`
- `get_findings(State, Path<String>, Query<FindingsListQuery>) -> AppResult<Json<serde_json::Value>>` -- return `Json(json!({"items":[], "total":0, "page":1, "limit":50, "total_pages":0}))`
- `delete_analysis(State, Path<String>) -> AppResult<StatusCode>` -- return `StatusCode::NO_CONTENT`
- `export_analysis(State, Path<(String, String)>) -> AppResult<impl IntoResponse>` -- return `StatusCode::NOT_IMPLEMENTED`
- `get_prompt_template(State) -> AppResult<Json<serde_json::Value>>` -- return `Json(json!({}))`
- `update_prompt_template(State, Json<serde_json::Value>) -> AppResult<Json<serde_json::Value>>` -- return `Json(json!({}))`

Each stub should have a `#[utoipa::path]` annotation with the correct HTTP method, path (prefixed with `/api/analyses`), tag `"analysis"`, and basic response codes. This prepares them for OpenAPI documentation. Example for `list_analyses`:

```rust
#[utoipa::path(
    get,
    path = "/api/analyses",
    tag = "analysis",
    params(AnalysisListQuery),
    responses(
        (status = 200, description = "List of analyses")
    )
)]
```

### 2. Register the module in mod.rs

**File: `backend/src/features/analysis/mod.rs`**

Add `pub mod routes;` to the existing module declarations. The file currently has:

```rust
pub mod engine;
pub mod matcher;
pub mod models;
pub mod parser;
pub mod tokenizer;
```

Add after `tokenizer`:

```rust
pub mod routes;
```

### 3. Nest the router in lib.rs

**File: `backend/src/lib.rs`**

In the `api_routes()` function, add the analysis router:

```rust
fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(features::ontology::routes::health_check))
        .nest("/ontology", features::ontology::routes::router())
        .nest("/compliance", features::compliance::routes::router())
        .nest("/reports", features::reports::routes::router())
        .nest("/auth", features::auth::routes::router())
        .nest("/analyses", features::analysis::routes::router())  // <-- add this line
}
```

### 4. Register the analysis tag in OpenAPI

**File: `backend/src/main.rs`**

Add the `"analysis"` tag to the `#[openapi]` attribute's `tags` list:

```rust
tags(
    (name = "health", description = "Health check endpoints"),
    (name = "ontology", description = "Ontology management endpoints"),
    (name = "compliance", description = "Compliance tracking endpoints"),
    (name = "reports", description = "Reporting endpoints"),
    (name = "auth", description = "Authentication endpoints"),
    (name = "analysis", description = "Document analysis endpoints"),  // <-- add
)
```

Also add the stub handler paths to the `paths(...)` list so they appear in the Swagger UI. For example:

```rust
paths(
    ontology_backend::features::ontology::routes::health_check,
    ontology_backend::features::analysis::routes::list_analyses,
    ontology_backend::features::analysis::routes::create_analysis,
    // ... remaining analysis handlers
)
```

---

## Verification

After completing this section, the following should pass:

1. `cargo check` compiles without errors from `backend/`
2. `cargo test test_router_registration` passes (GET `/api/analyses` returns 200)
3. All existing tests continue to pass (`cargo test`)

---

## Files Modified/Created

| File | Action |
|------|--------|
| `backend/src/features/analysis/routes.rs` | **Create** -- router function + stub handlers |
| `backend/src/features/analysis/mod.rs` | **Modify** -- add `pub mod routes;` |
| `backend/src/lib.rs` | **Modify** -- add `.nest("/analyses", ...)` to `api_routes()` |
| `backend/src/main.rs` | **Modify** -- add analysis tag and paths to OpenAPI |
| `backend/tests/analysis_tests.rs` | **Create** -- router registration test |
| `backend/tests/common/mod.rs` | **Not modified** -- `topics` field already added by Section 01 |

## Implementation Deviations

- **Handler visibility**: All 9 handlers made `pub` (plan didn't specify). Required for utoipa `paths(...)` macro in main.rs to reference them.
- **`routes` module ordering**: Added alphabetically before `tokenizer` in mod.rs (plan said after). Matches convention.
- **Unused imports cleaned**: `delete` and `put` routing functions removed from imports; Axum method chaining doesn't need them as separate imports.