# STIG Compliance Report -- Full Baseline

**Date**: 2026-03-19
**Scope**: Full project scan (all categories)
**Controls checked**: 53
**Result**: 8 passed, 30 failed, 11 N/A, 4 manual

---

## Auth

### FAIL: V-222425 -- Enforce approved authorizations for logical access (CAT I)
**File**: backend/src/features/auth/routes.rs:12-16
**Finding**: No RBAC middleware implemented. Auth routes are placeholder-only. Feature endpoints lack role guards.
**Remediation**: Implement RBAC guard middleware. Add RequireRole extractor that checks user role against permission matrix before allowing endpoint access.

### FAIL: V-222426 -- Enforce DAC policies over subjects and objects (CAT I)
**File**: backend/migrations/001_initial_schema.sql:114-122
**Finding**: No discretionary access control or object-level access policies implemented.
**Remediation**: Design and implement object-level access control with resource ownership tracking.

### FAIL: V-222432 -- Enforce 3 invalid logon attempts in 15 min (CAT I)
**File**: backend/Cargo.toml (missing governor crate)
**Finding**: No login rate limiting or failed attempt tracking implemented.
**Remediation**: Add rate limiting using `governor` crate with per-IP limits. Track and enforce 3-attempt limit per 15-minute window.

### FAIL: V-222520 -- Require reauthentication for sensitive ops (CAT II)
**Finding**: No sensitive operation reauthentication implemented. All endpoints use single session token.
**Remediation**: Define sensitive operations and implement reauthentication check requiring password verification.

### N/A: V-222524 -- Accept PIV credentials (CAT II)
**Requires**: PIV/CAC authentication out of scope per current spec. Future phase (OAuth/SSO).

### FAIL: V-222530 -- Replay-resistant auth for privileged accounts (CAT I)
**Finding**: No replay-resistant mechanisms. Session tokens are static across requests.
**Remediation**: Implement cryptographic binding between session token and client, or add per-request nonces/HMAC signatures.

### FAIL: V-222531 -- Replay-resistant auth for non-privileged accounts (CAT I)
**Finding**: Same as V-222530; affects all account types.
**Remediation**: Implement replay resistance globally.

### FAIL: V-222536 -- Enforce 15-character minimum password (CAT I)
**Finding**: Spec sets minimum at 8 characters (NIST 800-63B), not 15.
**Remediation**: Increase password minimum length to 15 characters, or document NIST 800-63B deviation.

### FAIL: V-222538 -- Enforce password complexity (CAT I)
**Finding**: No password complexity requirements. Spec follows NIST 800-63B "length over complexity."
**Remediation**: Add character class requirements, or document accepted deviation from STIG in favor of NIST 800-63B.

### FAIL: V-222542 -- Only store cryptographic password representations (CAT I)
**File**: backend/migrations/001_initial_schema.sql:104
**Finding**: Schema defines `password_hash TEXT NOT NULL` (correct), but Argon2id hashing not yet implemented.
**Remediation**: Implement Argon2id password hashing. Verify no plaintext passwords reach logs or responses.

### FAIL: V-222543 -- Transmit only cryptographically-protected passwords (CAT I)
**File**: backend/src/main.rs:163
**Finding**: Server binds to HTTP (no TLS). CORS allows any origin.
**Remediation**: Deploy with reverse proxy providing HTTPS, or add TLS directly to Axum. Enforce HTTPS-only.

### FAIL: V-222544 -- Enforce 24-hour minimum password lifetime (CAT II)
**Finding**: No password lifetime management. No `password_changed_at` field.
**Remediation**: Add timestamp tracking and enforce 24-hour minimum between password changes.

### FAIL: V-222545 -- Enforce 60-day maximum password lifetime (CAT II)
**Finding**: No password expiry tracking.
**Remediation**: Add `password_expires_at` and force reset after 60 days.

### FAIL: V-222546 -- Prohibit password reuse for 5 generations (CAT II)
**Finding**: No password history table.
**Remediation**: Create `password_history` table storing last 5 hashes. Check before allowing change.

### FAIL: V-222547 -- Allow temp password with immediate change (CAT II)
**Finding**: No temporary password or admin reset flow.
**Remediation**: Implement admin endpoint for temp password with `must_change_password` flag.

### N/A: V-222552 -- Map authenticated identity for PKI auth (CAT II)
**Requires**: PKI auth not in scope.

---

## Session Management

### FAIL: V-222577 -- Must not expose session IDs (CAT I)
**Finding**: Spec returns token in both cookie and response body. Body exposure is a risk.
**Remediation**: Return session token ONLY in httpOnly cookie. Remove from JSON response body.

### FAIL: V-222578 -- Destroy session ID on logoff (CAT I)
**File**: backend/src/features/auth/routes.rs
**Finding**: Logout not yet implemented. Only placeholder `/me` route exists.
**Remediation**: Implement POST /api/auth/logout that deletes session from DB and clears cookie.

### FAIL: V-222579 -- Generate unique session ID per session (CAT II)
**Finding**: CSPRNG session generation designed but not implemented.
**Remediation**: Add `rand` crate with CSPRNG token generation. Verify UNIQUE constraint on sessions.token.

### PASS: V-222581 -- Not use URL-embedded session IDs (CAT I)
**Evidence**: Design specifies cookie and Authorization header only. No URL embedding.

### PASS: V-222582 -- Not reuse/recycle session IDs (CAT I)
**Evidence**: Design includes single-session enforcement (invalidates previous session on new login).

### MANUAL: V-222583 -- Use FIPS 140-2 validated crypto modules (CAT II)
**Requires**: Verify FIPS requirement with stakeholders. Rust crypto crates are not FIPS-certified. If required, replace with OpenSSL FIPS or AWS-LC.

---

## Input Validation

### FAIL: V-222606 -- Validate all input (CAT I)
**File**: backend/src/features/analysis/routes.rs:392-400
**Finding**: `get_findings()` builds SQL via string formatting with user-supplied parameters using brittle escaping.
**Remediation**: Replace string formatting with SQLx parameter binding for all dynamic query construction.

### PASS: V-222609 -- Not subject to input handling vulnerabilities (CAT I)
**Evidence**: File upload validates extension allowlist, magic bytes, and size limits. Other inputs use proper SQLx binding.

### PASS: V-222612 -- Not vulnerable to overflow attacks (CAT I)
**Evidence**: Rust memory safety. Page/limit bounded. File size limit 20MB enforced.

### PASS: V-222605 -- Protect from canonical representation vulnerabilities (CAT II)
**Evidence**: UUIDs used for all IDs. File paths validated via UUID parsing. No path traversal vectors.

---

## Injection

### PASS: V-222602 -- Protect from XSS (CAT I)
**Evidence**: React auto-escapes all rendered content. No unsafe innerHTML patterns found.

### MANUAL: V-222603 -- Protect from CSRF (CAT I)
**Requires**: Currently stateless REST API (N/A). Must implement CSRF tokens if cookie-based auth is added. CORS `.allow_origin(Any)` should be restricted.

### PASS: V-222604 -- Protect from command injection (CAT I)
**Evidence**: No `std::process::Command` or shell execution found in codebase.

### FAIL: V-222607 -- Not vulnerable to SQL Injection (CAT I)
**File**: backend/src/features/analysis/routes.rs:392-400
**Finding**: CRITICAL. `get_findings()` constructs SQL with user input via string formatting. `.replace` escaping is insufficient.
**Remediation**: Use SQLx dynamic query building with proper bind variables.

### PASS: V-222608 -- Not vulnerable to XML attacks (CAT I)
**Evidence**: `quick_xml::Reader` does not process DTDs. No external entity loading.

---

## Error Handling

### PASS: V-222585 -- Fail to a secure state (CAT I)
**Evidence**: All database errors caught and converted to generic responses. Errors never directly exposed to client.

### PASS: V-222610 -- Error messages don't reveal exploitable info (CAT II)
**Evidence**: Returns generic messages to users. Internal errors logged via tracing server-side only.

### MANUAL: V-222611 -- Error messages only to authorized admins (CAT II)
**Requires**: Auth not yet implemented. Once implemented, verify logs accessible only to authorized admin roles.

### PASS: V-222586 -- Preserve failure information for analysis (CAT II)
**Evidence**: All errors logged via tracing framework with structured logging and context.

---

## Information Disclosure

### MANUAL: V-222601 -- Not store sensitive info in hidden fields (CAT I)
**Requires**: Verify prompt template config at `config/default-prompt-template.json` doesn't expose sensitive data to clients.

### FAIL: V-222596 -- Protect confidentiality/integrity of transmitted info (CAT II)
**File**: frontend/vite.config.ts:18-21
**Finding**: Frontend connects to backend via HTTP. No HTTPS enforcement.
**Remediation**: Require HTTPS in production. Implement HSTS headers.

### FAIL: V-222598 -- Confidentiality during preparation for transmission (CAT II)
**File**: backend/src/features/analysis/routes.rs:66,207
**Finding**: Analysis text stored in plaintext SQLite without encryption.
**Remediation**: Implement application-layer encryption for sensitive text fields.

### FAIL: V-222599 -- Confidentiality during reception (CAT II)
**File**: backend/src/features/analysis/routes.rs:138-162
**Finding**: Upload error messages may leak system paths via `AppError::Internal(e.to_string())`.
**Remediation**: Return generic "Upload failed" to client; log details server-side with correlation IDs.

### FAIL: V-222588 -- Approved crypto for data at rest (CAT I)
**Finding**: SQLite database and uploaded files stored in plaintext without encryption.
**Remediation**: Implement SQLite encryption (SQLCipher) and encrypt uploaded files.

### FAIL: V-222597 -- Crypto to prevent unauthorized disclosure in transit (CAT I)
**File**: backend/src/lib.rs:69-72
**Finding**: No TLS enforcement. CORS allows `Any` origin.
**Remediation**: Restrict CORS to known frontend origin. Implement TLS 1.2+.

---

## Cryptography

### N/A: V-222570 -- FIPS-validated modules for signing (CAT I)
**Reason**: No code signing implemented.

### N/A: V-222571 -- FIPS-validated modules for hashing (CAT I)
**Reason**: No cryptographic hashing currently used beyond planned Argon2id.

### N/A: V-222555 -- Federal requirements for crypto module auth (CAT I)
**Reason**: No FIPS certification scope at this stage.

### N/A: V-222573 -- SAML FIPS-approved random numbers (CAT II)
**Reason**: No SAML implementation.

### N/A: V-222553 -- Local cache of PKI revocation data (CAT II)
**Reason**: No PKI implementation.

---

## Audit Logging

### PASS: V-222474 -- Audit records with sufficient information (CAT II)
**Evidence**: audit_log table includes id, user_id, action, entity_type, entity_id, old_value, new_value, ip_address, created_at.

### FAIL: V-222475 -- Include outcome in audit records (CAT II)
**File**: backend/src/features/analysis/routes.rs:583-591
**Finding**: Audit INSERT does not record success/failure outcome.
**Remediation**: Add `outcome` column. Record HTTP status or error reason.

### FAIL: V-222483 -- Warn when audit storage reaches 75% (CAT II)
**Finding**: No audit storage monitoring implemented.
**Remediation**: Add periodic background task checking audit_log size with threshold alerting.

### FAIL: V-222485 -- Alert on audit processing failure (CAT I)
**File**: backend/src/features/analysis/routes.rs:593
**Finding**: Audit write failures logged as warning but don't alert. Export audit is "best-effort."
**Remediation**: Implement alerting mechanism. Do not silently suppress audit failures.

### FAIL: V-222487 -- Central audit review capability (CAT II)
**Finding**: Audit table exists but no general review/search endpoints exposed.
**Remediation**: Add `GET /api/audit-log` with filtering by user, action, entity_type, date range.

### FAIL: V-222489 -- Audit reduction/reporting capability (CAT II)
**Finding**: No audit report generation or export tools.
**Remediation**: Implement audit report endpoint with CSV/PDF export.

---

## Configuration

### FAIL: V-222642 -- Must not contain embedded auth data (CAT I)
**File**: backend/.env
**Finding**: `.env` file present in repository with DATABASE_URL and FRONTEND_URL.
**Remediation**: Remove .env from git. Add to .gitignore. Use .env.example with placeholders only.

### FAIL: V-222643 -- Must mark sensitive/classified output (CAT II)
**Finding**: Export documents (PDF, DOCX) contain no classification markings.
**Remediation**: Add classification header/footer to generated documents.

### N/A: V-222645 -- Hash files prior to deployment (CAT II)
**Reason**: No deployment pipeline configured.

### N/A: V-222646 -- Designate security tester (CAT II)
**Reason**: Organizational/procedural.

### N/A: V-222647 -- Test startup/shutdown/abort security (CAT II)
**Reason**: No deployment/ops automation in place.

### FAIL: V-222653 -- Follow coding standards (CAT II)
**File**: backend/src/error.rs:38-39
**Finding**: `tracing::error!` with `{:?}` could leak schema details in logs.
**Remediation**: Implement structured error logging with error codes. Sanitize before logging.

### FAIL: V-222615 -- Verify correct operation of security functions (CAT II)
**Finding**: No integration tests for security functions (audit logging, CORS, error handling).
**Remediation**: Add security-focused integration tests.
