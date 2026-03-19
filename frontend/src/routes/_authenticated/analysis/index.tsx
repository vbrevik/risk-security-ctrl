import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Settings } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useAnalyses } from "@/features/analysis/api";
import { AnalysisList } from "@/features/analysis/components/AnalysisList";

export const Route = createFileRoute("/_authenticated/analysis/")({
  component: AnalysisListPage,
  validateSearch: (search: Record<string, unknown>) => ({
    page: Number(search.page) || 1,
    status: (search.status as string) || undefined,
  }),
});

function AnalysisListPage() {
  const { t } = useTranslation("analysis");
  const { page, status } = Route.useSearch();
  const navigate = useNavigate();

  const { data, isLoading, isError, refetch } = useAnalyses({
    page,
    limit: 12,
    status: status as "pending" | "processing" | "completed" | "failed" | undefined,
  });

  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h1 className="text-2xl font-bold tracking-tight">{t("list.title")}</h1>
        <div className="flex gap-2">
          <Link to="/analysis/settings">
            <Button variant="outline" size="icon" aria-label={t("common.settings")}>
              <Settings className="h-4 w-4" />
            </Button>
          </Link>
          <Link to="/analysis/create">
            <Button>{t("list.newAnalysis")}</Button>
          </Link>
        </div>
      </div>

      <Select
        value={status ?? "__all__"}
        onValueChange={(value) =>
          navigate({
            search: { status: value === "__all__" ? undefined : value, page: 1 },
          })
        }
      >
        <SelectTrigger className="w-[180px]">
          <SelectValue placeholder={t("list.filters.all")} />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="__all__">{t("list.filters.all")}</SelectItem>
          <SelectItem value="pending">{t("status.pending")}</SelectItem>
          <SelectItem value="processing">{t("status.processing")}</SelectItem>
          <SelectItem value="completed">{t("status.completed")}</SelectItem>
          <SelectItem value="failed">{t("status.failed")}</SelectItem>
        </SelectContent>
      </Select>

      <AnalysisList
        analyses={data?.data}
        isLoading={isLoading}
        isError={isError}
        onRetry={refetch}
      />

      {data && data.total_pages > 1 && (
        <div className="flex items-center justify-center gap-4">
          <Button
            variant="outline"
            size="sm"
            disabled={page <= 1}
            onClick={() =>
              navigate({ search: { page: page - 1, status } })
            }
          >
            {t("list.pagination.previous")}
          </Button>
          <span className="text-sm text-muted-foreground">
            {t("list.pagination.pageOf", { page, total: data.total_pages })}
          </span>
          <Button
            variant="outline"
            size="sm"
            disabled={page >= data.total_pages}
            onClick={() =>
              navigate({ search: { page: page + 1, status } })
            }
          >
            {t("list.pagination.next")}
          </Button>
        </div>
      )}
    </div>
  );
}
