import { useMemo, useState, useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "@tanstack/react-router";
import { useFrameworks, useConcepts, useRelationships } from "../../api";
import { getFrameworkColor } from "../../utils/graphTransform";
import { ArrowRight, ArrowLeftRight, Layers, GitBranch, BookOpen } from "lucide-react";
import { Button } from "@/components/ui/button";
import type { Concept, Relationship } from "../../types";

export type MappingLevel = "map" | "submap" | "playbook";

interface Props {
  initialSource?: string;
  initialTarget?: string;
  initialLevel?: MappingLevel;
}

/** Determines the depth of a concept in its framework hierarchy */
function getConceptDepth(concept: Concept, conceptMap: Map<string, Concept>): number {
  let depth = 0;
  let current = concept;
  while (current.parent_id) {
    depth++;
    const parent = conceptMap.get(current.parent_id);
    if (!parent) break;
    current = parent;
  }
  return depth;
}

/** Classify a relationship into map/submap/playbook level based on concept depths */
function classifyLevel(
  rel: Relationship,
  conceptMap: Map<string, Concept>
): MappingLevel {
  const source = conceptMap.get(rel.source_concept_id);
  const target = conceptMap.get(rel.target_concept_id);
  if (!source || !target) return "map";

  const srcDepth = getConceptDepth(source, conceptMap);
  const tgtDepth = getConceptDepth(target, conceptMap);
  const maxDepth = Math.max(srcDepth, tgtDepth);

  if (maxDepth <= 1) return "map";
  if (maxDepth === 2) return "submap";
  return "playbook";
}

export function CrosswalkView({ initialSource, initialTarget, initialLevel }: Props) {
  const { t, i18n } = useTranslation("ontology");
  const language = i18n.language.startsWith("nb") ? "nb" : "en";

  const [sourceId, setSourceId] = useState(initialSource ?? "");
  const [targetId, setTargetId] = useState(initialTarget ?? "");
  const [levelFilter, setLevelFilter] = useState<MappingLevel | "all">(initialLevel ?? "all");
  const [selectedRelId, setSelectedRelId] = useState<string | null>(null);

  // Sync state changes back to URL for bookmarking/sharing
  const navigate = useNavigate();
  const navigateRef = useRef(navigate);
  navigateRef.current = navigate;
  const lastSearchRef = useRef("");

  useEffect(() => {
    const params: Record<string, string> = {};
    if (sourceId) params.source = sourceId;
    if (targetId) params.target = targetId;
    if (levelFilter !== "all") params.level = levelFilter;

    const serialized = JSON.stringify(params);
    if (serialized === lastSearchRef.current) return;
    lastSearchRef.current = serialized;

    navigateRef.current({ search: params, replace: true });
  }, [sourceId, targetId, levelFilter]);

  const { data: frameworks } = useFrameworks();
  const { data: sourceConcepts } = useConcepts(sourceId || undefined);
  const { data: targetConcepts } = useConcepts(targetId || undefined);
  const { data: relationships } = useRelationships();

  // Build concept lookup from both frameworks
  const sourceList = Array.isArray(sourceConcepts) ? sourceConcepts : [];
  const targetList = Array.isArray(targetConcepts) ? targetConcepts : [];

  const conceptMap = useMemo(() => {
    const map = new Map<string, Concept>();
    for (const c of sourceList) map.set(c.id, c);
    for (const c of targetList) map.set(c.id, c);
    return map;
  }, [sourceList, targetList]);

  // Get cross-framework relationships
  const crossRelationships = useMemo(() => {
    if (!relationships || !sourceId || !targetId) return [];
    const sourceIds = new Set(sourceList.map((c) => c.id));
    const targetIds = new Set(targetList.map((c) => c.id));

    return relationships.filter(
      (rel) =>
        (sourceIds.has(rel.source_concept_id) && targetIds.has(rel.target_concept_id)) ||
        (targetIds.has(rel.source_concept_id) && sourceIds.has(rel.target_concept_id))
    );
  }, [relationships, sourceId, targetId, sourceList, targetList]);

  // Classify and filter by level
  const classifiedRels = useMemo(() => {
    return crossRelationships.map((rel) => ({
      ...rel,
      level: classifyLevel(rel, conceptMap),
    }));
  }, [crossRelationships, conceptMap]);

  const filteredRels = useMemo(() => {
    if (levelFilter === "all") return classifiedRels;
    return classifiedRels.filter((r) => r.level === levelFilter);
  }, [classifiedRels, levelFilter]);

  // Count by level
  const levelCounts = useMemo(() => {
    const counts = { map: 0, submap: 0, playbook: 0 };
    for (const r of classifiedRels) counts[r.level]++;
    return counts;
  }, [classifiedRels]);

  // Count by type
  const typeCounts = useMemo(() => {
    const counts: Record<string, number> = {};
    for (const r of filteredRels) {
      counts[r.relationship_type] = (counts[r.relationship_type] || 0) + 1;
    }
    return counts;
  }, [filteredRels]);

  const getName = (concept: Concept) =>
    language === "nb" && concept.name_nb ? concept.name_nb : concept.name_en;

  const swapFrameworks = () => {
    setSourceId(targetId);
    setTargetId(sourceId);
  };

  const selectedRel = selectedRelId
    ? classifiedRels.find((r) => r.id === selectedRelId)
    : null;

  const levelIcons: Record<MappingLevel, React.ReactNode> = {
    map: <Layers className="w-3.5 h-3.5" />,
    submap: <GitBranch className="w-3.5 h-3.5" />,
    playbook: <BookOpen className="w-3.5 h-3.5" />,
  };

  const levelLabels: Record<MappingLevel, { en: string; nb: string }> = {
    map: { en: "Function / Step", nb: "Funksjon / Steg" },
    submap: { en: "Subcategory / Task", nb: "Underkategori / Oppgave" },
    playbook: { en: "Action / Playbook", nb: "Handling / Spillebok" },
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold">{t("crosswalk.title")}</h1>
        <p className="text-muted-foreground mt-1">{t("crosswalk.description")}</p>
      </div>

      {/* Framework selectors */}
      <div className="flex items-center gap-3 flex-wrap">
        <div className="flex-1 min-w-[200px]">
          <label className="text-xs font-mono font-semibold uppercase tracking-wider text-muted-foreground mb-1 block">
            {t("crosswalk.sourceFramework")}
          </label>
          <select
            value={sourceId}
            onChange={(e) => setSourceId(e.target.value)}
            className="w-full px-3 py-2 border rounded-md bg-background font-mono text-sm"
          >
            <option value="">{t("compare.selectFramework")}</option>
            {frameworks?.map((fw) => (
              <option key={fw.id} value={fw.id}>{fw.name}</option>
            ))}
          </select>
        </div>

        <Button variant="ghost" size="icon" onClick={swapFrameworks} className="mt-5">
          <ArrowLeftRight className="w-4 h-4" />
        </Button>

        <div className="flex-1 min-w-[200px]">
          <label className="text-xs font-mono font-semibold uppercase tracking-wider text-muted-foreground mb-1 block">
            {t("crosswalk.targetFramework")}
          </label>
          <select
            value={targetId}
            onChange={(e) => setTargetId(e.target.value)}
            className="w-full px-3 py-2 border rounded-md bg-background font-mono text-sm"
          >
            <option value="">{t("compare.selectFramework")}</option>
            {frameworks?.map((fw) => (
              <option key={fw.id} value={fw.id}>{fw.name}</option>
            ))}
          </select>
        </div>
      </div>

      {/* Level filter tabs */}
      {sourceId && targetId && (
        <div className="flex items-center gap-2 flex-wrap">
          <button
            onClick={() => setLevelFilter("all")}
            className={`px-3 py-1.5 rounded-md text-xs font-mono font-medium transition-colors ${
              levelFilter === "all"
                ? "bg-primary text-primary-foreground"
                : "bg-muted text-muted-foreground hover:text-foreground"
            }`}
          >
            {t("crosswalk.allLevels")} ({classifiedRels.length})
          </button>
          {(["map", "submap", "playbook"] as const).map((level) => (
            <button
              key={level}
              onClick={() => setLevelFilter(level)}
              className={`flex items-center gap-1.5 px-3 py-1.5 rounded-md text-xs font-mono font-medium transition-colors ${
                levelFilter === level
                  ? "bg-primary text-primary-foreground"
                  : "bg-muted text-muted-foreground hover:text-foreground"
              }`}
            >
              {levelIcons[level]}
              {levelLabels[level][language]} ({levelCounts[level]})
            </button>
          ))}
        </div>
      )}

      {/* Stats bar */}
      {sourceId && targetId && filteredRels.length > 0 && (
        <div className="flex items-center gap-4 text-xs font-mono text-muted-foreground">
          {Object.entries(typeCounts).map(([type, count]) => (
            <span key={type} className="flex items-center gap-1.5">
              <span
                className="w-2 h-2 rounded-full"
                style={{ backgroundColor: typeColor(type) }}
              />
              {type.replace(/_/g, " ")} ({count})
            </span>
          ))}
        </div>
      )}

      {/* Main content area */}
      {sourceId && targetId ? (
        <div className="flex gap-4 min-h-[500px]">
          {/* Mapping list */}
          <div className="flex-1 border rounded-lg overflow-hidden">
            {filteredRels.length === 0 ? (
              <div className="p-8 text-center text-muted-foreground">
                <ArrowLeftRight className="w-12 h-12 mx-auto mb-3 opacity-30" />
                <p className="font-medium">{t("crosswalk.noMappings")}</p>
                <p className="text-sm mt-1">{t("crosswalk.noMappingsHint")}</p>
              </div>
            ) : (
              <div className="divide-y">
                {filteredRels.map((rel) => {
                  const source = conceptMap.get(rel.source_concept_id);
                  const target = conceptMap.get(rel.target_concept_id);
                  if (!source || !target) return null;

                  const isSourceFromLeft = source.framework_id === sourceId;
                  const left = isSourceFromLeft ? source : target;
                  const right = isSourceFromLeft ? target : source;
                  const isSelected = selectedRelId === rel.id;

                  return (
                    <button
                      key={rel.id}
                      onClick={() => setSelectedRelId(isSelected ? null : rel.id)}
                      className={`w-full text-left p-3 hover:bg-muted/50 transition-colors ${
                        isSelected ? "bg-muted/70 ring-1 ring-inset ring-primary/20" : ""
                      }`}
                    >
                      <div className="flex items-center gap-3">
                        {/* Source concept */}
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-1.5">
                            <span
                              className="w-2 h-2 rounded-full flex-shrink-0"
                              style={{ backgroundColor: getFrameworkColor(left.framework_id) }}
                            />
                            {left.code && (
                              <span className="text-[10px] font-mono font-bold text-muted-foreground bg-muted px-1 rounded">
                                {left.code}
                              </span>
                            )}
                            <span className="text-sm font-medium truncate">
                              {getName(left)}
                            </span>
                          </div>
                        </div>

                        {/* Relationship type */}
                        <div className="flex flex-col items-center flex-shrink-0 px-2">
                          <span
                            className="text-[10px] font-mono px-1.5 py-0.5 rounded-full"
                            style={{
                              backgroundColor: typeColor(rel.relationship_type) + "20",
                              color: typeColor(rel.relationship_type),
                            }}
                          >
                            {rel.relationship_type.replace(/_/g, " ")}
                          </span>
                          <ArrowRight className="w-3 h-3 text-muted-foreground mt-0.5" />
                        </div>

                        {/* Target concept */}
                        <div className="flex-1 min-w-0">
                          <div className="flex items-center gap-1.5">
                            <span
                              className="w-2 h-2 rounded-full flex-shrink-0"
                              style={{ backgroundColor: getFrameworkColor(right.framework_id) }}
                            />
                            {right.code && (
                              <span className="text-[10px] font-mono font-bold text-muted-foreground bg-muted px-1 rounded">
                                {right.code}
                              </span>
                            )}
                            <span className="text-sm font-medium truncate">
                              {getName(right)}
                            </span>
                          </div>
                        </div>

                        {/* Level badge */}
                        <span className="text-[10px] font-mono text-muted-foreground flex-shrink-0">
                          {levelIcons[rel.level]}
                        </span>
                      </div>

                      {/* Expanded detail */}
                      {isSelected && rel.description && (
                        <p className="text-xs text-muted-foreground mt-2 leading-relaxed pl-3 border-l-2 border-primary/30">
                          {rel.description}
                        </p>
                      )}
                    </button>
                  );
                })}
              </div>
            )}
          </div>

          {/* Detail sidebar */}
          {selectedRel && (
            <div className="w-80 border rounded-lg p-4 space-y-4 flex-shrink-0">
              <h3 className="font-bold text-sm">{t("crosswalk.mappingDetail")}</h3>

              {(() => {
                const source = conceptMap.get(selectedRel.source_concept_id);
                const target = conceptMap.get(selectedRel.target_concept_id);
                if (!source || !target) return null;

                return (
                  <>
                    <DetailCard
                      concept={source}
                      language={language}
                      label={t("crosswalk.sourceFramework")}
                    />
                    <div className="flex items-center justify-center gap-2 text-xs font-mono">
                      <span
                        className="px-2 py-1 rounded-full"
                        style={{
                          backgroundColor: typeColor(selectedRel.relationship_type) + "20",
                          color: typeColor(selectedRel.relationship_type),
                        }}
                      >
                        {selectedRel.relationship_type.replace(/_/g, " ")}
                      </span>
                    </div>
                    <DetailCard
                      concept={target}
                      language={language}
                      label={t("crosswalk.targetFramework")}
                    />
                    {selectedRel.description && (
                      <div>
                        <h4 className="text-xs font-mono font-semibold uppercase tracking-wider text-muted-foreground mb-1">
                          {t("crosswalk.rationale")}
                        </h4>
                        <p className="text-xs text-muted-foreground leading-relaxed">
                          {selectedRel.description}
                        </p>
                      </div>
                    )}
                    <div className="text-[10px] font-mono text-muted-foreground">
                      {t("crosswalk.level")}: {levelLabels[selectedRel.level][language]}
                    </div>
                  </>
                );
              })()}
            </div>
          )}
        </div>
      ) : (
        <div className="border rounded-lg p-12 text-center text-muted-foreground">
          <ArrowLeftRight className="w-16 h-16 mx-auto mb-4 opacity-20" />
          <p className="text-lg font-medium">{t("crosswalk.selectBoth")}</p>
          <p className="text-sm mt-1">{t("crosswalk.selectBothHint")}</p>
        </div>
      )}
    </div>
  );
}

function DetailCard({
  concept,
  language,
  label,
}: {
  concept: Concept;
  language: "en" | "nb";
  label: string;
}) {
  const name = language === "nb" && concept.name_nb ? concept.name_nb : concept.name_en;
  const definition =
    language === "nb" && concept.definition_nb
      ? concept.definition_nb
      : concept.definition_en;

  return (
    <div
      className="border rounded-md p-3 space-y-1"
      style={{ borderLeftWidth: 3, borderLeftColor: getFrameworkColor(concept.framework_id) }}
    >
      <div className="text-[10px] font-mono uppercase tracking-wider text-muted-foreground">
        {label}
      </div>
      <div className="flex items-center gap-1.5">
        {concept.code && (
          <span className="text-[10px] font-mono font-bold bg-muted px-1 rounded">
            {concept.code}
          </span>
        )}
        <span className="text-sm font-semibold">{name}</span>
      </div>
      <div className="text-[10px] font-mono text-muted-foreground">
        {concept.concept_type}
      </div>
      {definition && (
        <p className="text-xs text-muted-foreground leading-relaxed line-clamp-4">
          {definition}
        </p>
      )}
    </div>
  );
}

function typeColor(type: string): string {
  switch (type) {
    case "maps_to": return "#3b82f6";
    case "related_to": return "#8b5cf6";
    case "supports": return "#10b981";
    case "implements": return "#f59e0b";
    default: return "#6b7280";
  }
}
