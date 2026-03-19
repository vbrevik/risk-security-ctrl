import { useState, useMemo } from "react";
import { PanelLeftClose, PanelLeft, Filter, SlidersHorizontal } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useQueries } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { useFrameworks, useTopics } from "../../api";
import { ontologyKeys } from "../../api";
import { useExplorer } from "../../context";
import { getFrameworkColor } from "../../utils/graphTransform";
import { FrameworkTree } from "./FrameworkTree";
import { SearchBox } from "./SearchBox";
import type { Concept, PaginatedResponse } from "../../types";

function FilterPanel() {
  const { t, i18n } = useTranslation("ontology");
  const { state, toggleFramework, setActiveFrameworks, setConceptType, toggleTopic, setActiveTopics } = useExplorer();
  const { data: frameworks } = useFrameworks();
  const { data: topics } = useTopics();
  const lang = i18n.language === "nb" ? "nb" : "en";

  // Fetch concepts from all frameworks to aggregate concept types
  const allFrameworkQueries = useQueries({
    queries: (frameworks ?? []).map((fw) => ({
      queryKey: ontologyKeys.concepts(fw.id),
      queryFn: async () => {
        const params = new URLSearchParams();
        params.set("framework_id", fw.id);
        params.set("limit", "500");
        const { data } = await api.get<PaginatedResponse<Concept>>(
          `/ontology/concepts?${params}`
        );
        return data.items;
      },
      staleTime: 1000 * 60 * 5,
    })),
  });

  // Use a stable dep key derived from query status to avoid re-running on every render
  const queryKey = allFrameworkQueries.map((q) => q.dataUpdatedAt).join(",");
  const conceptTypes = useMemo(() => {
    const allConcepts = allFrameworkQueries.flatMap((q) => q.data ?? []);
    const types = new Set(allConcepts.map((c) => c.concept_type));
    return Array.from(types).sort();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [queryKey]);

  const allChecked = state.activeFrameworks.length === 0;
  const hasActiveFilters = !allChecked || state.activeConceptType || state.activeTopics.length > 0;

  return (
    <div className="px-3 py-2 border-b space-y-3">
      <div className="flex items-center justify-between">
        <h3 className="text-xs font-medium text-muted-foreground uppercase tracking-wide">
          {t("filters.title")}
        </h3>
        {hasActiveFilters && (
          <button
            onClick={() => {
              setActiveFrameworks([]);
              setConceptType(null);
              setActiveTopics([]);
            }}
            className="text-xs text-muted-foreground hover:text-foreground"
          >
            {t("filters.clearAll")}
          </button>
        )}
      </div>

      {/* Framework checkboxes */}
      <div>
        <label className="text-xs text-muted-foreground">{t("filters.frameworks")}</label>
        <div className="space-y-1 mt-1">
          {frameworks?.map((fw) => {
            const isActive = allChecked || state.activeFrameworks.includes(fw.id);
            return (
              <label
                key={fw.id}
                className="flex items-center gap-2 cursor-pointer text-sm"
              >
                <input
                  type="checkbox"
                  checked={isActive}
                  onChange={() => {
                    if (allChecked) {
                      // First click when "all" is selected - select only the others
                      const others = frameworks
                        .filter((f) => f.id !== fw.id)
                        .map((f) => f.id);
                      setActiveFrameworks(others);
                    } else {
                      toggleFramework(fw.id);
                    }
                  }}
                  className="w-3.5 h-3.5 rounded"
                />
                <span
                  className="w-2 h-2 rounded-full"
                  style={{ backgroundColor: getFrameworkColor(fw.id) }}
                />
                <span className="truncate">{fw.name}</span>
              </label>
            );
          })}
        </div>
      </div>

      {/* Topic checkboxes */}
      {topics && topics.length > 0 && (
        <div>
          <label className="text-xs text-muted-foreground">{t("filters.topics")}</label>
          <div className="space-y-1 mt-1">
            {topics.map((topic) => {
              const isActive = state.activeTopics.includes(topic.id);
              const name = lang === "nb" ? topic.name_nb : topic.name_en;
              return (
                <label
                  key={topic.id}
                  className="flex items-center gap-2 cursor-pointer text-sm"
                  title={lang === "nb" ? topic.description_nb : topic.description_en}
                >
                  <input
                    type="checkbox"
                    checked={isActive}
                    onChange={() => toggleTopic(topic.id)}
                    className="w-3.5 h-3.5 rounded"
                  />
                  <span className="truncate">{name}</span>
                </label>
              );
            })}
          </div>
        </div>
      )}

      {/* Concept type filter */}
      <div>
        <label className="text-xs text-muted-foreground">{t("filters.conceptType")}</label>
        <select
          value={state.activeConceptType ?? ""}
          onChange={(e) => setConceptType(e.target.value || null)}
          className="w-full mt-1 px-2 py-1 text-sm border rounded bg-background"
        >
          <option value="">{t("filters.allTypes")}</option>
          {conceptTypes.map((type) => (
            <option key={type} value={type}>
              {type}
            </option>
          ))}
        </select>
      </div>
    </div>
  );
}

export function Sidebar() {
  const { t } = useTranslation("ontology");
  const [filterQuery, setFilterQuery] = useState("");
  const [showFilters, setShowFilters] = useState(false);
  const { state, toggleSidebar } = useExplorer();
  const { data: frameworks, isLoading } = useFrameworks();

  if (state.sidebarCollapsed) {
    return (
      <div className="w-10 border-r bg-card flex flex-col items-center py-2">
        <button
          onClick={toggleSidebar}
          className="p-2 hover:bg-accent rounded"
          title={t("sidebar.expand")}
        >
          <PanelLeft className="h-4 w-4" />
        </button>
      </div>
    );
  }

  return (
    <div className="w-72 border-r bg-card flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 border-b">
        <h2 className="font-semibold text-sm">{t("frameworks.title")}</h2>
        <div className="flex items-center gap-1">
          <button
            onClick={() => setShowFilters(!showFilters)}
            className={`p-1 hover:bg-accent rounded ${showFilters ? "bg-accent" : ""}`}
            title={t("filters.title")}
          >
            <SlidersHorizontal className="h-4 w-4" />
          </button>
          <button
            onClick={toggleSidebar}
            className="p-1 hover:bg-accent rounded"
            title={t("sidebar.collapse")}
          >
            <PanelLeftClose className="h-4 w-4" />
          </button>
        </div>
      </div>

      {/* Filter panel (collapsible) */}
      {showFilters && <FilterPanel />}

      {/* Text filter input */}
      <div className="flex items-center gap-2 px-3 py-2 border-b">
        <Filter className="h-4 w-4 text-muted-foreground" />
        <input
          type="text"
          value={filterQuery}
          onChange={(e) => setFilterQuery(e.target.value)}
          placeholder={t("concepts.filter")}
          className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        />
      </div>

      {/* Framework trees */}
      <div className="flex-1 overflow-y-auto">
        {isLoading ? (
          <div className="p-4 text-sm text-muted-foreground">{t("concepts.loading")}</div>
        ) : (
          frameworks?.map((framework) => (
            <FrameworkTree
              key={framework.id}
              framework={framework}
              filterQuery={filterQuery}
            />
          ))
        )}
      </div>

      {/* Search box */}
      <SearchBox />
    </div>
  );
}
