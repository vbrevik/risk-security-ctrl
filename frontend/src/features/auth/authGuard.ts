import { redirect } from "@tanstack/react-router";

interface AuthContext {
  auth: {
    isAuthenticated: boolean;
  };
}

/**
 * Checks auth context and throws a TanStack Router redirect if not authenticated.
 * Used in _authenticated.tsx beforeLoad.
 */
export function requireAuth(context: AuthContext): void {
  if (!context.auth.isAuthenticated) {
    throw redirect({
      to: "/login",
      search: {
        redirect: window.location.pathname,
      },
    });
  }
}
