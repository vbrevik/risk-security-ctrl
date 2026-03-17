import { useState, useEffect, useRef } from "react";
import { ChevronRight, Folder, Diamond, ArrowRight, Cog, CircleCheck, Shield, BookOpen, AlertTriangle } from "lucide-react";
import { cn } from "@/lib/utils";
import type { TreeNode as TreeNodeType } from "../../types";
import { useExplorer } from "../../context";

interface TreeNodeProps {
  node: TreeNodeType;
  level: number;
  autoExpandIds?: Set<string>;
}

const typeIcons: Record<string, typeof Folder> = {
  category: Folder,
  principle: Diamond,
  process: ArrowRight,
  technique: Cog,
  framework_component: Folder,
  function: Folder,
  subcategory: Folder,
  action: CircleCheck,
  control: Shield,
  requirement: BookOpen,
  tactic: AlertTriangle,
};

export function TreeNode({ node, level, autoExpandIds }: TreeNodeProps) {
  const shouldAutoExpand = autoExpandIds?.has(node.id) ?? false;
  const [manualExpanded, setManualExpanded] = useState<boolean | null>(null);
  const { state, selectConcept } = useExplorer();
  const isSelected = state.selectedConceptId === node.id;
  const hasChildren = node.children.length > 0;
  const nodeRef = useRef<HTMLDivElement>(null);

  // Expansion priority: manual toggle > auto-expand from selection > initial state
  const isExpanded = manualExpanded ?? (shouldAutoExpand || node.isExpanded);

  const Icon = typeIcons[node.type] ?? Folder;

  // Reset manual override when auto-expand changes (new concept selected)
  const prevAutoExpand = useRef(shouldAutoExpand);
  if (shouldAutoExpand !== prevAutoExpand.current) {
    prevAutoExpand.current = shouldAutoExpand;
    if (shouldAutoExpand) setManualExpanded(null);
  }

  // Scroll into view when selected
  useEffect(() => {
    if (isSelected && nodeRef.current) {
      nodeRef.current.scrollIntoView({ behavior: "smooth", block: "nearest" });
    }
  }, [isSelected]);

  const handleClick = () => {
    selectConcept(node.id);
  };

  const handleDoubleClick = () => {
    if (hasChildren) {
      setManualExpanded(!isExpanded);
    }
  };

  const handleChevronClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    setManualExpanded(!isExpanded);
  };

  return (
    <div>
      <div
        ref={nodeRef}
        className={cn(
          "flex items-center gap-1 py-1 px-2 cursor-pointer rounded text-sm",
          "hover:bg-accent/50 transition-colors",
          isSelected && "bg-accent text-accent-foreground"
        )}
        style={{ paddingLeft: `${level * 12 + 8}px` }}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
      >
        {hasChildren ? (
          <button
            onClick={handleChevronClick}
            className="p-0.5 hover:bg-accent rounded"
          >
            <ChevronRight
              className={cn(
                "h-3 w-3 transition-transform",
                isExpanded && "rotate-90"
              )}
            />
          </button>
        ) : (
          <span className="w-4" />
        )}
        <Icon className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
        <span className="truncate">
          {node.code && (
            <span className="text-muted-foreground mr-1">{node.code}</span>
          )}
          {node.name}
        </span>
      </div>
      {isExpanded && hasChildren && (
        <div>
          {node.children.map((child) => (
            <TreeNode
              key={child.id}
              node={child}
              level={level + 1}
              autoExpandIds={autoExpandIds}
            />
          ))}
        </div>
      )}
    </div>
  );
}
