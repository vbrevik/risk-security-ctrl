import { useState, useMemo } from "react";
import { ChevronRight, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cn } from "@/lib/utils";
import { useConcepts } from "../../api";
import { buildTree, filterTree } from "../../utils";
import { getFrameworkColor } from "../../utils/graphTransform";
import { TreeNode } from "./TreeNode";
import type { Framework } from "../../types";

interface FrameworkTreeProps {
  framework: Framework;
  filterQuery: string;
}

export function FrameworkTree({ framework, filterQuery }: FrameworkTreeProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const { i18n } = useTranslation();
  const { data: concepts, isLoading } = useConcepts(
    isExpanded ? framework.id : undefined
  );

  const tree = useMemo(() => {
    if (!concepts) return [];
    const language = i18n.language.startsWith("nb") ? "nb" : "en";
    const fullTree = buildTree(concepts, language);
    return filterQuery ? filterTree(fullTree, filterQuery) : fullTree;
  }, [concepts, filterQuery, i18n.language]);

  const borderColor = getFrameworkColor(framework.id);

  return (
    <div className="border-l-2" style={{ borderColor }}>
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className={cn(
          "flex items-center gap-2 w-full py-2 px-3 text-left",
          "hover:bg-accent/50 transition-colors font-medium text-sm"
        )}
      >
        <ChevronRight
          className={cn(
            "h-4 w-4 transition-transform shrink-0",
            isExpanded && "rotate-90"
          )}
        />
        <span className="truncate">{framework.name}</span>
        {isLoading && <Loader2 className="h-3 w-3 animate-spin ml-auto" />}
      </button>
      {isExpanded && (
        <div className="pb-2">
          {tree.length === 0 && !isLoading && (
            <div className="px-4 py-2 text-sm text-muted-foreground">
              {filterQuery ? "No matches" : "No concepts"}
            </div>
          )}
          {tree.map((node) => (
            <TreeNode key={node.id} node={node} level={1} />
          ))}
        </div>
      )}
    </div>
  );
}
