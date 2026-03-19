diff --git a/frontend/src/features/analysis/components/SummaryStats.tsx b/frontend/src/features/analysis/components/SummaryStats.tsx
index cc07771..3cb4b40 100644
--- a/frontend/src/features/analysis/components/SummaryStats.tsx
+++ b/frontend/src/features/analysis/components/SummaryStats.tsx
@@ -7,6 +7,7 @@ interface SummaryStatsProps {
   analysis: Analysis;
   chartData: ChartData;
   isLoading?: boolean;
+  overrideTypeCounts?: ChartData["typeCounts"];
 }
 
 function formatProcessingTime(ms: number | null): string {
@@ -24,23 +25,25 @@ function calcPercent(count: number, total: number): string {
   return `${Math.round((count / total) * 100)}%`;
 }
 
-export function SummaryStats({ analysis, chartData, isLoading }: SummaryStatsProps) {
+export function SummaryStats({ analysis, chartData, isLoading, overrideTypeCounts }: SummaryStatsProps) {
   const { t } = useTranslation("analysis");
 
+  const typeCounts = overrideTypeCounts ?? chartData.typeCounts;
+
   const cards = [
     {
       label: t("stats.totalFindings"),
-      value: String(chartData.typeCounts.total),
+      value: String(typeCounts.total),
     },
     {
       label: t("stats.addressed"),
-      value: String(chartData.typeCounts.addressed),
-      secondary: calcPercent(chartData.typeCounts.addressed, chartData.typeCounts.total),
+      value: String(typeCounts.addressed),
+      secondary: calcPercent(typeCounts.addressed, typeCounts.total),
     },
     {
       label: t("stats.gaps"),
-      value: String(chartData.typeCounts.gap),
-      secondary: calcPercent(chartData.typeCounts.gap, chartData.typeCounts.total),
+      value: String(typeCounts.gap),
+      secondary: calcPercent(typeCounts.gap, typeCounts.total),
     },
     {
       label: t("stats.frameworks"),
diff --git a/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx b/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx
index 3014732..c917ab9 100644
--- a/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx
+++ b/frontend/src/features/analysis/components/__tests__/SummaryStats.test.tsx
@@ -44,6 +44,7 @@ function makeChartData(overrides: Partial<ChartData> = {}): ChartData {
       notApplicable: 3,
       total: 58,
     },
+    radarData: [],
     ...overrides,
   };
 }
@@ -104,4 +105,62 @@ describe("SummaryStats", () => {
     const pulseElements = container.querySelectorAll(".animate-pulse");
     expect(pulseElements.length).toBe(6);
   });
+
+  it("when overrideTypeCounts is provided, finding-type cards show overridden values", () => {
+    const overrideCounts: ChartData["typeCounts"] = {
+      addressed: 10,
+      partiallyAddressed: 2,
+      gap: 3,
+      notApplicable: 1,
+      total: 16,
+    };
+    render(
+      <SummaryStats
+        analysis={makeAnalysis()}
+        chartData={makeChartData()}
+        overrideTypeCounts={overrideCounts}
+      />
+    );
+    // Overridden values should appear
+    expect(screen.getByText("16")).toBeInTheDocument();
+    expect(screen.getByText("10")).toBeInTheDocument();
+    expect(screen.getByText("3")).toBeInTheDocument();
+    // Original chartData values should NOT appear
+    expect(screen.queryByText("58")).not.toBeInTheDocument();
+    expect(screen.queryByText("42")).not.toBeInTheDocument();
+  });
+
+  it("when overrideTypeCounts is provided, framework/processing/token cards remain unchanged", () => {
+    const overrideCounts: ChartData["typeCounts"] = {
+      addressed: 10,
+      partiallyAddressed: 2,
+      gap: 3,
+      notApplicable: 1,
+      total: 16,
+    };
+    render(
+      <SummaryStats
+        analysis={makeAnalysis()}
+        chartData={makeChartData()}
+        overrideTypeCounts={overrideCounts}
+      />
+    );
+    // Analysis-level cards unchanged
+    expect(screen.getByText("2")).toBeInTheDocument(); // frameworks count
+    expect(screen.getByText("2.3s")).toBeInTheDocument(); // processing time
+    expect(screen.getByText("15,420")).toBeInTheDocument(); // token count
+  });
+
+  it("when overrideTypeCounts is not provided, behaves as before", () => {
+    render(
+      <SummaryStats
+        analysis={makeAnalysis()}
+        chartData={makeChartData()}
+      />
+    );
+    // Original chartData.typeCounts values appear
+    expect(screen.getByText("58")).toBeInTheDocument();
+    expect(screen.getByText("42")).toBeInTheDocument();
+    expect(screen.getByText("8")).toBeInTheDocument();
+  });
 });
diff --git a/frontend/src/routes/analysis/$id.tsx b/frontend/src/routes/analysis/$id.tsx
index 4f4d312..08855ea 100644
--- a/frontend/src/routes/analysis/$id.tsx
+++ b/frontend/src/routes/analysis/$id.tsx
@@ -1,13 +1,15 @@
-import { useState } from "react";
+import { useState, useCallback, useRef, useMemo } from "react";
 import { createFileRoute, Link } from "@tanstack/react-router";
 import { useTranslation } from "react-i18next";
-import { ArrowLeft } from "lucide-react";
+import { ArrowLeft, X } from "lucide-react";
 import { useAnalysis, useFindings } from "@/features/analysis/api";
 import { StatusBadge } from "@/features/analysis/components/StatusBadge";
 import { SummaryStats } from "@/features/analysis/components/SummaryStats";
 import { CoverageHeatmap } from "@/features/analysis/components/CoverageHeatmap";
 import { PriorityChart } from "@/features/analysis/components/PriorityChart";
+import { FrameworkRadar } from "@/features/analysis/components/FrameworkRadar";
 import { FindingsTable } from "@/features/analysis/components/FindingsTable";
+import { ConceptDrawer } from "@/features/analysis/components/ConceptDrawer";
 import { ExportButtons } from "@/features/analysis/components/ExportButtons";
 import { EmptyFindings } from "@/features/analysis/components/EmptyFindings";
 import { useChartData } from "@/features/analysis/hooks/useChartData";
@@ -41,6 +43,8 @@ function AnalysisDetailPage() {
     priority?: number;
   }>({});
   const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());
+  const [selectedConceptId, setSelectedConceptId] = useState<string | null>(null);
+  const findingsRef = useRef<HTMLDivElement>(null);
 
   const { data: paginatedFindings } = useFindings(id, {
     page,
@@ -48,6 +52,28 @@ function AnalysisDetailPage() {
     ...filters,
   });
 
+  const handleBarClick = useCallback((frameworkId: string) => {
+    setFilters(prev => ({
+      ...prev,
+      framework_id: prev.framework_id === frameworkId ? undefined : frameworkId,
+    }));
+    setPage(1);
+    findingsRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
+  }, []);
+
+  const filteredTypeCounts = useMemo(() => {
+    if (!filters.framework_id || !allFindingsData?.data) return undefined;
+    const filtered = allFindingsData.data.filter(
+      f => f.framework_id === filters.framework_id
+    );
+    const addressed = filtered.filter(f => f.finding_type === "addressed").length;
+    const partiallyAddressed = filtered.filter(f => f.finding_type === "partially_addressed").length;
+    const gap = filtered.filter(f => f.finding_type === "gap").length;
+    const notApplicable = filtered.filter(f => f.finding_type === "not_applicable").length;
+    const total = filtered.length;
+    return { addressed, partiallyAddressed, gap, notApplicable, total };
+  }, [filters.framework_id, allFindingsData?.data]);
+
   function handleFilterChange(newFilters: typeof filters) {
     setFilters(newFilters);
     setPage(1);
@@ -195,16 +221,41 @@ function AnalysisDetailPage() {
             analysis={analysis}
             chartData={chartData}
             isLoading={isChartDataLoading}
+            overrideTypeCounts={filteredTypeCounts}
           />
 
+          {/* Filter Banner */}
+          {filters.framework_id && (
+            <div className="flex items-center gap-2 bg-muted rounded px-3 py-1 text-sm">
+              <span>{t("detail.filteredBy", { framework: filters.framework_id })}</span>
+              <button
+                onClick={() => setFilters(prev => ({ ...prev, framework_id: undefined }))}
+                className="ml-auto hover:bg-accent rounded p-0.5"
+                aria-label={t("detail.clearFilter")}
+              >
+                <X className="h-3 w-3" />
+              </button>
+            </div>
+          )}
+
           {/* Charts */}
-          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
-            <CoverageHeatmap data={chartData.frameworkCoverage} />
+          <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
+            <CoverageHeatmap
+              data={chartData.frameworkCoverage}
+              onBarClick={handleBarClick}
+              selectedFrameworkId={filters.framework_id}
+              frameworkIds={analysis.matched_framework_ids}
+            />
+            <FrameworkRadar
+              data={chartData.radarData}
+              selectedFrameworkId={filters.framework_id}
+              frameworkIds={analysis.matched_framework_ids}
+            />
             <PriorityChart data={chartData.priorityCounts} />
           </div>
 
           {/* Findings Table */}
-          <div>
+          <div ref={findingsRef}>
             <h2 className="text-lg font-semibold mb-4">
               {t("findings.title")}
             </h2>
@@ -218,8 +269,15 @@ function AnalysisDetailPage() {
               page={page}
               totalPages={paginatedFindings?.total_pages ?? 1}
               onPageChange={setPage}
+              onConceptClick={(conceptId) => setSelectedConceptId(conceptId)}
             />
           </div>
+
+          {/* Concept Drawer */}
+          <ConceptDrawer
+            conceptId={selectedConceptId}
+            onClose={() => setSelectedConceptId(null)}
+          />
         </div>
       )}
     </div>
