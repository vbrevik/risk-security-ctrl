import { useMemo } from "react";
import { getFrameworkColor } from "../utils/graphTransform";
import type { Framework, Relationship } from "../types";

interface LandscapeResultsProps {
  applicableFrameworkIds: string[];
  frameworks: Framework[];
  relationships: Relationship[];
  conceptCountMap: Map<string, number>;
  conceptToFramework: Map<string, string>;
}

export function LandscapeResults({
  applicableFrameworkIds,
  frameworks,
  relationships,
  conceptCountMap,
  conceptToFramework,
}: LandscapeResultsProps) {
  const applicableSet = useMemo(() => new Set(applicableFrameworkIds), [applicableFrameworkIds]);

  const applicableFrameworks = frameworks.filter((fw) => applicableSet.has(fw.id));
  const nonApplicableFrameworks = frameworks.filter((fw) => !applicableSet.has(fw.id));

  // Summary stats
  const totalConcepts = useMemo(() => {
    let sum = 0;
    for (const fwId of applicableFrameworkIds) {
      sum += conceptCountMap.get(fwId) ?? 0;
    }
    return sum;
  }, [applicableFrameworkIds, conceptCountMap]);

  const crossRelationshipCount = useMemo(() => {
    return relationships.filter((r) => {
      const srcFw = conceptToFramework.get(r.source_concept_id);
      const tgtFw = conceptToFramework.get(r.target_concept_id);
      return srcFw && tgtFw && applicableSet.has(srcFw) && applicableSet.has(tgtFw) && srcFw !== tgtFw;
    }).length;
  }, [relationships, conceptToFramework, applicableSet]);

  // Overlap between adjacent frameworks
  const getOverlapCount = (fwA: string, fwB: string) => {
    return relationships.filter((r) => {
      const srcFw = conceptToFramework.get(r.source_concept_id);
      const tgtFw = conceptToFramework.get(r.target_concept_id);
      return (
        (srcFw === fwA && tgtFw === fwB) ||
        (srcFw === fwB && tgtFw === fwA)
      );
    }).length;
  };

  return (
    <div className="space-y-6">
      {/* Summary banner */}
      <div className="feature-card corner-markers p-4" data-testid="summary-banner">
        <div className="flex gap-8 text-center">
          <div>
            <div className="stat-number">{applicableFrameworkIds.length}</div>
            <div className="text-xs text-foreground/50">Frameworks</div>
          </div>
          <div>
            <div className="stat-number">{totalConcepts}</div>
            <div className="text-xs text-foreground/50">Concepts</div>
          </div>
          <div>
            <div className="stat-number">{crossRelationshipCount}</div>
            <div className="text-xs text-foreground/50">Cross-links</div>
          </div>
        </div>
      </div>

      {/* Compliance stack */}
      <div>
        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-3">
          Applicable Frameworks
        </h3>
        <div className="space-y-1">
          {applicableFrameworks.map((fw, i) => {
            const count = conceptCountMap.get(fw.id) ?? 0;
            const nextFw = applicableFrameworks[i + 1];
            const overlap = nextFw ? getOverlapCount(fw.id, nextFw.id) : 0;

            return (
              <div key={fw.id}>
                <div className="feature-card p-3 flex items-center gap-3">
                  <span
                    className="w-3 h-3 rounded-full flex-shrink-0"
                    style={{ backgroundColor: getFrameworkColor(fw.id) }}
                  />
                  <div className="flex-1">
                    <div className="font-mono text-sm font-medium">{fw.name}</div>
                    {fw.description && (
                      <div className="text-[10px] text-foreground/40 truncate">{fw.description}</div>
                    )}
                  </div>
                  <span className="tech-badge">{count} concepts</span>
                </div>
                {overlap > 0 && (
                  <div className="flex justify-center py-0.5">
                    <span className="text-[9px] text-foreground/30 bg-muted px-2 py-0.5 rounded-full">
                      {overlap} shared mappings
                    </span>
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>

      {/* Non-applicable frameworks */}
      {nonApplicableFrameworks.length > 0 && (
        <div>
          <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
            Other Frameworks
          </h3>
          <div className="space-y-0.5 opacity-40">
            {nonApplicableFrameworks.map((fw) => (
              <div key={fw.id} className="flex items-center gap-2 text-xs py-0.5">
                <span
                  className="w-2 h-2 rounded-full flex-shrink-0"
                  style={{ backgroundColor: getFrameworkColor(fw.id) }}
                />
                <span className="font-mono">{fw.name}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
