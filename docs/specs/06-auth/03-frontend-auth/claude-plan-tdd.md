# TDD Plan: Frontend Auth

## Testing Infrastructure

- **Framework:** Vitest + React Testing Library
- **Component tests:** `@testing-library/react` with `render`, `screen`, `fireEvent`
- **Hook tests:** `renderHook` from `@testing-library/react`
- **Mocking:** Mock Axios/API calls, TanStack Query provider wrapper

---

## Section 1: API Client Updates — Tests

```typescript
// Test: Axios instance has withCredentials: true
// Test: Axios instance has X-Requested-With: XMLHttpRequest default header
// Test: 401 interceptor shows toast and redirects to /login
// Test: fetchCurrentUser returns UserProfile on 200
// Test: fetchCurrentUser returns null on 401
// Test: loginUser sends POST with credentials and returns AuthResponse
// Test: registerUser sends POST and returns UserProfile
// Test: logoutUser sends POST and returns void
```

## Section 2: Auth Context and useAuth — Tests

```typescript
// Test: useAuth returns isLoading=true while initial /me query pending
// Test: useAuth returns user=null when not authenticated
// Test: useAuth returns user object when authenticated
// Test: useAuth.isAuthenticated is true when user is present
// Test: useAuth.login() calls loginUser and updates query data
// Test: useAuth.logout() calls logoutUser, clears user, navigates to /login
// Test: useAuth.register() calls registerUser and returns result
// Test: useHasPermission returns true for admin on any feature/action
// Test: useHasPermission returns false for viewer on write actions
// Test: useHasPermission returns true for specialist on compliance:create
```

## Section 3: Route Protection — Tests

```typescript
// Test: _authenticated beforeLoad redirects to /login when no user
// Test: _authenticated beforeLoad allows access when user present
// Test: redirect includes current path as search param
// Test: public routes (/, /login, /register) accessible without auth
// Test: router.invalidate() is called when auth state changes
```

## Section 4: Login Page — Tests

```typescript
// Test: LoginPage renders email and password inputs
// Test: LoginPage renders submit button
// Test: LoginPage shows link to register page
// Test: submit with valid credentials calls login()
// Test: submit shows loading state on button
// Test: invalid credentials shows error message
// Test: successful login redirects to redirect param or home
// Test: empty fields show validation errors
```

## Section 5: Register Page — Tests

```typescript
// Test: RegisterPage renders email, name, password, confirm password inputs
// Test: RegisterPage shows link to login page
// Test: submit with valid data calls register()
// Test: password mismatch shows inline error on blur
// Test: password too short shows validation error
// Test: successful registration redirects to /login with success toast
// Test: server error (422) shows generic error message
// Test: empty required fields show validation errors
```

## Section 6: Navbar Updates — Tests

```typescript
// Test: navbar shows user name and role badge when authenticated
// Test: navbar shows logout button when authenticated
// Test: navbar shows login link when not authenticated
// Test: clicking logout calls useAuth().logout()
```

## Section 7: i18n — Tests

```typescript
// Test: auth namespace loads in en locale
// Test: auth namespace loads in nb locale
// Test: LoginPage renders translated labels
// Test: RegisterPage renders translated labels
```

## Section 8: Wiring — Tests

```typescript
// Test: full flow: visit /compliance → redirected to /login → login → redirected to /compliance
// Test: full flow: register → redirect to /login → login → access dashboard
// Test: logout clears auth state and redirects to /login
// Test: page refresh preserves auth state (cookie-based)
```
