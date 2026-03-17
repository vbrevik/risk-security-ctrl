import type { Framework, FrameworkStats } from "../types";
import { groupFrameworksByDomain } from "../utils/frameworkDomains";
import { getFrameworkColor } from "../utils/graphTransform";

interface FrameworkSidebarProps {
  frameworks: Framework[];
  stats: Map<string, FrameworkStats>;
  selectedId: string | null;
  onSelect: (id: string) => void;
  isLoading: boolean;
}

export function FrameworkSidebar({
  frameworks,
  stats,
  selectedId,
  onSelect,
  isLoading,
}: FrameworkSidebarProps) {
  if (isLoading) {
    return (
      <div className="w-[280px] flex-shrink-0 overflow-y-auto space-y-6">
        {Array.from({ length: 4 }).map((_, i) => (
          <div key={i} className="space-y-2">
            <div className="h-3 w-32 bg-muted rounded animate-pulse" />
            {Array.from({ length: 3 }).map((_, j) => (
              <div key={j} className="h-8 bg-muted rounded animate-pulse" />
            ))}
          </div>
        ))}
      </div>
    );
  }

  if (frameworks.length === 0) {
    return (
      <div className="w-[280px] flex-shrink-0 flex items-center justify-center text-foreground/40 text-sm">
        No frameworks loaded
      </div>
    );
  }

  const groups = groupFrameworksByDomain(frameworks);
  const frameworkById = new Map(frameworks.map((fw) => [fw.id, fw]));

  return (
    <div className="w-[280px] flex-shrink-0 overflow-y-auto space-y-4">
      {groups.map((group) => (
        <div key={group.label}>
          <div className="flex items-center gap-2 mb-2">
            <span className="text-xs font-mono uppercase tracking-widest text-foreground/50">
              {group.label}
            </span>
            <span className="text-xs text-foreground/30">({group.frameworkIds.length})</span>
          </div>
          <div className="space-y-0.5">
            {group.frameworkIds.map((fwId) => {
              const fw = frameworkById.get(fwId);
              if (!fw) return null;
              const isActive = fwId === selectedId;
              const count = stats.get(fwId)?.conceptCount;
              return (
                <button
                  key={fwId}
                  data-active={isActive}
                  onClick={() => onSelect(fwId)}
                  className={`w-full flex items-center gap-2 px-2 py-1.5 rounded text-left text-sm transition-colors ${
                    isActive
                      ? "bg-accent/10 border-l-2 border-accent"
                      : "hover:bg-muted/50 border-l-2 border-transparent"
                  }`}
                >
                  <span
                    className="w-2.5 h-2.5 rounded-full flex-shrink-0"
                    style={{ backgroundColor: getFrameworkColor(fwId) }}
                  />
                  <span className="flex-1 truncate font-mono text-xs">{fw.name}</span>
                  {count != null && (
                    <span className="tech-badge text-[10px]">{count}</span>
                  )}
                </button>
              );
            })}
          </div>
        </div>
      ))}
    </div>
  );
}
