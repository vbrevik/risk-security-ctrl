import { useState } from "react";
import { Search, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useSearchConcepts } from "../../api";
import { useExplorer } from "../../context";
import { cn } from "@/lib/utils";

export function SearchBox() {
  const { t } = useTranslation("ontology");
  const [query, setQuery] = useState("");
  const [isOpen, setIsOpen] = useState(false);
  const { selectConcept, setViewMode } = useExplorer();
  const { data: results, isLoading } = useSearchConcepts(query);

  const handleSelect = (conceptId: string) => {
    selectConcept(conceptId);
    setViewMode("detail");
    setQuery("");
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
      </div>
      {isOpen && query.length >= 2 && results && results.length > 0 && (
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
              <div className="font-medium">{concept.name_en}</div>
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
