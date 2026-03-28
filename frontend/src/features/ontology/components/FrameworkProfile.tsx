import { useState, useEffect } from "react";
import { BookOpen, ExternalLink, ChevronRight, ChevronDown, ShieldCheck } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { Framework, Concept, Relationship, FrameworkStats } from "../types";
import { getFrameworkColor } from "../utils/graphTransform";
import { VerificationBadge } from "./VerificationBadge";
import { ProofPanel } from "./ProofPanel";

interface FrameworkProfileProps {
  framework: Framework | null;
  concepts: Concept[];
  relationships: Relationship[];
  stats: FrameworkStats | null;
  frameworks: Framework[];
  conceptToFramework: Map<string, string>;
  isLoading: boolean;
}

function typeToHue(type: string): number {
  let hash = 0;
  for (const ch of type) hash = (hash * 31 + ch.charCodeAt(0)) & 0xffffff;
  return hash % 360;
}

const REL_TYPE_COLORS: Record<string, string> = {
  maps_to: "bg-blue-500/20 text-blue-700",
  implements: "bg-green-500/20 text-green-700",
  related_to: "bg-gray-500/20 text-gray-700",
  supports: "bg-amber-500/20 text-amber-700",
};

export function FrameworkProfile({
  framework,
  concepts,
  relationships,
  stats,
  frameworks,
  conceptToFramework,
  isLoading,
}: FrameworkProfileProps) {
  const [expanded, setExpanded] = useState<Set<string>>(new Set());
  const [showProof, setShowProof] = useState(false);
  const { t } = useTranslation("ontology");

  // Reset expanded state when framework changes
  useEffect(() => {
    setExpanded(new Set());
  }, [framework?.id]);

  // Reset proof panel when framework changes
  useEffect(() => {
    setShowProof(false);
  }, [framework?.id]);

  if (!framework) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center text-foreground/40">
        <BookOpen className="w-12 h-12 mb-3" />
        <p className="text-sm">Select a framework from the sidebar</p>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex-1 space-y-4">
        <div className="h-8 w-48 bg-muted rounded animate-pulse" />
        <div className="h-4 w-96 bg-muted rounded animate-pulse" />
        <div className="grid grid-cols-4 gap-4">
          {Array.from({ length: 4 }).map((_, i) => (
            <div key={i} className="h-20 bg-muted rounded animate-pulse" />
          ))}
        </div>
      </div>
    );
  }

  // Cross-framework connections
  const conceptIds = new Set(concepts.map((c) => c.id));
  const fwRelationships = relationships.filter(
    (r) => conceptIds.has(r.source_concept_id) || conceptIds.has(r.target_concept_id)
  );

  const connectionMap = new Map<string, { count: number; types: Set<string> }>();
  for (const rel of fwRelationships) {
    const otherConceptId = conceptIds.has(rel.source_concept_id)
      ? rel.target_concept_id
      : rel.source_concept_id;
    const otherFwId = conceptToFramework.get(otherConceptId);
    if (!otherFwId || otherFwId === framework.id) continue;
    const entry = connectionMap.get(otherFwId) ?? { count: 0, types: new Set() };
    entry.count++;
    entry.types.add(rel.relationship_type);
    connectionMap.set(otherFwId, entry);
  }

  const connections = [...connectionMap.entries()]
    .map(([fwId, data]) => ({
      fwId,
      name: frameworks.find((f) => f.id === fwId)?.name ?? fwId,
      count: data.count,
      types: [...data.types],
    }))
    .sort((a, b) => b.count - a.count);

  // Concept hierarchy
  const topLevel = concepts.filter((c) => c.parent_id === null);

  const toggleExpand = (id: string) => {
    setExpanded((prev) => {
      const next = new Set(prev);
      next.has(id) ? next.delete(id) : next.add(id);
      return next;
    });
  };

  return (
    <div className="flex-1 overflow-y-auto space-y-6">
      {/* Header */}
      <div>
        <div className="flex items-center gap-3 mb-2">
          <h2 className="text-2xl font-bold font-mono">{framework.name}</h2>
          {framework.version && (
            <span className="tech-badge">{framework.version}</span>
          )}
          <VerificationBadge status={framework.verification_status} />
        </div>
        {framework.description && (
          <p className="text-sm text-foreground/70 mb-2">{framework.description}</p>
        )}
        <div className="flex flex-wrap items-center gap-3">
          {framework.source_url && (
            <a
              href={framework.source_url}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-1 text-xs text-foreground/50 hover:text-foreground/80 transition-colors"
            >
              <ExternalLink className="w-3 h-3" />
              {t("common.source", "Source")}
            </a>
          )}
          {framework.verification_status !== null && (
            <button
              onClick={() => setShowProof((prev) => !prev)}
              className="inline-flex items-center gap-1 text-xs text-foreground/50 hover:text-foreground/80 transition-colors"
            >
              <ShieldCheck className="w-3 h-3" />
              {showProof ? t("proof.hideProof", "Hide Proof") : t("proof.viewProof", "View Proof")}
            </button>
          )}
        </div>
      </div>

      {/* Proof Panel */}
      {showProof && <ProofPanel frameworkId={framework.id} />}

      {/* Stats Strip */}
      {stats && (
        <div className="grid grid-cols-4 gap-3">
          {[
            { label: "Concepts", value: stats.conceptCount },
            { label: "Types", value: Object.keys(stats.conceptTypes).length },
            { label: "Connected", value: stats.connectedFrameworks },
            { label: "Relationships", value: stats.relationshipCount },
          ].map((s) => (
            <div key={s.label} className="feature-card corner-markers p-3 text-center">
              <div className="text-xs text-foreground/50 mb-1">{s.label}</div>
              <div className="stat-number">{s.value}</div>
            </div>
          ))}
        </div>
      )}

      {/* Concept Type Breakdown */}
      {stats && Object.keys(stats.conceptTypes).length > 0 && (
        <div>
          <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
            Concept Types
          </h3>
          <div
            data-testid="type-breakdown-bar"
            className="flex h-3 rounded-full overflow-hidden mb-2"
          >
            {Object.entries(stats.conceptTypes).map(([type, count]) => (
              <div
                key={type}
                style={{
                  width: `${(count / stats.conceptCount) * 100}%`,
                  backgroundColor: `hsl(${typeToHue(type)}, 60%, 55%)`,
                }}
                title={`${type}: ${count}`}
              />
            ))}
          </div>
          <div className="flex flex-wrap gap-3 text-xs">
            {Object.entries(stats.conceptTypes).map(([type, count]) => (
              <div key={type} className="flex items-center gap-1.5">
                <span
                  className="w-2 h-2 rounded-full"
                  style={{ backgroundColor: `hsl(${typeToHue(type)}, 60%, 55%)` }}
                />
                <span className="text-foreground/70 capitalize">{type}</span>
                <span className="text-foreground/40">{count}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Cross-Framework Connections */}
      <div>
        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
          Cross-Framework Connections
        </h3>
        {connections.length === 0 ? (
          <p className="text-xs text-foreground/40">No cross-framework connections</p>
        ) : (
          <div className="space-y-1">
            {connections.map((conn) => (
              <div
                key={conn.fwId}
                className="flex items-center gap-2 px-2 py-1.5 rounded hover:bg-muted/50 transition-colors text-sm"
              >
                <span
                  className="w-2 h-2 rounded-full flex-shrink-0"
                  style={{ backgroundColor: getFrameworkColor(conn.fwId) }}
                />
                <span className="flex-1 font-mono text-xs">{conn.name}</span>
                <span className="tech-badge text-[10px]">{conn.count}</span>
                {conn.types.map((relType) => (
                  <span
                    key={relType}
                    className={`text-[9px] px-1.5 py-0.5 rounded-full ${REL_TYPE_COLORS[relType] ?? "bg-gray-500/20 text-gray-700"}`}
                  >
                    {relType.replace(/_/g, " ")}
                  </span>
                ))}
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Concept Hierarchy Preview */}
      <div>
        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
          Concept Hierarchy
        </h3>
        {topLevel.length === 0 ? (
          <p className="text-xs text-foreground/40">No concepts</p>
        ) : (
          <div className="space-y-0.5">
            {topLevel.map((concept) => {
              const isExpanded = expanded.has(concept.id);
              const children = concepts.filter((c) => c.parent_id === concept.id);
              return (
                <div key={concept.id}>
                  <button
                    onClick={() => toggleExpand(concept.id)}
                    className="w-full flex items-center gap-1.5 px-1 py-1 rounded hover:bg-muted/50 transition-colors text-left text-xs"
                  >
                    {children.length > 0 ? (
                      isExpanded ? (
                        <ChevronDown className="w-3 h-3 text-foreground/40" />
                      ) : (
                        <ChevronRight className="w-3 h-3 text-foreground/40" />
                      )
                    ) : (
                      <span className="w-3" />
                    )}
                    {concept.code && (
                      <span className="font-mono text-foreground/50">{concept.code}</span>
                    )}
                    <span className="flex-1">{concept.name_en}</span>
                    <span className="tech-badge text-[9px]">{concept.concept_type}</span>
                  </button>
                  {isExpanded &&
                    children.map((child) => (
                      <div
                        key={child.id}
                        className="flex items-center gap-1.5 pl-6 px-1 py-1 text-xs text-foreground/70"
                      >
                        {child.code && (
                          <span className="font-mono text-foreground/40">{child.code}</span>
                        )}
                        <span className="flex-1">{child.name_en}</span>
                        <span className="tech-badge text-[9px]">{child.concept_type}</span>
                      </div>
                    ))}
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
