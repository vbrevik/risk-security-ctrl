import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { useFrameworks, useConcepts } from "../../api";
import { useExplorer } from "../../context";
import { buildTree, findNodePath } from "../../utils";
import { getFrameworkColor } from "../../utils/graphTransform";
import { TreeNode } from "../Sidebar/TreeNode";
import type { Framework, TreeNode as TreeNodeType } from "../../types";

function filterByConceptType(nodes: TreeNodeType[], conceptType: string): TreeNodeType[] {
  const filter = (nodeList: TreeNodeType[]): TreeNodeType[] => {
    return nodeList
      .map((node) => {
        const filteredChildren = filter(node.children);
        if (node.concept.concept_type === conceptType || filteredChildren.length > 0) {
          return { ...node, children: filteredChildren, isExpanded: filteredChildren.length > 0 };
        }
        return null;
      })
      .filter((n): n is TreeNodeType => n !== null);
  };
  return filter(nodes);
}

function FrameworkSection({
  framework,
  conceptType,
  selectedConceptId,
}: {
  framework: Framework;
  conceptType: string | null;
  selectedConceptId: string | null;
}) {
  const { i18n } = useTranslation();
  const { data: concepts } = useConcepts(framework.id);

  const language = i18n.language.startsWith("nb") ? "nb" : "en";

  const tree = useMemo(() => {
    if (!concepts) return [];
    let result = buildTree(concepts, language);
    if (conceptType) {
      result = filterByConceptType(result, conceptType);
    }
    return result;
  }, [concepts, language, conceptType]);

  // Compute ancestor IDs for auto-expanding to the selected concept
  const autoExpandIds = useMemo(() => {
    if (!selectedConceptId || tree.length === 0) return new Set<string>();
    const path = findNodePath(tree, selectedConceptId);
    // All nodes except the last (the selected one) should be expanded
    return new Set(path.slice(0, -1).map((n) => n.id));
  }, [tree, selectedConceptId]);

  const borderColor = getFrameworkColor(framework.id);

  if (!concepts || tree.length === 0) {
    return null;
  }

  return (
    <div className="border rounded-lg overflow-hidden">
      <div
        className="px-4 py-2 font-medium text-sm border-l-4 bg-muted/30"
        style={{ borderLeftColor: borderColor }}
      >
        {framework.name}
        {framework.version && (
          <span className="text-muted-foreground ml-2 text-xs">
            {framework.version}
          </span>
        )}
      </div>
      <div className="py-1">
        {tree.map((node) => (
          <TreeNode key={node.id} node={node} level={0} autoExpandIds={autoExpandIds} />
        ))}
      </div>
    </div>
  );
}

export function TreeView() {
  const { t } = useTranslation("ontology");
  const { state } = useExplorer();
  const { data: frameworks, isLoading } = useFrameworks();

  // Filter frameworks based on active filter
  const visibleFrameworks = useMemo(() => {
    if (!frameworks) return [];
    if (state.activeFrameworks.length === 0) return frameworks;
    return frameworks.filter((fw) => state.activeFrameworks.includes(fw.id));
  }, [frameworks, state.activeFrameworks]);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        {t("concepts.loading")}
      </div>
    );
  }

  return (
    <div className="h-full overflow-y-auto p-4">
      <div className="max-w-3xl mx-auto space-y-4">
        {visibleFrameworks.map((framework) => (
          <FrameworkSection
            key={framework.id}
            framework={framework}
            conceptType={state.activeConceptType}
            selectedConceptId={state.selectedConceptId}
          />
        ))}
      </div>
    </div>
  );
}
