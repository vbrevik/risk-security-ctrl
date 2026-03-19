import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/login")({
  validateSearch: (search: Record<string, unknown>) => ({
    redirect: (search.redirect as string) || undefined,
  }),
  component: LoginPagePlaceholder,
});

function LoginPagePlaceholder() {
  return <div>Login page (placeholder — implemented in section 04)</div>;
}
