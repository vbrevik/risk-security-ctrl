<!-- PROJECT_CONFIG
runtime: typescript-pnpm
test_command: pnpm test
END_PROJECT_CONFIG -->

<!-- SECTION_MANIFEST
section-01-api-client
section-02-auth-context
section-03-route-protection
section-04-login-page
section-05-register-page
section-06-navbar-layout
section-07-i18n
section-08-wiring
END_MANIFEST -->

# Implementation Sections Index

## Dependency Graph

| Section | Depends On | Blocks | Parallelizable |
|---------|------------|--------|----------------|
| section-01-api-client | - | 02, 04, 05 | Yes |
| section-02-auth-context | 01 | 03, 04, 05, 06 | No |
| section-03-route-protection | 02 | 08 | Yes |
| section-04-login-page | 01, 02 | 08 | Yes (parallel with 03, 05, 06, 07) |
| section-05-register-page | 01, 02 | 08 | Yes (parallel with 03, 04, 06, 07) |
| section-06-navbar-layout | 02 | 08 | Yes (parallel with 03, 04, 05, 07) |
| section-07-i18n | - | 04, 05 | Yes (parallel with 01-06) |
| section-08-wiring | 03, 04, 05, 06, 07 | - | No |

## Execution Order

1. section-01-api-client, section-07-i18n (parallel, no dependencies)
2. section-02-auth-context (after 01)
3. section-03-route-protection, section-04-login-page, section-05-register-page, section-06-navbar-layout (parallel after 02)
4. section-08-wiring (final)

## Section Summaries

### section-01-api-client
Update Axios config (withCredentials, X-Requested-With), create auth types and API functions.

### section-02-auth-context
AuthProvider with TanStack Query, useAuth hook, useHasPermission hook, AuthLoadingScreen.

### section-03-route-protection
Create _authenticated.tsx layout route, move existing routes under it, update __root.tsx with createRootRouteWithContext.

### section-04-login-page
Login route and LoginPage component with shadcn/ui, form validation, error display, redirect handling.

### section-05-register-page
Register route and RegisterPage component with shadcn/ui, password confirmation, client-side validation.

### section-06-navbar-layout
Update navbar with auth-aware controls (user name, role badge, logout button, login link).

### section-07-i18n
Create auth.json namespace files for en and nb locales, register namespace.

### section-08-wiring
Wire AuthProvider into app entry, update RouterProvider with auth context, file moves, integration tests.
