Now I have all the context needed. Let me generate the section content.

# Section 1: Dependencies and AppState Updates

## Overview

This section adds the Cargo dependencies required by the auth system, extends `AppState` with a cookie encryption key, updates the `Config` struct with auth-related settings, implements cookie key initialization logic, and fixes the CORS configuration to support cookie-based authentication. This section has no dependencies and blocks all other auth sections.

## Files to Create or Modify

| File | Action |
|------|--------|
| `backend/Cargo.toml` | Modify -- add new dependencies |
| `backend/src/config.rs` | Modify -- add `cookie_key` and `session_duration_hours` fields |
| `backend/src/lib.rs` | Modify -- add `cookie_key` field to `AppState`, implement `FromRef`, update CORS |
| `backend/src/main.rs` | Modify -- initialize cookie key before constructing `AppState` |
| `backend/.gitignore` | Modify -- add `.cookie_key` |
| `backend/tests/common/mod.rs` | Modify -- update `create_test_app` to include `cookie_key` |

## Tests First

These tests belong inline in `backend/src/config.rs` (unit tests) and in the existing test infrastructure.

### Unit test: Config parses COOKIE_KEY from env

```rust
/// In backend/src/config.rs, #[cfg(test)] mod tests
///
/// Set COOKIE_KEY env var to a valid 64-char hex string.
/// Call Config::from_env().
/// Assert cookie_key is Some and decoded bytes length >= 32.
#[test]
fn config_parses_cookie_key_from_env() { todo!() }
```

### Unit test: Config rejects short COOKIE_KEY

```rust
/// Set COOKIE_KEY to a 16-char hex string (only 8 bytes).
/// Assert that cookie key validation fails with a clear error message.
/// This validation happens at startup, not in Config::from_env itself --
/// the init_cookie_key function should reject it.
#[test]
fn rejects_short_cookie_key() { todo!() }
```

### Integration test: CORS layer includes credentials

```rust
/// In backend/tests/ or inline integration test.
/// Build the app router via create_test_app().
/// Send an OPTIONS preflight request with Origin header.
/// Assert response includes Access-Control-Allow-Credentials: true.
/// Assert Access-Control-Allow-Origin is NOT "*" (must be a specific origin).
#[tokio::test]
async fn cors_includes_credentials() { todo!() }
```

## Implementation Details

### 1. Add Cargo Dependencies

Add the following to `backend/Cargo.toml` under `[dependencies]`:

```toml
# Auth
argon2 = "0.5"
axum-extra = { version = "0.10", features = ["cookie-private", "cookie-key-expansion"] }
validator = { version = "0.20", features = ["derive"] }
rand = "0.9"
hex = "0.4"
time = "0.3"
sha2 = "0.10"
```

Note: `sha2` is listed explicitly even though it is a transitive dependency of `argon2`, because session token hashing (Section 3) uses it directly and an explicit dependency ensures version stability.

**Deviation:** Added `cookie-key-expansion` feature to `axum-extra` — required for `Key::derive_from()` which uses HKDF to derive a 512-bit key from a shorter (>= 32 byte) master secret. Without this feature, `Key::from()` requires the caller to provide a full 64-byte key directly.

### 2. Update Config Struct

In `backend/src/config.rs`, add two new fields to `Config`:

- `cookie_key: Option<String>` -- raw hex string from `COOKIE_KEY` env var. `None` means the key will be loaded from file or auto-generated at startup.
- `session_duration_hours: u64` -- from `SESSION_DURATION_HOURS` env var, default `8`.

The existing `frontend_url` field (already present, defaulting to `http://localhost:5173`) is used for CORS origin. No new field needed for that.

Updated `from_env` should read:

```rust
cookie_key: env::var("COOKIE_KEY").ok(),
session_duration_hours: env::var("SESSION_DURATION_HOURS")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(8),
```

### 3. Update AppState

In `backend/src/lib.rs`, add a new field to the `AppState` struct:

```rust
pub cookie_key: axum_extra::extract::cookie::Key,
```

This requires importing `axum_extra::extract::cookie::Key`.

Then implement `FromRef` so that `PrivateCookieJar` can extract the key from state:

```rust
impl axum::extract::FromRef<AppState> for axum_extra::extract::cookie::Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}
```

### 4. Cookie Key Initialization

Add a public function `init_cookie_key` in `backend/src/lib.rs` (or a new `backend/src/auth_init.rs` if preferred, but `lib.rs` is simpler since it is already where `AppState` lives). The function signature:

```rust
/// Initialize the cookie encryption key from env, file, or auto-generation.
/// Returns the Key suitable for PrivateCookieJar AES-GCM encryption.
pub fn init_cookie_key(config: &Config) -> axum_extra::extract::cookie::Key { ... }
```

Logic:

1. If `config.cookie_key` is `Some(hex_string)`:
   - Decode hex to bytes via `hex::decode`.
   - Validate that decoded length >= 32 bytes. If shorter, **panic with a clear message** (this is a startup-time fatal misconfiguration).
   - Call `Key::derive_from(&bytes)` and return.

2. If `config.cookie_key` is `None`:
   - Check if `.cookie_key` file exists in the current working directory.
   - If the file exists:
     - Read its contents, trim whitespace, hex-decode.
     - On Unix, check file permissions. If mode is more permissive than `0600`, log a warning via `tracing::warn!`.
     - Call `Key::derive_from(&bytes)` and return.
   - If the file does not exist:
     - Generate 32 random bytes using `rand::Rng::fill` with `rand::rng()` (the `rand 0.9` API).
     - Hex-encode the bytes.
     - Write the hex string to `.cookie_key`.
     - On Unix, set file permissions to `0600` using `std::os::unix::fs::PermissionsExt`.
     - Log `tracing::warn!("Generated new cookie key and saved to .cookie_key. Set COOKIE_KEY env var for production.")`.
     - Call `Key::derive_from(&bytes)` and return.

For the Unix permission check on existing files, use conditional compilation:

```rust
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let mode = std::fs::metadata(".cookie_key")?.permissions().mode();
    if mode & 0o077 != 0 {
        tracing::warn!(".cookie_key file has overly permissive permissions ({:o}), recommend 0600", mode);
    }
}
```

### 5. Update CORS Configuration

In `backend/src/lib.rs`, the `create_router` function currently uses `CorsLayer::new().allow_origin(Any)`. This must change to support cookie authentication. Browsers will not send cookies on cross-origin requests when `Access-Control-Allow-Origin` is `*`.

Update `create_router` to accept or derive the frontend URL and build CORS accordingly:

```rust
let cors = CorsLayer::new()
    .allow_origin(
        state.config.frontend_url
            .parse::<http::header::HeaderValue>()
            .expect("Invalid FRONTEND_URL for CORS origin"),
    )
    .allow_methods([
        http::Method::GET, http::Method::POST, http::Method::PUT,
        http::Method::DELETE, http::Method::PATCH, http::Method::OPTIONS,
    ])
    .allow_headers(tower_http::cors::AllowHeaders::mirror_request())
    .allow_credentials(true);
```

**Deviation from original plan:** The plan used `allow_methods(Any)` and `allow_headers(Any)`, but the CORS spec forbids wildcard `*` responses with `allow_credentials(true)`. Browsers reject such preflights. Fixed during code review to use explicit methods and `mirror_request()` for headers.

### 6. Update main.rs

In `backend/src/main.rs`, after loading `Config` and before constructing `AppState`, call:

```rust
let cookie_key = ontology_backend::init_cookie_key(&config);
```

Then include it in the `AppState` construction:

```rust
let state = AppState {
    db,
    config: config.clone(),
    topics,
    cookie_key,
};
```

### 7. Update .gitignore

Add to `backend/.gitignore`:

```
# Auth cookie key (auto-generated)
.cookie_key
```

### 8. Update Test Helper

In `backend/tests/common/mod.rs`, the `create_test_app` function constructs `AppState` and must now include `cookie_key`. For tests, generate a deterministic key:

```rust
let cookie_key = axum_extra::extract::cookie::Key::generate();
```

Add it to the `AppState` construction:

```rust
let state = AppState {
    db: pool,
    config: config.clone(),
    topics,
    cookie_key,
};
```

This uses `Key::generate()` which produces a random key each test run. This is fine for tests -- they do not need persistence across runs.

## Dependencies on Other Sections

None. This is the foundational section that all other auth sections depend on.

## Verification Checklist

After implementation, confirm:

1. `cargo check` passes with all new dependencies resolved.
2. The three tests described above pass.
3. The existing test suite still compiles and passes (the `AppState` change touches `create_test_app`).
4. The CORS preflight response for `http://localhost:5173` includes `Access-Control-Allow-Credentials: true` and a specific origin (not `*`).
5. Starting the server without `COOKIE_KEY` set creates a `.cookie_key` file and logs a warning.
6. Starting the server with a too-short `COOKIE_KEY` panics with a clear error.