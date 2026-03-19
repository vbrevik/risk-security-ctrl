import { useRef, useEffect, useState } from "react";
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

interface CoverageHeatmapProps {
  data: Array<{
    frameworkId: string;
    percentage: number;
    addressed: number;
    total: number;
  }>;
}

const BAR_HEIGHT = 40;
const MARGINS = { top: 10, right: 60, bottom: 10, left: 120 };

export function CoverageHeatmap({ data }: CoverageHeatmapProps) {
  const { t } = useTranslation("analysis");
  const containerRef = useRef<HTMLDivElement>(null);
  const svgRef = useRef<SVGSVGElement>(null);
  const { width } = useContainerDimensions(containerRef);
  const [tooltip, setTooltip] = useState<{
    x: number;
    y: number;
    text: string;
  } | null>(null);

  useEffect(() => {
    if (!svgRef.current || width === 0 || data.length === 0) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    const innerWidth = width - MARGINS.left - MARGINS.right;
    const innerHeight = data.length * BAR_HEIGHT;
    const totalHeight = innerHeight + MARGINS.top + MARGINS.bottom;

    svg.attr("viewBox", `0 0 ${width} ${totalHeight}`);

    // Add accessibility elements
    svg
      .append("title")
      .attr("id", "coverage-heatmap-title")
      .text(t("charts.coverage.title"));
    svg
      .append("desc")
      .attr("id", "coverage-heatmap-desc")
      .text(t("charts.coverage.description"));

    const g = svg
      .append("g")
      .attr("transform", `translate(${MARGINS.left},${MARGINS.top})`);

    const yScale = d3
      .scaleBand()
      .domain(data.map((d) => d.frameworkId))
      .range([0, innerHeight])
      .padding(0.2);

    const xScale = d3.scaleLinear().domain([0, 100]).range([0, innerWidth]);

    // Draw bars
    g.selectAll("rect")
      .data(data)
      .join("rect")
      .attr("x", 0)
      .attr("y", (d) => yScale(d.frameworkId) ?? 0)
      .attr("width", (d) => xScale(d.percentage))
      .attr("height", yScale.bandwidth())
      .attr("fill", (d) => d3.interpolateRdYlGn(d.percentage / 100))
      .attr("rx", 2)
      .on("mouseover", (event, d) => {
        const rect = (
          event.currentTarget as SVGRectElement
        ).getBoundingClientRect();
        const containerRect = containerRef.current?.getBoundingClientRect();
        if (containerRect) {
          setTooltip({
            x: rect.right - containerRect.left,
            y: rect.top - containerRect.top + rect.height / 2,
            text: `${d.frameworkId}: ${d.addressed}/${d.total} (${d.percentage.toFixed(1)}%)`,
          });
        }
      })
      .on("mouseout", () => setTooltip(null));

    // Y axis labels
    g.selectAll(".label")
      .data(data)
      .join("text")
      .attr("class", "label")
      .attr("x", -8)
      .attr("y", (d) => (yScale(d.frameworkId) ?? 0) + yScale.bandwidth() / 2)
      .attr("text-anchor", "end")
      .attr("dominant-baseline", "central")
      .attr("font-size", "12px")
      .attr("fill", "currentColor")
      .text((d) => d.frameworkId);

    // Percentage text on bars
    g.selectAll(".pct")
      .data(data)
      .join("text")
      .attr("class", "pct")
      .attr("x", (d) => xScale(d.percentage) + 4)
      .attr("y", (d) => (yScale(d.frameworkId) ?? 0) + yScale.bandwidth() / 2)
      .attr("dominant-baseline", "central")
      .attr("font-size", "11px")
      .attr("fill", "currentColor")
      .text((d) => `${Math.round(d.percentage)}%`);

    return () => {
      svg.selectAll("*").remove();
    };
  }, [data, width, t]);

  return (
    <Card data-testid="coverage-heatmap">
      <CardHeader>
        <CardTitle>{t("charts.coverage.title")}</CardTitle>
        <CardDescription>{t("charts.coverage.description")}</CardDescription>
      </CardHeader>
      <CardContent>
        {data.length === 0 ? (
          <p className="text-center text-muted-foreground py-8">
            {t("charts.coverage.noData")}
          </p>
        ) : (
          <div ref={containerRef} className="relative w-full">
            <svg
              ref={svgRef}
              role="img"
              aria-labelledby="coverage-heatmap-title coverage-heatmap-desc"
              className="w-full"
              style={{
                height: data.length * BAR_HEIGHT + MARGINS.top + MARGINS.bottom,
              }}
            />
            {tooltip && (
              <div
                className="absolute bg-popover text-popover-foreground border rounded px-2 py-1 text-xs shadow-md pointer-events-none z-10"
                style={{ left: tooltip.x + 8, top: tooltip.y - 12 }}
              >
                {tooltip.text}
              </div>
            )}
          </div>
        )}
      </CardContent>
    </Card>
  );
}
