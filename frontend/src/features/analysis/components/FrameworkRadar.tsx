import { useRef, useEffect, useState, useMemo } from "react";
import { useTranslation } from "react-i18next";
import * as d3 from "d3";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { useContainerDimensions } from "../hooks/useContainerDimensions";
import { buildFrameworkColorMap } from "../utils/frameworkColors";

interface FrameworkRadarProps {
  data: Array<{
    frameworkId: string;
    values: { addressed: number; partial: number; gap: number; notApplicable: number };
    total: number;
  }>;
  selectedFrameworkId?: string | null;
  frameworkIds: string[];
}

const MARGIN = 60;
const GRID_LEVELS = 4;
const MAX_FRAMEWORKS = 8;
const AXES = ["addressed", "partial", "gap", "notApplicable"] as const;
const AXIS_LABEL_KEYS: Record<typeof AXES[number], string> = {
  addressed: "charts.radar.addressed",
  partial: "charts.radar.partial",
  gap: "charts.radar.gap",
  notApplicable: "charts.radar.notApplicable",
};
const ANGLE_SLICE = (2 * Math.PI) / AXES.length;

export function FrameworkRadar({ data, selectedFrameworkId, frameworkIds }: FrameworkRadarProps) {
  const { t } = useTranslation("analysis");
  const containerRef = useRef<HTMLDivElement>(null);
  const svgRef = useRef<SVGSVGElement>(null);
  const { width } = useContainerDimensions(containerRef);
  const [tooltip, setTooltip] = useState<{
    x: number;
    y: number;
    text: string;
  } | null>(null);

  const displayData = useMemo(
    () =>
      [...data]
        .sort((a, b) => b.total - a.total)
        .slice(0, MAX_FRAMEWORKS),
    [data]
  );

  const colorOf = useMemo(() => buildFrameworkColorMap(frameworkIds), [frameworkIds]);

  useEffect(() => {
    if (!svgRef.current || width === 0 || displayData.length === 0) return;

    const size = Math.min(width, 400);
    const radius = size / 2 - MARGIN;
    const centerX = size / 2;
    const centerY = size / 2;

    const radialScale = d3.scaleLinear().domain([0, 100]).range([0, radius]);

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();
    svg.attr("viewBox", `0 0 ${size} ${size}`);

    svg
      .append("title")
      .attr("id", "framework-radar-title")
      .text(t("charts.radar.title"));
    svg
      .append("desc")
      .attr("id", "framework-radar-desc")
      .text(t("charts.radar.description"));

    const g = svg.append("g").attr("transform", `translate(${centerX},${centerY})`);

    // Grid rings
    for (let level = 1; level <= GRID_LEVELS; level++) {
      const r = radialScale((100 / GRID_LEVELS) * level);
      g.append("circle")
        .attr("class", "radar-grid")
        .attr("cx", 0)
        .attr("cy", 0)
        .attr("r", r)
        .attr("fill", "none")
        .attr("stroke", "#e5e7eb")
        .attr("stroke-width", 1);
    }

    // Axis lines and labels
    AXES.forEach((axis, i) => {
      const angle = Math.PI / 2 + ANGLE_SLICE * i;
      const x = Math.cos(angle) * radius;
      const y = -Math.sin(angle) * radius;

      // Axis line
      g.append("line")
        .attr("x1", 0)
        .attr("y1", 0)
        .attr("x2", x)
        .attr("y2", y)
        .attr("stroke", "#e5e7eb")
        .attr("stroke-width", 1);

      // Axis label
      const labelOffset = 16;
      g.append("text")
        .attr("x", Math.cos(angle) * (radius + labelOffset))
        .attr("y", -Math.sin(angle) * (radius + labelOffset))
        .attr("text-anchor", "middle")
        .attr("dominant-baseline", "central")
        .attr("font-size", "11px")
        .attr("fill", "currentColor")
        .text(t(AXIS_LABEL_KEYS[axis]));
    });

    // Percentage labels along top axis
    for (let level = 1; level <= GRID_LEVELS; level++) {
      const pct = (100 / GRID_LEVELS) * level;
      const r = radialScale(pct);
      g.append("text")
        .attr("x", 4)
        .attr("y", -r)
        .attr("font-size", "9px")
        .attr("fill", "#94a3b8")
        .text(`${Math.round(pct)}%`);
    }

    // Polygon for each framework
    const lineGen = d3
      .lineRadial<number>()
      .angle((_, i) => -ANGLE_SLICE * i + Math.PI / 2)
      .radius((d) => radialScale(d))
      .curve(d3.curveLinearClosed);

    displayData.forEach((fw) => {
      const values = [fw.values.addressed, fw.values.partial, fw.values.gap, fw.values.notApplicable];
      const color = colorOf(fw.frameworkId);

      const isSelected = selectedFrameworkId === fw.frameworkId;
      const hasSelection = !!selectedFrameworkId;

      const fillOpacity = hasSelection ? (isSelected ? 0.3 : 0.08) : 0.15;
      const strokeOpacity = hasSelection && !isSelected ? 0.3 : 1;
      const strokeWidth = isSelected ? 3 : 2;

      g.append("path")
        .attr("class", "radar-polygon")
        .attr("d", lineGen(values))
        .attr("fill", color)
        .attr("fill-opacity", fillOpacity)
        .attr("stroke", color)
        .attr("stroke-opacity", strokeOpacity)
        .attr("stroke-width", strokeWidth);

      // Invisible vertex circles for tooltip
      values.forEach((val, i) => {
        const angle = Math.PI / 2 + ANGLE_SLICE * i;
        const cx = Math.cos(angle) * radialScale(val);
        const cy = -Math.sin(angle) * radialScale(val);

        g.append("circle")
          .attr("cx", cx)
          .attr("cy", cy)
          .attr("r", 6)
          .attr("fill", color)
          .attr("fill-opacity", 0)
          .attr("stroke", "none")
          .on("mouseover", (event) => {
            const [px, py] = d3.pointer(event, svgRef.current);
            setTooltip({
              x: px,
              y: py - 8,
              text: `${fw.frameworkId}: ${t(AXIS_LABEL_KEYS[AXES[i]])} ${val.toFixed(1)}%`,
            });
          })
          .on("mouseout", () => setTooltip(null));
      });
    });

    return () => {
      svg.selectAll("*").remove();
    };
  }, [displayData, width, selectedFrameworkId, colorOf, t]);

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t("charts.radar.title")}</CardTitle>
        <CardDescription>{t("charts.radar.description")}</CardDescription>
      </CardHeader>
      <CardContent>
        {data.length === 0 ? (
          <p className="text-center text-muted-foreground py-8">
            {t("charts.radar.noData")}
          </p>
        ) : (
          <div ref={containerRef} className="relative w-full">
            <svg
              ref={svgRef}
              role="img"
              aria-labelledby="framework-radar-title framework-radar-desc"
              className="w-full"
              style={{ height: Math.min(width || 400, 400) }}
            />
            {tooltip && (
              <div
                className="absolute bg-popover text-popover-foreground border rounded px-2 py-1 text-xs shadow-md pointer-events-none z-10"
                style={{ left: tooltip.x, top: tooltip.y }}
              >
                {tooltip.text}
              </div>
            )}
            {/* Legend */}
            <div className="flex flex-wrap gap-3 mt-2 justify-center">
              {displayData.map((fw) => (
                <div key={fw.frameworkId} className="flex items-center gap-1.5 text-xs">
                  <span
                    className="w-2.5 h-2.5 rounded-full inline-block"
                    style={{ backgroundColor: colorOf(fw.frameworkId) }}
                  />
                  <span>{fw.frameworkId}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
