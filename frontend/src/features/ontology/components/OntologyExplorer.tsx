import { useState } from "react";
import { Save, Download, Keyboard } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ExplorerProvider, useExplorer } from "../context";
import { Sidebar } from "./Sidebar";
import { GraphView } from "./Graph";
import { DetailView } from "./Detail";
import { CompareView } from "./Compare";
import { ExportDialog } from "./ExportDialog";
import type { ViewMode } from "../types";

function ExplorerContent() {
  const { state, setViewMode } = useExplorer();
  const [showExportDialog, setShowExportDialog] = useState(false);
  const [showKeyboardHelp, setShowKeyboardHelp] = useState(false);

  const viewModes: { mode: ViewMode; label: string }[] = [
    { mode: "graph", label: "Graph" },
    { mode: "detail", label: "Detail" },
    { mode: "compare", label: "Compare" },
  ];

  const handleSaveLayout = () => {
    // Layout is auto-saved, but this could trigger a manual save
    console.log("Layout saved");
  };

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
              title="Keyboard shortcuts"
            >
              <Keyboard className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleSaveLayout}
              title="Save layout (Ctrl+S)"
            >
              <Save className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowExportDialog(true)}
              title="Export (Ctrl+E)"
            >
              <Download className="h-4 w-4" />
            </Button>
          </div>
        </div>

        {/* Keyboard help overlay */}
        {showKeyboardHelp && (
          <div className="absolute top-20 right-4 z-40 bg-card border rounded-lg shadow-lg p-4 w-64">
            <h3 className="font-semibold mb-2">Keyboard Shortcuts</h3>
            <div className="space-y-1 text-sm">
              <div className="flex justify-between">
                <span>Zoom in</span>
                <kbd className="px-1 bg-muted rounded">+</kbd>
              </div>
              <div className="flex justify-between">
                <span>Zoom out</span>
                <kbd className="px-1 bg-muted rounded">-</kbd>
              </div>
              <div className="flex justify-between">
                <span>Reset view</span>
                <kbd className="px-1 bg-muted rounded">0</kbd>
              </div>
              <div className="flex justify-between">
                <span>Clear selection</span>
                <kbd className="px-1 bg-muted rounded">Esc</kbd>
              </div>
              <div className="flex justify-between">
                <span>Search</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+F</kbd>
              </div>
              <div className="flex justify-between">
                <span>Save</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+S</kbd>
              </div>
              <div className="flex justify-between">
                <span>Export</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+E</kbd>
              </div>
            </div>
          </div>
        )}

        {/* Main content area */}
        <div className="flex-1 min-h-0 relative">
          {state.viewMode === "graph" && <GraphView />}
          {state.viewMode === "detail" && <DetailView />}
          {state.viewMode === "compare" && <CompareView />}
        </div>

        {/* Status bar */}
        <div className="flex items-center justify-between px-4 py-1 border-t bg-muted/30 text-xs text-muted-foreground">
          <div>
            {state.selectedConceptId ? (
              <span>Selected: {state.selectedConceptId}</span>
            ) : (
              <span>No selection</span>
            )}
          </div>
          <div className="flex gap-4">
            <span>{state.selectedConcepts.length} selected</span>
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
