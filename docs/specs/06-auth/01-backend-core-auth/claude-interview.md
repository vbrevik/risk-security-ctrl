# Backend Core Auth — Interview Transcript

## Q1: CLI admin seed command format?

**Options:** Flag on main binary, separate binary, env-var triggered on startup.

**Answer:** Separate binary (`cargo run --bin seed-admin`). Clean separation with its own `[[bin]]` entry.

## Q2: What should happen when registering with an existing email?

**Options:** Return 409 Conflict with message, or return generic 422 without revealing whether email exists.

**Answer:** Return generic 422 (no email leak). Don't reveal whether an email is registered — more secure, prevents account enumeration.

## Q3: COOKIE_KEY provisioning in air-gapped environment?

**Options:** Env var only, auto-generate and persist to file, or both.

**Answer:** Both — check env var `COOKIE_KEY` first, fall back to reading/generating a key file on disk. This supports both container deploys (env var) and single-node dev setups (file).

## Q4: What information should GET /api/auth/me return?

**Options:** Minimal (id, email, name, role) or extended (+ last_login, is_active, created_at).

**Answer:** Minimal — just `id`, `email`, `name`, `role`. No sensitive or metadata fields.

## Q5: Password complexity requirements?

**Options:** Min 8 chars only, min 8 + number + uppercase, or min 12 no complexity.

**Answer:** Min 8 characters, no other rules. Follows NIST 800-63B guidance recommending length over complexity.

## Prior Decisions (from deep-project interview)

These were established during the project-level interview:

- **Token transport:** Both httpOnly cookie (AES-GCM) AND Bearer header. Cookie takes precedence in extractor.
- **Registration:** Open — anyone can register as viewer role
- **Sessions:** 8-hour expiry, single session per user (new login invalidates previous)
- **Password hashing:** Argon2id via RustCrypto `argon2` crate with OWASP defaults
- **Audit logging:** Login/logout events to existing `audit_log` table
