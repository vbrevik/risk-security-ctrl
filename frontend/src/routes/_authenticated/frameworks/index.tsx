import { useEffect } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { useFrameworks, useConcepts, useRelationships, useFrameworkStats, useAllConcepts } from "@/features/ontology/api";
import { FrameworkSidebar } from "@/features/ontology/components/FrameworkSidebar";
import { FrameworkProfile } from "@/features/ontology/components/FrameworkProfile";

export const Route = createFileRoute("/_authenticated/frameworks/")({
  component: FrameworkCatalogPage,
  validateSearch: (search: Record<string, unknown>): { id?: string } => ({
    id: typeof search.id === "string" ? search.id : undefined,
  }),
});

function FrameworkCatalogPage() {
  const { id } = Route.useSearch();
  const navigate = Route.useNavigate();
  const { data: frameworks = [], isLoading: fwLoading } = useFrameworks();
  const { data: statsMap = new Map(), isLoading: statsLoading } = useFrameworkStats();
  const { data: relationships = [] } = useRelationships();
  const { conceptToFramework } = useAllConcepts();

  // Auto-select first framework if no ?id
  useEffect(() => {
    if (fwLoading || frameworks.length === 0) return;
    if (!id) {
      navigate({ search: { id: frameworks[0].id }, replace: true });
    } else if (!frameworks.find((fw) => fw.id === id)) {
      navigate({ search: { id: frameworks[0].id }, replace: true });
    }
  }, [id, frameworks, fwLoading, navigate]);

  const selectedId = id ?? frameworks[0]?.id ?? null;
  const selectedFramework = frameworks.find((fw) => fw.id === selectedId) ?? null;
  const { data: concepts = [], isLoading: conceptsLoading } = useConcepts(selectedId ?? undefined);

  return (
    <div className="animate-fadeInUp">
      <h1 className="text-2xl font-bold font-mono mb-6">Framework Catalog</h1>
      <div className="flex gap-6 h-[calc(100vh-12rem)]">
        <FrameworkSidebar
          frameworks={frameworks}
          stats={statsMap}
          selectedId={selectedId}
          onSelect={(fwId) => navigate({ search: { id: fwId } })}
          isLoading={fwLoading || statsLoading}
        />
        <FrameworkProfile
          framework={selectedFramework}
          concepts={concepts}
          relationships={relationships}
          stats={selectedId ? statsMap.get(selectedId) ?? null : null}
          frameworks={frameworks}
          conceptToFramework={conceptToFramework}
          isLoading={conceptsLoading}
        />
      </div>
    </div>
  );
}
