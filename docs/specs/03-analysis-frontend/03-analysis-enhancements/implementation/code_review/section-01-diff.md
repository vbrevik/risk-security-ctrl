diff --git a/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts b/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts
index cc00aad..86b79cc 100644
--- a/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts
+++ b/frontend/src/features/analysis/hooks/__tests__/useChartData.test.ts
@@ -96,6 +96,11 @@ describe("useChartData", () => {
     expect(result.current.frameworkCoverage).toHaveLength(3);
   });
 
+  it("returns empty radarData when no findings", () => {
+    const { result } = renderHook(() => useChartData([]));
+    expect(result.current.radarData).toEqual([]);
+  });
+
   it("frameworkCoverage percentage = addressed / total per framework x 100", () => {
     const findings = [
       makeFinding({ id: "f1", framework_id: "fw-x", finding_type: "addressed" }),
@@ -107,3 +112,64 @@ describe("useChartData", () => {
     expect(fw.percentage).toBeCloseTo(66.67, 1);
   });
 });
+
+describe("radarData", () => {
+  it("groups findings by framework with normalized percentages", () => {
+    const findings = [
+      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
+      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "addressed" }),
+      makeFinding({ id: "f3", framework_id: "fw-a", finding_type: "gap" }),
+      makeFinding({ id: "f4", framework_id: "fw-a", finding_type: "partially_addressed" }),
+      makeFinding({ id: "f5", framework_id: "fw-b", finding_type: "not_applicable" }),
+      makeFinding({ id: "f6", framework_id: "fw-b", finding_type: "addressed" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    const fwA = result.current.radarData.find((r) => r.frameworkId === "fw-a");
+    expect(fwA).toBeDefined();
+    expect(fwA!.values.addressed).toBe(50);
+    expect(fwA!.values.gap).toBe(25);
+    expect(fwA!.values.partial).toBe(25);
+    expect(fwA!.values.notApplicable).toBe(0);
+
+    const fwB = result.current.radarData.find((r) => r.frameworkId === "fw-b");
+    expect(fwB).toBeDefined();
+    expect(fwB!.values.addressed).toBe(50);
+    expect(fwB!.values.notApplicable).toBe(50);
+  });
+
+  it("percentages sum to 100 per framework", () => {
+    const findings = [
+      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
+      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "gap" }),
+      makeFinding({ id: "f3", framework_id: "fw-a", finding_type: "partially_addressed" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    const fwA = result.current.radarData[0];
+    const sum = fwA.values.addressed + fwA.values.partial + fwA.values.gap + fwA.values.notApplicable;
+    expect(sum).toBeCloseTo(100, 1);
+  });
+
+  it("handles single framework with all one type", () => {
+    const findings = [
+      makeFinding({ id: "f1", framework_id: "fw-x", finding_type: "gap" }),
+      makeFinding({ id: "f2", framework_id: "fw-x", finding_type: "gap" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    const fw = result.current.radarData[0];
+    expect(fw.values.gap).toBe(100);
+    expect(fw.values.addressed).toBe(0);
+    expect(fw.values.partial).toBe(0);
+    expect(fw.values.notApplicable).toBe(0);
+  });
+
+  it("includes total raw count per framework", () => {
+    const findings = [
+      makeFinding({ id: "f1", framework_id: "fw-a", finding_type: "addressed" }),
+      makeFinding({ id: "f2", framework_id: "fw-a", finding_type: "gap" }),
+      makeFinding({ id: "f3", framework_id: "fw-b", finding_type: "gap" }),
+    ];
+    const { result } = renderHook(() => useChartData(findings));
+    expect(result.current.radarData.find((r) => r.frameworkId === "fw-a")!.total).toBe(2);
+    expect(result.current.radarData.find((r) => r.frameworkId === "fw-b")!.total).toBe(1);
+  });
+});
diff --git a/frontend/src/features/analysis/hooks/useChartData.ts b/frontend/src/features/analysis/hooks/useChartData.ts
index dcfb97c..2c8a21a 100644
--- a/frontend/src/features/analysis/hooks/useChartData.ts
+++ b/frontend/src/features/analysis/hooks/useChartData.ts
@@ -19,6 +19,11 @@ export interface ChartData {
     notApplicable: number;
     total: number;
   };
+  radarData: Array<{
+    frameworkId: string;
+    values: { addressed: number; partial: number; gap: number; notApplicable: number };
+    total: number;
+  }>;
 }
 
 const EMPTY_TYPE_COUNTS: ChartData["typeCounts"] = {
@@ -33,6 +38,7 @@ const EMPTY_CHART_DATA: ChartData = {
   frameworkCoverage: [],
   priorityCounts: [],
   typeCounts: EMPTY_TYPE_COUNTS,
+  radarData: [],
 };
 
 export function useChartData(
@@ -99,6 +105,32 @@ export function useChartData(
       .map(([priority, count]) => ({ priority, count }))
       .sort((a, b) => a.priority - b.priority);
 
-    return { frameworkCoverage, priorityCounts, typeCounts };
+    // Radar data: normalized percentages per framework by finding type
+    const radarMap = new Map<string, { addressed: number; partial: number; gap: number; notApplicable: number; total: number }>();
+    for (const f of findings) {
+      const entry = radarMap.get(f.framework_id) ?? { addressed: 0, partial: 0, gap: 0, notApplicable: 0, total: 0 };
+      entry.total++;
+      switch (f.finding_type) {
+        case "addressed": entry.addressed++; break;
+        case "partially_addressed": entry.partial++; break;
+        case "gap": entry.gap++; break;
+        case "not_applicable": entry.notApplicable++; break;
+      }
+      radarMap.set(f.framework_id, entry);
+    }
+    const radarData = Array.from(radarMap.entries())
+      .map(([frameworkId, counts]) => ({
+        frameworkId,
+        values: {
+          addressed: (counts.addressed / counts.total) * 100,
+          partial: (counts.partial / counts.total) * 100,
+          gap: (counts.gap / counts.total) * 100,
+          notApplicable: (counts.notApplicable / counts.total) * 100,
+        },
+        total: counts.total,
+      }))
+      .sort((a, b) => a.frameworkId.localeCompare(b.frameworkId));
+
+    return { frameworkCoverage, priorityCounts, typeCounts, radarData };
   }, [findings]);
 }
diff --git a/frontend/src/features/analysis/index.ts b/frontend/src/features/analysis/index.ts
index 8bd57dc..2c7a7a0 100644
--- a/frontend/src/features/analysis/index.ts
+++ b/frontend/src/features/analysis/index.ts
@@ -17,3 +17,4 @@ export { FindingTypeTag } from "./components/FindingTypeTag";
 export { FindingsTable } from "./components/FindingsTable";
 export { ExportButtons } from "./components/ExportButtons";
 export { EmptyFindings } from "./components/EmptyFindings";
+export { getFrameworkColor } from "./utils/frameworkColors";
diff --git a/frontend/src/features/analysis/utils/__tests__/frameworkColors.test.ts b/frontend/src/features/analysis/utils/__tests__/frameworkColors.test.ts
new file mode 100644
index 0000000..94cd913
--- /dev/null
+++ b/frontend/src/features/analysis/utils/__tests__/frameworkColors.test.ts
@@ -0,0 +1,41 @@
+import { describe, it, expect } from "vitest";
+import { getFrameworkColor } from "../frameworkColors";
+
+describe("getFrameworkColor", () => {
+  it("returns a hex color string for a known framework ID", () => {
+    const color = getFrameworkColor(["fw-a", "fw-b"], "fw-a");
+    expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
+  });
+
+  it("same framework ID always gets same color given same frameworkIds array", () => {
+    const ids = ["fw-a", "fw-b", "fw-c"];
+    const color1 = getFrameworkColor(ids, "fw-b");
+    const color2 = getFrameworkColor(ids, "fw-b");
+    expect(color1).toBe(color2);
+  });
+
+  it("different frameworks get different colors (up to 10)", () => {
+    const ids = Array.from({ length: 10 }, (_, i) => `fw-${String(i).padStart(2, "0")}`);
+    const colors = ids.map((id) => getFrameworkColor(ids, id));
+    expect(new Set(colors).size).toBe(10);
+  });
+
+  it("wraps around after 10 frameworks (mod 10 behavior)", () => {
+    const ids = Array.from({ length: 11 }, (_, i) => `fw-${String(i).padStart(2, "0")}`);
+    // After sorting alphabetically, fw-00 is index 0 and fw-10 is index 10
+    const color0 = getFrameworkColor(ids, "fw-00");
+    const color10 = getFrameworkColor(ids, "fw-10");
+    expect(color10).toBe(color0);
+  });
+
+  it("order is deterministic (sorts IDs alphabetically before indexing)", () => {
+    const color1 = getFrameworkColor(["fw-b", "fw-a"], "fw-a");
+    const color2 = getFrameworkColor(["fw-a", "fw-b"], "fw-a");
+    expect(color1).toBe(color2);
+  });
+
+  it("falls back to index 0 color when frameworkId not found", () => {
+    const color = getFrameworkColor(["fw-a", "fw-b"], "fw-unknown");
+    expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
+  });
+});
diff --git a/frontend/src/features/analysis/utils/frameworkColors.ts b/frontend/src/features/analysis/utils/frameworkColors.ts
new file mode 100644
index 0000000..0c25292
--- /dev/null
+++ b/frontend/src/features/analysis/utils/frameworkColors.ts
@@ -0,0 +1,10 @@
+import * as d3 from "d3";
+
+export function getFrameworkColor(
+  frameworkIds: string[],
+  frameworkId: string
+): string {
+  const sorted = [...frameworkIds].sort();
+  const index = sorted.indexOf(frameworkId);
+  return d3.schemeTableau10[(index < 0 ? 0 : index) % 10];
+}
