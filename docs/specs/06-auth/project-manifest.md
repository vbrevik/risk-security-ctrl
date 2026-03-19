<!-- SPLIT_MANIFEST
01-backend-core-auth
02-rbac-and-hardening
03-frontend-auth
END_MANIFEST -->

# Project Manifest: Authentication & Authorization

## Overview

Three-split linear pipeline. Each split builds on the previous, progressing from foundational backend auth through authorization middleware to frontend integration.

## Splits

### 01-backend-core-auth (Foundation)

**Purpose:** Implement user registration, login, logout, session management, and the auth extractor that all protected endpoints will use.

**Scope:**
- Add dependencies: `argon2`, `rand`, `axum-extra` (cookie-private), `validator`
- User model operations: create (with argon2id hashing), verify credentials, lookup by email
- Session model operations: create (with CSPRNG token), validate, invalidate, enforce single-session-per-user
- Auth endpoints: POST /api/auth/register, POST /api/auth/login, POST /api/auth/logout, GET /api/auth/me
- Axum extractor (`AuthUser`) that validates session from both cookie and Bearer header
- CLI seed command for creating the initial admin user
- Audit log integration for login/logout events
- OpenAPI/utoipa annotations for all endpoints

**Outputs:**
- `AuthUser` extractor usable by any route handler
- Working auth endpoints against existing DB schema
- CLI binary or subcommand for admin seeding

**Dependencies:** None (uses existing DB schema)

---

### 02-rbac-and-hardening (Authorization Layer)

**Purpose:** Add role-based access control middleware, rate limiting, and security hardening on top of the core auth.

**Scope:**
- Add dependencies: `governor`, `tower_governor`
- Permission matrix: define which roles (admin, risk_manager, specialist, viewer) can perform which actions on which features (ontology, compliance, analysis, reports)
- RBAC middleware/guard that checks `AuthUser.role` against the permission matrix
- Apply RBAC to all existing endpoints (ontology, compliance, analysis routes)
- Rate limiting on login endpoint (per-IP via tower_governor)
- Security headers via tower-http (HSTS, X-Content-Type-Options, X-Frame-Options)
- Session cleanup: expired session pruning on access
- CSRF protection: SameSite=Strict cookie + custom header validation

**Outputs:**
- `RequireRole` guard/middleware usable per-endpoint
- Permission matrix (code or config)
- All existing endpoints protected with appropriate role checks
- Rate-limited login

**Dependencies:** 01-backend-core-auth (needs AuthUser extractor and session model)

---

### 03-frontend-auth (UI Integration)

**Purpose:** Build the frontend authentication experience — login page, registration page, auth state management, and protected route guards.

**Scope:**
- Login page (/login): email + password form, error display, redirect on success
- Registration page (/register): email, name, password, confirm password, client-side validation
- AuthContext provider: current user state, loading state, useAuth hook
- Token handling: cookie-based (automatic) + Bearer header injection for API calls
- Protected route wrapper: redirect to /login if unauthenticated
- Role-based UI: show/hide elements based on user role
- Home page remains public, all feature routes wrapped with auth guard
- i18next translations for auth UI strings (en + nb namespaces)
- Logout functionality in navbar/sidebar

**Outputs:**
- Working login/register/logout flow
- Auth-aware navigation and route guards
- Role-based UI rendering

**Dependencies:** 01-backend-core-auth (needs working auth API), 02-rbac-and-hardening (needs RBAC for role-based UI)

---

## Execution Order

```
01-backend-core-auth
         │
         ▼
02-rbac-and-hardening
         │
         ▼
03-frontend-auth
```

**Strictly sequential.** Each split depends on the previous. No parallelism possible since:
- Split 02 needs the AuthUser extractor from split 01
- Split 03 needs both the API endpoints (01) and the RBAC behavior (02) to build against

## Cross-Cutting Concerns

- **Existing DB schema:** All three splits use the existing `users`, `sessions`, and `audit_log` tables. No new migrations needed.
- **OpenAPI:** All new backend endpoints must have utoipa annotations and appear in Swagger UI.
- **Error format:** All auth errors must follow the existing `{ "error": "...", "message": "..." }` response format from `src/error.rs`.
- **Feature module structure:** Auth code goes in `backend/src/features/auth/` and `frontend/src/features/auth/`.

## Next Steps

```bash
/deep-plan @docs/specs/06-auth/01-backend-core-auth/spec.md
/deep-plan @docs/specs/06-auth/02-rbac-and-hardening/spec.md
/deep-plan @docs/specs/06-auth/03-frontend-auth/spec.md
```
