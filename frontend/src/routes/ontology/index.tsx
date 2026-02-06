import { createFileRoute } from "@tanstack/react-router";
import { OntologyExplorer } from "@/features/ontology/components";

export interface OntologySearch {
  view?: "graph" | "tree" | "detail" | "compare";
  concept?: string;
  frameworks?: string;
  type?: string;
}

export const Route = createFileRoute("/ontology/")({
  component: OntologyPage,
  validateSearch: (search: Record<string, unknown>): OntologySearch => ({
    view: (search.view as OntologySearch["view"]) ?? undefined,
    concept: (search.concept as string) ?? undefined,
    frameworks: (search.frameworks as string) ?? undefined,
    type: (search.type as string) ?? undefined,
  }),
});

function OntologyPage() {
  return <OntologyExplorer />;
}
