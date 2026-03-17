import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/concepts/search")({
  component: ConceptSearchPage,
  validateSearch: (
    search: Record<string, unknown>
  ): { q?: string; frameworks?: string; types?: string } => ({
    q: typeof search.q === "string" ? search.q : undefined,
    frameworks: typeof search.frameworks === "string" ? search.frameworks : undefined,
    types: typeof search.types === "string" ? search.types : undefined,
  }),
});

function ConceptSearchPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold font-mono">Concept Search</h1>
    </div>
  );
}
