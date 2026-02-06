import { useState, useMemo, useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { useQueries } from "@tanstack/react-query";
import { api } from "@/lib/api";
import { useFrameworks, useRelationships } from "../../api";
import { ontologyKeys } from "../../api";
import { useExplorer } from "../../context";
import { buildGraphData } from "../../utils/graphTransform";
import { useD3Graph } from "../../hooks/useD3Graph";
import { GraphControls } from "./GraphControls";
import { Minimap } from "./Minimap";
import type { GraphNode, Framework, Concept, PaginatedResponse } from "../../types";

export function GraphView() {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });
  const [showMinimap, setShowMinimap] = useState(true);
  const { i18n } = useTranslation();
  const { state, selectConcept } = useExplorer();

  // Load all frameworks dynamically
  const { data: frameworks } = useFrameworks();

  // Determine which frameworks to show
  const visibleFrameworkIds = useMemo(() => {
    if (!frameworks) return [];
    if (state.activeFrameworks.length > 0) return state.activeFrameworks;
    return frameworks.map((fw: Framework) => fw.id);
  }, [frameworks, state.activeFrameworks]);

  // Fetch concepts for all visible frameworks using useQueries (avoids rules-of-hooks violation)
  const frameworkQueries = useQueries({
    queries: visibleFrameworkIds.map((frameworkId) => ({
      queryKey: ontologyKeys.concepts(frameworkId),
      queryFn: async () => {
        const params = new URLSearchParams();
        params.set("framework_id", frameworkId);
        params.set("limit", "500");
        const { data } = await api.get<PaginatedResponse<Concept>>(
          `/ontology/concepts?${params}`
        );
        return data.data;
      },
      staleTime: 1000 * 60 * 5,
    })),
  });

  const { data: relationships } = useRelationships();

  // Combine concepts from all visible frameworks
  const allConcepts = useMemo(() => {
    const concepts = frameworkQueries.flatMap((q) => q.data ?? []);

    // Filter by concept type if active
    const typeFiltered = state.activeConceptType
      ? concepts.filter((c) => c.concept_type === state.activeConceptType)
      : concepts;

    // Show top-level concepts + one level of children, up to 200 nodes
    const roots = typeFiltered.filter((c) => !c.parent_id);
    const rootIds = new Set(roots.map((c) => c.id));
    const children = typeFiltered.filter((c) => c.parent_id && rootIds.has(c.parent_id));

    return [...roots, ...children].slice(0, 200);
  }, [frameworkQueries, state.activeConceptType]);

  const graphData = useMemo(() => {
    const language = i18n.language.startsWith("nb") ? "nb" : "en";
    return buildGraphData(allConcepts, relationships ?? [], language);
  }, [allConcepts, relationships, i18n.language]);

  const handleNodeClick = (node: GraphNode | null) => {
    if (node) {
      selectConcept(node.id);
    } else {
      selectConcept(null);
    }
  };

  const handleNodeDoubleClick = (node: GraphNode) => {
    selectConcept(node.id);
  };

  const { svgRef, zoomIn, zoomOut, resetView, fitToScreen, panToNode } = useD3Graph({
    data: graphData,
    onNodeClick: handleNodeClick,
    onNodeDoubleClick: handleNodeDoubleClick,
    selectedNodeId: state.selectedConceptId,
    highlightedNodeIds: state.searchHighlightIds,
    width: dimensions.width,
    height: dimensions.height,
  });

  // Auto-pan to selected concept when it changes from outside the graph
  const prevSelectedRef = useRef(state.selectedConceptId);
  useEffect(() => {
    if (
      state.selectedConceptId &&
      state.selectedConceptId !== prevSelectedRef.current
    ) {
      panToNode(state.selectedConceptId);
    }
    prevSelectedRef.current = state.selectedConceptId;
  }, [state.selectedConceptId, panToNode]);

  // Handle resize (both window and container changes e.g. context panel open/close)
  useEffect(() => {
    if (!containerRef.current) return;
    const el = containerRef.current;
    const observer = new ResizeObserver(() => {
      setDimensions({ width: el.clientWidth, height: el.clientHeight });
    });
    observer.observe(el);
    return () => observer.disconnect();
  }, []);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement) return;

      switch (e.key) {
        case "+":
        case "=":
          zoomIn();
          break;
        case "-":
          zoomOut();
          break;
        case "0":
          resetView();
          break;
        case "Escape":
          selectConcept(null);
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [zoomIn, zoomOut, resetView, selectConcept]);

  return (
    <div ref={containerRef} className="relative w-full h-full bg-muted/20">
      <svg
        ref={svgRef}
        width={dimensions.width}
        height={dimensions.height}
        className="w-full h-full"
      />
      <GraphControls
        onZoomIn={zoomIn}
        onZoomOut={zoomOut}
        onResetView={resetView}
        onFitToScreen={fitToScreen}
        onToggleMinimap={() => setShowMinimap(!showMinimap)}
        minimapVisible={showMinimap}
      />
      {showMinimap && (
        <Minimap
          data={graphData}
          width={dimensions.width}
          height={dimensions.height}
        />
      )}
    </div>
  );
}
