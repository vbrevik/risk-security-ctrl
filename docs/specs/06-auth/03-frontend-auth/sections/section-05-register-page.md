I have enough context now. Let me generate the section content.

# Section 5: Register Page

## Overview

This section creates the registration page for new users. It includes a route file at `frontend/src/routes/register.tsx` and a `RegisterPage` component at `frontend/src/features/auth/components/RegisterPage.tsx`. The page uses shadcn/ui components (Card, Input, Button, Label), client-side form validation with password confirmation, and i18n via the `auth` namespace.

## Dependencies

- **Section 01 (API Client):** Provides `registerUser()` API function and `RegisterRequest` type from `frontend/src/features/auth/api.ts` and `frontend/src/features/auth/types.ts`
- **Section 02 (Auth Context):** Provides `useAuth()` hook with `register()` method from `frontend/src/features/auth/useAuth.ts`
- **Section 07 (i18n):** Provides `auth` namespace translations (the component will use translation keys; if i18n is not yet wired, tests mock `useTranslation`)

## Files to Create

1. `frontend/src/routes/register.tsx` -- route file
2. `frontend/src/features/auth/components/RegisterPage.tsx` -- page component
3. `frontend/src/features/auth/components/__tests__/RegisterPage.test.tsx` -- tests

## Tests First

Create `frontend/src/features/auth/components/__tests__/RegisterPage.test.tsx`.

The test file should follow the project's established pattern (see `CreateAnalysisForm.test.tsx` for reference): use Vitest, React Testing Library, mock `react-i18next` to return keys, mock `useAuth`, and wrap rendering in a TanStack Router + QueryClient provider.

### Test stubs

```typescript
// frontend/src/features/auth/components/__tests__/RegisterPage.test.tsx

import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
// ... router and query client imports following project pattern

// Mock react-i18next to return keys
vi.mock("react-i18next", () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}));

// Mock useAuth hook
const mockRegister = vi.fn();
vi.mock("@/features/auth/useAuth", () => ({
  useAuth: () => ({
    register: mockRegister,
    // other fields as needed
  }),
}));

// Mock react-hot-toast or sonner (whichever toast lib the project uses)

describe("RegisterPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders email, name, password, and confirm password inputs");
  // Verify four input fields are present with correct types and labels.

  it("shows link to login page");
  // Verify a link with text matching "register.hasAccount" / "register.login" keys
  // points to "/login".

  it("submits with valid data and calls register()");
  // Fill all fields with valid data (email, name, password >= 8 chars,
  // matching confirm). Submit form. Assert mockRegister was called with
  // { email, name, password }.

  it("shows inline error on blur when passwords do not match");
  // Fill password with "abcd1234", fill confirm with "different",
  // blur the confirm field. Assert error text matching
  // "register.passwordMismatch" key is visible.

  it("shows validation error when password is too short");
  // Fill password with "short" (< 8 chars). Submit or blur.
  // Assert error text matching "register.passwordTooShort" key is visible.

  it("redirects to /login with success toast on successful registration");
  // mockRegister resolves successfully. After submit, assert navigation
  // to "/login" and toast was called with "register.success" key.

  it("shows generic error message on server error (422)");
  // mockRegister rejects with a 422 error. Assert an error message
  // is displayed in the UI.

  it("shows validation errors for empty required fields on submit");
  // Submit the form with all fields empty. Assert inline errors appear
  // for email, name, and password fields.
});
```

### Render helper

Use the same `renderWithRouter` pattern as `CreateAnalysisForm.test.tsx`: create a memory router with the component mounted at `/`, wrap in `QueryClientProvider`, and return the render result. Add a `/login` route stub so navigation assertions work.

## Implementation Details

### Route File: `frontend/src/routes/register.tsx`

This is a public route (not under `_authenticated`). It creates a file-based route at `/register`.

```typescript
// frontend/src/routes/register.tsx
import { createFileRoute } from "@tanstack/react-router";
import { RegisterPage } from "@/features/auth/components/RegisterPage";

export const Route = createFileRoute("/register")({
  component: RegisterPage,
});
```

No search params are needed for the register page (unlike login which has `redirect`).

### RegisterPage Component: `frontend/src/features/auth/components/RegisterPage.tsx`

#### Structure

The component renders a centered shadcn `Card` containing a registration form. Layout should match the LoginPage from section 04 for visual consistency.

#### Imports needed

- `useState` from React (for form state and error state)
- `useNavigate` from `@tanstack/react-router`
- `useTranslation` from `react-i18next` with namespace `'auth'`
- `useAuth` from `@/features/auth/useAuth`
- shadcn components: `Card`, `CardContent`, `CardHeader`, `CardTitle` from `@/components/ui/card`; `Button` from `@/components/ui/button`; `Input` from `@/components/ui/input`; `Label` from `@/components/ui/label`
- Toast function (for success message after registration)

#### Form State

Maintain local state for:
- `email: string` -- controlled input, type="email"
- `name: string` -- controlled input
- `password: string` -- controlled input, type="password"
- `confirmPassword: string` -- controlled input, type="password"
- `errors: Record<string, string>` -- field-level validation errors keyed by field name
- `serverError: string | null` -- generic server error message

#### Client-Side Validation

Implement a `validate()` function that checks all fields and returns an errors object:

- **email:** Required, must match a basic email regex pattern. Error key: a generic "required" message.
- **name:** Required, must be non-empty after trimming. Error key: a generic "required" message.
- **password:** Required, minimum 8 characters. Error key: `t('register.passwordTooShort')` if too short.
- **confirmPassword:** Must match `password`. Error key: `t('register.passwordMismatch')` if different.

Password mismatch should also be validated on the confirm password field's `onBlur` event, showing the error immediately without waiting for form submission. Implement this as a separate handler that updates only the `confirmPassword` error in the errors state.

#### Submit Handler

On form submit (`onSubmit`):

1. Prevent default form submission
2. Run `validate()`. If any errors, set them in state and return early.
3. Call `auth.register({ email, name, password })` (from `useAuth()`)
4. On success: navigate to `/login` using `useNavigate()`, and show a toast with `t('register.success')` ("Account created. Please sign in.")
5. On error: catch the error, check if it has a response status of 422, and set `serverError` to a generic error message. For other errors, set a generic server error message as well.

The submit button should show a loading/pending state while the register mutation is in flight. Use the `isPending` state from the mutation or track it locally.

#### Component JSX outline

```
<div className="flex min-h-screen items-center justify-center">
  <Card className="w-full max-w-md">
    <CardHeader>
      <CardTitle>{t('register.title')}</CardTitle>
    </CardHeader>
    <CardContent>
      <form onSubmit={handleSubmit}>
        {serverError && <error alert>}

        <Label>{t('register.email')}</Label>
        <Input type="email" ... />
        {errors.email && <inline error>}

        <Label>{t('register.name')}</Label>
        <Input ... />
        {errors.name && <inline error>}

        <Label>{t('register.password')}</Label>
        <Input type="password" ... />
        {errors.password && <inline error>}

        <Label>{t('register.confirmPassword')}</Label>
        <Input type="password" onBlur={handleConfirmBlur} ... />
        {errors.confirmPassword && <inline error>}

        <Button type="submit" disabled={isPending}>
          {t('register.submit')}
        </Button>
      </form>

      <p>
        {t('register.hasAccount')}{' '}
        <Link to="/login">{t('register.login')}</Link>
      </p>
    </CardContent>
  </Card>
</div>
```

#### Error display

- Field-level errors render as small red text (`text-sm text-destructive`) directly below each input.
- Server errors render as an alert/banner at the top of the form inside the card.
- Clear field errors when the user starts typing in that field (in each input's `onChange`, remove the corresponding key from the errors object).

#### Accessibility

- Each `Label` should have a matching `htmlFor` attribute tied to the input's `id`.
- Error messages should use `aria-describedby` on the input pointing to the error element's `id`.
- The submit button should show a loading spinner or "..." text when pending.