import { describe, it, expect } from "vitest";
import { renderHook } from "@testing-library/react";
import { useChartData } from "../useChartData";
import { makeFinding } from "../../test-utils/factories";

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

describe("radarData", () => {
  it("returns empty array when no findings", () => {
    const { result } = renderHook(() => useChartData([]));
    expect(result.current.radarData).toEqual([]);
  });

  it("groups findings by framework with normalized percentages", () => {
    const findings = [
      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "addressed" }),
      makeFinding({ id: "f3", framework_id: "fw-a", finding_type: "gap" }),
      makeFinding({ id: "f4", framework_id: "fw-a", finding_type: "partially_addressed" }),
      makeFinding({ id: "f5", framework_id: "fw-b", finding_type: "not_applicable" }),
      makeFinding({ id: "f6", framework_id: "fw-b", finding_type: "addressed" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    const fwA = result.current.radarData.find((r) => r.frameworkId === "fw-a");
    expect(fwA).toBeDefined();
    expect(fwA!.values.addressed).toBe(50);
    expect(fwA!.values.gap).toBe(25);
    expect(fwA!.values.partial).toBe(25);
    expect(fwA!.values.notApplicable).toBe(0);

    const fwB = result.current.radarData.find((r) => r.frameworkId === "fw-b");
    expect(fwB).toBeDefined();
    expect(fwB!.values.addressed).toBe(50);
    expect(fwB!.values.notApplicable).toBe(50);
  });

  it("percentages sum to 100 per framework", () => {
    const findings = [
      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "gap" }),
      makeFinding({ id: "f3", framework_id: "fw-a", finding_type: "partially_addressed" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    const fwA = result.current.radarData[0];
    const sum = fwA.values.addressed + fwA.values.partial + fwA.values.gap + fwA.values.notApplicable;
    expect(sum).toBeCloseTo(100, 1);
  });

  it("handles single framework with all one type", () => {
    const findings = [
      makeFinding({ id: "f1", framework_id: "fw-x", finding_type: "gap" }),
      makeFinding({ id: "f2", framework_id: "fw-x", finding_type: "gap" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    const fw = result.current.radarData[0];
    expect(fw.values.gap).toBe(100);
    expect(fw.values.addressed).toBe(0);
    expect(fw.values.partial).toBe(0);
    expect(fw.values.notApplicable).toBe(0);
  });

  it("includes total raw count per framework", () => {
    const findings = [
      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "gap" }),
      makeFinding({ id: "f3", framework_id: "fw-b", finding_type: "gap" }),
    ];
    const { result } = renderHook(() => useChartData(findings));
    expect(result.current.radarData.find((r) => r.frameworkId === "fw-a")!.total).toBe(2);
    expect(result.current.radarData.find((r) => r.frameworkId === "fw-b")!.total).toBe(1);
  });
});
