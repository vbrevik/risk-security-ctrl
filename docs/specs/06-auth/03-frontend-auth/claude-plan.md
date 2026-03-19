# Implementation Plan: Frontend Auth

## Context

Risk-security-ctrl is a React/TypeScript application using TanStack Router (file-based), TanStack Query, shadcn/ui, and Tailwind. The backend (Rust/Axum) provides auth API endpoints (register, login, logout, me) that use httpOnly encrypted cookies for session management. The frontend never handles tokens directly — the browser manages cookie transmission.

This plan adds login/register pages, auth state management, route protection, role-based UI, and i18n translations.

## Architecture Overview

Four layers:

1. **Auth API hooks** — TanStack Query mutations/queries for auth endpoints
2. **Auth context** — React context + useAuth hook bridging Query state to components and router
3. **Route protection** — TanStack Router `beforeLoad` guards via pathless layout route
4. **Auth UI** — Login page, register page, navbar auth controls

Data flow: Login form → useMutation → POST /api/auth/login → backend sets cookie → mutation sets query data → router.invalidate() → guards re-run → redirect to dashboard.

## Section 1: API Client Updates

### Axios Configuration

Update `frontend/src/lib/api.ts`:

- Add `withCredentials: true` to the Axios instance config — required for cookie transmission
- Add `X-Requested-With: XMLHttpRequest` to default headers — required by CSRF middleware (split 02)
- **Critical: Update the 401 interceptor** to skip redirect for `/auth/me` requests. The `fetchCurrentUser()` function expects 401 to return null (not trigger a redirect). Without this exclusion, unauthenticated visitors hitting any page will get a misleading "Session expired" toast and hard redirect before the auth context even resolves. The interceptor should check `error.config?.url` and skip `/auth/me`:
  ```
  if (status === 401 && !url.includes('/auth/me')) { redirect to /login with toast }
  ```

### Auth API Types

Create `frontend/src/features/auth/types.ts`:

```typescript
interface LoginRequest { email: string; password: string }
interface RegisterRequest { email: string; name: string; password: string }
interface UserProfile { id: string; email: string; name: string; role: string }
interface AuthResponse { token: string; user: UserProfile }  // token is returned by backend but ignored by frontend (cookie handles auth). Kept for type completeness.
```

### Auth API Functions

Create `frontend/src/features/auth/api.ts`:

- `fetchCurrentUser()` — GET /api/auth/me → UserProfile | null. Returns null on 401.
- `loginUser(data: LoginRequest)` — POST /api/auth/login → AuthResponse
- `registerUser(data: RegisterRequest)` — POST /api/auth/register → UserProfile
- `logoutUser()` — POST /api/auth/logout → void

All use the shared Axios instance (cookies sent automatically).

## Section 2: Auth Context and useAuth Hook

### AuthProvider

Create `frontend/src/features/auth/AuthProvider.tsx`:

The provider uses TanStack Query to manage auth state:

- `useQuery({ queryKey: ['auth', 'me'], queryFn: fetchCurrentUser, staleTime: 5 * 60 * 1000, retry: false, refetchOnWindowFocus: true })` as the single source of truth. Uses 5-minute staleTime (not Infinity) so session revocations are detected within a reasonable window. `refetchOnWindowFocus: true` catches cases where the session expired while the tab was in the background.
- Shows `AuthLoadingScreen` (full-screen spinner) while the initial query is loading. **Note:** This blocks the entire app including public pages during the initial check. For a government air-gapped app with fast local network, this is acceptable. If latency becomes an issue, consider only blocking `_authenticated` routes.
- Once resolved, renders children with auth context available
- **Error handling:** If `/me` returns a network error or 500, show an error screen with a retry button instead of spinning forever.

### useAuth Hook

Create `frontend/src/features/auth/useAuth.ts`:

Exposes:
- `user: UserProfile | null` — current user or null
- `isAuthenticated: boolean` — shorthand for `!!user`
- `isLoading: boolean` — true during initial auth check
- `login(data: LoginRequest): Promise<void>` — calls loginUser mutation, sets query data, calls `router.invalidate()`
- `logout(): Promise<void>` — calls logoutUser mutation, clears query data, calls `router.invalidate()`, navigates to /login
- `register(data: RegisterRequest): Promise<UserProfile>` — calls registerUser mutation

### useHasPermission Hook

Create `frontend/src/features/auth/useHasPermission.ts`:

```typescript
function useHasPermission(feature: string, action: string): boolean
```

Mirrors the backend permission matrix from split 02. Maps the user's role against a client-side copy of the permission table. Used for conditional UI rendering (hide delete buttons for viewers, hide admin controls for non-admins).

The permission matrix should be defined as a simple object mapping `role → feature → action → boolean`.

## Section 3: Route Protection

### Restructure Route Tree

Move all protected routes under a `_authenticated` pathless layout route:

**New file structure:**
```
frontend/src/routes/
  __root.tsx                    <-- root layout (keep existing)
  index.tsx                     <-- / home page (public, keep existing)
  login.tsx                     <-- /login (new)
  register.tsx                  <-- /register (new)
  _authenticated.tsx            <-- auth guard layout (new)
  _authenticated/
    ontology/                   <-- move from routes/ontology/
    compliance/                 <-- move from routes/compliance/
    concepts/                   <-- move from routes/concepts/
    crosswalk.tsx               <-- move from routes/
    frameworks.tsx              <-- move from routes/
    landscape.tsx               <-- move from routes/
    reports.tsx                 <-- move from routes/
```

### _authenticated.tsx Guard

```typescript
export const Route = createFileRoute('/_authenticated')({
  beforeLoad: ({ context, location }) => {
    if (!context.auth.isAuthenticated) {
      throw redirect({
        to: '/login',
        search: { redirect: location.href },  // use router's location, not window.location — preserves search params
      })
    }
  },
  component: () => <Outlet />,
})
```

### Root Route Context

Update `__root.tsx` to use `createRootRouteWithContext`:

```typescript
interface RouterContext {
  auth: ReturnType<typeof useAuth>
  queryClient: QueryClient
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: RootComponent,
})
```

### Router Provider Update

In the app entry point, wrap `RouterProvider` to inject live auth context:

```typescript
function InnerApp() {
  const auth = useAuth()
  useEffect(() => { router.invalidate() }, [auth.user])
  return <RouterProvider router={router} context={{ auth, queryClient }} />
}
```

## Section 4: Login Page

### Route File

Create `frontend/src/routes/login.tsx`:

- Search params: `{ redirect?: string }` — preserved from the auth guard redirect
- Component: `LoginPage`

### LoginPage Component

Create `frontend/src/features/auth/components/LoginPage.tsx`:

- shadcn Card centered on page
- Email input (type="email", required)
- Password input (type="password", required)
- Submit button with loading state
- Error display for invalid credentials (from 401 response)
- Link to /register ("Don't have an account?")
- On success: redirect to `search.redirect` or `/` (home)
- Uses `useAuth().login()` mutation
- All text via `useTranslation('auth')`

### Form Validation

Client-side validation:
- Email: required, valid format
- Password: required, min 8 chars (match server-side minimum — no valid password is shorter than 8)
- Show field-level errors inline

## Section 5: Register Page

### Route File

Create `frontend/src/routes/register.tsx`

### RegisterPage Component

Create `frontend/src/features/auth/components/RegisterPage.tsx`:

- shadcn Card centered on page
- Email input (type="email", required)
- Name input (required)
- Password input (type="password", min 8 chars)
- Confirm password input (must match password)
- Submit button with loading state
- Error display for server errors (generic 422)
- Link to /login ("Already have an account?")
- On success: redirect to /login with toast "Account created. Please log in."
- Uses `useAuth().register()` mutation
- All text via `useTranslation('auth')`

### Form Validation

Client-side:
- Email: required, valid format
- Name: required, non-empty
- Password: required, min 8 characters
- Confirm password: must match password field
- Show field-level errors inline. Show password mismatch immediately on blur.

## Section 6: Navbar and Layout Updates

### Navbar Auth Controls

Update the existing navbar/sidebar in `__root.tsx` or its layout components:

**When authenticated:**
- Show user name and role badge (e.g., "Admin", "Viewer")
- Show logout button
- Clicking logout calls `useAuth().logout()`

**When not authenticated:**
- Show "Login" link pointing to /login
- Optionally show "Register" link

### Sidebar Updates

The sidebar navigation should only show links to routes the user can access. Use `useHasPermission` to conditionally render navigation items. All authenticated users see all nav links (since read access is universal), but admin-only items (like future user management) should be conditionally shown.

## Section 7: i18n Translations

### Create Auth Namespace Files

**`frontend/src/i18n/locales/en/auth.json`:**
```json
{
  "login": {
    "title": "Sign In",
    "email": "Email",
    "password": "Password",
    "submit": "Sign In",
    "noAccount": "Don't have an account?",
    "register": "Create Account",
    "error": "Invalid email or password"
  },
  "register": {
    "title": "Create Account",
    "email": "Email",
    "name": "Full Name",
    "password": "Password",
    "confirmPassword": "Confirm Password",
    "submit": "Create Account",
    "hasAccount": "Already have an account?",
    "login": "Sign In",
    "passwordMismatch": "Passwords do not match",
    "passwordTooShort": "Password must be at least 8 characters",
    "success": "Account created. Please sign in."
  },
  "session": {
    "expired": "Session expired. Please sign in again."
  },
  "navbar": {
    "logout": "Sign Out"
  }
}
```

**`frontend/src/i18n/locales/nb/auth.json`:**
Norwegian Bokmål translations for all the same keys.

### Register Namespace

Add `'auth'` to the i18next namespace configuration so it's loaded alongside existing namespaces.

## Section 8: Wiring and Integration

### App Entry Point

Update the app entry to wire everything together:

1. `QueryClientProvider` wraps everything (already exists)
2. `AuthProvider` wraps inside QueryClientProvider
3. `RouterProvider` receives auth context

### File Moves

Move existing route files under `_authenticated/`:
- `routes/ontology/` → `routes/_authenticated/ontology/`
- `routes/compliance/` → `routes/_authenticated/compliance/`
- `routes/concepts/` → `routes/_authenticated/concepts/`
- `routes/crosswalk.tsx` → `routes/_authenticated/crosswalk.tsx`
- `routes/frameworks.tsx` → `routes/_authenticated/frameworks.tsx`
- `routes/landscape.tsx` → `routes/_authenticated/landscape.tsx`
- `routes/reports.tsx` → `routes/_authenticated/reports.tsx`

Update any internal imports/links that reference moved routes.

### TanStack Router Code Generation

After moving files, run the TanStack Router code generator to update the route tree:
```bash
pnpm dev  # or pnpm build — triggers route generation
```

Verify that the generated route tree reflects the new `_authenticated` layout route.

## Edge Cases

- **Page refresh while authenticated:** Browser has cookie → GET /me succeeds → user state restored
- **Deep link while unauthenticated:** User visits /compliance → guard saves redirect → login → redirect back
- **Multiple tabs:** Login in one tab, other tabs still have stale state until they make an API call. The 401 interceptor handles this gracefully.
- **Network error during auth check:** Show error state, allow retry
- **Race condition on logout:** Clear query data immediately (optimistic), don't wait for POST /logout to complete

## Opus Review Integration

Critical fixes applied:
- **401 interceptor must skip /auth/me** — prevents infinite redirect loop for unauthenticated visitors
- **staleTime reduced to 5 min** with refetchOnWindowFocus — catches server-side session revocation
- **Use router's `location` in beforeLoad** — preserves search params on redirect
- **Login password validation: min 8** — match server-side minimum
- **AuthResponse.token documented as ignored** — clarifies cookie-only architecture
- **Error boundary on auth loading** — retry button if /me fails with network error
- **Analysis routes added to file moves** — were missing from the list

Acknowledged but not changed:
- **Duplicated permission matrix:** Frontend copy is intentional — it's a UI convenience, backend enforces truth. Sync is manual but the matrix is small and stable.
- **Register doesn't auto-login:** Intentional per spec. Forces user through login flow for security.
- **AuthLoadingScreen blocks public pages:** Acceptable for fast air-gapped network. Optimize later if needed.
- **No test plan in claude-plan.md:** Tests are in claude-plan-tdd.md (40+ test stubs). This is the project's convention.

## Known Gaps (Deferred)

- **Password change UI:** Not in scope (no email service for reset)
- **User profile page:** Not in scope for this split
- **Admin user management UI:** Future feature, guarded by useHasPermission
