<!-- PROJECT_CONFIG
runtime: rust-cargo
test_command: cargo test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-deps-and-appstate
section-02-auth-models
section-03-password-and-session-utils
section-04-auth-service
section-05-auth-extractor
section-06-route-handlers
section-07-seed-admin
section-08-wiring-and-integration
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-deps-and-appstate | - | all | Yes |
| section-02-auth-models | 01 | 03, 04, 05, 06 | Yes |
| section-03-password-and-session-utils | 01, 02 | 04, 05, 06 | Yes |
| section-04-auth-service | 01, 02, 03 | 05, 06, 07 | No |
| section-05-auth-extractor | 01, 02, 04 | 06 | No |
| section-06-route-handlers | 01-05 | 08 | No |
| section-07-seed-admin | 01, 03, 04 | 08 | Yes (parallel with 06) |
| section-08-wiring-and-integration | 06, 07 | - | No |

## Execution Order

1. section-01-deps-and-appstate (no dependencies)
2. section-02-auth-models (after 01)
3. section-03-password-and-session-utils (after 02)
4. section-04-auth-service (after 03)
5. section-05-auth-extractor, section-07-seed-admin (parallel after 04)
6. section-06-route-handlers (after 05)
7. section-08-wiring-and-integration (final)

## Section Summaries

### section-01-deps-and-appstate
Add Cargo dependencies (argon2, axum-extra, validator, rand, sha2, hex, time). Update AppState with cookie_key. Cookie key initialization with validation. CORS configuration update. Config struct changes.

### section-02-auth-models
Request types (RegisterRequest, LoginRequest) with validation derives. Response types (AuthResponse, UserProfile). AuthUser struct with session_id. AppError extensions (InvalidCredentials, ValidationError).

### section-03-password-and-session-utils
Argon2id password hashing/verification. CSPRNG session token generation. SHA-256 session token hashing. Unit tests for all utilities.

### section-04-auth-service
User CRUD operations (create, find by email/id). Session management (create with hash, validate, delete, single-session enforcement). Audit logging with ip_address. Integration tests with in-memory SQLite.

### section-05-auth-extractor
AuthUser FromRequestParts implementation. Cookie extraction via PrivateCookieJar. Bearer header extraction. Session validation and user loading. Returns AuthUser with session_id populated.

### section-06-route-handlers
Register, login, logout, me handlers. OpenAPI annotations. Cookie setting on login. Audit log integration. Full flow integration tests.

### section-07-seed-admin
Separate binary for bootstrapping admin user. Reads from env vars. Idempotent (skip if exists). Uses shared password hashing.

### section-08-wiring-and-integration
Module structure (mod.rs). Router mounting at /api/auth. OpenAPI registration. End-to-end integration tests. Full lifecycle test.
