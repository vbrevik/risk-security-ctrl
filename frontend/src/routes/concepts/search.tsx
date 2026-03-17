import { useState, useEffect, useRef, useMemo, useCallback } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { Search, X } from "lucide-react";
import { useFrameworks, useSearchConcepts } from "@/features/ontology/api";
import { SearchFilters } from "@/features/ontology/components/SearchFilters";
import { SearchResults } from "@/features/ontology/components/SearchResults";
import { parseCommaSeparated } from "@/features/ontology/utils/urlParams";
import type { Concept } from "@/features/ontology/types";

export const Route = createFileRoute("/concepts/search")({
  component: ConceptSearchPage,
  validateSearch: (
    search: Record<string, unknown>
  ): { q?: string; frameworks?: string; types?: string } => ({
    q: typeof search.q === "string" ? search.q : undefined,
    frameworks: typeof search.frameworks === "string" ? search.frameworks : undefined,
    types: typeof search.types === "string" ? search.types : undefined,
  }),
});

export function computeFacets(concepts: Concept[]) {
  const frameworks = new Map<string, number>();
  const types = new Map<string, number>();
  for (const c of concepts) {
    frameworks.set(c.framework_id, (frameworks.get(c.framework_id) || 0) + 1);
    types.set(c.concept_type, (types.get(c.concept_type) || 0) + 1);
  }
  return { frameworks, types };
}

function ConceptSearchPage() {
  const { q = "", frameworks: fwParam = "", types: typeParam = "" } = Route.useSearch();
  const navigate = Route.useNavigate();
  const { data: allFrameworks = [] } = useFrameworks();
  const inputRef = useRef<HTMLInputElement>(null);

  const [inputValue, setInputValue] = useState(q);
  const [debouncedQuery, setDebouncedQuery] = useState(q);

  // Auto-focus on mount
  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  // Debounce input → URL
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedQuery(inputValue);
      navigate({ search: (prev) => ({ ...prev, q: inputValue || undefined }), replace: true });
    }, 300);
    return () => clearTimeout(timer);
  }, [inputValue, navigate]);

  // Sync URL → input (e.g., suggestion clicks)
  useEffect(() => {
    if (q !== inputValue && q !== undefined) {
      setInputValue(q);
      setDebouncedQuery(q);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [q]);

  const { data: rawResults = [], isLoading } = useSearchConcepts(debouncedQuery);
  const activeFrameworks = parseCommaSeparated(fwParam);
  const activeTypes = parseCommaSeparated(typeParam);

  // Facets from unfiltered results
  const facets = useMemo(() => computeFacets(rawResults), [rawResults]);

  const frameworkFacets = useMemo(() => {
    return allFrameworks
      .map((fw) => ({ id: fw.id, name: fw.name, count: facets.frameworks.get(fw.id) || 0 }));
  }, [allFrameworks, facets]);

  const typeFacets = useMemo(() => {
    return [...facets.types.entries()].map(([type, count]) => ({ type, count }));
  }, [facets]);

  // Filter results
  const filteredResults = useMemo(() => {
    let results = rawResults;
    if (activeFrameworks.length > 0) {
      const set = new Set(activeFrameworks);
      results = results.filter((c) => set.has(c.framework_id));
    }
    if (activeTypes.length > 0) {
      const set = new Set(activeTypes);
      results = results.filter((c) => set.has(c.concept_type));
    }
    return results;
  }, [rawResults, activeFrameworks, activeTypes]);

  // Group by framework
  const groupedResults = useMemo(() => {
    const map = new Map<string, Concept[]>();
    for (const c of filteredResults) {
      const list = map.get(c.framework_id) ?? [];
      list.push(c);
      map.set(c.framework_id, list);
    }
    return [...map.entries()].map(([fwId, concepts]) => ({
      frameworkId: fwId,
      frameworkName: allFrameworks.find((f) => f.id === fwId)?.name ?? fwId,
      concepts,
    }));
  }, [filteredResults, allFrameworks]);

  const toggleFilter = useCallback(
    (param: "frameworks" | "types", value: string) => {
      const current = parseCommaSeparated(param === "frameworks" ? fwParam : typeParam);
      const next = current.includes(value)
        ? current.filter((v) => v !== value)
        : [...current, value];
      navigate({
        search: (prev) => ({ ...prev, [param]: next.length > 0 ? next.join(",") : undefined }),
        replace: true,
      });
    },
    [fwParam, typeParam, navigate]
  );

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "Escape") {
      setInputValue("");
    }
  };

  return (
    <div className="animate-fadeInUp space-y-4">
      {/* Search input */}
      <div className="relative">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40" />
        <input
          ref={inputRef}
          type="text"
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Search concepts..."
          className="w-full pl-10 pr-10 py-2.5 border rounded-lg bg-background font-mono text-sm focus:outline-none focus:ring-2 focus:ring-accent/50"
        />
        {inputValue && (
          <button
            onClick={() => setInputValue("")}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-foreground/40 hover:text-foreground/70"
          >
            <X className="w-4 h-4" />
          </button>
        )}
      </div>

      {/* Result count */}
      {debouncedQuery.length >= 2 && (
        <p className="text-xs text-foreground/50">
          {isLoading
            ? "Searching..."
            : `${filteredResults.length} results across ${groupedResults.length} frameworks`}
          {rawResults.length >= 500 && (
            <span className="ml-2 text-foreground/30">
              (showing first 500 — refine your search)
            </span>
          )}
        </p>
      )}

      {/* Active filter pills */}
      {(activeFrameworks.length > 0 || activeTypes.length > 0) && (
        <div className="flex gap-1.5 flex-wrap items-center">
          {activeFrameworks.map((fw) => (
            <span
              key={fw}
              className="tech-badge text-[10px] cursor-pointer"
              onClick={() => toggleFilter("frameworks", fw)}
            >
              {allFrameworks.find((f) => f.id === fw)?.name ?? fw} ×
            </span>
          ))}
          {activeTypes.map((t) => (
            <span
              key={t}
              className="tech-badge text-[10px] cursor-pointer"
              onClick={() => toggleFilter("types", t)}
            >
              {t} ×
            </span>
          ))}
          <button
            onClick={() => navigate({ search: (prev) => ({ ...prev, frameworks: undefined, types: undefined }), replace: true })}
            className="text-[10px] text-foreground/40 hover:text-foreground/70"
          >
            Clear all
          </button>
        </div>
      )}

      {/* Main content */}
      <div className="flex gap-6">
        {debouncedQuery.length >= 2 && rawResults.length > 0 && (
          <SearchFilters
            frameworkFacets={frameworkFacets}
            typeFacets={typeFacets}
            activeFrameworks={activeFrameworks}
            activeTypes={activeTypes}
            onToggleFramework={(id) => toggleFilter("frameworks", id)}
            onToggleType={(type) => toggleFilter("types", type)}
          />
        )}
        <SearchResults
          groupedResults={groupedResults}
          query={debouncedQuery}
          frameworks={allFrameworks}
          totalCount={filteredResults.length}
        />
      </div>
    </div>
  );
}
