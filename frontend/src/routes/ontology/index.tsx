import { createFileRoute } from "@tanstack/react-router";
import { OntologyExplorer } from "@/features/ontology/components";

export const Route = createFileRoute("/ontology/")({
  component: OntologyPage,
});

function OntologyPage() {
  return <OntologyExplorer />;
}
