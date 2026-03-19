# Frontend Auth — Research

## 1. TanStack Router Protected Routes

### Key pattern: `beforeLoad` + `createRootRouteWithContext`

Auth state is injected via router context, not React hooks (which can't run outside components):

```typescript
// __root.tsx
export const Route = createRootRouteWithContext<{ auth: AuthState }>()({
  component: RootComponent,
})

// In a protected route's beforeLoad:
beforeLoad: ({ context }) => {
  if (!context.auth.user) {
    throw redirect({ to: '/login', search: { redirect: location.href } })
  }
}
```

### Layout route guard pattern (`_authenticated`)

Use a pathless layout route to protect groups of routes:

```
routes/
  _authenticated.tsx           <-- guard, no URL segment
  _authenticated/
    compliance.tsx             <-- /compliance (protected)
    ontology.tsx               <-- /ontology (protected)
  login.tsx                    <-- /login (public)
  index.tsx                    <-- / (public)
```

The `_authenticated.tsx` file runs `beforeLoad` for all nested routes.

**Sources:** [TanStack Router docs](https://tanstack.com/router/latest/docs/framework/react/guide/authenticated-routes), [Atomic Object blog](https://spin.atomicobject.com/authenticated-routes-tanstack-router/)

## 2. TanStack Query + Router Auth Bridge

### Pattern: Query owns auth state, Router consumes via context

```typescript
function AuthedRouterProvider() {
  const auth = useAuth()  // wraps useQuery internally
  useEffect(() => { router.invalidate() }, [auth])
  return <RouterProvider router={router} context={{ auth }} />
}
```

Login/logout mutations update query cache + call `router.invalidate()` to re-run guards.

**Sources:** [TanStack discussions](https://github.com/TanStack/router/discussions/1092)

## 3. httpOnly Cookie Auth Pattern

Since JS can't read httpOnly cookies:
- Use `GET /api/auth/me` to check auth status on app boot
- `credentials: 'include'` on all fetch/axios calls
- Login sets cookie server-side → mutation sets query data → `router.invalidate()`
- Page refresh: browser has cookie → `/me` succeeds → user stays logged in

## 4. Existing Frontend Patterns (from audit)

- **Axios client** (`src/lib/api.ts`): base URL `/api`, 401 interceptor redirects to `/login`
- **TanStack Router**: file-based routing in `src/routes/`
- **TanStack Query**: hooks in `features/*/api/`
- **shadcn/ui**: form components, buttons, inputs, toasts
- **i18next**: namespace-per-feature, en + nb locales
- **Layout**: `__root.tsx` with navbar and sidebar

## 5. Testing

- **Framework:** Vitest (from `pnpm test`)
- **Component tests:** React Testing Library
- **Hook tests:** `@testing-library/react-hooks` or renderHook from RTL
