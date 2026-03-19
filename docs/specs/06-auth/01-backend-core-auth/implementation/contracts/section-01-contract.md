# Prompt Contract: Section 01 — Dependencies and AppState

## GOAL
Add auth-related Cargo dependencies, extend Config with cookie_key and session_duration_hours, add cookie_key to AppState with FromRef impl, implement cookie key initialization (env/file/auto-gen), update CORS for credentials, update test helper.

## CONTEXT
Section 01 is the foundation for all auth sections. No auth code depends on anything else yet — this purely sets up infrastructure.

## CONSTRAINTS
- All new deps must have exact major versions matching section spec
- Cookie key must be >= 32 bytes or panic at startup
- CORS must use specific origin (not `*`) and `allow_credentials(true)`
- `.cookie_key` file must be 0600 on Unix
- `Key::derive_from` (not `Key::from`) for user-provided keys
- Existing tests must continue to compile and pass after AppState changes

## FORMAT
Files to modify:
- `backend/Cargo.toml` — add 7 auth dependencies
- `backend/src/config.rs` — add cookie_key, session_duration_hours fields + tests
- `backend/src/lib.rs` — add cookie_key to AppState, FromRef impl, init_cookie_key fn, CORS update
- `backend/src/main.rs` — call init_cookie_key, pass to AppState
- `backend/.gitignore` — add .cookie_key
- `backend/tests/common/mod.rs` — add cookie_key to test AppState

## FAILURE CONDITIONS
- SHALL NOT leave CORS with `allow_origin(Any)` — breaks cookie auth
- SHALL NOT allow cookie keys shorter than 32 bytes without panicking
- SHALL NOT break existing test compilation (AppState field must be added to test helper)
- SHALL NOT skip unit tests for Config parsing and key validation
