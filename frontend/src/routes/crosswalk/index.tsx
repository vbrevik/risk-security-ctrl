import { createFileRoute } from "@tanstack/react-router";
import { CrosswalkView } from "@/features/ontology/components/Crosswalk/CrosswalkView";

export interface CrosswalkSearch {
  source?: string;
  target?: string;
  level?: "map" | "submap" | "playbook";
}

export const Route = createFileRoute("/crosswalk/")({
  component: CrosswalkPage,
  validateSearch: (search: Record<string, unknown>): CrosswalkSearch => ({
    source: (search.source as string) ?? undefined,
    target: (search.target as string) ?? undefined,
    level: (search.level as CrosswalkSearch["level"]) ?? undefined,
  }),
});

function CrosswalkPage() {
  const { source, target, level } = Route.useSearch();
  return (
    <CrosswalkView
      initialSource={source}
      initialTarget={target}
      initialLevel={level}
    />
  );
}
