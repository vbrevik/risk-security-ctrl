import { useEffect, useRef } from "react";
import * as d3 from "d3";
import type { GraphData, GraphNode } from "../../types";
import { getFrameworkColor } from "../../utils/graphTransform";

interface MinimapProps {
  data: GraphData;
  width: number;
  height: number;
  viewportBounds?: { x: number; y: number; width: number; height: number };
}

export function Minimap({ data, width, height }: MinimapProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const minimapWidth = 150;
  const minimapHeight = 100;

  useEffect(() => {
    if (!svgRef.current || !data.nodes.length) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    // Calculate scale to fit all nodes
    const bounds = {
      minX: d3.min(data.nodes, (d) => d.x ?? 0) ?? 0,
      maxX: d3.max(data.nodes, (d) => d.x ?? 0) ?? width,
      minY: d3.min(data.nodes, (d) => d.y ?? 0) ?? 0,
      maxY: d3.max(data.nodes, (d) => d.y ?? 0) ?? height,
    };

    const graphWidth = bounds.maxX - bounds.minX + 50;
    const graphHeight = bounds.maxY - bounds.minY + 50;
    const scale = Math.min(minimapWidth / graphWidth, minimapHeight / graphHeight) * 0.9;

    const g = svg.append("g")
      .attr("transform", `translate(${minimapWidth / 2 - (bounds.minX + graphWidth / 2) * scale}, ${minimapHeight / 2 - (bounds.minY + graphHeight / 2) * scale}) scale(${scale})`);

    // Draw edges
    g.selectAll("line")
      .data(data.edges)
      .join("line")
      .attr("x1", (d) => (d.source as GraphNode).x ?? 0)
      .attr("y1", (d) => (d.source as GraphNode).y ?? 0)
      .attr("x2", (d) => (d.target as GraphNode).x ?? 0)
      .attr("y2", (d) => (d.target as GraphNode).y ?? 0)
      .attr("stroke", "#94a3b8")
      .attr("stroke-width", 1 / scale);

    // Draw nodes
    g.selectAll("circle")
      .data(data.nodes)
      .join("circle")
      .attr("cx", (d) => d.x ?? 0)
      .attr("cy", (d) => d.y ?? 0)
      .attr("r", 4 / scale)
      .attr("fill", (d) => getFrameworkColor(d.frameworkId));

  }, [data, width, height]);

  return (
    <div className="absolute top-4 right-4 bg-card/90 border rounded-md shadow-sm p-1">
      <svg
        ref={svgRef}
        width={minimapWidth}
        height={minimapHeight}
        className="bg-muted/30 rounded"
      />
    </div>
  );
}
