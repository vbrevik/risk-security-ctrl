import { useState } from "react";
import { ChevronRight, Folder, Diamond, ArrowRight, Cog } from "lucide-react";
import { cn } from "@/lib/utils";
import type { TreeNode as TreeNodeType } from "../../types";
import { useExplorer } from "../../context";

interface TreeNodeProps {
  node: TreeNodeType;
  level: number;
}

const typeIcons: Record<string, typeof Folder> = {
  category: Folder,
  principle: Diamond,
  process: ArrowRight,
  technique: Cog,
  framework_component: Folder,
  function: Folder,
  subcategory: Folder,
};

export function TreeNode({ node, level }: TreeNodeProps) {
  const [isExpanded, setIsExpanded] = useState(node.isExpanded);
  const { state, selectConcept } = useExplorer();
  const isSelected = state.selectedConceptId === node.id;
  const hasChildren = node.children.length > 0;

  const Icon = typeIcons[node.type] ?? Folder;

  const handleClick = () => {
    selectConcept(node.id);
  };

  const handleDoubleClick = () => {
    if (hasChildren) {
      setIsExpanded(!isExpanded);
    }
  };

  const handleChevronClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsExpanded(!isExpanded);
  };

  return (
    <div>
      <div
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
            <TreeNode key={child.id} node={child} level={level + 1} />
          ))}
        </div>
      )}
    </div>
  );
}
