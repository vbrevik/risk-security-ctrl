# Integration Notes: Opus Review Feedback

## Integrating

### CORS restriction (Critical)
**Why:** Cookie-based auth won't work with `allow_origin(Any)`. Must restrict to frontend_url and enable credentials.
**Action:** Add to Section 1 (AppState/wiring) — restrict CORS origin and add `allow_credentials(true)`.

### `session_id` in AuthUser (Issue #6)
**Why:** Logout handler needs session_id to delete session and audit log it. Without this, logout is broken.
**Action:** Add `session_id: String` field to `AuthUser` struct in Section 2, update extractor in Section 5.

### `ip_address` in audit log (Issue #3)
**Why:** Compliance gap — audit_log table has the column, plan's function signature omits it.
**Action:** Update `log_audit` signature in Section 4 to include `ip_address` parameter.

### Session token hashing (Issue #1)
**Why:** Defense in depth for air-gapped SQLite file. Store SHA-256 hash, lookup by hashing presented token.
**Action:** Add to Section 3 (password/session utils) — `hash_session_token()` function using SHA-256.

### LoginRequest validation (Issue #10)
**Why:** Avoid unnecessary Argon2 computation on empty input.
**Action:** Add basic validation to `LoginRequest` (non-empty email/password) in Section 2.

### `.cookie_key` file permissions (Issue #14)
**Why:** Security hygiene for key material.
**Action:** Set 0600 on creation in Section 1.

### COOKIE_KEY minimum length validation (Issue #15)
**Why:** Reject insecure short keys at startup.
**Action:** Add validation in Section 1 cookie key initialization.

### `InvalidCredentials` vs `Unauthorized` clarification (Issue #16)
**Why:** Existing `Unauthorized` already maps to 401; need clear distinction.
**Action:** Use existing `Unauthorized` for missing auth, add `InvalidCredentials` for bad credentials with different error message. Client distinguishes by `error` field value.

## NOT Integrating

### Session garbage collection (Issue #9)
**Why:** Deferred to follow-up. Expired sessions are cleaned on lookup. Table won't grow large enough to matter for a single-instance air-gapped deploy. Can add background task later.

### Rate limiting (Issue #2)
**Why:** Out of scope for 01-backend-core-auth. Better addressed at reverse proxy or middleware level. Note as known gap for 02-rbac-and-hardening.

### Open registration flag (Issue #13)
**Why:** Spec explicitly requires open registration as viewer. This is the spec's decision, not a plan gap.

### Single-session UX acknowledgment (Issue #8)
**Why:** Already in spec. Add brief note in plan but no behavior change.

### `updated_at` handling (Issue #11)
**Why:** Existing pattern in other features handles this. Not auth-specific. Deferred.

### `time` vs `chrono` (Issue #5)
**Why:** The `time` crate is only used for cookie max_age (required by axum-extra). Chrono continues for all other datetime work. No conflict.

### Test section (Issue #12)
**Why:** TDD plan (step 16) will cover this separately. Not needed in the main plan.

---

## Second Review (Opus #2) — Additional Items Integrated

### Add `sha2` as explicit dependency
**Why:** Relying on transitive deps from argon2 is fragile. If argon2 changes internals, build breaks.
**Action:** Added `sha2 = "0.10"` to Section 1 dependencies.

### Wrap DB operations in transactions
**Why:** Login flow (delete sessions + insert + audit + update last_login) should be atomic.
**Action:** Added transaction notes to Section 4 and Section 6.

### Rename cookie from `session_id` to `session`
**Why:** Cookie holds a token, not a session ID. Name was misleading.
**Action:** Renamed throughout Sections 5 and 6.

### Client IP extraction details
**Why:** Behind a reverse proxy, ConnectInfo gives proxy IP.
**Action:** Added note to Section 6 about using ConnectInfo for direct connections and X-Forwarded-For awareness.

### Validator error conversion
**Why:** Need to convert validator::ValidationErrors to AppError.
**Action:** Added note to Section 2 about From impl.

### DateTime format in session validation
**Why:** Must match SQLite's ISO 8601 format for correct comparison.
**Action:** Added note to Section 4 about using datetime('now') in SQL.

### Token in JSON response tradeoff documented
**Why:** Opus review flagged that returning token in body undermines httpOnly cookie security.
**Action:** Added explicit documentation of the tradeoff in Section 6. Token in body is intentional for Bearer-header API clients.

### axum-extra version compatibility check
**Why:** axum-extra 0.10 may not be compatible with axum 0.7.
**Action:** Added note to Section 1 to verify compatibility and pin correct version.

## NOT Integrating (Second Review)

### Cookie key 64 vs 32 bytes (#1) — Plan correctly uses `Key::derive_from()` which accepts ≥32 bytes. No change needed.
### PrivateCookieJar return type (#2) — Plan already shows correct tuple returns. Full type signatures are code, not plan.
### Constant-time comparison (#5) — Already handled by argon2 crate. Over-documenting.
### Open registration (#6) — Deliberate spec decision. Air-gapped mitigates risk.
### is_active check in create_session (#7) — Login handler already checks; extractor also checks. Two layers sufficient.
### FRONTEND_URL exists (#9) — Already noted in CORS section.
### time vs chrono (#15) — Minor. time only for cookie max_age. No conflict.
