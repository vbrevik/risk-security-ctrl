# 02 — RBAC & Security Hardening

## Purpose

Add role-based access control, rate limiting, and security hardening on top of the core auth system. This split protects all existing and future endpoints with appropriate role checks.

## Requirements Reference

See `../requirements.md` sections: Backend Authorization (items 6-7), Security Requirements (items 13-15).

## Key Decisions (from interview)

- **RBAC granularity:** Per-role per-feature — each feature (ontology, compliance, analysis, reports) has its own permission mapping
- **Rate limiting:** Include from the start, per-IP on login endpoint using `governor` + `tower_governor`
- **CSRF:** SameSite=Strict cookie + custom header check (no dedicated CSRF crate)
- **Roles:** admin, risk_manager, specialist, viewer (from existing DB schema)

## Scope

### Dependencies to Add
- `governor` (GCRA rate limiting algorithm)
- `tower_governor` (Tower/Axum middleware wrapper)

### Permission Matrix

Define which roles can perform which operations per feature:

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

### Core Components

1. **Permission matrix** — code-based (enum/const) definition of role-feature-action permissions
2. **RequireRole guard** — Axum middleware or extractor that wraps `AuthUser` and checks role against required permission. Returns 403 Forbidden if insufficient.
3. **Apply RBAC to existing routes** — wrap ontology, compliance, analysis, and report endpoints with appropriate role guards
4. **Rate limiting** — `GovernorLayer` on login endpoint, configurable requests-per-minute per IP
5. **Security headers** — tower-http layer adding HSTS, X-Content-Type-Options, X-Frame-Options, X-XSS-Protection
6. **Session cleanup** — prune expired sessions on each auth check (or periodic background task)
7. **CSRF protection** — set SameSite=Strict on auth cookie; for state-changing requests, validate a custom header (e.g., X-Requested-With)

## Existing Infrastructure

- **AuthUser extractor** from split 01 — provides user identity and role
- **Existing routes to protect:** ontology (routes.rs), compliance (routes.rs), analysis (routes.rs), reports (routes.rs), auth (routes.rs)
- **tower-http** already in Cargo.toml — add `sensitive-headers` and other security features

## Depends On

- **01-backend-core-auth:** AuthUser extractor, session model, user role field

## Provides to Other Splits

- **RequireRole guard** — reusable for any future endpoints
- **Permission matrix** — referenced by 03-frontend-auth for role-based UI rendering
- **Security middleware stack** — applied globally, no action needed from frontend

## Testing Strategy

- Unit tests for permission matrix lookups
- Integration tests: viewer blocked from write endpoints (403)
- Integration tests: admin can access all endpoints
- Integration test: rate limiter blocks rapid login attempts (429)
- Test CSRF header validation on state-changing requests
- Test expired session cleanup behavior
