import { useState, useMemo, useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { useConcepts, useRelationships } from "../../api";
import { useExplorer } from "../../context";
import { buildGraphData } from "../../utils/graphTransform";
import { useD3Graph } from "../../hooks/useD3Graph";
import { GraphControls } from "./GraphControls";
import { Minimap } from "./Minimap";
import type { GraphNode } from "../../types";

export function GraphView() {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });
  const [showMinimap, setShowMinimap] = useState(true);
  const { i18n } = useTranslation();
  const { state, selectConcept, setViewMode } = useExplorer();

  // Fetch all concepts from all frameworks for now
  // In production, you might want to paginate or filter
  const { data: iso31000Concepts } = useConcepts("iso31000");
  const { data: iso31010Concepts } = useConcepts("iso31010");
  const { data: nistCsfConcepts } = useConcepts("nist-csf");
  const { data: relationships } = useRelationships();

  // Combine concepts from different frameworks
  const allConcepts = useMemo(() => {
    const concepts = [
      ...(iso31000Concepts ?? []),
      ...(iso31010Concepts ?? []),
      ...(nistCsfConcepts ?? []),
    ];
    // Limit to top-level concepts for initial view
    return concepts.filter((c) => !c.parent_id).slice(0, 50);
  }, [iso31000Concepts, iso31010Concepts, nistCsfConcepts]);

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
    setViewMode("detail");
  };

  const { svgRef, zoomIn, zoomOut, resetView, fitToScreen } = useD3Graph({
    data: graphData,
    onNodeClick: handleNodeClick,
    onNodeDoubleClick: handleNodeDoubleClick,
    selectedNodeId: state.selectedConceptId,
    width: dimensions.width,
    height: dimensions.height,
  });

  // Handle resize
  useEffect(() => {
    const handleResize = () => {
      if (containerRef.current) {
        setDimensions({
          width: containerRef.current.clientWidth,
          height: containerRef.current.clientHeight,
        });
      }
    };

    handleResize();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
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
