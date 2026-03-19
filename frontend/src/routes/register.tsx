import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/register")({
  component: RegisterPagePlaceholder,
});

function RegisterPagePlaceholder() {
  return <div>Register page (placeholder — implemented in section 05)</div>;
}
