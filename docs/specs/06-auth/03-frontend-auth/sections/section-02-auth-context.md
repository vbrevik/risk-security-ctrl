I have all the context needed. Here is the section content:

# Section 2: Auth Context and useAuth Hook

## Overview

This section creates the `AuthProvider` React context, the `useAuth` hook, and the `useHasPermission` hook. Together these form the central auth state layer that bridges TanStack Query (where the actual user data lives) to components and the router. An `AuthLoadingScreen` component is also created to show a spinner during the initial `/me` check.

**Depends on:** Section 01 (API client updates) -- specifically `fetchCurrentUser`, `loginUser`, `registerUser`, `logoutUser` from `frontend/src/features/auth/api.ts` and the types from `frontend/src/features/auth/types.ts`.

**Blocks:** Sections 03, 04, 05, 06 (route protection, login page, register page, navbar).

## Files to Create

| File | Purpose |
|------|---------|
| `frontend/src/features/auth/AuthProvider.tsx` | React context provider wrapping the auth query |
| `frontend/src/features/auth/useAuth.ts` | Hook exposing auth state and mutations |
| `frontend/src/features/auth/useHasPermission.ts` | Hook for client-side role-based permission checks |
| `frontend/src/features/auth/components/AuthLoadingScreen.tsx` | Full-screen spinner shown during initial auth check |
| `frontend/src/features/auth/__tests__/useAuth.test.ts` | Tests for useAuth hook |
| `frontend/src/features/auth/__tests__/useHasPermission.test.ts` | Tests for useHasPermission hook |

## Tests First

### `frontend/src/features/auth/__tests__/useAuth.test.ts`

Tests use `renderHook` from `@testing-library/react` with a wrapper that provides both `QueryClientProvider` and `AuthProvider`. API functions from `../api` are mocked via `vi.mock`.

```typescript
import { renderHook, waitFor } from "@testing-library/react";
import { vi, describe, it, expect } from "vitest";

// Mock the API module
vi.mock("../api");

describe("useAuth", () => {
  // Test: useAuth returns isLoading=true while initial /me query is pending
  // Render hook with fetchCurrentUser that never resolves.
  // Assert isLoading === true and user === null.

  // Test: useAuth returns user=null when not authenticated
  // fetchCurrentUser resolves to null (simulating 401).
  // Assert user === null, isAuthenticated === false.

  // Test: useAuth returns user object when authenticated
  // fetchCurrentUser resolves to { id: "1", email: "a@b.c", name: "A", role: "admin" }.
  // Assert user matches, isAuthenticated === true.

  // Test: useAuth.isAuthenticated is true when user is present
  // Same as above, explicit boolean check.

  // Test: useAuth.login() calls loginUser and updates query data
  // Call result.current.login({ email, password }).
  // Assert loginUser was called with correct args.
  // After settling, user should be the returned user from AuthResponse.

  // Test: useAuth.logout() calls logoutUser, clears user, navigates to /login
  // Start with an authenticated user.
  // Call result.current.logout().
  // Assert logoutUser was called, user becomes null, isAuthenticated false.

  // Test: useAuth.register() calls registerUser and returns result
  // Call result.current.register({ email, name, password }).
  // Assert registerUser called with correct args, returns UserProfile.
});
```

### `frontend/src/features/auth/__tests__/useHasPermission.test.ts`

```typescript
import { renderHook } from "@testing-library/react";
import { describe, it, expect } from "vitest";

describe("useHasPermission", () => {
  // Test: returns true for admin on any feature/action
  // Mock useAuth to return user with role "admin".
  // Assert useHasPermission("compliance", "delete") === true.

  // Test: returns false for viewer on write actions
  // Mock useAuth to return user with role "viewer".
  // Assert useHasPermission("compliance", "create") === false.
  // Assert useHasPermission("compliance", "delete") === false.

  // Test: returns true for specialist on compliance:create
  // Mock useAuth to return user with role "specialist".
  // Assert useHasPermission("compliance", "create") === true.
  // Assert useHasPermission("compliance", "read") === true.

  // Test: returns false when not authenticated
  // Mock useAuth to return user = null.
  // Assert useHasPermission("anything", "read") === false.
});
```

### Test Wrapper Utility

Create a shared test utility for wrapping hooks:

```typescript
// frontend/src/features/auth/__tests__/test-utils.tsx
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AuthProvider } from "../AuthProvider";

export function createAuthWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return (
      <QueryClientProvider client={queryClient}>
        <AuthProvider>{children}</AuthProvider>
      </QueryClientProvider>
    );
  };
}
```

## Implementation Details

### AuthLoadingScreen Component

**File:** `frontend/src/features/auth/components/AuthLoadingScreen.tsx`

A simple full-screen centered spinner. Uses a Tailwind utility layout (`flex items-center justify-center h-screen`). Can use a shadcn `Loader2` icon from `lucide-react` with `animate-spin`. No props required. This is displayed by `AuthProvider` while the initial `/me` query is in the `isLoading` state.

### AuthProvider

**File:** `frontend/src/features/auth/AuthProvider.tsx`

Creates a React context (`AuthContext`) and exports `AuthProvider`.

Key design decisions:

1. **Single query as source of truth:** Uses `useQuery` with `queryKey: ['auth', 'me']` and `queryFn: fetchCurrentUser`. The `staleTime` is set to `Infinity` (the data only changes on explicit login/logout, not by time). `retry` is set to `false` (a 401 is not transient).

2. **Loading gate:** While `isLoading` is true (first render, before `/me` resolves), render `<AuthLoadingScreen />` instead of `children`. This prevents route guards from seeing `user === undefined` and incorrectly redirecting to `/login` on page refresh.

3. **Context value:** The context exposes the full return of `useAuth`-style values. However, the actual logic lives in `useAuth` -- the context simply stores the query result and mutation callbacks so they are accessible anywhere in the tree.

The context type:

```typescript
interface AuthContextType {
  user: UserProfile | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (data: LoginRequest) => Promise<void>;
  logout: () => Promise<void>;
  register: (data: RegisterRequest) => Promise<UserProfile>;
}
```

The provider internally uses `useMutation` for login, logout, and register. After a successful login mutation, it calls `queryClient.setQueryData(['auth', 'me'], response.user)` to immediately update the cached user without a refetch. After logout, it calls `queryClient.setQueryData(['auth', 'me'], null)`.

### useAuth Hook

**File:** `frontend/src/features/auth/useAuth.ts`

A thin wrapper that reads from `AuthContext`:

```typescript
export function useAuth(): AuthContextType {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return context;
}
```

The hook itself is simple because all the logic lives in `AuthProvider`. This separation keeps the hook lightweight and testable.

**Login flow inside AuthProvider:**
1. Call `loginUser(data)` via mutation
2. On success, `queryClient.setQueryData(['auth', 'me'], response.user)`
3. Call `router.invalidate()` -- this triggers TanStack Router to re-run `beforeLoad` guards, which now see the user and allow access

**Logout flow inside AuthProvider:**
1. Call `queryClient.setQueryData(['auth', 'me'], null)` immediately (optimistic)
2. Fire `logoutUser()` mutation (don't await -- the cookie is cleared server-side but the UI should respond instantly)
3. Call `router.invalidate()`
4. Navigate to `/login`

**Router reference:** The provider needs access to the router instance. Import `useRouter` from `@tanstack/react-router` to call `router.invalidate()` and `router.navigate()`.

### useHasPermission Hook

**File:** `frontend/src/features/auth/useHasPermission.ts`

```typescript
export function useHasPermission(feature: string, action: string): boolean
```

This hook mirrors the backend RBAC permission matrix from the backend auth spec (split 02). It defines a client-side permission table as a constant:

```typescript
const PERMISSIONS: Record<string, Record<string, Record<string, boolean>>> = {
  admin: {
    // admin has access to everything -- use a wildcard pattern
  },
  specialist: {
    compliance: { create: true, read: true, update: true, delete: false },
    analysis: { create: true, read: true, update: true, delete: false },
    ontology: { read: true },
    reports: { create: true, read: true },
  },
  viewer: {
    compliance: { read: true },
    analysis: { read: true },
    ontology: { read: true },
    reports: { read: true },
  },
};
```

Logic:
1. Get user from `useAuth()`
2. If no user, return `false`
3. If user role is `"admin"`, return `true` (admin has all permissions)
4. Look up `PERMISSIONS[role][feature][action]`, return the boolean or `false` if not found

This is a UI convenience only -- the backend enforces the real authorization. The client-side matrix is used to hide/show UI elements (e.g., hide "Delete" buttons for viewers).

### Barrel Export

Create or update `frontend/src/features/auth/index.ts` to re-export:
- `AuthProvider` from `./AuthProvider`
- `useAuth` from `./useAuth`
- `useHasPermission` from `./useHasPermission`
- All types from `./types`

## Important Notes

- The `AuthProvider` must be placed inside `QueryClientProvider` but outside `RouterProvider` in the component tree. This wiring is handled in Section 08, but the provider is designed with this constraint in mind.
- The `router.invalidate()` call is critical for TanStack Router's `beforeLoad` guards to re-evaluate after login/logout. Without it, the guards use stale context.
- `fetchCurrentUser` returning `null` on 401 (instead of throwing) is essential. The auth query should succeed with a `null` value for unauthenticated users, not enter an error state. This is handled in Section 01's API implementation.