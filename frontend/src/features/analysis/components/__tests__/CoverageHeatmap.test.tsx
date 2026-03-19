import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { CoverageHeatmap } from "../CoverageHeatmap";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

vi.mock("../../hooks/useContainerDimensions", () => ({
  useContainerDimensions: () => ({ width: 800, height: 400 }),
}));

const sampleData = [
  { frameworkId: "iso-31000", percentage: 75, addressed: 15, total: 20 },
  { frameworkId: "nist-csf", percentage: 50, addressed: 10, total: 20 },
  { frameworkId: "iso-31010", percentage: 90, addressed: 18, total: 20 },
];

describe("CoverageHeatmap", () => {
  it("renders SVG element inside a Card", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} />);
    const card = screen.getByTestId("coverage-heatmap");
    expect(card).toBeInTheDocument();
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
  });

  it("renders correct number of bars matching data length", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} />);
    const rects = container.querySelectorAll("svg rect");
    expect(rects.length).toBe(3);
  });

  it("shows chart title from i18n", () => {
    render(<CoverageHeatmap data={sampleData} />);
    expect(screen.getAllByText("charts.coverage.title").length).toBeGreaterThan(0);
  });

  it("shows no data placeholder when data is empty", () => {
    const { container } = render(<CoverageHeatmap data={[]} />);
    expect(screen.getByText("charts.coverage.noData")).toBeInTheDocument();
    expect(container.querySelector("svg")).toBeNull();
  });

  it("renders with accessibility attributes", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} />);
    const svg = container.querySelector("svg");
    expect(svg?.getAttribute("role")).toBe("img");
    expect(svg?.getAttribute("aria-labelledby")).toContain(
      "coverage-heatmap-title"
    );
  });

  it("does not crash when data changes between renders", () => {
    const { container, rerender } = render(
      <CoverageHeatmap data={sampleData.slice(0, 2)} />
    );
    expect(container.querySelectorAll("svg rect").length).toBe(2);

    rerender(<CoverageHeatmap data={sampleData} />);
    expect(container.querySelectorAll("svg rect").length).toBe(3);
  });
});
