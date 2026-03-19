import { useState, useCallback, useRef, useMemo } from "react";
import { createFileRoute, Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { ArrowLeft, X } from "lucide-react";
import { useAnalysis, useFindings } from "@/features/analysis/api";
import { StatusBadge } from "@/features/analysis/components/StatusBadge";
import { SummaryStats } from "@/features/analysis/components/SummaryStats";
import { CoverageHeatmap } from "@/features/analysis/components/CoverageHeatmap";
import { PriorityChart } from "@/features/analysis/components/PriorityChart";
import { FrameworkRadar } from "@/features/analysis/components/FrameworkRadar";
import { FindingsTable } from "@/features/analysis/components/FindingsTable";
import { ConceptDrawer } from "@/features/analysis/components/ConceptDrawer";
import { ExportButtons } from "@/features/analysis/components/ExportButtons";
import { EmptyFindings } from "@/features/analysis/components/EmptyFindings";
import { useChartData } from "@/features/analysis/hooks/useChartData";
import type { FindingsFilter } from "@/features/analysis/types";

export const Route = createFileRoute("/_authenticated/analysis/$id")({
  component: AnalysisDetailPage,
});

function AnalysisDetailPage() {
  const { id } = Route.useParams();
  const { t } = useTranslation("analysis");
  const { data: analysis, isLoading, isError, error } = useAnalysis(id);

  const isCompleted = analysis?.status === "completed";

  // All findings for chart/stat aggregation
  const { data: allFindingsData, isLoading: isChartDataLoading } = useFindings(
    id,
    { limit: 1000 },
  );
  const chartData = useChartData(
    isCompleted ? allFindingsData?.data : undefined
  );

  // Paginated findings for table
  const [page, setPage] = useState(1);
  const [filters, setFilters] = useState<FindingsFilter>({});
  const [expandedIds, setExpandedIds] = useState<Set<string>>(new Set());
  const [selectedConceptId, setSelectedConceptId] = useState<string | null>(null);
  const findingsRef = useRef<HTMLDivElement>(null);

  const { data: paginatedFindings } = useFindings(id, {
    page,
    limit: 20,
    ...filters,
  });

  const handleBarClick = useCallback((frameworkId: string) => {
    setFilters(prev => ({
      ...prev,
      framework_id: prev.framework_id === frameworkId ? undefined : frameworkId,
    }));
    setPage(1);
    findingsRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
  }, []);

  const filteredFindings = useMemo(() => {
    if (!filters.framework_id || !allFindingsData?.data) return undefined;
    return allFindingsData.data.filter(f => f.framework_id === filters.framework_id);
  }, [filters.framework_id, allFindingsData?.data]);
  const filteredChartData = useChartData(filteredFindings);

  function handleFilterChange(newFilters: typeof filters) {
    setFilters(newFilters);
    setPage(1);
  }

  function handleToggleExpand(findingId: string) {
    setExpandedIds((prev) => {
      const next = new Set(prev);
      if (next.has(findingId)) next.delete(findingId);
      else next.add(findingId);
      return next;
    });
  }

  // Loading state
  if (isLoading) {
    return (
      <div className="max-w-7xl mx-auto p-6 space-y-6">
        <div className="animate-pulse space-y-4">
          <div className="h-8 bg-muted rounded w-1/3" />
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
            {Array.from({ length: 6 }).map((_, i) => (
              <div key={i} className="h-24 bg-muted rounded" />
            ))}
          </div>
          <div className="h-64 bg-muted rounded" />
        </div>
      </div>
    );
  }

  // Error state
  if (isError) {
    const is404 =
      error &&
      "status" in error &&
      (error as { status: number }).status === 404;
    return (
      <div className="max-w-7xl mx-auto p-6 space-y-4">
        <Link
          to="/analysis"
          search={{ page: 1, status: undefined }}
          className="inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground"
        >
          <ArrowLeft className="h-4 w-4" />
          {t("detail.backToList")}
        </Link>
        <h1 className="text-2xl font-bold">
          {is404 ? t("detail.notFound.title") : t("common.error")}
        </h1>
        <p className="text-muted-foreground">
          {is404
            ? t("detail.notFound.message")
            : error?.message || t("common.error")}
        </p>
      </div>
    );
  }

  if (!analysis) return null;

  const isProcessing = analysis.status === "processing";
  const isFailed = analysis.status === "failed";
  const hasFindings =
    isCompleted && (allFindingsData?.total ?? 0) > 0;
  const hasZeroFindings =
    isCompleted && !isChartDataLoading && (allFindingsData?.total ?? 0) === 0;

  return (
    <div className="max-w-7xl mx-auto p-6 space-y-6">
      {/* Page Header */}
      <div className="space-y-2">
        <Link
          to="/analysis"
          search={{ page: 1, status: undefined }}
          className="inline-flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground"
        >
          <ArrowLeft className="h-4 w-4" />
          {t("detail.backToList")}
        </Link>
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-3">
            <h1 className="text-2xl font-bold">{analysis.name}</h1>
            <StatusBadge status={analysis.status} />
          </div>
          <ExportButtons
            analysisId={id}
            analysisName={analysis.name}
            status={analysis.status}
          />
        </div>
        <p className="text-sm text-muted-foreground">
          {t("detail.inputType", { type: analysis.input_type })}
          {" · "}
          {t("detail.createdAt", {
            date: new Date(analysis.created_at).toLocaleDateString(),
          })}
        </p>
      </div>

      {/* Processing Banner */}
      {isProcessing && (
        <div className="rounded-lg border border-blue-200 bg-blue-50 p-4 dark:border-blue-800 dark:bg-blue-950">
          <h3 className="font-semibold text-blue-900 dark:text-blue-100">
            {t("detail.processing.banner")}
          </h3>
          <p className="text-sm text-blue-700 dark:text-blue-300 mt-1">
            {t("detail.processing.message")}
          </p>
        </div>
      )}

      {/* Failed State */}
      {isFailed && (
        <div className="rounded-lg border border-red-200 bg-red-50 p-4 dark:border-red-800 dark:bg-red-950">
          <p className="text-sm text-red-700 dark:text-red-300">
            {analysis.error_message || t("detail.failed.message")}
          </p>
        </div>
      )}

      {/* Completed: Chart data loading */}
      {isCompleted && isChartDataLoading && (
        <div className="animate-pulse space-y-4">
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
            {Array.from({ length: 6 }).map((_, i) => (
              <div key={i} className="h-24 bg-muted rounded" />
            ))}
          </div>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="h-64 bg-muted rounded" />
            <div className="h-64 bg-muted rounded" />
          </div>
        </div>
      )}

      {/* Completed: Zero findings */}
      {hasZeroFindings && <EmptyFindings />}

      {/* Completed: Has findings */}
      {hasFindings && (
        <div className="space-y-6">
          {/* Summary Statistics */}
          <SummaryStats
            analysis={analysis}
            chartData={chartData}
            isLoading={isChartDataLoading}
            overrideTypeCounts={filteredFindings ? filteredChartData.typeCounts : undefined}
          />

          {/* Filter Banner */}
          {filters.framework_id && (
            <div className="flex items-center gap-2 bg-muted rounded px-3 py-1 text-sm">
              <span>{t("detail.filteredBy", { framework: filters.framework_id })}</span>
              <button
                onClick={() => setFilters(prev => ({ ...prev, framework_id: undefined }))}
                className="ml-auto hover:bg-accent rounded p-0.5"
                aria-label={t("detail.clearFilter")}
              >
                <X className="h-3 w-3" />
              </button>
            </div>
          )}

          {/* Charts */}
          <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 gap-6">
            <CoverageHeatmap
              data={chartData.frameworkCoverage}
              onBarClick={handleBarClick}
              selectedFrameworkId={filters.framework_id}
              frameworkIds={analysis.matched_framework_ids}
            />
            <FrameworkRadar
              data={chartData.radarData}
              selectedFrameworkId={filters.framework_id}
              frameworkIds={analysis.matched_framework_ids}
            />
            <PriorityChart data={chartData.priorityCounts} />
          </div>

          {/* Findings Table */}
          <div ref={findingsRef}>
            <h2 className="text-lg font-semibold mb-4">
              {t("findings.title")}
            </h2>
            <FindingsTable
              findings={paginatedFindings?.data ?? []}
              expandedIds={expandedIds}
              onToggleExpand={handleToggleExpand}
              frameworkIds={analysis.matched_framework_ids}
              filters={filters}
              onFilterChange={handleFilterChange}
              page={page}
              totalPages={paginatedFindings?.total_pages ?? 1}
              onPageChange={setPage}
              onConceptClick={(conceptId) => setSelectedConceptId(conceptId)}
            />
          </div>

          {/* Concept Drawer */}
          <ConceptDrawer
            conceptId={selectedConceptId}
            onClose={() => setSelectedConceptId(null)}
          />
        </div>
      )}
    </div>
  );
}
