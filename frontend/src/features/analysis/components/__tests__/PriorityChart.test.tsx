import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { PriorityChart } from "../PriorityChart";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

vi.mock("../../hooks/useContainerDimensions", () => ({
  useContainerDimensions: () => ({ width: 800, height: 400 }),
}));

const sampleData = [
  { priority: 1, count: 5 },
  { priority: 2, count: 12 },
  { priority: 3, count: 8 },
  { priority: 4, count: 3 },
];

describe("PriorityChart", () => {
  it("renders SVG element inside a Card", () => {
    const { container } = render(<PriorityChart data={sampleData} />);
    const card = screen.getByTestId("priority-chart");
    expect(card).toBeInTheDocument();
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
  });

  it("renders 4 bars for P1-P4", () => {
    const { container } = render(<PriorityChart data={sampleData} />);
    const rects = container.querySelectorAll("svg rect");
    expect(rects.length).toBe(4);
  });

  it("shows chart title from i18n", () => {
    render(<PriorityChart data={sampleData} />);
    expect(screen.getAllByText("charts.priority.title").length).toBeGreaterThan(0);
  });

  it("shows no data placeholder when all counts are zero", () => {
    const zeroData = [
      { priority: 1, count: 0 },
      { priority: 2, count: 0 },
      { priority: 3, count: 0 },
      { priority: 4, count: 0 },
    ];
    const { container } = render(<PriorityChart data={zeroData} />);
    expect(screen.getByText("charts.priority.noData")).toBeInTheDocument();
    // SVG should exist but not have bars rendered (guard skips rendering)
    const rects = container.querySelectorAll("svg rect");
    expect(rects.length).toBe(0);
  });

  it("renders with accessibility attributes", () => {
    const { container } = render(<PriorityChart data={sampleData} />);
    const svg = container.querySelector("svg");
    expect(svg?.getAttribute("role")).toBe("img");
    expect(svg?.getAttribute("aria-labelledby")).toContain(
      "priority-chart-title"
    );
  });
});
