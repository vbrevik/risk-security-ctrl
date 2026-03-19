import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { FrameworkRadar } from "../FrameworkRadar";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}));

vi.mock("../../hooks/useContainerDimensions", () => ({
  useContainerDimensions: () => ({ width: 400, height: 400 }),
}));

vi.mock("../../utils/frameworkColors", () => ({
  getFrameworkColor: (_ids: string[], _id: string) => "#3366cc",
  buildFrameworkColorMap: (_ids: string[]) => (_id: string) => "#3366cc",
}));

const sampleData = [
  {
    frameworkId: "iso-31000",
    values: { addressed: 60, partial: 20, gap: 15, notApplicable: 5 },
    total: 20,
  },
  {
    frameworkId: "nist-csf",
    values: { addressed: 40, partial: 30, gap: 20, notApplicable: 10 },
    total: 15,
  },
];

const frameworkIds = ["iso-31000", "nist-csf"];

describe("FrameworkRadar", () => {
  it("renders SVG element with role='img' and aria-labelledby", () => {
    const { container } = render(
      <FrameworkRadar data={sampleData} frameworkIds={frameworkIds} />
    );
    const svg = container.querySelector("svg");
    expect(svg).not.toBeNull();
    expect(svg?.getAttribute("role")).toBe("img");
    expect(svg?.getAttribute("aria-labelledby")).toContain("framework-radar-title");
  });

  it("renders one path element per framework in data", () => {
    const { container } = render(
      <FrameworkRadar data={sampleData} frameworkIds={frameworkIds} />
    );
    const polygons = container.querySelectorAll(".radar-polygon");
    expect(polygons.length).toBe(2);
  });

  it("renders no paths when data is empty", () => {
    const { container } = render(
      <FrameworkRadar data={[]} frameworkIds={[]} />
    );
    const polygons = container.querySelectorAll(".radar-polygon");
    expect(polygons.length).toBe(0);
  });

  it("shows noData message when data array is empty", () => {
    render(<FrameworkRadar data={[]} frameworkIds={[]} />);
    expect(screen.getByText("charts.radar.noData")).toBeInTheDocument();
  });

  it("renders 4 axis labels (one per finding type)", () => {
    const { container } = render(
      <FrameworkRadar data={sampleData} frameworkIds={frameworkIds} />
    );
    const texts = container.querySelectorAll("svg text");
    const textContents = Array.from(texts).map((t) => t.textContent);
    expect(textContents).toContain("charts.radar.addressed");
    expect(textContents).toContain("charts.radar.partial");
    expect(textContents).toContain("charts.radar.gap");
    expect(textContents).toContain("charts.radar.notApplicable");
  });

  it("renders concentric grid circles", () => {
    const { container } = render(
      <FrameworkRadar data={sampleData} frameworkIds={frameworkIds} />
    );
    const circles = container.querySelectorAll("svg circle.radar-grid");
    expect(circles.length).toBeGreaterThanOrEqual(4);
  });

  it("legend section shows framework names matching data", () => {
    render(<FrameworkRadar data={sampleData} frameworkIds={frameworkIds} />);
    expect(screen.getByText("iso-31000")).toBeInTheDocument();
    expect(screen.getByText("nist-csf")).toBeInTheDocument();
  });

  it("limits rendering to 8 frameworks when more provided", () => {
    const bigData = Array.from({ length: 10 }, (_, i) => ({
      frameworkId: `fw-${i}`,
      values: { addressed: 50, partial: 20, gap: 20, notApplicable: 10 },
      total: 10 + i,
    }));
    const bigIds = bigData.map((d) => d.frameworkId);
    const { container } = render(
      <FrameworkRadar data={bigData} frameworkIds={bigIds} />
    );
    const polygons = container.querySelectorAll(".radar-polygon");
    expect(polygons.length).toBe(8);
  });

  it("when selectedFrameworkId is set, selected path has different styling", () => {
    const { container } = render(
      <FrameworkRadar
        data={sampleData}
        frameworkIds={frameworkIds}
        selectedFrameworkId="iso-31000"
      />
    );
    const polygons = container.querySelectorAll(".radar-polygon");
    // Selected polygon should have stroke-width 3
    const strokeWidths = Array.from(polygons).map((p) =>
      p.getAttribute("stroke-width")
    );
    expect(strokeWidths).toContain("3");
    expect(strokeWidths).toContain("2");
  });
});
