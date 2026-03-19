Now I have all the context needed. Here is the section content:

# Section 11: Prompt Template Endpoints

## Overview

This section implements two handlers in `backend/src/features/analysis/routes.rs` for managing the analysis prompt template (a `MatcherConfig` JSON file). The `get_prompt_template` handler reads the current configuration from disk (or returns defaults), and `update_prompt_template` validates and writes a new configuration, with audit logging.

These handlers are registered on the routes already scaffolded in section 02:
- `GET /api/analyses/prompt-template` -- `get_prompt_template`
- `PUT /api/analyses/prompt-template` -- `update_prompt_template`

## Dependencies

- **Section 02 (Route Scaffold):** The router function and route registrations must exist. The handler stubs created in section 02 are replaced with real implementations here.
- **Existing code:** `MatcherConfig` in `backend/src/features/analysis/matcher.rs` (already implemented with `Default`, `Serialize`, `Deserialize`, `from_json`, and `validate_thresholds`).
- **Existing code:** `AppError` in `backend/src/error.rs` with `BadRequest`, `Internal`, `NotFound` variants.
- **Existing code:** `AppState` in `backend/src/lib.rs`.

## Key Types

The `MatcherConfig` struct (already in `backend/src/features/analysis/matcher.rs`) is the canonical type:

```rust
pub struct MatcherConfig {
    pub version: u32,
    pub min_confidence_threshold: f64,
    pub addressed_threshold: f64,
    pub partial_threshold: f64,
    pub max_findings_per_framework: usize,
    pub include_addressed_findings: bool,
    pub boost_terms: HashMap<String, f64>,
}
```

It has `#[serde(default)]` on the struct and a `Default` impl providing sensible values (e.g., `addressed_threshold: 0.6`, `partial_threshold: 0.3`, `min_confidence_threshold: 0.1`).

## Tests

Tests go in a `#[cfg(test)]` block at the bottom of `backend/src/features/analysis/routes.rs` (or in a dedicated test file if that is the pattern used by section 02). They follow the existing integration test pattern: build a test app via `create_router(state)`, send `.oneshot(Request)`, assert response.

### test_get_prompt_template_returns_defaults

When no config file exists on disk, `GET /api/analyses/prompt-template` returns 200 with a JSON body containing `MatcherConfig::default()` fields. Assert the response has `addressed_threshold`, `partial_threshold`, and `min_confidence_threshold` keys with the expected default values.

### test_update_prompt_template

`PUT /api/analyses/prompt-template` with a valid `MatcherConfig` JSON body returns 200. A subsequent `GET` returns the updated values. For example, send `{"addressed_threshold": 0.8, "partial_threshold": 0.4, "min_confidence_threshold": 0.2, "version": 1, "max_findings_per_framework": 50, "include_addressed_findings": true, "boost_terms": {}}` and verify the GET response reflects these values.

After the test, clean up the written config file to avoid test pollution.

### test_update_prompt_template_invalid_json

`PUT /api/analyses/prompt-template` with a body that cannot deserialize as `MatcherConfig` (e.g., `{"addressed_threshold": "not_a_number"}`) returns 400.

## Implementation Details

### File: `backend/src/features/analysis/routes.rs`

Add two handler functions (replacing the stubs from section 02).

### get_prompt_template

Signature: `async fn get_prompt_template(State(state): State<AppState>) -> AppResult<Json<MatcherConfig>>`

Logic:
1. Attempt to read the file at `backend/config/default-prompt-template.json` using `tokio::fs::read_to_string`.
2. If the file exists and reads successfully, deserialize its contents as `MatcherConfig` via `serde_json::from_str`. If deserialization fails, log a warning and return `MatcherConfig::default()`.
3. If the file does not exist (io::ErrorKind::NotFound), return `MatcherConfig::default()` serialized as JSON.
4. For other I/O errors, return `AppError::Internal`.
5. Return `Ok(Json(config))`.

The config file path should be resolved relative to the working directory. Consider defining it as a constant:

```rust
const PROMPT_TEMPLATE_PATH: &str = "config/default-prompt-template.json";
```

### update_prompt_template

Signature: `async fn update_prompt_template(State(state): State<AppState>, Json(body): Json<serde_json::Value>) -> AppResult<Json<MatcherConfig>>`

Logic:
1. Accept the body as `serde_json::Value` first (not directly as `MatcherConfig`) so that malformed input can be caught and returned as a 400 rather than Axum's default 422.
2. Attempt `serde_json::from_value::<MatcherConfig>(body)`. On failure, return `AppError::BadRequest("Invalid prompt template configuration")`.
3. Optionally call the config's `validate_thresholds()` method (it logs warnings but does not reject -- thresholds outside valid ranges are warned about but still saved, matching existing behavior in `MatcherConfig::from_json`).
4. Ensure the parent directory exists: `tokio::fs::create_dir_all("config").await`.
5. Serialize the validated config to pretty JSON: `serde_json::to_string_pretty(&config)`.
6. Write to a temporary file first, then rename atomically to prevent partial writes on concurrent access. Use `tokio::fs::write` to a temp path like `config/default-prompt-template.json.tmp`, then `tokio::fs::rename` to the final path. This provides the file-lock-like safety mentioned in the plan without requiring an actual file lock crate.
7. Insert an audit log entry following the existing pattern from `compliance/routes.rs`:
   ```rust
   sqlx::query(
       r#"INSERT INTO audit_log (id, user_id, action, entity_type, entity_id, new_value, created_at)
          VALUES (?, NULL, 'prompt_template_updated', 'prompt_template', 'default', ?, ?)"#
   )
   ```
   Bind a new UUID, the serialized config as `new_value`, and the current timestamp.
8. Return `Ok(Json(config))`.

### Error Handling

- File read failures (other than NotFound): `AppError::Internal("Failed to read prompt template")`
- File write failures: `AppError::Internal("Failed to save prompt template")`
- Deserialization failures on PUT: `AppError::BadRequest("Invalid prompt template configuration: ...")`
- Directory creation failure: `AppError::Internal("Failed to create config directory")`

### Audit Logging

Only the `update_prompt_template` handler logs an audit entry. The `get_prompt_template` handler is a read-only operation and does not create audit entries. The audit event uses:
- `action`: `"prompt_template_updated"`
- `entity_type`: `"prompt_template"`
- `entity_id`: `"default"` (there is only one prompt template)
- `new_value`: the full serialized `MatcherConfig` JSON

## File Paths Summary

- **Modified:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/routes.rs` -- add `get_prompt_template` and `update_prompt_template` handler implementations
- **Created at runtime:** `backend/config/default-prompt-template.json` -- written by `update_prompt_template`, read by `get_prompt_template`
- **Read (existing):** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/matcher.rs` -- `MatcherConfig` struct with `Default`, `Serialize`, `Deserialize`
- **Read (existing):** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/error.rs` -- `AppError` variants