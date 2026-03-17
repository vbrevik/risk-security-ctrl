import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/frameworks/")({
  component: FrameworkCatalogPage,
  validateSearch: (search: Record<string, unknown>): { id?: string } => ({
    id: typeof search.id === "string" ? search.id : undefined,
  }),
});

function FrameworkCatalogPage() {
  return (
    <div>
      <h1 className="text-2xl font-bold font-mono">Framework Catalog</h1>
    </div>
  );
}
