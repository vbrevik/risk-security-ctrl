# RBAC & Hardening — Synthesized Specification

## Overview

Add role-based access control, rate limiting, security headers, CSRF protection, and session cleanup to the risk-security-ctrl application. Builds on the AuthUser extractor from split 01-backend-core-auth.

## System Context

- Rust/Axum 0.7 backend, SQLite3, air-gapped government environment
- AuthUser extractor already provides `{ id, email, name, role, session_id }`
- Roles: admin, risk_manager, specialist, viewer
- Existing routes: ontology, compliance, analysis, reports, auth

## Permission Matrix

Read access is universal for all authenticated users. Write/delete restricted by role:

| Feature | Action | admin | risk_manager | specialist | viewer |
|---------|--------|-------|-------------|------------|--------|
| Ontology | read | yes | yes | yes | yes |
| Ontology | write | yes | yes | no | no |
| Compliance | read | yes | yes | yes | yes |
| Compliance | create/update | yes | yes | yes | no |
| Compliance | delete | yes | yes | no | no |
| Analysis | read | yes | yes | yes | yes |
| Analysis | create | yes | yes | yes | no |
| Analysis | delete | yes | yes | no | no |
| Reports | read | yes | yes | yes | yes |
| Reports | export | yes | yes | yes | no |
| Auth/Users | manage users | yes | no | no | no |

## Rate Limiting

- Dependencies: `governor` + `tower_governor`
- Strict on auth endpoints: ~5 requests burst, 4-second replenishment
- Moderate on general API: 30 burst, 1/sec replenishment
- IP extraction: configurable via `BEHIND_PROXY` env var (default: PeerIpKeyExtractor)
- Response headers enabled (X-RateLimit-*)
- Must use `.into_make_service_with_connect_info::<SocketAddr>()`

## Security Headers

Configurable for HTTP (dev) vs HTTPS (prod) via env var:

- X-Frame-Options: DENY
- X-Content-Type-Options: nosniff
- X-XSS-Protection: 0 (disabled per modern guidance — CSP is preferred)
- Content-Security-Policy: default-src 'self'
- Permissions-Policy: camera=(), microphone=(), geolocation=()
- HSTS: max-age=31536000 (HTTPS only)
- Referrer-Policy: strict-origin-when-cross-origin

## CSRF Protection

- SameSite=Lax on auth cookie (from split 01)
- Custom header check: `X-Requested-With: XMLHttpRequest` required on all POST/PUT/DELETE
- Implemented as tower middleware — reject requests missing the header with 403

## Session Cleanup

- Prune expired sessions during auth extractor validation (lazy cleanup)
- No separate background task needed for single-instance deployment

## Swagger UI

- Gate behind cargo feature flag `swagger` (default: enabled for dev)
- Production builds: compile without `--features swagger`

## Error Sanitization

- Return generic error messages to API clients
- Log detailed errors server-side via tracing
- Return 404 instead of 403 for resource existence confirmation prevention (where applicable)

## Config Additions

- `BEHIND_PROXY` — boolean, enables SmartIpKeyExtractor (default: false)
- `ENABLE_HTTPS` — boolean, enables HSTS and Secure cookie flags (default: false)
