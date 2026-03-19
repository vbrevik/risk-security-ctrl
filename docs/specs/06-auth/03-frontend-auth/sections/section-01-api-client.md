I have all the context I need. Here is the section content:

# Section 1: API Client Updates

## Overview

This section updates the shared Axios instance in `frontend/src/lib/api.ts` with cookie and CSRF support, then creates the auth feature's type definitions and API functions. These are the foundational building blocks that all other auth sections depend on.

**Files to create:**
- `frontend/src/features/auth/types.ts`
- `frontend/src/features/auth/api.ts`
- `frontend/src/features/auth/__tests__/api.test.ts`

**Files to modify:**
- `frontend/src/lib/api.ts`

**Dependencies:** None (this is the first section).

**Blocks:** Section 02 (auth context), Section 04 (login page), Section 05 (register page).

---

## Tests First

Create `frontend/src/features/auth/__tests__/api.test.ts`. The project uses Vitest with `vi.mock` for mocking modules and `@testing-library/react` for component/hook tests. Follow the existing pattern in `frontend/src/features/ontology/api/__tests__/hooks.test.ts` which mocks `@/lib/api`.

### Axios Configuration Tests

These tests verify the Axios instance exported from `@/lib/api`:

```typescript
// Test: Axios instance has withCredentials: true
// Import the api instance and assert api.defaults.withCredentials === true

// Test: Axios instance has X-Requested-With: XMLHttpRequest default header
// Assert api.defaults.headers.common['X-Requested-With'] === 'XMLHttpRequest'
//   or that it appears in api.defaults.headers (Axios stores common headers in .common)
```

### 401 Interceptor Tests

The 401 interceptor behavior is harder to unit test because it involves `window.location` side effects. A pragmatic approach:

```typescript
// Test: 401 interceptor redirects to /login
// Mock window.location, call api.get() with a mocked 401 response,
// verify window.location.href is set to '/login'
```

Note: The plan calls for adding a toast notification on 401 ("Session expired, please log in again"). However, the project does not currently have a toast library installed (no sonner, react-hot-toast, or shadcn toast component). For now, implement the redirect only. The toast can be added later when a toast system is introduced (likely in Section 04 or 06). If you do install a toast library, prefer `sonner` which integrates well with shadcn/ui.

### Auth API Function Tests

```typescript
// Test: fetchCurrentUser returns UserProfile on 200
// Mock api.get('/auth/me') to resolve with { data: { id: '1', email: '...', name: '...', role: 'admin' } }
// Assert return value matches the UserProfile shape

// Test: fetchCurrentUser returns null on 401
// Mock api.get('/auth/me') to reject with a 401 error
// Assert return value is null (function catches 401 and returns null)

// Test: loginUser sends POST with credentials and returns AuthResponse
// Mock api.post('/auth/login', { email, password }) to resolve with AuthResponse data
// Assert the correct payload is sent and response is returned

// Test: registerUser sends POST and returns UserProfile
// Mock api.post('/auth/register', { email, name, password }) to resolve
// Assert correct payload and return value

// Test: logoutUser sends POST and returns void
// Mock api.post('/auth/logout') to resolve
// Assert it was called
```

Use the same mocking pattern as the existing test file: `vi.mock("@/lib/api")` at module level, then `vi.mocked(api)` for type-safe mock assertions.

---

## Implementation Details

### 1. Update Axios Instance

**File:** `frontend/src/lib/api.ts`

Modify the existing `axios.create()` call to add two properties:

- `withCredentials: true` in the instance config object. This tells the browser to include cookies (including the httpOnly session cookie set by the backend) on all requests to the API.
- `"X-Requested-With": "XMLHttpRequest"` in the `headers` object. This header is required by the CSRF middleware on the backend (from spec 06-auth split 02). The backend rejects non-XHR requests to mutating endpoints without this header.

The 401 interceptor already exists and redirects to `/login`. Keep this behavior. The current implementation does a hard redirect via `window.location.href = "/login"`. This is acceptable for now. Later sections may refine this to use the TanStack Router navigate function instead.

The updated file should look approximately like:

```typescript
import axios from "axios";

export const api = axios.create({
  baseURL: "/api",
  withCredentials: true,
  headers: {
    "Content-Type": "application/json",
    "X-Requested-With": "XMLHttpRequest",
  },
});

api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      window.location.href = "/login";
    }
    return Promise.reject(error);
  }
);
```

### 2. Create Auth Types

**File:** `frontend/src/features/auth/types.ts`

Define the following interfaces. These mirror the backend's auth API request/response shapes:

```typescript
interface LoginRequest { email: string; password: string }
interface RegisterRequest { email: string; name: string; password: string }
interface UserProfile { id: string; email: string; name: string; role: string }
interface AuthResponse { token: string; user: UserProfile }
```

Export all interfaces. The `AuthResponse` includes a `token` field because the backend returns it in the login response body, but the frontend does not need to store or use it -- the httpOnly cookie is the actual session credential. The `token` field is kept for type completeness.

### 3. Create Auth API Functions

**File:** `frontend/src/features/auth/api.ts`

Create four exported async functions that use the shared Axios instance from `@/lib/api`:

**`fetchCurrentUser(): Promise<UserProfile | null>`**
- Calls `GET /auth/me`
- On success (200), returns the response data as `UserProfile`
- On 401 error, catches the error and returns `null` (user is not authenticated)
- Important: This function must catch the 401 specifically and return null rather than letting it propagate. Otherwise the Axios interceptor would redirect to /login during the initial auth check, which would be incorrect. Check `error.response?.status === 401` in the catch block. Re-throw any non-401 errors.

**`loginUser(data: LoginRequest): Promise<AuthResponse>`**
- Calls `POST /auth/login` with the `LoginRequest` body
- Returns the `AuthResponse` from the response data
- Let errors propagate (the calling code in useAuth will handle them)

**`registerUser(data: RegisterRequest): Promise<UserProfile>`**
- Calls `POST /auth/register` with the `RegisterRequest` body
- Returns the `UserProfile` from the response data
- Let errors propagate

**`logoutUser(): Promise<void>`**
- Calls `POST /auth/logout`
- Returns void
- Let errors propagate (but callers should clear local state optimistically regardless)

All functions import `api` from `@/lib/api` and use `api.get()` / `api.post()`. The cookie is sent automatically because `withCredentials: true` is set on the instance. No manual token handling is needed.

### Important Note on 401 Interceptor Interaction

The Axios 401 interceptor in `api.ts` redirects to `/login` on any 401 response. The `fetchCurrentUser` function must handle 401 responses before the interceptor fires, or the interceptor needs to be updated to skip redirects for the `/auth/me` endpoint. The cleanest approach: update the interceptor to check whether the request URL is `/auth/me` and skip the redirect for that endpoint. Alternatively, `fetchCurrentUser` can use a separate Axios instance or a try/catch that swallows the interceptor's redirect. The recommended approach is to update the interceptor:

```typescript
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // Don't redirect on /auth/me — that endpoint is expected to return 401
      // when the user is not authenticated
      const url = error.config?.url ?? "";
      if (!url.includes("/auth/me")) {
        window.location.href = "/login";
      }
    }
    return Promise.reject(error);
  }
);
```

This way `fetchCurrentUser` can simply catch the rejected promise and return `null`, without triggering a page redirect.