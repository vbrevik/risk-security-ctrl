# 03 — Frontend Auth

## Purpose

Build the frontend authentication UI — login page, registration page, auth state management, protected route guards, and role-based UI rendering.

## Requirements Reference

See `../requirements.md` sections: Frontend Authentication (items 8-11).

## Key Decisions (from interview)

- **Public pages:** Home page only — all feature pages require login
- **Token handling:** Cookie-based (automatic via httpOnly cookie) + Bearer header injection for API calls
- **Registration:** Open — anyone can sign up as viewer
- **i18n:** Use existing i18next setup, add `auth` namespace for en + nb locales

## Scope

### Routes to Create
| Path | Component | Auth Required |
|------|-----------|---------------|
| /login | LoginPage | No |
| /register | RegisterPage | No |

### Core Components

1. **LoginPage** — email + password form, error display for invalid credentials, redirect to previous page or dashboard on success
2. **RegisterPage** — email, name, password, confirm password form, client-side validation (matching passwords, min length), redirect to /login on success with success message
3. **AuthProvider** — React context wrapping the app, provides current user state, loading state, and auth methods
4. **useAuth hook** — exposes: `user`, `isAuthenticated`, `isLoading`, `login()`, `logout()`, `register()`
5. **ProtectedRoute wrapper** — checks auth state, redirects to /login if unauthenticated, preserves intended destination for post-login redirect
6. **Role-based rendering** — `useHasPermission(feature, action)` hook or `<RequireRole>` component for conditionally rendering UI based on user role and the permission matrix from split 02
7. **API client updates** — update `lib/api.ts` to inject Bearer token from auth state for non-cookie scenarios
8. **Navbar/sidebar updates** — show user name, role badge, logout button when authenticated
9. **i18n** — add `auth.json` namespace files for en and nb locales

### Auth Flow
```
Unauthenticated user visits /compliance
  → ProtectedRoute redirects to /login?redirect=/compliance
  → User logs in
  → POST /api/auth/login (cookie set automatically by backend)
  → AuthProvider calls GET /api/auth/me to populate user state
  → Redirect to /compliance
```

### Logout Flow
```
User clicks logout
  → POST /api/auth/logout (cookie cleared by backend)
  → AuthProvider clears user state
  → Redirect to /login
```

## Existing Infrastructure

- **TanStack Router** — file-based routing in `frontend/src/routes/`
- **TanStack Query** — API hooks pattern in `features/*/api/`
- **shadcn/ui** — form components, buttons, input fields, toast notifications
- **Axios client** — `src/lib/api.ts` with 401 interceptor already redirecting to /login
- **i18next** — setup in place with namespace-per-feature pattern
- **Layout** — `__root.tsx` with navbar and sidebar

## Depends On

- **01-backend-core-auth:** Auth API endpoints (register, login, logout, me)
- **02-rbac-and-hardening:** Permission matrix (for role-based UI), RBAC behavior (to know what errors to expect for insufficient permissions)

## Testing Strategy

- Component test: LoginPage renders form, submits, shows errors
- Component test: RegisterPage validates matching passwords, shows field errors
- Component test: ProtectedRoute redirects when unauthenticated
- Hook test: useAuth returns correct state after login/logout
- Integration test: full login → navigate → logout flow
