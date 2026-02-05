import { useState } from "react";
import { PanelLeftClose, PanelLeft, Filter } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useFrameworks } from "../../api";
import { useExplorer } from "../../context";
import { FrameworkTree } from "./FrameworkTree";
import { SearchBox } from "./SearchBox";

export function Sidebar() {
  const { t } = useTranslation("ontology");
  const [filterQuery, setFilterQuery] = useState("");
  const { state, toggleSidebar } = useExplorer();
  const { data: frameworks, isLoading } = useFrameworks();

  if (state.sidebarCollapsed) {
    return (
      <div className="w-10 border-r bg-card flex flex-col items-center py-2">
        <button
          onClick={toggleSidebar}
          className="p-2 hover:bg-accent rounded"
          title="Expand sidebar"
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
        <button
          onClick={toggleSidebar}
          className="p-1 hover:bg-accent rounded"
          title="Collapse sidebar"
        >
          <PanelLeftClose className="h-4 w-4" />
        </button>
      </div>

      {/* Filter input */}
      <div className="flex items-center gap-2 px-3 py-2 border-b">
        <Filter className="h-4 w-4 text-muted-foreground" />
        <input
          type="text"
          value={filterQuery}
          onChange={(e) => setFilterQuery(e.target.value)}
          placeholder="Filter..."
          className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        />
      </div>

      {/* Framework trees */}
      <div className="flex-1 overflow-y-auto">
        {isLoading ? (
          <div className="p-4 text-sm text-muted-foreground">Loading...</div>
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
