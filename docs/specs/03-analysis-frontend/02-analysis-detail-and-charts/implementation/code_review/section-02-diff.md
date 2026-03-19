diff --git a/frontend/src/features/analysis/components/SummaryStats.tsx b/frontend/src/features/analysis/components/SummaryStats.tsx
new file mode 100644
index 0000000..cc07771
--- /dev/null
+++ b/frontend/src/features/analysis/components/SummaryStats.tsx
@@ -0,0 +1,87 @@
+import { useTranslation } from "react-i18next";
+import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
+import type { Analysis } from "../types";
+import type { ChartData } from "../hooks/useChartData";
+
+interface SummaryStatsProps {
+  analysis: Analysis;
+  chartData: ChartData;
+  isLoading?: boolean;
+}
+
+function formatProcessingTime(ms: number | null): string {
+  if (ms == null) return "\u2014";
+  return `${(ms / 1000).toFixed(1)}s`;
+}
+
+function formatTokenCount(count: number | null): string {
+  if (count == null) return "\u2014";
+  return new Intl.NumberFormat("en-US").format(count);
+}
+
+function calcPercent(count: number, total: number): string {
+  if (total === 0) return "0%";
+  return `${Math.round((count / total) * 100)}%`;
+}
+
+export function SummaryStats({ analysis, chartData, isLoading }: SummaryStatsProps) {
+  const { t } = useTranslation("analysis");
+
+  const cards = [
+    {
+      label: t("stats.totalFindings"),
+      value: String(chartData.typeCounts.total),
+    },
+    {
+      label: t("stats.addressed"),
+      value: String(chartData.typeCounts.addressed),
+      secondary: calcPercent(chartData.typeCounts.addressed, chartData.typeCounts.total),
+    },
+    {
+      label: t("stats.gaps"),
+      value: String(chartData.typeCounts.gap),
+      secondary: calcPercent(chartData.typeCounts.gap, chartData.typeCounts.total),
+    },
+    {
+      label: t("stats.frameworks"),
+      value: String(analysis.matched_framework_ids.length),
+    },
+    {
+      label: t("stats.processingTime"),
+      value: formatProcessingTime(analysis.processing_time_ms),
+    },
+    {
+      label: t("stats.tokenCount"),
+      value: formatTokenCount(analysis.token_count),
+    },
+  ];
+
+  return (
+    <div
+      className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4"
+      data-testid="summary-stats"
+    >
+      {cards.map((card) => (
+        <Card key={card.label} data-testid="stat-card">
+          <CardHeader className="pb-2">
+            <CardTitle className="text-sm font-medium text-muted-foreground">
+              {card.label}
+            </CardTitle>
+          </CardHeader>
+          <CardContent>
+            {isLoading ? (
+              <div className="animate-pulse bg-muted rounded h-8 w-20" />
+            ) : (
+              <>
+                <p className="text-2xl font-bold">{card.value}</p>
+                {card.secondary && (
+                  <p className="text-xs text-muted-foreground">{card.secondary}</p>
+                )}
+              </>
+            )}
+          </CardContent>
+        </Card>
+      ))}
+    </div>
+  );
+}
diff --git a/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx b/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx
new file mode 100644
index 0000000..49a531f
--- /dev/null
+++ b/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx
@@ -0,0 +1,107 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen } from "@testing-library/react";
+import { SummaryStats } from "../SummaryStats";
+import type { Analysis } from "../../types";
+import type { ChartData } from "../../hooks/useChartData";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => key,
+  }),
+}));
+
+function makeAnalysis(overrides: Partial<Analysis> = {}): Analysis {
+  return {
+    id: "a1",
+    name: "Test Analysis",
+    description: null,
+    input_type: "text",
+    input_text: null,
+    original_filename: null,
+    file_path: null,
+    extracted_text: null,
+    status: "completed",
+    error_message: null,
+    prompt_template: null,
+    matched_framework_ids: ["fw-a", "fw-b"],
+    processing_time_ms: 2300,
+    token_count: 15420,
+    created_by: null,
+    created_at: "2026-03-19T10:00:00Z",
+    updated_at: "2026-03-19T10:00:00Z",
+    ...overrides,
+  };
+}
+
+function makeChartData(overrides: Partial<ChartData> = {}): ChartData {
+  return {
+    frameworkCoverage: [],
+    priorityCounts: [],
+    typeCounts: {
+      addressed: 42,
+      partiallyAddressed: 5,
+      gap: 8,
+      notApplicable: 3,
+      total: 58,
+    },
+    ...overrides,
+  };
+}
+
+describe("SummaryStats", () => {
+  it("renders 6 stat cards", () => {
+    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
+    const cards = screen.getAllByTestId("stat-card");
+    expect(cards).toHaveLength(6);
+  });
+
+  it("displays total findings count", () => {
+    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
+    expect(screen.getByText("58")).toBeDefined();
+  });
+
+  it("displays addressed count with percentage", () => {
+    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
+    expect(screen.getByText("42")).toBeDefined();
+    expect(screen.getByText("72%")).toBeDefined();
+  });
+
+  it("displays gaps count with percentage", () => {
+    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
+    expect(screen.getByText("8")).toBeDefined();
+    expect(screen.getByText("14%")).toBeDefined();
+  });
+
+  it("displays frameworks count", () => {
+    render(<SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} />);
+    expect(screen.getByText("2")).toBeDefined();
+  });
+
+  it("displays formatted processing time", () => {
+    render(
+      <SummaryStats
+        analysis={makeAnalysis({ processing_time_ms: 2300 })}
+        chartData={makeChartData()}
+      />
+    );
+    expect(screen.getByText("2.3s")).toBeDefined();
+  });
+
+  it("displays formatted token count", () => {
+    render(
+      <SummaryStats
+        analysis={makeAnalysis({ token_count: 15420 })}
+        chartData={makeChartData()}
+      />
+    );
+    expect(screen.getByText("15,420")).toBeDefined();
+  });
+
+  it("renders skeleton state when isLoading is true", () => {
+    const { container } = render(
+      <SummaryStats analysis={makeAnalysis()} chartData={makeChartData()} isLoading />
+    );
+    const pulseElements = container.querySelectorAll(".animate-pulse");
+    expect(pulseElements.length).toBe(6);
+  });
+});
diff --git a/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts b/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts
new file mode 100644
index 0000000..cc00aad
--- /dev/null
+++ b/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts
@@ -0,0 +1,109 @@
+import { describe, it, expect } from "vitest";
+import { renderHook } from "@testing-library/react";
+import { useChartData } from "../useChartData";
+import type { AnalysisFinding } from "../../types";
+
+function makeFinding(overrides: Partial<AnalysisFinding> = {}): AnalysisFinding {
+  return {
+    id: "f1",
+    concept_id: "c1",
+    framework_id: "fw1",
+    finding_type: "gap",
+    confidence_score: 0.85,
+    evidence_text: null,
+    recommendation: null,
+    priority: 1,
+    sort_order: 1,
+    concept_code: null,
+    concept_name: null,
+    concept_definition: null,
+    ...overrides,
+  };
+}
+
+describe("useChartData", () => {
+  it("returns zero counts when findings array is empty", () => {
+    const { result } = renderHook(() => useChartData([]));
+    expect(result.current.typeCounts.total).toBe(0);
+    expect(result.current.frameworkCoverage).toEqual([]);
+    expect(result.current.priorityCounts).toEqual([]);
+  });
+
+  it("returns zero counts when findings is undefined", () => {
+    const { result } = renderHook(() => useChartData(undefined));
+    expect(result.current.typeCounts.total).toBe(0);
+    expect(result.current.frameworkCoverage).toEqual([]);
+    expect(result.current.priorityCounts).toEqual([]);
+  });
+
+  it("computes correct typeCounts", () => {
+    const findings = [
+      makeFinding({ id: "f1", finding_type: "addressed" }),
+      makeFinding({ id: "f2", finding_type: "addressed" }),
+      makeFinding({ id: "f3", finding_type: "gap" }),
+      makeFinding({ id: "f4", finding_type: "partially_addressed" }),
+      makeFinding({ id: "f5", finding_type: "not_applicable" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    expect(result.current.typeCounts).toEqual({
+      addressed: 2,
+      partiallyAddressed: 1,
+      gap: 1,
+      notApplicable: 1,
+      total: 5,
+    });
+  });
+
+  it("computes correct frameworkCoverage with percentage per framework", () => {
+    const findings = [
+      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
+      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "gap" }),
+      makeFinding({ id: "f3", framework_id: "fw-b", finding_type: "addressed" }),
+      makeFinding({ id: "f4", framework_id: "fw-b", finding_type: "addressed" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    expect(result.current.frameworkCoverage).toEqual([
+      { frameworkId: "fw-a", total: 2, addressed: 1, percentage: 50 },
+      { frameworkId: "fw-b", total: 2, addressed: 2, percentage: 100 },
+    ]);
+  });
+
+  it("computes correct priorityCounts for P1-P4", () => {
+    const findings = [
+      makeFinding({ id: "f1", priority: 1 }),
+      makeFinding({ id: "f2", priority: 1 }),
+      makeFinding({ id: "f3", priority: 2 }),
+      makeFinding({ id: "f4", priority: 3 }),
+      makeFinding({ id: "f5", priority: 4 }),
+      makeFinding({ id: "f6", priority: 4 }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    expect(result.current.priorityCounts).toEqual([
+      { priority: 1, count: 2 },
+      { priority: 2, count: 1 },
+      { priority: 3, count: 1 },
+      { priority: 4, count: 2 },
+    ]);
+  });
+
+  it("handles findings with mixed framework_ids correctly", () => {
+    const findings = [
+      makeFinding({ id: "f1", framework_id: "fw-a" }),
+      makeFinding({ id: "f2", framework_id: "fw-b" }),
+      makeFinding({ id: "f3", framework_id: "fw-c" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    expect(result.current.frameworkCoverage).toHaveLength(3);
+  });
+
+  it("frameworkCoverage percentage = addressed / total per framework x 100", () => {
+    const findings = [
+      makeFinding({ id: "f1", framework_id: "fw-x", finding_type: "addressed" }),
+      makeFinding({ id: "f2", framework_id: "fw-x", finding_type: "addressed" }),
+      makeFinding({ id: "f3", framework_id: "fw-x", finding_type: "gap" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    const fw = result.current.frameworkCoverage[0];
+    expect(fw.percentage).toBeCloseTo(66.67, 1);
+  });
+});
diff --git a/frontend/src/features/analysis/hooks/useChartData.ts b/frontend/src/features/analysis/hooks/useChartData.ts
new file mode 100644
index 0000000..3ba6512
--- /dev/null
+++ b/frontend/src/features/analysis/hooks/useChartData.ts
@@ -0,0 +1,102 @@
+import { useMemo } from "react";
+import type { AnalysisFinding } from "../types";
+
+export interface ChartData {
+  frameworkCoverage: Array<{
+    frameworkId: string;
+    total: number;
+    addressed: number;
+    percentage: number;
+  }>;
+  priorityCounts: Array<{
+    priority: number;
+    count: number;
+  }>;
+  typeCounts: {
+    addressed: number;
+    partiallyAddressed: number;
+    gap: number;
+    notApplicable: number;
+    total: number;
+  };
+}
+
+const EMPTY_CHART_DATA: ChartData = {
+  frameworkCoverage: [],
+  priorityCounts: [],
+  typeCounts: {
+    addressed: 0,
+    partiallyAddressed: 0,
+    gap: 0,
+    notApplicable: 0,
+    total: 0,
+  },
+};
+
+export function useChartData(
+  findings: AnalysisFinding[] | undefined
+): ChartData {
+  return useMemo(() => {
+    if (!findings || findings.length === 0) {
+      return EMPTY_CHART_DATA;
+    }
+
+    // Type counts
+    const typeCounts = {
+      addressed: 0,
+      partiallyAddressed: 0,
+      gap: 0,
+      notApplicable: 0,
+      total: findings.length,
+    };
+    for (const f of findings) {
+      switch (f.finding_type) {
+        case "addressed":
+          typeCounts.addressed++;
+          break;
+        case "partially_addressed":
+          typeCounts.partiallyAddressed++;
+          break;
+        case "gap":
+          typeCounts.gap++;
+          break;
+        case "not_applicable":
+          typeCounts.notApplicable++;
+          break;
+      }
+    }
+
+    // Framework coverage
+    const fwMap = new Map<
+      string,
+      { total: number; addressed: number }
+    >();
+    for (const f of findings) {
+      const entry = fwMap.get(f.framework_id) ?? { total: 0, addressed: 0 };
+      entry.total++;
+      if (f.finding_type === "addressed") {
+        entry.addressed++;
+      }
+      fwMap.set(f.framework_id, entry);
+    }
+    const frameworkCoverage = Array.from(fwMap.entries())
+      .map(([frameworkId, { total, addressed }]) => ({
+        frameworkId,
+        total,
+        addressed,
+        percentage: (addressed / total) * 100,
+      }))
+      .sort((a, b) => a.frameworkId.localeCompare(b.frameworkId));
+
+    // Priority counts
+    const prioMap = new Map<number, number>();
+    for (const f of findings) {
+      prioMap.set(f.priority, (prioMap.get(f.priority) ?? 0) + 1);
+    }
+    const priorityCounts = Array.from(prioMap.entries())
+      .map(([priority, count]) => ({ priority, count }))
+      .sort((a, b) => a.priority - b.priority);
+
+    return { frameworkCoverage, priorityCounts, typeCounts };
+  }, [findings]);
+}
diff --git a/frontend/src/features/analysis/index.ts b/frontend/src/features/analysis/index.ts
index 5f61412..4eb631d 100644
--- a/frontend/src/features/analysis/index.ts
+++ b/frontend/src/features/analysis/index.ts
@@ -7,3 +7,6 @@ export { CreateAnalysisForm } from "./components/CreateAnalysisForm";
 export { FileDropZone } from "./components/FileDropZone";
 export { SettingsForm } from "./components/SettingsForm";
 export { BoostTermsEditor } from "./components/BoostTermsEditor";
+export { useChartData } from "./hooks/useChartData";
+export type { ChartData } from "./hooks/useChartData";
+export { SummaryStats } from "./components/SummaryStats";
