import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
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
const sampleFrameworkIds = sampleData.map(d => d.frameworkId);

describe("CoverageHeatmap", () => {
  it("renders SVG element inside a Card", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} />);
    const card = screen.getByTestId("coverage-heatmap");
    expect(card).toBeInTheDocument();
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
  });

  it("renders correct number of bars matching data length", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} />);
    const rects = container.querySelectorAll("svg rect");
    expect(rects.length).toBe(3);
  });

  it("shows chart title from i18n", () => {
    render(<CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} />);
    expect(screen.getAllByText("charts.coverage.title").length).toBeGreaterThan(0);
  });

  it("shows no data placeholder when data is empty", () => {
    const { container } = render(<CoverageHeatmap data={[]} frameworkIds={[]} />);
    expect(screen.getByText("charts.coverage.noData")).toBeInTheDocument();
    expect(container.querySelector("svg")).toBeNull();
  });

  it("renders with accessibility attributes", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} />);
    const svg = container.querySelector("svg");
    expect(svg?.getAttribute("role")).toBe("img");
    expect(svg?.getAttribute("aria-labelledby")).toContain(
      "coverage-heatmap-title"
    );
  });

  it("does not crash when data changes between renders", () => {
    const { container, rerender } = render(
      <CoverageHeatmap data={sampleData.slice(0, 2)} frameworkIds={sampleFrameworkIds} />
    );
    expect(container.querySelectorAll("svg rect").length).toBe(2);

    rerender(<CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} />);
    expect(container.querySelectorAll("svg rect").length).toBe(3);
  });

  it("fires onBarClick with correct frameworkId when bar is clicked", () => {
    const mockFn = vi.fn();
    const { container } = render(
      <CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} onBarClick={mockFn} />
    );
    const rects = container.querySelectorAll("svg rect");
    fireEvent.click(rects[0]);
    expect(mockFn).toHaveBeenCalledOnce();
    expect(mockFn).toHaveBeenCalledWith("iso-31000");
  });

  it("does not error when onBarClick is not provided", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} />);
    const rects = container.querySelectorAll("svg rect");
    expect(() => fireEvent.click(rects[0])).not.toThrow();
  });

  it("dims non-selected bars when selectedFrameworkId is set", () => {
    const { container } = render(
      <CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} selectedFrameworkId="iso-31000" />
    );
    const rects = container.querySelectorAll("svg rect");
    expect(rects[0].getAttribute("opacity")).toBe("1");
    expect(rects[1].getAttribute("opacity")).toBe("0.3");
    expect(rects[2].getAttribute("opacity")).toBe("0.3");
  });

  it("all bars have full opacity when selectedFrameworkId is null", () => {
    const { container } = render(
      <CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} selectedFrameworkId={null} />
    );
    const rects = container.querySelectorAll("svg rect");
    for (const rect of rects) {
      expect(rect.getAttribute("opacity")).toBe("1");
    }
  });

  it("bar rects have role='button' and tabindex='0'", () => {
    const { container } = render(
      <CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} onBarClick={vi.fn()} />
    );
    const rects = container.querySelectorAll("svg rect");
    for (const rect of rects) {
      expect(rect.getAttribute("role")).toBe("button");
      expect(rect.getAttribute("tabindex")).toBe("0");
    }
  });

  it("bar rects have aria-label with framework ID", () => {
    const { container } = render(<CoverageHeatmap data={sampleData} frameworkIds={sampleFrameworkIds} />);
    const rects = container.querySelectorAll("svg rect");
    expect(rects[0].getAttribute("aria-label")).toBe("iso-31000");
    expect(rects[1].getAttribute("aria-label")).toBe("nist-csf");
    expect(rects[2].getAttribute("aria-label")).toBe("iso-31010");
  });
});
