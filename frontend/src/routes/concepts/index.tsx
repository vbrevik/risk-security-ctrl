import { createFileRoute, Navigate } from "@tanstack/react-router";

export const Route = createFileRoute("/concepts/")({
  component: ConceptsRedirect,
});

function ConceptsRedirect() {
  return <Navigate to="/concepts/search" />;
}
