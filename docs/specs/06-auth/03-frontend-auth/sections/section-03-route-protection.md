I have all the context needed. Here is the section content.

# Section 3: Route Protection

## Overview

This section creates the `_authenticated` pathless layout route that guards all protected routes, updates `__root.tsx` to use `createRootRouteWithContext` for passing auth state into the route tree, and establishes the `InnerApp` pattern so the router re-evaluates guards when auth state changes.

**Dependencies:**
- Section 02 (Auth Context) must be completed first -- this section consumes `useAuth` and its `RouterContext` type.

**Blocks:**
- Section 08 (Wiring) depends on this section for the `_authenticated` layout and the root route context changes.

---

## Tests First

Create test file `frontend/src/routes/__tests__/route-protection.test.tsx`.

These tests validate the route guard behavior. Because TanStack Router's `beforeLoad` is tightly coupled to the router runtime, the most practical approach is unit-testing the guard logic as an extracted function plus integration-style tests using a minimal router setup.

```typescript
// frontend/src/routes/__tests__/route-protection.test.tsx

import { describe, it, expect, vi } from 'vitest'
import { redirect } from '@tanstack/react-router'

// Test: _authenticated beforeLoad redirects to /login when no user
//   - Call the guard function with context.auth.isAuthenticated = false
//   - Expect it to throw a redirect to /login

// Test: _authenticated beforeLoad allows access when user present
//   - Call the guard function with context.auth.isAuthenticated = true
//   - Expect it NOT to throw

// Test: redirect includes current path as search param
//   - Call the guard function with isAuthenticated = false, verify the redirect
//     includes search: { redirect: <current path> }

// Test: public routes (/, /login, /register) accessible without auth
//   - These routes are NOT under _authenticated, so no guard runs
//   - This is structural: verified by confirming these files live outside
//     routes/_authenticated/

// Test: router.invalidate() is called when auth state changes
//   - In the InnerApp component, when auth.user changes the useEffect
//     should call router.invalidate()
//   - Render InnerApp with a mocked router, change the auth user, verify
//     invalidate was called
```

### Guard Logic Extraction for Testability

Extract the `beforeLoad` guard logic into a standalone function so it can be unit tested without spinning up a full router:

```typescript
// frontend/src/features/auth/authGuard.ts

/**
 * Checks auth context and throws a TanStack Router redirect if not authenticated.
 * Used in _authenticated.tsx beforeLoad.
 */
export function requireAuth(context: { auth: { isAuthenticated: boolean } }): void
```

Test this function directly:

```typescript
// frontend/src/features/auth/__tests__/authGuard.test.ts

// Test: requireAuth throws redirect when isAuthenticated is false
// Test: requireAuth does not throw when isAuthenticated is true
// Test: thrown redirect targets /login with search.redirect = window.location.pathname
```

---

## Implementation Details

### 1. Create the Auth Guard Function

**File:** `frontend/src/features/auth/authGuard.ts`

Export a `requireAuth` function that:
- Accepts `{ auth: { isAuthenticated: boolean } }` as its argument (matching the router context shape)
- If `isAuthenticated` is `false`, throws `redirect({ to: '/login', search: { redirect: window.location.pathname } })`
- If `isAuthenticated` is `true`, returns void (allows route to load)

This is imported by `_authenticated.tsx` and also directly testable.

### 2. Create the _authenticated Layout Route

**File:** `frontend/src/routes/_authenticated.tsx`

This is a pathless layout route -- it does not add a URL segment but wraps child routes with auth guard logic.

```typescript
import { createFileRoute, Outlet, redirect } from '@tanstack/react-router'
import { requireAuth } from '@/features/auth/authGuard'

export const Route = createFileRoute('/_authenticated')({
  beforeLoad: ({ context }) => {
    requireAuth(context)
  },
  component: () => <Outlet />,
})
```

Key behaviors:
- `beforeLoad` runs before any child route loads. If the user is not authenticated, the `redirect` exception is caught by TanStack Router and the browser navigates to `/login`.
- The `search` param `{ redirect: window.location.pathname }` preserves the original URL so the login page can redirect back after successful login.
- The `component` simply renders `<Outlet />` -- no additional layout wrapper. The root layout in `__root.tsx` already provides the page chrome.

### 3. Update __root.tsx to Use createRootRouteWithContext

**File:** `frontend/src/routes/__root.tsx`

Change the root route from `createRootRoute` to `createRootRouteWithContext<RouterContext>()` so the router has typed access to the auth context.

Define the `RouterContext` interface. This should be placed in a shared location (e.g., `frontend/src/features/auth/routerContext.ts`) or inline in `__root.tsx`:

```typescript
interface RouterContext {
  auth: {
    user: UserProfile | null
    isAuthenticated: boolean
    isLoading: boolean
    login: (data: LoginRequest) => Promise<void>
    logout: () => Promise<void>
    register: (data: RegisterRequest) => Promise<UserProfile>
  }
  queryClient: QueryClient
}
```

Update the root route creation:

```typescript
import { createRootRouteWithContext, Link, Outlet } from '@tanstack/react-router'

export const Route = createRootRouteWithContext<RouterContext>()({
  component: RootLayout,
})
```

The `RootLayout` function body stays the same (the navbar, sidebar, `<Outlet />`, etc. are unchanged in this section -- navbar auth controls are handled in Section 06).

**Important:** After this change, `createRouter({ routeTree })` in `main.tsx` will require a `context` option. This is wired in Section 08, but you must be aware that the app will not compile until that wiring is done. To keep things building during development, you can temporarily provide a stub context to `createRouter`.

### 4. Create the InnerApp Component Pattern

**File:** `frontend/src/InnerApp.tsx` (or inline in `main.tsx` -- Section 08 decides final placement)

This component sits inside `AuthProvider` and `QueryClientProvider`, uses the `useAuth` hook to get live auth state, and passes it to `RouterProvider` via the `context` prop. It also calls `router.invalidate()` whenever `auth.user` changes so route guards re-evaluate.

```typescript
function InnerApp() {
  const auth = useAuth()

  useEffect(() => {
    router.invalidate()
  }, [auth.user])

  return <RouterProvider router={router} context={{ auth, queryClient }} />
}
```

Key detail: `router.invalidate()` causes all active route loaders and `beforeLoad` guards to re-run. This is what makes logout immediately trigger the redirect-to-login behavior and login immediately allow access to protected routes.

### 5. Route File Moves (Preparation)

This section defines which files must move under `_authenticated/`. The actual file moves happen in Section 08, but the `_authenticated.tsx` route created here establishes the parent for those child routes.

Routes that will move under `_authenticated/`:
- `routes/ontology/` directory
- `routes/compliance/` directory
- `routes/concepts/` directory
- `routes/crosswalk/` directory (note: currently a directory with `index.tsx`, not a single file)
- `routes/frameworks/` directory (same)
- `routes/landscape/` directory (same)
- `routes/reports/` directory (same)
- `routes/analysis/` directory

Routes that stay at the top level (public):
- `routes/__root.tsx`
- `routes/index.tsx` (home page)
- `routes/login.tsx` (created in Section 04)
- `routes/register.tsx` (created in Section 05)

After moving files, TanStack Router's code generator (triggered by `pnpm dev` or `pnpm build`) will regenerate `routeTree.gen.ts` to reflect the new hierarchy.

---

## Files Summary

| File | Action |
|------|--------|
| `frontend/src/features/auth/authGuard.ts` | Create -- extracted guard function |
| `frontend/src/features/auth/__tests__/authGuard.test.ts` | Create -- unit tests for guard |
| `frontend/src/routes/_authenticated.tsx` | Create -- pathless layout route with beforeLoad guard |
| `frontend/src/routes/__root.tsx` | Modify -- switch to `createRootRouteWithContext<RouterContext>()` |
| `frontend/src/routes/__tests__/route-protection.test.tsx` | Create -- integration tests for route protection |
| `frontend/src/InnerApp.tsx` | Create -- component that bridges auth context to router |

---

## Edge Cases

- **Deep link while unauthenticated:** User visits `/compliance` directly. The `_authenticated` guard fires, captures `/compliance` as the redirect path, throws redirect to `/login?redirect=%2Fcompliance`. After login, Section 04's LoginPage reads `search.redirect` and navigates there.
- **Auth state race on page refresh:** The `AuthProvider` (Section 02) shows a loading screen while the initial `/me` query is in flight. The router does not render routes until `AuthProvider` resolves, so `beforeLoad` always sees a definitive `isAuthenticated` value, never an intermediate loading state.
- **Multiple rapid auth changes:** `router.invalidate()` is idempotent and debounced internally by TanStack Router, so rapid calls from `useEffect` are safe.