import { useState, useEffect, useRef } from "react";
import { Download, Keyboard } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useNavigate, useSearch } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import { ExplorerProvider, useExplorer } from "../context";
import { Sidebar } from "./Sidebar";
import { GraphView } from "./Graph";
import { CompareView } from "./Compare";
import { TreeView } from "./Tree";
import { ContextPanel } from "./ContextPanel";
import { ExportDialog } from "./ExportDialog";
import type { ViewMode } from "../types";

function ExplorerContent() {
  const { t } = useTranslation("ontology");
  const { state, setViewMode, selectConcept, setActiveFrameworks, setConceptType } = useExplorer();
  const [showExportDialog, setShowExportDialog] = useState(false);
  const [showKeyboardHelp, setShowKeyboardHelp] = useState(false);
  const [initialized, setInitialized] = useState(false);

  const search = useSearch({ from: "/ontology/" });
  const navigate = useNavigate({ from: "/ontology/" });
  const navigateRef = useRef(navigate);
  navigateRef.current = navigate;
  const lastSearchRef = useRef("");

  // Initialize state from URL on first render
  useEffect(() => {
    if (initialized) return;
    // "detail" view was removed — redirect to graph (panel shows automatically)
    if (search.view && search.view !== "detail") setViewMode(search.view as ViewMode);
    if (search.concept) selectConcept(search.concept);
    if (search.frameworks) setActiveFrameworks(search.frameworks.split(","));
    if (search.type) setConceptType(search.type);
    setInitialized(true);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Sync state to URL when it changes (using serialized comparison to avoid loops)
  const viewMode = state.viewMode;
  const selectedConceptId = state.selectedConceptId;
  const activeFrameworks = state.activeFrameworks;
  const activeConceptType = state.activeConceptType;

  useEffect(() => {
    if (!initialized) return;
    const params: Record<string, string> = {};
    if (viewMode !== "graph") params.view = viewMode;
    if (selectedConceptId) params.concept = selectedConceptId;
    if (activeFrameworks.length > 0) params.frameworks = activeFrameworks.join(",");
    if (activeConceptType) params.type = activeConceptType;

    const serialized = JSON.stringify(params);
    if (serialized === lastSearchRef.current) return;
    lastSearchRef.current = serialized;

    navigateRef.current({
      search: params,
      replace: true,
    });
  }, [initialized, viewMode, selectedConceptId, activeFrameworks, activeConceptType]);

  const viewModes: { mode: ViewMode; label: string }[] = [
    { mode: "graph", label: t("views.graph") },
    { mode: "tree", label: t("views.tree") },
    { mode: "compare", label: t("views.compare") },
  ];

  return (
    <div className="flex h-[calc(100vh-4rem)]">
      <Sidebar />

      <div className="flex-1 flex flex-col min-w-0">
        {/* Toolbar */}
        <div className="flex items-center justify-between px-4 py-2 border-b bg-card">
          {/* View toggle */}
          <div className="flex gap-1">
            {viewModes.map(({ mode, label }) => (
              <Button
                key={mode}
                variant={state.viewMode === mode ? "secondary" : "ghost"}
                size="sm"
                onClick={() => setViewMode(mode)}
              >
                {label}
              </Button>
            ))}
          </div>

          {/* Actions */}
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowKeyboardHelp(!showKeyboardHelp)}
              title={t("keyboard.title")}
            >
              <Keyboard className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowExportDialog(true)}
              title={`${t("export.export")} (Ctrl+E)`}
            >
              <Download className="h-4 w-4" />
            </Button>
          </div>
        </div>

        {/* Keyboard help overlay */}
        {showKeyboardHelp && (
          <div className="absolute top-20 right-4 z-40 bg-card border rounded-lg shadow-lg p-4 w-64">
            <h3 className="font-semibold mb-2">{t("keyboard.title")}</h3>
            <div className="space-y-1 text-sm">
              <div className="flex justify-between">
                <span>{t("keyboard.zoomIn")}</span>
                <kbd className="px-1 bg-muted rounded">+</kbd>
              </div>
              <div className="flex justify-between">
                <span>{t("keyboard.zoomOut")}</span>
                <kbd className="px-1 bg-muted rounded">-</kbd>
              </div>
              <div className="flex justify-between">
                <span>{t("keyboard.resetView")}</span>
                <kbd className="px-1 bg-muted rounded">0</kbd>
              </div>
              <div className="flex justify-between">
                <span>{t("keyboard.clearSelection")}</span>
                <kbd className="px-1 bg-muted rounded">Esc</kbd>
              </div>
              <div className="flex justify-between">
                <span>{t("keyboard.search")}</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+F</kbd>
              </div>
              <div className="flex justify-between">
                <span>{t("keyboard.exportShortcut")}</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+E</kbd>
              </div>
            </div>
          </div>
        )}

        {/* Main content area + context panel */}
        <div className="flex-1 min-h-0 flex relative">
          <div className="flex-1 min-w-0 relative">
            {state.viewMode === "graph" && <GraphView />}
            {state.viewMode === "tree" && <TreeView />}
            {state.viewMode === "compare" && <CompareView />}
          </div>
          {state.selectedConceptId && <ContextPanel />}
        </div>

        {/* Status bar */}
        <div className="flex items-center justify-between px-4 py-1 border-t bg-muted/30 text-xs text-muted-foreground">
          <div>
            {state.selectedConceptId ? (
              <span>{t("status.selected")}: {state.selectedConceptId}</span>
            ) : (
              <span>{t("status.noSelection")}</span>
            )}
          </div>
          <div className="flex gap-4">
            <span>{t("status.nSelected", { count: state.selectedConcepts.length })}</span>
          </div>
        </div>
      </div>

      <ExportDialog
        isOpen={showExportDialog}
        onClose={() => setShowExportDialog(false)}
      />
    </div>
  );
}

export function OntologyExplorer() {
  return (
    <ExplorerProvider>
      <ExplorerContent />
    </ExplorerProvider>
  );
}
