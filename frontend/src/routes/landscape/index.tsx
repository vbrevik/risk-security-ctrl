import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/landscape/")({
  component: RegulatoryLandscapePage,
  validateSearch: (
    search: Record<string, unknown>
  ): { sector?: string; activities?: string } => ({
    sector: typeof search.sector === "string" ? search.sector : undefined,
    activities: typeof search.activities === "string" ? search.activities : undefined,
  }),
});

function RegulatoryLandscapePage() {
  return (
    <div>
      <h1 className="text-2xl font-bold font-mono">Regulatory Landscape</h1>
    </div>
  );
}
