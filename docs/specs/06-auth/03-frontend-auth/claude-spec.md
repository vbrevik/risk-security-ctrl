# Frontend Auth — Synthesized Specification

## Overview

Build the frontend authentication experience for risk-security-ctrl using React, TanStack Router, TanStack Query, and shadcn/ui. The auth state is managed via httpOnly cookies set by the backend — the frontend never touches tokens directly. Auth status is determined by calling GET /api/auth/me.

## Key Architecture Decisions

- **Auth state owner:** TanStack Query via `useQuery(['auth', 'me'])`. Single source of truth.
- **Router integration:** `createRootRouteWithContext` passes auth state to all routes. Protected routes use `beforeLoad` guards.
- **Route protection:** Pathless layout route `_authenticated.tsx` guards all feature routes. Home and login/register remain public.
- **Cookie-based auth:** Browser sends httpOnly cookie automatically. Frontend adds `X-Requested-With: XMLHttpRequest` and `credentials: 'include'` on all requests.
- **Session expiry:** 401 interceptor redirects to /login with toast "Session expired."
- **Loading state:** Full-screen spinner while initial auth check runs.

## Routes

| Path | Component | Auth | Description |
|------|-----------|------|-------------|
| `/` | HomePage | No | Public landing page |
| `/login` | LoginPage | No | Email + password form |
| `/register` | RegisterPage | No | Registration form |
| `/ontology/*` | OntologyExplorer | Yes | Protected via `_authenticated` |
| `/compliance/*` | CompliancePage | Yes | Protected |
| `/concepts/*` | ConceptsPage | Yes | Protected |
| `/crosswalk` | CrosswalkView | Yes | Protected |
| `/frameworks` | FrameworksPage | Yes | Protected |
| `/landscape` | LandscapePage | Yes | Protected |
| `/reports` | ReportsPage | Yes | Protected |

## Components

1. **LoginPage** — shadcn Card with email/password inputs, error display, submit button, link to register
2. **RegisterPage** — shadcn Card with email/name/password/confirm-password, client-side validation, link to login
3. **AuthProvider** — wraps app, provides auth context via TanStack Query
4. **useAuth hook** — `user`, `isAuthenticated`, `isLoading`, `login()`, `logout()`, `register()`
5. **ProtectedRoute** — `_authenticated.tsx` layout route with `beforeLoad` guard
6. **useHasPermission** — checks user role against permission matrix for conditional UI
7. **AuthLoadingScreen** — full-screen spinner shown during initial auth check

## API Integration

- **Axios client:** Add `withCredentials: true` and `X-Requested-With: XMLHttpRequest` headers
- **Auth API hooks:** `useLogin`, `useLogout`, `useRegister`, `useCurrentUser` (wrapping TanStack Query mutations/queries)
- **Query invalidation:** On login/logout, set query data + call `router.invalidate()`

## i18n

Add `auth.json` namespace with translations for:
- Form labels, placeholders, validation messages
- Error messages (invalid credentials, session expired, registration failed)
- Button text (login, register, logout)
- Both `en` and `nb` locales
