# Section 8: Wiring and Integration

## Overview

This section completes the frontend auth implementation by wiring all previously built pieces together: wrapping the app in `AuthProvider`, updating the router to pass auth context, moving existing route files under the `_authenticated` pathless layout, and verifying the full integration with end-to-end flow tests.

**Dependencies:** Sections 01 through 07 must be complete before this section. Specifically:
- Section 01 provides auth API functions and types
- Section 02 provides `AuthProvider` and `useAuth`
- Section 03 provides `_authenticated.tsx` guard and `createRootRouteWithContext` setup in `__root.tsx`
- Section 04 provides `login.tsx` route
- Section 05 provides `register.tsx` route
- Section 06 provides navbar auth controls
- Section 07 provides i18n `auth` namespace translations

## Tests First

Create the integration test file at `frontend/src/features/auth/__tests__/auth-wiring.test.tsx`.

These tests verify the full wiring works end-to-end. They require a test harness that renders the actual router with mocked API responses.

```typescript
// frontend/src/features/auth/__tests__/auth-wiring.test.tsx

import { describe, it, expect, vi } from 'vitest'
import { render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'

// Test: full flow: visit /compliance -> redirected to /login -> login -> redirected to /compliance
//
// Setup: mock fetchCurrentUser to return null (unauthenticated), render app with
// initial URL /compliance. Verify that /login is displayed with redirect search param.
// Then mock loginUser to succeed and fetchCurrentUser to return a user. Submit the
// login form. Verify navigation to /compliance.

// Test: full flow: register -> redirect to /login -> login -> access dashboard
//
// Setup: mock fetchCurrentUser to return null. Navigate to /register. Fill out the
// registration form. Mock registerUser to succeed. Submit. Verify redirect to /login.
// Fill login form, mock loginUser to succeed, submit. Verify navigation to home or
// dashboard.

// Test: logout clears auth state and redirects to /login
//
// Setup: mock fetchCurrentUser to return a valid user. Render app at /compliance.
// Verify the page renders (user is authenticated). Click the logout button in navbar.
// Mock logoutUser to succeed. Verify redirect to /login and user state is cleared.

// Test: page refresh preserves auth state (cookie-based)
//
// Setup: mock fetchCurrentUser to return a valid user (simulating browser sending
// cookie on refresh). Render app at /compliance. Verify the compliance page renders
// without being redirected to /login. This confirms the AuthProvider's initial
// useQuery call restores state from the cookie-backed session.
```

### Test Utilities

The integration tests need a helper that creates a full app render context. Create or update `frontend/src/test/render-app.tsx`:

```typescript
// frontend/src/test/render-app.tsx
//
// Provides a renderApp(initialUrl: string) helper that:
// 1. Creates a QueryClient with test defaults (retry: false, cacheTime: 0)
// 2. Wraps with QueryClientProvider
// 3. Wraps with AuthProvider
// 4. Creates a router with the real routeTree and injects auth context
// 5. Renders RouterProvider at the given initial URL
//
// This mirrors the production wiring from main.tsx but in a test-friendly way.
```

## Implementation

### 1. Update App Entry Point

**File:** `frontend/src/main.tsx`

The current `main.tsx` renders `QueryClientProvider > RouterProvider`. It needs to be updated to insert `AuthProvider` and pass auth context to the router.

**Current structure:**
```typescript
const router = createRouter({ routeTree })
const queryClient = new QueryClient({ ... })

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </StrictMode>
)
```

**New structure:**

```typescript
import { AuthProvider } from '@/features/auth/AuthProvider'
import { useAuth } from '@/features/auth/useAuth'

// Router now requires context (from Section 03's createRootRouteWithContext)
const router = createRouter({
  routeTree,
  context: {
    auth: undefined!, // provided at render time
    queryClient,
  },
})

// Update the Register type declaration to include context
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router
  }
}

// InnerApp component consumes useAuth inside AuthProvider scope and injects
// live auth context into the router
function InnerApp() {
  const auth = useAuth()
  // Re-invalidate router when auth state changes so beforeLoad guards re-run
  useEffect(() => {
    router.invalidate()
  }, [auth.user])
  return <RouterProvider router={router} context={{ auth, queryClient }} />
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        <InnerApp />
      </AuthProvider>
    </QueryClientProvider>
  </StrictMode>
)
```

Key points:
- `AuthProvider` must be inside `QueryClientProvider` because it uses `useQuery`
- `InnerApp` must be inside `AuthProvider` because it calls `useAuth()`
- `router.invalidate()` in a `useEffect` watching `auth.user` ensures route guards re-evaluate when auth state changes (login/logout)
- The router's `context` option gets a placeholder (`undefined!`) that is overridden at render time by the `context` prop on `RouterProvider`

### 2. Move Existing Route Files Under `_authenticated/`

All protected routes must be moved into the `frontend/src/routes/_authenticated/` directory so they fall under the `_authenticated.tsx` layout guard created in Section 03.

**Files and directories to move:**

| Source | Destination |
|--------|-------------|
| `routes/ontology/` | `routes/_authenticated/ontology/` |
| `routes/compliance/` | `routes/_authenticated/compliance/` |
| `routes/concepts/` | `routes/_authenticated/concepts/` |
| `routes/crosswalk/` | `routes/_authenticated/crosswalk/` |
| `routes/frameworks/` | `routes/_authenticated/frameworks/` |
| `routes/landscape/` | `routes/_authenticated/landscape/` |
| `routes/reports/` | `routes/_authenticated/reports/` |
| `routes/analysis/` | `routes/_authenticated/analysis/` |

**Files that stay in `routes/` (public):**

| File | Reason |
|------|--------|
| `routes/__root.tsx` | Root layout, always rendered |
| `routes/index.tsx` | Home page, public |
| `routes/login.tsx` | Login page (Section 04) |
| `routes/register.tsx` | Register page (Section 05) |
| `routes/_authenticated.tsx` | Auth guard layout (Section 03) |

**After moving**, each route file must update its `createFileRoute` path string to include the `/_authenticated` prefix. For example:

- `createFileRoute('/ontology/')` becomes `createFileRoute('/_authenticated/ontology/')`
- `createFileRoute('/compliance/')` becomes `createFileRoute('/_authenticated/compliance/')`
- `createFileRoute('/concepts/search')` becomes `createFileRoute('/_authenticated/concepts/search')`
- And so on for every route file that was moved

**Important:** The URL paths visible to users do NOT change. The `_authenticated` segment is a pathless layout (prefixed with underscore), so `/ontology` still works as the browser URL. Only the internal route IDs change.

### 3. Update Internal Links

After moving routes, scan all moved route files and components for `<Link to="...">` usages. The `to` prop values should remain unchanged (e.g., `to="/ontology"`) because TanStack Router resolves links by URL path, not route ID. No link updates should be needed.

However, verify that any `navigate()` calls or `redirect()` calls in route `beforeLoad` or `loader` functions still work correctly. The `to` parameter in these calls uses URL paths, so they should be unaffected.

### 4. Update Nav Links in `__root.tsx`

The nav links in `__root.tsx` use `to="/ontology"`, `to="/compliance"`, etc. These URL paths are unchanged (the `_authenticated` prefix is pathless), so no link updates are needed in the navbar.

### 5. Regenerate Route Tree

After all file moves are complete, the TanStack Router code generator must run to update `frontend/src/routeTree.gen.ts`. This happens automatically when running:

```bash
cd /Users/vidarbrevik/projects/risk-security-ctrl/frontend
pnpm dev
```

Or explicitly with the route generation command if configured. Verify that the generated `routeTree.gen.ts` contains:
- The `_authenticated` layout route
- All moved routes nested under `_authenticated`
- The `login` and `register` routes at the top level
- The `index` route at the top level

### 6. Update Existing Tests

Any existing route tests (found at `frontend/src/routes/__tests__/`) that render specific routes may need their test setup updated to provide auth context. Specifically:

- `frontend/src/routes/__tests__/root-nav.test.tsx` - May need to mock auth context since the navbar now shows auth-aware controls (Section 06)
- `frontend/src/routes/__tests__/analysis-nav.test.tsx` - May need auth context since analysis routes are now under `_authenticated`
- `frontend/src/routes/analysis/__tests__/*.test.tsx` - Same, need auth context

For each existing test file, wrap the test render with the auth test utilities (QueryClient + AuthProvider + mocked `fetchCurrentUser` returning a user) so the route guards pass.

### 7. Verify Complete Provider Hierarchy

The final provider nesting order in the rendered app is:

```
StrictMode
  QueryClientProvider (provides TanStack Query)
    AuthProvider (runs useQuery for /me, provides auth context)
      InnerApp
        RouterProvider (receives auth + queryClient context)
          __root.tsx (RootLayout with navbar)
            Outlet
              _authenticated.tsx (beforeLoad guard)
                Outlet
                  [protected route pages]
              login.tsx (public)
              register.tsx (public)
              index.tsx (public)
```

This hierarchy ensures:
1. Query client is available before auth provider runs its initial `/me` query
2. Auth state is resolved before the router renders any routes
3. The `AuthLoadingScreen` (from Section 02) displays during the initial auth check
4. Route guards have access to live auth state via router context
5. Login/register pages can access `useAuth()` for form submissions

## Checklist

1. Update `main.tsx` with `AuthProvider` wrapper and `InnerApp` component
2. Create `routes/_authenticated/` directory
3. Move all protected route files and directories into `_authenticated/`
4. Update `createFileRoute` path strings in every moved file to include `/_authenticated` prefix
5. Run route code generation and verify `routeTree.gen.ts`
6. Update existing test files to provide auth context
7. Create integration test file `auth-wiring.test.tsx`
8. Verify the app compiles (`pnpm build`) and all tests pass (`pnpm test`)