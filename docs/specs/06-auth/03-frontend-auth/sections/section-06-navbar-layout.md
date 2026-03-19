I have all the context needed. Let me produce the section content.

# Section 6: Navbar and Layout Updates

## Overview

This section adds auth-aware controls to the existing navbar in the root layout. When the user is authenticated, the navbar displays their name, a role badge, and a logout button. When not authenticated, it shows a "Login" link. The sidebar navigation conditionally renders items based on permissions.

**Depends on:** section-02-auth-context (provides `useAuth` hook and `useHasPermission` hook)

## Files to Create or Modify

| File | Action |
|------|--------|
| `frontend/src/routes/__root.tsx` | Modify -- add auth controls to the header |
| `frontend/src/features/auth/components/NavbarAuthControls.tsx` | Create -- extracted component for auth portion of navbar |

## Tests First

Create `frontend/src/features/auth/components/__tests__/NavbarAuthControls.test.tsx`:

```typescript
// Test: navbar shows user name and role badge when authenticated
//   - Render NavbarAuthControls with a mocked useAuth returning a user
//   - Expect screen.getByText(user.name) to be present
//   - Expect screen.getByText(user.role) to be present (inside a badge)

// Test: navbar shows logout button when authenticated
//   - Render NavbarAuthControls with a mocked useAuth returning a user
//   - Expect a button with translated text t('auth:navbar.logout') to be present

// Test: navbar shows login link when not authenticated
//   - Render NavbarAuthControls with a mocked useAuth returning user=null
//   - Expect a link to /login to be present
//   - Expect no logout button to be present

// Test: clicking logout calls useAuth().logout()
//   - Render NavbarAuthControls with a mocked useAuth returning a user
//   - Click the logout button
//   - Expect the mocked logout function to have been called once
```

### Test Setup Notes

- Mock `useAuth` from `@/features/auth/useAuth` to control the returned user/isAuthenticated/logout values.
- Wrap renders in a test `QueryClientProvider` and a stub TanStack Router context (or use `MemoryRouter` equivalent for link rendering).
- Use `@testing-library/react` `render`, `screen`, `fireEvent`/`userEvent`.
- Use `useTranslation` mock or the real i18n with test resources loaded so that translation keys resolve.

## Implementation Details

### NavbarAuthControls Component

Create `frontend/src/features/auth/components/NavbarAuthControls.tsx`.

This is a small presentational component extracted from the root layout to keep the navbar clean. It receives no props -- it calls `useAuth()` internally.

**Behavior when `isAuthenticated` is true:**

- Display the user's `name` in a small monospace text span
- Display a `Badge` (from `@/components/ui/badge`) showing the user's `role` (capitalized). Use the `outline` variant for the badge.
- Display a logout `Button` (ghost variant, small size) that calls `auth.logout()` on click.
- All visible text uses `useTranslation('auth')` -- the logout button label is `t('auth:navbar.logout')`.

**Behavior when `isAuthenticated` is false:**

- Display a `Link` (from `@tanstack/react-router`) pointing to `/login` with text from `t('auth:login.title')` (which resolves to "Sign In").
- Optionally display a `Link` to `/register` with text from `t('auth:register.title')`.

**Skeleton structure (not full implementation):**

```typescript
import { useAuth } from '@/features/auth/useAuth'
import { useTranslation } from 'react-i18next'
import { Link } from '@tanstack/react-router'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { LogOut } from 'lucide-react'

export function NavbarAuthControls() {
  const { user, isAuthenticated, logout } = useAuth()
  const { t } = useTranslation('auth')

  if (!isAuthenticated) {
    // Render Link to /login (and optionally /register)
  }

  // Render user name, role Badge (outline variant), and logout Button (ghost, size="sm")
  // Logout button onClick calls logout()
  // Use LogOut icon from lucide-react alongside or instead of text
}
```

### Modifying __root.tsx

The existing `__root.tsx` has a header with a `<div>` on the right side containing only the language toggle button. Insert `<NavbarAuthControls />` into that trailing `<div>`, placed before the language toggle.

The relevant area in the current file (around line 88-93):

```tsx
<div className="flex flex-1 items-center justify-end space-x-2 flex-shrink-0">
  {/* ADD NavbarAuthControls HERE */}
  <NavbarAuthControls />
  <Button variant="ghost" size="icon" onClick={toggleLanguage} className="h-7 w-7">
    <Globe className="h-3.5 w-3.5" />
    <span className="sr-only">Toggle language</span>
  </Button>
</div>
```

Add the import at the top of `__root.tsx`:

```typescript
import { NavbarAuthControls } from '@/features/auth/components/NavbarAuthControls'
```

No other changes are needed in `__root.tsx` for this section. The route context changes (`createRootRouteWithContext`) happen in section-03 and section-08.

### Sidebar Considerations

The current application does not have a general sidebar -- the only sidebar is the ontology-specific one at `frontend/src/features/ontology/components/Sidebar/Sidebar.tsx`. The main navigation is the horizontal nav bar in `__root.tsx`.

The nav links in the header are currently visible to everyone. Once auth is wired (section-08), these links will only be reachable behind the `_authenticated` guard, so no permission-based conditional rendering is needed for existing nav items. If admin-only routes are added in the future, use `useHasPermission` to conditionally render those specific links.

No sidebar modifications are required in this section.

### Styling Notes

- The auth controls should use the same `text-xs font-medium font-mono` style as existing nav items for consistency.
- The role badge should be small -- use `className="text-[10px] px-1.5 py-0"` or similar overrides on the Badge to keep it compact.
- The user name and controls group should have `space-x-2` or `gap-2` between elements.
- Use the `LogOut` icon from `lucide-react` (already a project dependency) for the logout button, keeping it consistent with the existing `Globe` icon button style.