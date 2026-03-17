import { getFrameworkColor } from "../utils/graphTransform";

interface SearchFiltersProps {
  frameworkFacets: { id: string; name: string; count: number }[];
  typeFacets: { type: string; count: number }[];
  activeFrameworks: string[];
  activeTypes: string[];
  onToggleFramework: (frameworkId: string) => void;
  onToggleType: (type: string) => void;
}

export function SearchFilters({
  frameworkFacets,
  typeFacets,
  activeFrameworks,
  activeTypes,
  onToggleFramework,
  onToggleType,
}: SearchFiltersProps) {
  const visibleFrameworks = frameworkFacets
    .filter((f) => f.count > 0)
    .sort((a, b) => b.count - a.count);

  const visibleTypes = typeFacets
    .filter((t) => t.count > 0)
    .sort((a, b) => b.count - a.count);

  return (
    <div className="w-[240px] flex-shrink-0 space-y-6">
      {/* Framework filters */}
      <div>
        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
          Frameworks
        </h3>
        <div className="space-y-1">
          {visibleFrameworks.map((fw) => (
            <label
              key={fw.id}
              className="flex items-center gap-2 text-xs cursor-pointer hover:bg-muted/50 rounded px-1 py-0.5"
            >
              <input
                type="checkbox"
                checked={activeFrameworks.includes(fw.id)}
                onChange={() => onToggleFramework(fw.id)}
                className="rounded border-border"
              />
              <span
                className="w-2 h-2 rounded-full flex-shrink-0"
                style={{ backgroundColor: getFrameworkColor(fw.id) }}
              />
              <span className="flex-1 truncate">{fw.name}</span>
              <span className="text-foreground/40">({fw.count})</span>
            </label>
          ))}
        </div>
      </div>

      {/* Type filters */}
      <div>
        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
          Concept Types
        </h3>
        <div className="space-y-1">
          {visibleTypes.map((t) => (
            <label
              key={t.type}
              className="flex items-center gap-2 text-xs cursor-pointer hover:bg-muted/50 rounded px-1 py-0.5"
            >
              <input
                type="checkbox"
                checked={activeTypes.includes(t.type)}
                onChange={() => onToggleType(t.type)}
                className="rounded border-border"
              />
              <span className="flex-1 capitalize">{t.type}</span>
              <span className="text-foreground/40">({t.count})</span>
            </label>
          ))}
        </div>
      </div>
    </div>
  );
}
