Now I have all the context needed to generate the section content.

# Section 1: Dependencies and Configuration

## Overview

This section is the foundation for the entire RBAC & Security Hardening split. It adds new Cargo dependencies (`governor`, `tower_governor`), extends `tower-http` features, adds two new configuration fields (`behind_proxy` and `enable_https`), and gates the Swagger UI behind a cargo feature flag so production builds can exclude it.

No other sections can proceed until this section is complete.

## Background

The application is a Rust/Axum backend (`backend/` directory). The current `Cargo.toml` at `/Users/vidarbrevik/projects/risk-security-ctrl/backend/Cargo.toml` has `tower-http` with features `["cors", "compression-gzip", "trace"]`. The `utoipa-swagger-ui` dependency is unconditional. The `Config` struct in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/config.rs` has six fields. The server in `src/main.rs` currently calls `axum::serve(listener, app).await` without `into_make_service_with_connect_info`.

## Tests

Write these tests first. They verify that the dependency and config changes work correctly.

### Config tests (in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/config.rs`)

Add to the existing `#[cfg(test)] mod tests` block:

```rust
// Test: Config parses BEHIND_PROXY from env var (default false)
#[test]
fn config_defaults_behind_proxy_to_false() {
    let _lock = ENV_LOCK.lock().unwrap();
    env::remove_var("BEHIND_PROXY");
    let config = Config::from_env();
    assert!(!config.behind_proxy);
}

// Test: Config parses BEHIND_PROXY=true
#[test]
fn config_parses_behind_proxy_true() {
    let _lock = ENV_LOCK.lock().unwrap();
    env::set_var("BEHIND_PROXY", "true");
    let config = Config::from_env();
    env::remove_var("BEHIND_PROXY");
    assert!(config.behind_proxy);
}

// Test: Config parses ENABLE_HTTPS from env var (default false)
#[test]
fn config_defaults_enable_https_to_false() {
    let _lock = ENV_LOCK.lock().unwrap();
    env::remove_var("ENABLE_HTTPS");
    let config = Config::from_env();
    assert!(!config.enable_https);
}

// Test: Config parses ENABLE_HTTPS=true
#[test]
fn config_parses_enable_https_true() {
    let _lock = ENV_LOCK.lock().unwrap();
    env::set_var("ENABLE_HTTPS", "true");
    let config = Config::from_env();
    env::remove_var("ENABLE_HTTPS");
    assert!(config.enable_https);
}
```

### Compilation tests (manual verification)

These are verified by running the build commands rather than unit tests:

- `cargo build` should succeed (swagger feature enabled by default)
- `cargo build --no-default-features` should succeed (swagger feature disabled)
- `tower-http` `set-header` and `sensitive-headers` features should compile without errors

### Swagger feature-gating tests (integration, in `/Users/vidarbrevik/projects/risk-security-ctrl/backend/tests/`)

These are better verified as integration tests or manually. The key assertions:

- When compiled with the `swagger` feature (default): the Swagger UI route at `/swagger-ui` returns 200
- When compiled without the `swagger` feature: the route returns 404

## Implementation

### Step 1: Update `Cargo.toml`

File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/Cargo.toml`

Add a `[features]` section and two new dependencies. Modify the existing `tower-http` and `utoipa-swagger-ui` lines.

**Add `[features]` section** (place it before `[dependencies]`):

```toml
[features]
default = ["swagger"]
swagger = ["dep:utoipa-swagger-ui"]
```

**Add new dependencies** to the `[dependencies]` section:

```toml
# Rate limiting
governor = "0.10"
tower_governor = "0.8"
```

**Update `tower-http`** to add `set-header` and `sensitive-headers` features:

```toml
tower-http = { version = "0.6", features = ["cors", "compression-gzip", "trace", "set-header", "sensitive-headers"] }
```

**Make `utoipa-swagger-ui` optional** by adding `optional = true`:

```toml
utoipa-swagger-ui = { version = "8", features = ["axum"], optional = true }
```

### Step 2: Add Config fields

File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/config.rs`

Add two new fields to the `Config` struct:

```rust
pub behind_proxy: bool,
pub enable_https: bool,
```

In `Config::from_env()`, parse them from environment variables with `false` as the default:

```rust
behind_proxy: env::var("BEHIND_PROXY")
    .map(|v| v == "true" || v == "1")
    .unwrap_or(false),
enable_https: env::var("ENABLE_HTTPS")
    .map(|v| v == "true" || v == "1")
    .unwrap_or(false),
```

**Important:** Every place that constructs a `Config` in tests must be updated to include the two new fields. Search the codebase for `Config {` to find all construction sites. Currently these exist in:

- `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/lib.rs` (two test functions: `rejects_short_cookie_key` and `accepts_valid_cookie_key`)

Add `behind_proxy: false, enable_https: false,` to each.

### Step 3: Gate Swagger UI behind feature flag

File: `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/main.rs`

The current `main.rs` unconditionally imports and uses `utoipa_swagger_ui::SwaggerUi`. Wrap these with `#[cfg(feature = "swagger")]`:

1. The `use utoipa_swagger_ui::SwaggerUi;` import should be wrapped:
   ```rust
   #[cfg(feature = "swagger")]
   use utoipa_swagger_ui::SwaggerUi;
   ```

2. The `use utoipa::OpenApi;` import and the `#[derive(OpenApi)]` struct should also be gated since they are only needed for Swagger:
   ```rust
   #[cfg(feature = "swagger")]
   use utoipa::OpenApi;
   ```

3. The `ApiDoc` struct and its derive should be gated:
   ```rust
   #[cfg(feature = "swagger")]
   #[derive(OpenApi)]
   #[openapi(/* ... existing config ... */)]
   struct ApiDoc;
   ```

4. The `.merge(SwaggerUi::new(...))` call in `main()` should be conditionally compiled:
   ```rust
   #[cfg(feature = "swagger")]
   let app = app.merge(
       SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
   );
   ```

   This requires restructuring the `let app = ...` binding slightly. First build the base router, then conditionally merge:
   ```rust
   let app = ontology_backend::create_router(state);

   #[cfg(feature = "swagger")]
   let app = app.merge(
       SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()),
   );
   ```

5. The tracing log about Swagger UI should also be gated:
   ```rust
   #[cfg(feature = "swagger")]
   tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);
   ```

### Step 4: Verify compilation

After making all changes, verify:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/backend
cargo check                      # default features (swagger enabled)
cargo check --no-default-features # swagger disabled
cargo test                       # all tests pass
```

## Files Modified

| File | Change |
|------|--------|
| `backend/Cargo.toml` | Add `[features]`, add `governor`/`tower_governor`, update `tower-http` features, make `utoipa-swagger-ui` optional |
| `backend/src/config.rs` | Add `behind_proxy: bool` and `enable_https: bool` fields, parse from env, add tests |
| `backend/src/main.rs` | Gate Swagger UI imports, `ApiDoc` struct, and `.merge()` behind `#[cfg(feature = "swagger")]` |
| `backend/src/lib.rs` | Update test `Config` construction sites to include new fields |

## Dependencies

- **Depends on:** Nothing (this is the foundation section)
- **Blocks:** All other sections (02 through 08)

## Notes

- The `governor` and `tower_governor` crates are used in Section 4 (Rate Limiting). This section only adds them as dependencies.
- The `set-header` and `sensitive-headers` tower-http features are used in Section 5 (Security Headers). This section only enables them.
- The `into_make_service_with_connect_info` change to `main.rs` is deferred to Section 4 or Section 8, as it is specifically needed for rate limiter IP extraction.