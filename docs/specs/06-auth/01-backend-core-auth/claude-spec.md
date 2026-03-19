# Backend Core Auth — Synthesized Specification

## Overview

Implement the foundational authentication system for risk-security-ctrl: user registration, login, logout, session management, and an Axum auth extractor. This runs in an air-gapped government environment managing sensitive IT security compliance data.

## System Context

### Existing Infrastructure
- **Database:** SQLite3 via SQLx 0.8. Tables `users`, `sessions`, `audit_log` already exist with indexes.
- **Web framework:** Axum 0.7 with tower middleware stack.
- **Feature structure:** `backend/src/features/auth/` exists with placeholder routes.
- **Error handling:** `AppError` enum in `src/error.rs` with `Unauthorized`/`Forbidden` variants.
- **OpenAPI:** utoipa 5 with Swagger UI.
- **State:** `AppState` holds `SqlitePool` and `topics: HashMap`.

### Dependencies to Add
| Crate | Version | Purpose |
|-------|---------|---------|
| `argon2` | 0.5 | Argon2id password hashing (RustCrypto) |
| `axum-extra` | 0.10+ | `cookie-private` for AES-GCM encrypted session cookies |
| `validator` | 0.20 | Input validation with derive macros |
| `rand` | 0.9+ | CSPRNG for session token generation |
| `hex` | 0.4 | Decoding COOKIE_KEY from hex string |

## Endpoints

### POST /api/auth/register
- **Auth:** None required
- **Body:** `{ "email": "...", "name": "...", "password": "..." }`
- **Validation:**
  - Email: valid format (validator crate)
  - Password: min 8 characters (NIST 800-63B — length over complexity)
  - Name: non-empty, max 255 chars
- **Behavior:**
  1. Validate input
  2. Hash password with Argon2id (default OWASP params: 19 MiB, 2 iterations, 1 parallelism)
  3. Generate UUID for user ID
  4. Insert into `users` table with role=viewer, is_active=1
  5. Return 201 with `{ "id": "...", "email": "...", "name": "...", "role": "viewer" }`
- **Error:** If email exists, return **generic 422** — do NOT reveal whether email is registered (prevents account enumeration)

### POST /api/auth/login
- **Auth:** None required
- **Body:** `{ "email": "...", "password": "..." }`
- **Behavior:**
  1. Look up user by email
  2. Verify password against stored Argon2id hash
  3. If user not found or password wrong: return 401 `{ "error": "unauthorized", "message": "Invalid credentials" }`
  4. If user is_active=0: return 403 `{ "error": "forbidden", "message": "Account disabled" }`
  5. Invalidate any existing session for this user (single-session enforcement)
  6. Generate CSPRNG session token (256-bit, hex-encoded)
  7. Insert into `sessions` table with 8-hour expiry, client IP, user agent
  8. Update `last_login_at` on user record
  9. Log to `audit_log` (action=login)
  10. Set encrypted httpOnly cookie with session token
  11. Return 200 with `{ "token": "...", "user": { "id", "email", "name", "role" } }`
- **Note:** Returns token in both cookie (for SPA) and response body (for API consumers using Bearer header)

### POST /api/auth/logout
- **Auth:** Required (AuthUser extractor)
- **Behavior:**
  1. Delete session from `sessions` table
  2. Log to `audit_log` (action=logout)
  3. Remove session cookie
  4. Return 204

### GET /api/auth/me
- **Auth:** Required (AuthUser extractor)
- **Response:** `{ "id": "...", "email": "...", "name": "...", "role": "..." }`
- **Fields:** Minimal — id, email, name, role only. No sensitive or metadata fields.

## AuthUser Extractor

Custom Axum `FromRequestParts<S>` implementation:

1. Check `Authorization: Bearer <token>` header
2. If no header, check encrypted `session_id` cookie via `PrivateCookieJar`
3. If neither present: reject with 401
4. Look up session in DB by token
5. Check session not expired (created_at + 8h > now)
6. If expired: delete session, reject with 401
7. Load user by session.user_id
8. Check user.is_active
9. Return `AuthUser { id, email, name, role }`

**AppState requirements:**
- `SqlitePool` accessible via `FromRef`
- `Key` (cookie encryption key) accessible via `FromRef`

## Cookie Key Management

1. Check env var `COOKIE_KEY` (hex-encoded, 32+ bytes)
2. If not set, check file `.cookie_key` in working directory
3. If file doesn't exist, generate random 32-byte key, save as hex to `.cookie_key`
4. Use `Key::derive_from(&master_key)` to derive signing + encryption keys

**Security:** `.cookie_key` file should be in `.gitignore`. Log a warning if auto-generating.

## CLI Admin Seed

Separate binary: `[[bin]] name = "seed-admin"`

**Usage:** `cargo run --bin seed-admin`

**Reads from env vars or CLI args:**
- `ADMIN_EMAIL` (required)
- `ADMIN_PASSWORD` (required)
- `ADMIN_NAME` (optional, defaults to "Admin")

**Behavior:**
1. Connect to database (same `DATABASE_URL` config)
2. Check if user with email already exists
3. If exists: print warning and exit
4. Hash password with Argon2id
5. Insert user with role=admin
6. Print success with user ID

## Audit Log Integration

For login/logout events, insert into `audit_log`:
- `user_id`: the authenticated user's ID
- `action`: "login" or "logout"
- `entity_type`: "session"
- `entity_id`: the session ID
- `created_at`: current timestamp

## Testing Strategy

- **Unit:** Argon2id hash/verify round-trip, session token generation uniqueness
- **Integration:** Register → login → me → logout full flow
- **Integration:** Single-session enforcement (login invalidates previous session)
- **Integration:** Expired session rejection
- **Integration:** Bearer header auth works same as cookie auth
- **Integration:** Disabled user (is_active=0) rejected at login
- **Integration:** Duplicate email returns generic 422
