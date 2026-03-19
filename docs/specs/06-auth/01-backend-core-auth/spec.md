# 01 — Backend Core Auth

## Purpose

Implement user registration, login, logout, session management, and the `AuthUser` Axum extractor. This is the foundation that all other auth splits build on.

## Requirements Reference

See `../requirements.md` sections: Backend Authentication (items 1-5), Security Requirements (items 12-13).

## Key Decisions (from interview)

- **Password hashing:** Argon2id via `argon2` crate (RustCrypto)
- **Session tokens:** CSPRNG via `rand` + `getrandom`, min 256-bit
- **Token transport:** Both httpOnly cookie (AES-GCM via `axum-extra` cookie-private) AND Bearer header. Cookie takes precedence.
- **Registration:** Open — anyone can register as viewer role
- **Sessions:** 8-hour expiry, single session per user (new login invalidates previous)
- **Admin bootstrap:** CLI seed command reading from env vars or CLI args
- **Audit logging:** Login/logout events written to existing `audit_log` table

## Scope

### Dependencies to Add
- `argon2` (RustCrypto password-hash)
- `rand` (CSPRNG for session tokens)
- `axum-extra` with `cookie-private` feature
- `validator` with derive feature

### Endpoints
| Method | Path | Auth | Description |
|--------|------|------|-------------|
| POST | /api/auth/register | None | Create user account (viewer role) |
| POST | /api/auth/login | None | Verify credentials, create session |
| POST | /api/auth/logout | Required | Invalidate current session |
| GET | /api/auth/me | Required | Return current user profile |

### Core Components
1. **User operations** — create user with argon2id hash, verify password, lookup by email/id
2. **Session operations** — create with CSPRNG token, validate (check expiry + existence), invalidate, enforce single-session (delete previous on login)
3. **AuthUser extractor** — Axum `FromRequestParts` impl that checks cookie first, then Authorization Bearer header, loads user from DB
4. **CLI seed** — standalone binary or cargo subcommand to create admin user
5. **Audit integration** — log login/logout to `audit_log` table with user_id and action

### Error Types
Extend existing `AppError` in `src/error.rs`:
- InvalidCredentials (401)
- EmailAlreadyExists (409)
- InvalidInput with field-level validation errors (422)
- SessionExpired (401)

## Existing Infrastructure

- **DB schema exists:** `users`, `sessions`, `audit_log` tables with indexes — no migrations needed
- **Error format:** `{ "error": "...", "message": "..." }` from `src/error.rs`
- **Feature module:** `backend/src/features/auth/` exists with placeholder `mod.rs` and `routes.rs`
- **OpenAPI:** Must use utoipa annotations; existing pattern in `features/ontology/routes.rs`

## Provides to Other Splits

- `AuthUser` extractor — used by 02-rbac-and-hardening to build role guards
- Session model/operations — used by 02 for cleanup and by 03 for understanding session behavior
- Auth API contract — used by 03-frontend-auth to build login/register forms

## Testing Strategy

- Unit tests for password hashing round-trip
- Unit tests for session token generation uniqueness
- Integration tests for register → login → me → logout flow
- Integration test for single-session enforcement
- Test expired session rejection
