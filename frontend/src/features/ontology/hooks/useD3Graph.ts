import { useEffect, useRef, useCallback } from "react";
import * as d3 from "d3";
import type { GraphData, GraphNode, GraphEdge } from "../types";
import { getFrameworkColor } from "../utils/graphTransform";

interface UseD3GraphOptions {
  data: GraphData;
  onNodeClick?: (node: GraphNode) => void;
  onNodeDoubleClick?: (node: GraphNode) => void;
  selectedNodeId?: string | null;
  width: number;
  height: number;
}

export function useD3Graph({
  data,
  onNodeClick,
  onNodeDoubleClick,
  selectedNodeId,
  width,
  height,
}: UseD3GraphOptions) {
  const svgRef = useRef<SVGSVGElement>(null);
  const simulationRef = useRef<d3.Simulation<GraphNode, GraphEdge> | null>(null);
  const zoomRef = useRef<d3.ZoomBehavior<SVGSVGElement, unknown> | null>(null);

  const zoomIn = useCallback(() => {
    if (svgRef.current && zoomRef.current) {
      d3.select(svgRef.current)
        .transition()
        .duration(300)
        .call(zoomRef.current.scaleBy, 1.3);
    }
  }, []);

  const zoomOut = useCallback(() => {
    if (svgRef.current && zoomRef.current) {
      d3.select(svgRef.current)
        .transition()
        .duration(300)
        .call(zoomRef.current.scaleBy, 0.7);
    }
  }, []);

  const resetView = useCallback(() => {
    if (svgRef.current && zoomRef.current) {
      d3.select(svgRef.current)
        .transition()
        .duration(500)
        .call(zoomRef.current.transform, d3.zoomIdentity);
    }
  }, []);

  const fitToScreen = useCallback(() => {
    if (!svgRef.current || !data.nodes.length) return;

    const svg = d3.select(svgRef.current);
    const bounds = {
      minX: d3.min(data.nodes, (d) => d.x ?? 0) ?? 0,
      maxX: d3.max(data.nodes, (d) => d.x ?? 0) ?? width,
      minY: d3.min(data.nodes, (d) => d.y ?? 0) ?? 0,
      maxY: d3.max(data.nodes, (d) => d.y ?? 0) ?? height,
    };

    const graphWidth = bounds.maxX - bounds.minX + 100;
    const graphHeight = bounds.maxY - bounds.minY + 100;
    const scale = Math.min(width / graphWidth, height / graphHeight, 1) * 0.9;
    const translateX = (width - graphWidth * scale) / 2 - bounds.minX * scale + 50;
    const translateY = (height - graphHeight * scale) / 2 - bounds.minY * scale + 50;

    if (zoomRef.current) {
      svg
        .transition()
        .duration(500)
        .call(
          zoomRef.current.transform,
          d3.zoomIdentity.translate(translateX, translateY).scale(scale)
        );
    }
  }, [data.nodes, width, height]);

  useEffect(() => {
    if (!svgRef.current || !data.nodes.length) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    // Create container group for zoom
    const g = svg.append("g");

    // Setup zoom
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on("zoom", (event) => {
        g.attr("transform", event.transform);
      });

    svg.call(zoom);
    zoomRef.current = zoom;

    // Create arrow marker
    svg.append("defs").append("marker")
      .attr("id", "arrowhead")
      .attr("viewBox", "-0 -5 10 10")
      .attr("refX", 20)
      .attr("refY", 0)
      .attr("orient", "auto")
      .attr("markerWidth", 6)
      .attr("markerHeight", 6)
      .append("path")
      .attr("d", "M 0,-5 L 10,0 L 0,5")
      .attr("fill", "#94a3b8");

    // Create links
    const link = g.append("g")
      .selectAll("path")
      .data(data.edges)
      .join("path")
      .attr("stroke", "#94a3b8")
      .attr("stroke-width", 1.5)
      .attr("fill", "none")
      .attr("marker-end", "url(#arrowhead)")
      .attr("opacity", 0.6);

    // Create link labels
    const linkLabel = g.append("g")
      .selectAll("text")
      .data(data.edges)
      .join("text")
      .attr("font-size", 10)
      .attr("fill", "#64748b")
      .attr("text-anchor", "middle")
      .text((d) => d.label);

    // Create nodes
    const node = g.append("g")
      .selectAll("g")
      .data(data.nodes)
      .join("g")
      .attr("cursor", "pointer")
      .call(
        d3.drag<SVGGElement, GraphNode>()
          .on("start", (event, d) => {
            if (!event.active) simulationRef.current?.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
          })
          .on("drag", (event, d) => {
            d.fx = event.x;
            d.fy = event.y;
          })
          .on("end", (event) => {
            if (!event.active) simulationRef.current?.alphaTarget(0);
            // Keep position fixed after drag
          })
      );

    // Node background
    node.append("rect")
      .attr("width", 140)
      .attr("height", 36)
      .attr("x", -70)
      .attr("y", -18)
      .attr("rx", 6)
      .attr("fill", "white")
      .attr("stroke", (d) => getFrameworkColor(d.frameworkId))
      .attr("stroke-width", 2);

    // Node text
    node.append("text")
      .attr("text-anchor", "middle")
      .attr("dominant-baseline", "middle")
      .attr("font-size", 11)
      .attr("fill", "#1e293b")
      .text((d) => {
        const text = d.code ? `${d.code} ${d.name}` : d.name;
        return text.length > 18 ? text.slice(0, 16) + "..." : text;
      });

    // Node interactions
    node
      .on("click", (event, d) => {
        event.stopPropagation();
        onNodeClick?.(d);
      })
      .on("dblclick", (event, d) => {
        event.stopPropagation();
        onNodeDoubleClick?.(d);
      })
      .on("mouseenter", function (event, d) {
        d3.select(this).select("rect").attr("filter", "drop-shadow(0 4px 6px rgb(0 0 0 / 0.1))");
        // Highlight connected edges
        link.attr("opacity", (l) =>
          (l.source as GraphNode).id === d.id || (l.target as GraphNode).id === d.id ? 1 : 0.2
        );
      })
      .on("mouseleave", function () {
        d3.select(this).select("rect").attr("filter", null);
        link.attr("opacity", 0.6);
      });

    // Highlight selected node
    node.select("rect")
      .attr("fill", (d) => d.id === selectedNodeId ? getFrameworkColor(d.frameworkId) : "white")
      .attr("stroke-width", (d) => d.id === selectedNodeId ? 3 : 2);

    node.select("text")
      .attr("fill", (d) => d.id === selectedNodeId ? "white" : "#1e293b");

    // Setup simulation
    const simulation = d3.forceSimulation(data.nodes)
      .force("link", d3.forceLink<GraphNode, GraphEdge>(data.edges)
        .id((d) => d.id)
        .distance(150))
      .force("charge", d3.forceManyBody().strength(-400))
      .force("center", d3.forceCenter(width / 2, height / 2))
      .force("collision", d3.forceCollide().radius(80));

    simulationRef.current = simulation;

    simulation.on("tick", () => {
      link.attr("d", (d) => {
        const source = d.source as GraphNode;
        const target = d.target as GraphNode;
        const dx = (target.x ?? 0) - (source.x ?? 0);
        const dy = (target.y ?? 0) - (source.y ?? 0);
        const dr = Math.sqrt(dx * dx + dy * dy) * 1.5;
        return `M${source.x},${source.y}A${dr},${dr} 0 0,1 ${target.x},${target.y}`;
      });

      linkLabel
        .attr("x", (d) => {
          const source = d.source as GraphNode;
          const target = d.target as GraphNode;
          return ((source.x ?? 0) + (target.x ?? 0)) / 2;
        })
        .attr("y", (d) => {
          const source = d.source as GraphNode;
          const target = d.target as GraphNode;
          return ((source.y ?? 0) + (target.y ?? 0)) / 2 - 5;
        });

      node.attr("transform", (d) => `translate(${d.x},${d.y})`);
    });

    // Click on background to deselect
    svg.on("click", () => onNodeClick?.(null as unknown as GraphNode));

    return () => {
      simulation.stop();
    };
  }, [data, width, height, selectedNodeId, onNodeClick, onNodeDoubleClick]);

  return {
    svgRef,
    zoomIn,
    zoomOut,
    resetView,
    fitToScreen,
  };
}
