import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { SummaryStats } from "../SummaryStats";
import type { Analysis } from "../../types";
import type { ChartData } from "../../hooks/useChartData";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

function makeAnalysis(overrides: Partial<Analysis> = {}): Analysis {
  return {
    id: "a1",
    name: "Test Analysis",
    description: null,
    input_type: "text",
    input_text: null,
    original_filename: null,
    file_path: null,
    extracted_text: null,
    status: "completed",
    error_message: null,
    prompt_template: null,
    matched_framework_ids: ["fw-a", "fw-b"],
    processing_time_ms: 2300,
    token_count: 15420,
    created_by: null,
    created_at: "2026-03-19T10:00:00Z",
    updated_at: "2026-03-19T10:00:00Z",
    ...overrides,
  };
}

function makeChartData(overrides: Partial<ChartData> = {}): ChartData {
  return {
    frameworkCoverage: [],
    priorityCounts: [],
    typeCounts: {
      addressed: 42,
      partiallyAddressed: 5,
      gap: 8,
      notApplicable: 3,
      total: 58,
    },
    ...overrides,
  };
}

describe("SummaryStats", () => {
  it("renders 6 stat cards", () => {
    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
    const cards = screen.getAllByTestId("stat-card");
    expect(cards).toHaveLength(6);
  });

  it("displays total findings count", () => {
    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
    expect(screen.getByText("58")).toBeInTheDocument();
  });

  it("displays addressed count with percentage", () => {
    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
    expect(screen.getByText("42")).toBeInTheDocument();
    expect(screen.getByText("72%")).toBeInTheDocument();
  });

  it("displays gaps count with percentage", () => {
    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
    expect(screen.getByText("8")).toBeInTheDocument();
    expect(screen.getByText("14%")).toBeInTheDocument();
  });

  it("displays frameworks count", () => {
    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
    expect(screen.getByText("2")).toBeInTheDocument();
  });

  it("displays formatted processing time", () => {
    render(
      <SummaryStats
        analysis={makeAnalysis({ processing_time_ms: 2300 })}
        chartData={makeChartData()}
      />
    );
    expect(screen.getByText("2.3s")).toBeInTheDocument();
  });

  it("displays formatted token count", () => {
    render(
      <SummaryStats
        analysis={makeAnalysis({ token_count: 15420 })}
        chartData={makeChartData()}
      />
    );
    expect(screen.getByText("15,420")).toBeInTheDocument();
  });

  it("renders skeleton state when isLoading is true", () => {
    const { container } = render(
      <SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} isLoading />
    );
    const pulseElements = container.querySelectorAll(".animate-pulse");
    expect(pulseElements.length).toBe(6);
  });
});
