import { describe, it, expect } from "vitest";
import { renderHook } from "@testing-library/react";
import { useChartData } from "../useChartData";
import type { AnalysisFinding } from "../../types";

function makeFinding(overrides: Partial<AnalysisFinding> = {}): AnalysisFinding {
  return {
    id: "f1",
    concept_id: "c1",
    framework_id: "fw1",
    finding_type: "gap",
    confidence_score: 0.85,
    evidence_text: null,
    recommendation: null,
    priority: 1,
    sort_order: 1,
    concept_code: null,
    concept_name: null,
    concept_definition: null,
    ...overrides,
  };
}

describe("useChartData", () => {
  it("returns zero counts when findings array is empty", () => {
    const { result } = renderHook(() => useChartData([]));
    expect(result.current.typeCounts.total).toBe(0);
    expect(result.current.frameworkCoverage).toEqual([]);
    expect(result.current.priorityCounts).toEqual([]);
  });

  it("returns zero counts when findings is undefined", () => {
    const { result } = renderHook(() => useChartData(undefined));
    expect(result.current.typeCounts.total).toBe(0);
    expect(result.current.frameworkCoverage).toEqual([]);
    expect(result.current.priorityCounts).toEqual([]);
  });

  it("computes correct typeCounts", () => {
    const findings = [
      makeFinding({ id: "f1", finding_type: "addressed" }),
      makeFinding({ id: "f2", finding_type: "addressed" }),
      makeFinding({ id: "f3", finding_type: "gap" }),
      makeFinding({ id: "f4", finding_type: "partially_addressed" }),
      makeFinding({ id: "f5", finding_type: "not_applicable" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    expect(result.current.typeCounts).toEqual({
      addressed: 2,
      partiallyAddressed: 1,
      gap: 1,
      notApplicable: 1,
      total: 5,
    });
  });

  it("computes correct frameworkCoverage with percentage per framework", () => {
    const findings = [
      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "gap" }),
      makeFinding({ id: "f3", framework_id: "fw-b", finding_type: "addressed" }),
      makeFinding({ id: "f4", framework_id: "fw-b", finding_type: "addressed" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    expect(result.current.frameworkCoverage).toEqual([
      { frameworkId: "fw-a", total: 2, addressed: 1, percentage: 50 },
      { frameworkId: "fw-b", total: 2, addressed: 2, percentage: 100 },
    ]);
  });

  it("computes correct priorityCounts for P1-P4", () => {
    const findings = [
      makeFinding({ id: "f1", priority: 1 }),
      makeFinding({ id: "f2", priority: 1 }),
      makeFinding({ id: "f3", priority: 2 }),
      makeFinding({ id: "f4", priority: 3 }),
      makeFinding({ id: "f5", priority: 4 }),
      makeFinding({ id: "f6", priority: 4 }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    expect(result.current.priorityCounts).toEqual([
      { priority: 1, count: 2 },
      { priority: 2, count: 1 },
      { priority: 3, count: 1 },
      { priority: 4, count: 2 },
    ]);
  });

  it("handles findings with mixed framework_ids correctly", () => {
    const findings = [
      makeFinding({ id: "f1", framework_id: "fw-a" }),
      makeFinding({ id: "f2", framework_id: "fw-b" }),
      makeFinding({ id: "f3", framework_id: "fw-c" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    expect(result.current.frameworkCoverage).toHaveLength(3);
  });

  it("frameworkCoverage percentage = addressed / total per framework x 100", () => {
    const findings = [
      makeFinding({ id: "f1", framework_id: "fw-x", finding_type: "addressed" }),
      makeFinding({ id: "f2", framework_id: "fw-x", finding_type: "addressed" }),
      makeFinding({ id: "f3", framework_id: "fw-x", finding_type: "gap" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    const fw = result.current.frameworkCoverage[0];
    expect(fw.percentage).toBeCloseTo(66.67, 1);
  });
});
