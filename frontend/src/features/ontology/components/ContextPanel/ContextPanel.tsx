import { X, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useConceptRelationships, useFramework, useConcepts } from "../../api";
import { useExplorer } from "../../context";
import { findNodePath, buildTree } from "../../utils";
import { getFrameworkColor } from "../../utils/graphTransform";

export function ContextPanel() {
  const { t, i18n } = useTranslation("ontology");
  const { state, selectConcept, clearSelection, navigateBack } = useExplorer();
  const selectedId = state.selectedConceptId;

  const { data: conceptData, isLoading } = useConceptRelationships(selectedId ?? "");
  const { data: framework } = useFramework(conceptData?.framework_id ?? "");
  const { data: frameworkConcepts } = useConcepts(conceptData?.framework_id);

  const language = i18n.language.startsWith("nb") ? "nb" : "en";

  if (!selectedId) return null;

  const borderColor = conceptData ? getFrameworkColor(conceptData.framework_id) : undefined;

  // Build hierarchy breadcrumb
  const hierarchyPath = (() => {
    if (!frameworkConcepts || !selectedId) return [];
    const tree = buildTree(frameworkConcepts, language);
    return findNodePath(tree, selectedId);
  })();

  // Navigation history breadcrumb (deduplicated consecutive)
  const navHistory = state.navigationHistory.filter(
    (id, i, arr) => i === 0 || id !== arr[i - 1]
  );
  const visibleHistory = navHistory.slice(-5);
  const hiddenHistory = navHistory.slice(0, -5);

  // Split relationships: same-framework vs cross-framework
  const sameFrameworkRels = conceptData?.related_concepts.filter(
    (rel) => rel.concept_framework_id === conceptData.framework_id
  ) ?? [];
  const crossMappings = conceptData?.related_concepts.filter(
    (rel) => rel.concept_framework_id !== conceptData.framework_id
  ) ?? [];

  return (
    <div
      className="w-80 border-l bg-card flex flex-col h-full"
      style={{ borderTopColor: borderColor, borderTopWidth: borderColor ? 3 : 0 }}
    >
      {/* Navigation breadcrumb trail */}
      {navHistory.length > 1 && (
        <div className="px-3 py-2 border-b bg-muted/30 flex items-center gap-1 text-xs text-muted-foreground overflow-hidden">
          {hiddenHistory.length > 0 && (
            <button
              onClick={() => navigateBack(hiddenHistory[0])}
              className="hover:text-foreground transition-colors flex-shrink-0"
              title={`${hiddenHistory.length} earlier`}
            >
              &hellip;
            </button>
          )}
          {hiddenHistory.length > 0 && (
            <ChevronRight className="h-3 w-3 flex-shrink-0 text-muted-foreground/50" />
          )}
          {visibleHistory.map((id, i) => {
            const isCurrent = i === visibleHistory.length - 1;
            return (
              <span key={`${id}-${i}`} className="flex items-center gap-1 min-w-0">
                {i > 0 && <ChevronRight className="h-3 w-3 flex-shrink-0 text-muted-foreground/50" />}
                {isCurrent ? (
                  <span className="font-medium text-foreground truncate">
                    {id.split("-").pop()}
                  </span>
                ) : (
                  <button
                    onClick={() => navigateBack(id)}
                    className="hover:text-foreground transition-colors truncate"
                  >
                    {id.split("-").pop()}
                  </button>
                )}
              </span>
            );
          })}
        </div>
      )}

      {/* Loading state */}
      {isLoading && (
        <div className="flex-1 flex items-center justify-center text-muted-foreground text-sm">
          {t("concepts.loading")}
        </div>
      )}

      {/* Concept content */}
      {conceptData && !isLoading && (
        <div className="flex-1 overflow-y-auto">
          {/* Header */}
          <div className="px-4 pt-4 pb-3">
            <div className="flex items-start justify-between gap-2">
              <div className="min-w-0 flex-1">
                <h2 className="text-base font-semibold leading-tight">
                  {conceptData.code && (
                    <span className="text-muted-foreground mr-1.5">{conceptData.code}</span>
                  )}
                  {language === "nb" && conceptData.name_nb ? conceptData.name_nb : conceptData.name_en}
                </h2>
                <div className="flex items-center gap-2 mt-1.5">
                  <span
                    className="px-1.5 py-0.5 text-[10px] rounded text-white font-medium"
                    style={{ backgroundColor: borderColor }}
                  >
                    {framework?.name ?? conceptData.framework_id}
                  </span>
                  <span className="text-xs text-muted-foreground">
                    {conceptData.concept_type}
                  </span>
                </div>
              </div>
              <Button
                variant="ghost"
                size="icon"
                className="h-7 w-7 flex-shrink-0"
                onClick={clearSelection}
                title={t("panel.close")}
              >
                <X className="h-4 w-4" />
              </Button>
            </div>
          </div>

          <div className="px-4 pb-4 space-y-4">
            {/* Definition */}
            {(conceptData.definition_en || conceptData.definition_nb) && (
              <div>
                <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
                  {t("detail.definition")}
                </h3>
                <p className="text-sm leading-relaxed">
                  {language === "nb" && conceptData.definition_nb
                    ? conceptData.definition_nb
                    : conceptData.definition_en}
                </p>
              </div>
            )}

            {/* Hierarchy path */}
            {hierarchyPath.length > 1 && (
              <div>
                <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
                  {t("detail.hierarchy")}
                </h3>
                <div className="flex items-center flex-wrap gap-0.5 text-xs">
                  {hierarchyPath.map((node, index) => (
                    <span key={node.id} className="flex items-center">
                      {index > 0 && <ChevronRight className="h-3 w-3 mx-0.5 text-muted-foreground/50" />}
                      {index === hierarchyPath.length - 1 ? (
                        <span className="font-medium">{node.name}</span>
                      ) : (
                        <button
                          onClick={() => selectConcept(node.id)}
                          className="text-muted-foreground hover:text-foreground transition-colors"
                        >
                          {node.name}
                        </button>
                      )}
                    </span>
                  ))}
                </div>
              </div>
            )}

            {/* Source */}
            {conceptData.source_reference && (
              <div>
                <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
                  {t("detail.source")}
                </h3>
                <p className="text-xs text-muted-foreground">{conceptData.source_reference}</p>
              </div>
            )}

            {/* Relationships (same framework) */}
            {sameFrameworkRels.length > 0 && (
              <div>
                <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
                  {t("concepts.relationships")}
                  <span className="ml-1 font-normal">({sameFrameworkRels.length})</span>
                </h3>
                <div className="space-y-0.5">
                  {sameFrameworkRels.map((rel) => (
                    <button
                      key={rel.relationship_id}
                      onClick={() => selectConcept(rel.concept_id)}
                      className={cn(
                        "w-full flex items-center gap-1.5 px-2 py-1.5 text-sm rounded",
                        "hover:bg-accent transition-colors text-left"
                      )}
                    >
                      <span className="text-muted-foreground text-xs flex-shrink-0">
                        {rel.direction === "outgoing" ? "\u2192" : "\u2190"}
                      </span>
                      <span className="text-muted-foreground text-[10px] flex-shrink-0">
                        {rel.relationship_type}
                      </span>
                      <span className="flex-1 truncate text-xs">
                        {language === "nb" && rel.concept_name_nb
                          ? rel.concept_name_nb
                          : rel.concept_name_en}
                      </span>
                    </button>
                  ))}
                </div>
              </div>
            )}

            {/* Cross-framework mappings */}
            {crossMappings.length > 0 && (
              <div>
                <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
                  {t("detail.crossFrameworkMappings")}
                  <span className="ml-1 font-normal">({crossMappings.length})</span>
                </h3>
                <div className="space-y-0.5">
                  {crossMappings.map((rel) => (
                    <button
                      key={rel.relationship_id}
                      onClick={() => selectConcept(rel.concept_id)}
                      className={cn(
                        "w-full flex items-center gap-1.5 px-2 py-1.5 text-sm rounded",
                        "hover:bg-accent transition-colors text-left"
                      )}
                    >
                      <span
                        className="w-2 h-2 rounded-full flex-shrink-0"
                        style={{ backgroundColor: getFrameworkColor(rel.concept_framework_id) }}
                      />
                      <span className="text-muted-foreground text-[10px] flex-shrink-0">
                        {rel.relationship_type}
                      </span>
                      <span className="flex-1 truncate text-xs">
                        {language === "nb" && rel.concept_name_nb
                          ? rel.concept_name_nb
                          : rel.concept_name_en}
                      </span>
                      <span className="text-[10px] text-muted-foreground flex-shrink-0">
                        {rel.concept_framework_id}
                      </span>
                    </button>
                  ))}
                </div>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
