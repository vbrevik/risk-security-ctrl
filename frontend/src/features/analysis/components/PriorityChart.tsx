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

interface PriorityChartProps {
  data: Array<{ priority: number; count: number }>;
}

const CHART_HEIGHT = 300;
const MARGINS = { top: 20, right: 20, bottom: 30, left: 40 };

const PRIORITY_COLORS: Record<number, string> = {
  1: "#ef4444",
  2: "#f97316",
  3: "#eab308",
  4: "#22c55e",
};

export function PriorityChart({ data }: PriorityChartProps) {
  const { t } = useTranslation("analysis");
  const containerRef = useRef<HTMLDivElement>(null);
  const svgRef = useRef<SVGSVGElement>(null);
  const { width } = useContainerDimensions(containerRef);
  const [tooltip, setTooltip] = useState<{
    x: number;
    y: number;
    text: string;
  } | null>(null);

  const hasData = data.some((d) => d.count > 0);

  useEffect(() => {
    if (!svgRef.current || width === 0 || !hasData) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    svg.attr("viewBox", `0 0 ${width} ${CHART_HEIGHT}`);

    // Accessibility
    svg
      .append("title")
      .attr("id", "priority-chart-title")
      .text(t("charts.priority.title"));
    svg
      .append("desc")
      .attr("id", "priority-chart-desc")
      .text(t("charts.priority.description"));

    const innerWidth = width - MARGINS.left - MARGINS.right;
    const innerHeight = CHART_HEIGHT - MARGINS.top - MARGINS.bottom;

    const g = svg
      .append("g")
      .attr("transform", `translate(${MARGINS.left},${MARGINS.top})`);

    const labels = data.map((d) => `P${d.priority}`);
    const maxCount = d3.max(data, (d) => d.count) ?? 0;

    const xScale = d3
      .scaleBand()
      .domain(labels)
      .range([0, innerWidth])
      .padding(0.3);

    const yScale = d3
      .scaleLinear()
      .domain([0, maxCount])
      .nice()
      .range([innerHeight, 0]);

    // Draw bars
    g.selectAll("rect")
      .data(data)
      .join("rect")
      .attr("x", (d) => xScale(`P${d.priority}`) ?? 0)
      .attr("y", (d) => yScale(d.count))
      .attr("width", xScale.bandwidth())
      .attr("height", (d) => innerHeight - yScale(d.count))
      .attr("fill", (d) => PRIORITY_COLORS[d.priority] ?? "#94a3b8")
      .attr("rx", 2)
      .on("mouseover", (event, d) => {
        const rect = (
          event.currentTarget as SVGRectElement
        ).getBoundingClientRect();
        const containerRect = containerRef.current?.getBoundingClientRect();
        if (containerRect) {
          setTooltip({
            x: rect.left - containerRect.left + rect.width / 2,
            y: rect.top - containerRect.top,
            text: `P${d.priority}: ${d.count} findings`,
          });
        }
      })
      .on("mouseout", () => setTooltip(null));

    // Count labels above bars
    g.selectAll(".count")
      .data(data)
      .join("text")
      .attr("class", "count")
      .attr(
        "x",
        (d) => (xScale(`P${d.priority}`) ?? 0) + xScale.bandwidth() / 2
      )
      .attr("y", (d) => yScale(d.count) - 4)
      .attr("text-anchor", "middle")
      .attr("font-size", "12px")
      .attr("fill", "currentColor")
      .text((d) => d.count);

    // X axis
    g.append("g")
      .attr("transform", `translate(0,${innerHeight})`)
      .call(d3.axisBottom(xScale).tickSize(0))
      .select(".domain")
      .remove();

    // Y axis
    g.append("g")
      .call(d3.axisLeft(yScale).ticks(5).tickSize(-innerWidth))
      .select(".domain")
      .remove();

    // Style tick lines
    g.selectAll(".tick line").attr("stroke", "#e5e7eb").attr("stroke-dasharray", "2,2");

    return () => {
      svg.selectAll("*").remove();
    };
  }, [data, width, hasData, t]);

  return (
    <Card data-testid="priority-chart">
      <CardHeader>
        <CardTitle>{t("charts.priority.title")}</CardTitle>
        <CardDescription>{t("charts.priority.description")}</CardDescription>
      </CardHeader>
      <CardContent>
        {!hasData ? (
          <p className="text-center text-muted-foreground py-8">
            {t("charts.priority.noData")}
          </p>
        ) : (
          <div ref={containerRef} className="relative w-full">
            <svg
              ref={svgRef}
              role="img"
              aria-labelledby="priority-chart-title priority-chart-desc"
              className="w-full"
              style={{ height: CHART_HEIGHT }}
            />
            {tooltip && (
              <div
                className="absolute bg-popover text-popover-foreground border rounded px-2 py-1 text-xs shadow-md pointer-events-none z-10"
                style={{ left: tooltip.x, top: tooltip.y - 24 }}
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
