import { useState, useEffect, useRef } from "react";
import { Search, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useSearchConcepts } from "../../api";
import { useExplorer } from "../../context";
import { cn } from "@/lib/utils";
import type { Concept } from "../../types";

export function SearchBox() {
  const { t, i18n } = useTranslation("ontology");
  const language = i18n.language.startsWith("nb") ? "nb" : "en";
  const getName = (c: Concept) =>
    language === "nb" && c.name_nb ? c.name_nb : c.name_en;
  const [query, setQuery] = useState("");
  const [debouncedQuery, setDebouncedQuery] = useState("");
  const [isOpen, setIsOpen] = useState(false);
  const { selectConcept, setSearchHighlights } = useExplorer();
  const { data: results, isLoading } = useSearchConcepts(debouncedQuery);
  const debounceRef = useRef<ReturnType<typeof setTimeout>>();

  // Debounce search input
  useEffect(() => {
    if (debounceRef.current) clearTimeout(debounceRef.current);
    debounceRef.current = setTimeout(() => {
      setDebouncedQuery(query);
    }, 300);
    return () => {
      if (debounceRef.current) clearTimeout(debounceRef.current);
    };
  }, [query]);

  // Update graph highlights when results change
  useEffect(() => {
    if (results && results.length > 0 && debouncedQuery.length >= 2) {
      setSearchHighlights(results.map((c) => c.id));
    } else {
      setSearchHighlights([]);
    }
  }, [results, debouncedQuery, setSearchHighlights]);

  // Clear highlights on unmount
  useEffect(() => {
    return () => setSearchHighlights([]);
  }, [setSearchHighlights]);

  const handleSelect = (conceptId: string) => {
    selectConcept(conceptId);
    setQuery("");
    setIsOpen(false);
    setSearchHighlights([]);
  };

  const handleClear = () => {
    setQuery("");
    setDebouncedQuery("");
    setSearchHighlights([]);
    setIsOpen(false);
  };

  return (
    <div className="relative">
      <div className="flex items-center gap-2 px-3 py-2 border-t">
        <Search className="h-4 w-4 text-muted-foreground" />
        <input
          type="text"
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setIsOpen(true);
          }}
          onFocus={() => setIsOpen(true)}
          placeholder={t("concepts.search")}
          className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        />
        {isLoading && <Loader2 className="h-4 w-4 animate-spin" />}
        {query && (
          <button
            onClick={handleClear}
            className="text-xs text-muted-foreground hover:text-foreground"
          >
            &times;
          </button>
        )}
      </div>
      {isOpen && debouncedQuery.length >= 2 && results && results.length > 0 && (
        <div className="absolute bottom-full left-0 right-0 mb-1 bg-popover border rounded-md shadow-lg max-h-64 overflow-y-auto z-50">
          {results.map((concept) => (
            <button
              key={concept.id}
              onClick={() => handleSelect(concept.id)}
              className={cn(
                "w-full px-3 py-2 text-left text-sm",
                "hover:bg-accent transition-colors",
                "border-b last:border-b-0"
              )}
            >
              <div className="font-medium">{getName(concept)}</div>
              <div className="text-xs text-muted-foreground">
                {concept.framework_id} · {concept.concept_type}
              </div>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
