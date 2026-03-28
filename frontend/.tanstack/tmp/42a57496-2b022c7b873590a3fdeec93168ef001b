import { createFileRoute, Navigate } from "@tanstack/react-router";

export const Route = createFileRoute("/_authenticated/concepts/")({
  component: ConceptsRedirect,
});

function ConceptsRedirect() {
  return <Navigate to="/concepts/search" />;
}
