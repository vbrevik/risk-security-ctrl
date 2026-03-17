import { Link } from "@tanstack/react-router";
import { getFrameworkColor } from "../utils/graphTransform";
import type { Concept, Framework } from "../types";

interface SearchResultsProps {
  groupedResults: { frameworkId: string; frameworkName: string; concepts: Concept[] }[];
  query: string;
  frameworks: Framework[];
  totalCount: number;
}

function highlightMatch(text: string, query: string): React.ReactNode {
  if (!query || query.length < 2) return text;
  const idx = text.toLowerCase().indexOf(query.toLowerCase());
  if (idx === -1) return text;
  return (
    <>
      {text.slice(0, idx)}
      <mark className="bg-yellow-200/60 rounded px-0.5">{text.slice(idx, idx + query.length)}</mark>
      {text.slice(idx + query.length)}
    </>
  );
}

export function SearchResults({
  groupedResults,
  query,
  frameworks,
  totalCount,
}: SearchResultsProps) {
  if (!query || query.length < 2) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center text-foreground/40 py-16">
        <p className="text-sm mb-4">
          Search across concepts in {frameworks.length} frameworks
        </p>
        <div className="flex gap-2 flex-wrap justify-center">
          {["incident reporting", "access control", "risk assessment", "data protection"].map(
            (suggestion) => (
              <span
                key={suggestion}
                className="tech-badge cursor-pointer hover:bg-accent/20 transition-colors"
                data-suggestion={suggestion}
              >
                {suggestion}
              </span>
            )
          )}
        </div>
      </div>
    );
  }

  if (totalCount === 0) {
    return (
      <div className="flex-1 flex flex-col items-center justify-center text-foreground/40 py-16">
        <p className="text-sm">No concepts matching &quot;{query}&quot;</p>
        <p className="text-xs mt-1">Try broadening your search or adjusting filters</p>
      </div>
    );
  }

  return (
    <div className="flex-1 space-y-6">
      {groupedResults.map((group) => (
        <div key={group.frameworkId}>
          <div className="flex items-center gap-2 mb-2">
            <span
              className="w-2.5 h-2.5 rounded-full"
              style={{ backgroundColor: getFrameworkColor(group.frameworkId) }}
            />
            <span className="text-xs font-mono font-medium">{group.frameworkName}</span>
            <span className="text-xs text-foreground/40">({group.concepts.length} results)</span>
          </div>
          <div className="space-y-1">
            {group.concepts.map((concept) => {
              const excerpt = concept.definition_en
                ? concept.definition_en.slice(0, 100) + (concept.definition_en.length > 100 ? "..." : "")
                : null;
              return (
                <div
                  key={concept.id}
                  className="feature-card p-3 space-y-1"
                  tabIndex={0}
                >
                  <div className="flex items-center gap-2">
                    {concept.code && (
                      <span className="font-mono text-xs text-foreground/50">{concept.code}</span>
                    )}
                    <span className="font-medium text-sm">
                      {highlightMatch(concept.name_en, query)}
                    </span>
                    <span className="tech-badge text-[9px]">{concept.concept_type}</span>
                  </div>
                  {excerpt && (
                    <p className="text-xs text-foreground/60">{highlightMatch(excerpt, query)}</p>
                  )}
                  <div className="flex gap-3 text-[10px] pt-1">
                    <Link
                      to="/ontology"
                      search={{ concept: concept.id }}
                      className="text-foreground/40 hover:text-foreground/70 transition-colors"
                    >
                      Open in Explorer
                    </Link>
                    <Link
                      to="/frameworks"
                      search={{ id: concept.framework_id }}
                      className="text-foreground/40 hover:text-foreground/70 transition-colors"
                    >
                      Framework Detail
                    </Link>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      ))}
    </div>
  );
}
