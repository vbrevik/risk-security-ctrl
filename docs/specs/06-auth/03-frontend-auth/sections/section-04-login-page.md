Now I have all the context I need. Let me generate the section content.

# Section 4: Login Page

## Overview

This section creates the login page for the application. It consists of two files:

1. **Route file** at `frontend/src/routes/login.tsx` -- defines the `/login` route with optional `redirect` search param
2. **LoginPage component** at `frontend/src/features/auth/components/LoginPage.tsx` -- the actual login form UI

The login page uses shadcn/ui Card, Input, Button, and Label components (all already present in the project at `frontend/src/components/ui/`). All user-visible text uses the `auth` i18n namespace (created in section-07).

## Dependencies

- **section-01-api-client**: Provides `LoginRequest` type and the auth API functions
- **section-02-auth-context**: Provides `useAuth()` hook with `login()` method
- **section-07-i18n**: Provides `auth` namespace translations (keys like `login.title`, `login.email`, `login.password`, `login.submit`, `login.noAccount`, `login.register`, `login.error`)

## Tests

Create test file at `frontend/src/features/auth/components/__tests__/LoginPage.test.tsx`.

Tests use Vitest + React Testing Library. The `useAuth` hook and router should be mocked.

```typescript
// frontend/src/features/auth/components/__tests__/LoginPage.test.tsx

// Test: LoginPage renders email and password inputs
//   - render LoginPage in a test wrapper with QueryClient and i18n
//   - assert screen.getByLabelText matching email and password labels exist
//   - assert email input has type="email"
//   - assert password input has type="password"

// Test: LoginPage renders submit button
//   - assert screen.getByRole('button', { name: /sign in/i }) exists

// Test: LoginPage shows link to register page
//   - assert a link with text matching "Create Account" or the i18n key login.register exists
//   - assert the link points to /register

// Test: submit with valid credentials calls login()
//   - mock useAuth().login to return a resolved promise
//   - fill in email and password fields
//   - click submit button
//   - assert login was called with { email, password }

// Test: submit shows loading state on button
//   - mock useAuth().login to return a pending promise (never resolves during test)
//   - fill in fields and submit
//   - assert button is disabled or shows a spinner/loading indicator

// Test: invalid credentials shows error message
//   - mock useAuth().login to reject with a 401 error
//   - fill in fields and submit
//   - await the error message appearing (text matching login.error translation)

// Test: successful login redirects to redirect param or home
//   - mock useAuth().login to resolve
//   - render LoginPage with search params { redirect: '/compliance' }
//   - fill in and submit
//   - assert navigation to /compliance
//   - repeat without redirect param, assert navigation to /

// Test: empty fields show validation errors
//   - click submit without filling fields
//   - assert HTML5 validation prevents submission (required fields)
//   - or assert custom validation error messages appear
```

## Implementation

### Route File

Create `frontend/src/routes/login.tsx`:

- Use `createFileRoute('/login')` from TanStack Router
- Define search params validation to accept an optional `redirect` string parameter
- The component simply renders `<LoginPage />`
- This route is public (outside the `_authenticated` layout), so no auth guard applies

```typescript
// frontend/src/routes/login.tsx
// Stub:
import { createFileRoute } from '@tanstack/react-router'
import { LoginPage } from '@/features/auth/components/LoginPage'

// validateSearch should parse { redirect?: string } from the URL search params
// Component renders LoginPage
```

### LoginPage Component

Create `frontend/src/features/auth/components/LoginPage.tsx`:

**Layout and structure:**
- Centered on page using flexbox (`min-h-screen flex items-center justify-center`)
- shadcn `Card` with `CardHeader`, `CardContent`, `CardFooter`
- Card title uses `t('login.title')` from the `auth` namespace

**Form fields:**
- Email input: `type="email"`, `required`, labeled with `t('login.email')`
- Password input: `type="password"`, `required`, labeled with `t('login.password')`
- Both use shadcn `Input` and `Label` components
- Manage field values with React `useState`

**Submit behavior:**
- Form `onSubmit` handler calls `e.preventDefault()`, then calls `auth.login({ email, password })`
- The `login()` call is obtained from `useAuth()` (section-02)
- Track a local `isPending` state (or use `useMutation` isPending) to disable the button and show loading text during submission
- On success: use `useNavigate()` from TanStack Router to navigate to `search.redirect || '/'`
- Access `search.redirect` via `Route.useSearch()` where `Route` is imported from the route file, or via `useSearch({ from: '/login' })`

**Error handling:**
- Catch errors from `login()` in a try/catch
- On 401 (invalid credentials): set a local `error` state string, display it above or below the form using the `t('login.error')` translation
- Display the error in a styled div (e.g., `text-destructive` class)

**Register link:**
- Below the form (in `CardFooter`), show text `t('login.noAccount')` with a `Link` to `/register` using text `t('login.register')`

**Accessibility:**
- Labels are associated with inputs via `htmlFor`
- Submit button has descriptive text
- Error messages use `role="alert"` for screen readers

```typescript
// frontend/src/features/auth/components/LoginPage.tsx
// Stub:
// - imports: React useState, useTranslation, useNavigate, Link
// - imports: Card, CardHeader, CardTitle, CardContent, CardFooter from @/components/ui/card
// - imports: Input, Label, Button from @/components/ui
// - imports: useAuth from @/features/auth/useAuth
//
// export function LoginPage():
//   - const { t } = useTranslation('auth')
//   - const auth = useAuth()
//   - const navigate = useNavigate()
//   - local state: email, password, error, isPending
//   - get redirect from route search params
//   - handleSubmit: call auth.login, navigate on success, set error on failure
//   - render Card with form containing email/password inputs, submit button, error display, register link
```

### Key Implementation Details

**Search param access:** The `redirect` search parameter is set by the auth guard (section-03) when an unauthenticated user tries to access a protected route. The login page reads it to redirect back after successful login. Use TanStack Router's `Route.useSearch()` or `useSearch()` to access it.

**Form state management:** Use simple `useState` for email, password, error, and isPending. No need for a form library -- the form is simple enough that native HTML validation plus useState is sufficient.

**Error clearing:** Clear the error state whenever the user starts typing again (onChange handlers) so stale errors do not persist.

**Loading state:** While `isPending` is true, the submit button should be disabled and show a loading indicator. Use the shadcn Button's `disabled` prop and conditionally render a spinner icon (e.g., `Loader2` from lucide-react with `animate-spin` class) alongside the submit text.

## File Summary

| File | Action |
|------|--------|
| `frontend/src/routes/login.tsx` | Create |
| `frontend/src/features/auth/components/LoginPage.tsx` | Create |
| `frontend/src/features/auth/components/__tests__/LoginPage.test.tsx` | Create |