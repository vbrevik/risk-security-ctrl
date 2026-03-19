# Auth System — Deep Project Interview

## Context

The risk-security-ctrl application manages sensitive governmental IT security compliance data. It runs in an air-gapped environment. The database schema for users, sessions, and audit_log already exists. Backend has placeholder auth routes. Frontend API client already intercepts 401 and redirects to /login.

## Decisions

### Token Strategy
**Decision: Both cookie + Bearer header**
- HttpOnly cookie (AES-GCM encrypted via axum-extra `cookie-private`) for the browser SPA
- Bearer token in Authorization header for API consumers
- Backend accepts either mechanism, cookie takes precedence

### Route Access
**Decision: Home page only is public**
- Landing/dashboard page accessible without login
- All feature pages (ontology, compliance, analysis, reports) require authentication
- Unauthenticated users redirected to /login

### Registration Model
**Decision: Open registration**
- Anyone with network access can register as a viewer
- Admin can promote user roles after registration
- No invite tokens or admin-only creation for this phase

### Admin Bootstrap
**Decision: CLI seed command**
- A cargo subcommand or standalone binary creates an initial admin user
- Reads credentials from environment variables or CLI arguments
- Most auditable approach — no magic first-user behavior, no hardcoded passwords in migrations

### Session Management
**Decision: 8-hour sessions, single session per user**
- Sessions expire after 8 hours (working day)
- New login invalidates any existing session for that user
- Expired sessions cleaned up on access

### Rate Limiting
**Decision: Include from the start**
- Rate limit login attempts using governor/tower_governor
- Per-IP rate limiting on the login endpoint
- Important for a security-focused government application

### RBAC Granularity
**Decision: Per-role per-feature**
- Each feature (ontology, compliance, analysis, reports) has its own role-permission mapping
- Roles: admin, risk_manager, specialist, viewer
- More granular than simple read/write tiers
- Requires a permission matrix defining which roles can perform which actions on which features

### Library Choices (Security-Vetted)
| Need | Crate | Rationale |
|------|-------|-----------|
| Password hashing | `argon2` (RustCrypto) | Org-backed, pure Rust, no network calls |
| Session tokens | `rand` + `getrandom` | OS entropy, de facto standard |
| Cookies | `axum-extra` (cookie-private) | Tokio-team maintained, AES-GCM |
| Rate limiting | `governor` + `tower_governor` | In-memory GCRA, no external deps |
| Input validation | `validator` | 44M downloads, derive macros |
| CSRF | None (SameSite cookie + custom header) | CSRF crates have low adoption |

### Environment
- Air-gapped deployment
- All crates must work fully offline (no network calls) — verified for all chosen crates
- SQLite3 database (schema already exists)
- Must integrate with existing Axum/utoipa/feature-module patterns
