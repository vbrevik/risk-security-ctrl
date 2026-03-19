import { createFileRoute, Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { useAnalysis } from "@/features/analysis/api";
import { StatusBadge } from "@/features/analysis/components/StatusBadge";
import { ArrowLeft } from "lucide-react";

export const Route = createFileRoute("/analysis/$id")({
  component: AnalysisDetailPage,
});

function AnalysisDetailPage() {
  const { id } = Route.useParams();
  const { t } = useTranslation("analysis");
  const { data: analysis, isLoading, isError, error } = useAnalysis(id);

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

  if (isError) {
    const is404 =
      error && "status" in error && (error as { status: number }).status === 404;
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
  const isCompleted = analysis.status === "completed";

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
          {/* ExportButtons will be added in section 05/06 */}
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

      {/* Completed Content - placeholder slots wired in section 06 */}
      {isCompleted && (
        <div className="space-y-6">
          {/* SummaryStats, ChartsSection, FindingsSection added in section 06 */}
        </div>
      )}
    </div>
  );
}
