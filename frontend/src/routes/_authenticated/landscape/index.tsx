import { useMemo } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { useFrameworks, useRelationships, useAllConcepts } from "@/features/ontology/api";
import { getApplicableFrameworks } from "@/features/ontology/utils/landscapeMapping";
import { parseCommaSeparated } from "@/features/ontology/utils/urlParams";
import { LandscapeSelector } from "@/features/ontology/components/LandscapeSelector";
import { LandscapeResults } from "@/features/ontology/components/LandscapeResults";

export const Route = createFileRoute("/_authenticated/landscape/")({
  component: RegulatoryLandscapePage,
  validateSearch: (
    search: Record<string, unknown>
  ): { sector?: string; activities?: string } => ({
    sector: typeof search.sector === "string" ? search.sector : undefined,
    activities: typeof search.activities === "string" ? search.activities : undefined,
  }),
});

function RegulatoryLandscapePage() {
  const { sector, activities: activitiesParam } = Route.useSearch();
  const navigate = Route.useNavigate();
  const { data: frameworks = [] } = useFrameworks();
  const { data: relationships = [] } = useRelationships();
  const { data: allConcepts, conceptToFramework } = useAllConcepts();

  const activityList = parseCommaSeparated(activitiesParam);
  const applicableIds = getApplicableFrameworks(sector ?? "", activityList);

  const conceptCountMap = useMemo(() => {
    const map = new Map<string, number>();
    for (const c of allConcepts) {
      map.set(c.framework_id, (map.get(c.framework_id) || 0) + 1);
    }
    return map;
  }, [allConcepts]);

  return (
    <div className="animate-fadeInUp">
      <h1 className="text-2xl font-bold font-mono mb-6">Regulatory Landscape</h1>
      <div className="flex gap-6">
        <div className="w-80 shrink-0">
          <LandscapeSelector
            sector={sector}
            activities={activityList}
            onSectorChange={(s) =>
              navigate({ search: (prev) => ({ ...prev, sector: s }), replace: true })
            }
            onActivitiesChange={(acts) =>
              navigate({
                search: (prev) => ({
                  ...prev,
                  activities: acts.length > 0 ? acts.join(",") : undefined,
                }),
                replace: true,
              })
            }
          />
        </div>
        <div className="flex-1 min-w-0">
          <LandscapeResults
            applicableFrameworkIds={applicableIds}
            frameworks={frameworks}
            relationships={relationships}
            conceptCountMap={conceptCountMap}
            conceptToFramework={conceptToFramework}
          />
        </div>
      </div>
    </div>
  );
}
