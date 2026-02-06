import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useFrameworks, useConcepts, useRelationships } from "../../api";
import { useExplorer } from "../../context";
import { buildTree } from "../../utils";
import { getFrameworkColor } from "../../utils/graphTransform";
import { TreeNode } from "../Sidebar/TreeNode";
import type { Relationship } from "../../types";

export function CompareView() {
  const { t, i18n } = useTranslation("ontology");
  const { state, setCompareLeft, setCompareRight } = useExplorer();
  const [leftFrameworkId, rightFrameworkId] = state.compareFrameworks;

  const { data: frameworks } = useFrameworks();
  const { data: leftConcepts } = useConcepts(leftFrameworkId ?? undefined);
  const { data: rightConcepts } = useConcepts(rightFrameworkId ?? undefined);
  const { data: relationships } = useRelationships();

  const language = i18n.language.startsWith("nb") ? "nb" : "en";

  const leftTree = useMemo(() => {
    if (!leftConcepts) return [];
    return buildTree(leftConcepts, language);
  }, [leftConcepts, language]);

  const rightTree = useMemo(() => {
    if (!rightConcepts) return [];
    return buildTree(rightConcepts, language);
  }, [rightConcepts, language]);

  // Find cross-framework relationships
  const crossMappings = useMemo(() => {
    if (!relationships || !leftFrameworkId || !rightFrameworkId) return [];

    const leftIds = new Set(leftConcepts?.map((c) => c.id) ?? []);
    const rightIds = new Set(rightConcepts?.map((c) => c.id) ?? []);

    return relationships.filter(
      (rel) =>
        (leftIds.has(rel.source_concept_id) && rightIds.has(rel.target_concept_id)) ||
        (rightIds.has(rel.source_concept_id) && leftIds.has(rel.target_concept_id))
    );
  }, [relationships, leftFrameworkId, rightFrameworkId, leftConcepts, rightConcepts]);

  const getMappingsForConcept = (conceptId: string): Relationship[] => {
    return crossMappings.filter(
      (rel) => rel.source_concept_id === conceptId || rel.target_concept_id === conceptId
    );
  };

  return (
    <div className="h-full flex flex-col">
      {/* Framework selectors */}
      <div className="flex items-center justify-between p-4 border-b">
        <select
          value={leftFrameworkId ?? ""}
          onChange={(e) => setCompareLeft(e.target.value || null)}
          className="px-3 py-2 border rounded-md bg-background"
        >
          <option value="">{t("compare.selectFramework")}</option>
          {frameworks?.map((fw) => (
            <option key={fw.id} value={fw.id}>
              {fw.name}
            </option>
          ))}
        </select>
        <div className="text-muted-foreground">
          \u2194
        </div>
        <select
          value={rightFrameworkId ?? ""}
          onChange={(e) => setCompareRight(e.target.value || null)}
          className="px-3 py-2 border rounded-md bg-background"
        >
          <option value="">{t("compare.selectFramework")}</option>
          {frameworks?.map((fw) => (
            <option key={fw.id} value={fw.id}>
              {fw.name}
            </option>
          ))}
        </select>
      </div>

      {/* Comparison area */}
      <div className="flex-1 flex min-h-0">
        {/* Left tree */}
        <div
          className="flex-1 overflow-y-auto border-r border-l-4"
          style={{ borderLeftColor: leftFrameworkId ? getFrameworkColor(leftFrameworkId) : undefined }}
        >
          {leftFrameworkId ? (
            leftTree.length > 0 ? (
              <div className="py-2">
                {leftTree.map((node) => (
                  <div key={node.id}>
                    <TreeNode node={node} level={0} />
                    {getMappingsForConcept(node.id).length > 0 && (
                      <div className="ml-6 py-1">
                        {getMappingsForConcept(node.id).map((rel) => (
                          <div
                            key={rel.id}
                            className="flex items-center gap-2 text-xs text-muted-foreground"
                          >
                            <span className="w-2 h-px bg-amber-500" />
                            <span>{rel.relationship_type}</span>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-4 text-muted-foreground text-sm">
                {t("compare.loadingConcepts")}
              </div>
            )
          ) : (
            <div className="p-4 text-muted-foreground text-sm">
              {t("compare.selectAFramework")}
            </div>
          )}
        </div>

        {/* Center mappings column */}
        <div className="w-24 bg-muted/30 flex flex-col items-center justify-center">
          <div className="text-xs text-muted-foreground text-center p-2">
            {crossMappings.length} {t("compare.mappings")}
          </div>
          {crossMappings.slice(0, 10).map((rel) => (
            <div
              key={rel.id}
              className="w-full h-px bg-amber-500/50 my-1"
              title={rel.relationship_type}
            />
          ))}
          {crossMappings.length > 10 && (
            <div className="text-xs text-muted-foreground">
              {t("compare.nMore", { count: crossMappings.length - 10 })}
            </div>
          )}
        </div>

        {/* Right tree */}
        <div
          className="flex-1 overflow-y-auto border-l border-r-4"
          style={{ borderRightColor: rightFrameworkId ? getFrameworkColor(rightFrameworkId) : undefined }}
        >
          {rightFrameworkId ? (
            rightTree.length > 0 ? (
              <div className="py-2">
                {rightTree.map((node) => (
                  <div key={node.id}>
                    <TreeNode node={node} level={0} />
                    {getMappingsForConcept(node.id).length > 0 && (
                      <div className="ml-6 py-1">
                        {getMappingsForConcept(node.id).map((rel) => (
                          <div
                            key={rel.id}
                            className="flex items-center gap-2 text-xs text-muted-foreground"
                          >
                            <span className="w-2 h-px bg-amber-500" />
                            <span>{rel.relationship_type}</span>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-4 text-muted-foreground text-sm">
                {t("compare.loadingConcepts")}
              </div>
            )
          ) : (
            <div className="p-4 text-muted-foreground text-sm">
              {t("compare.selectAFramework")}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
