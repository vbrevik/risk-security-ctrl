import { useTranslation } from "react-i18next";
import { Link } from "@tanstack/react-router";
import { FileText } from "lucide-react";
import { Button } from "@/components/ui/button";
import { AnalysisCard } from "./AnalysisCard";
import type { AnalysisListItem } from "../types";

interface AnalysisListProps {
  analyses: AnalysisListItem[] | undefined;
  isLoading: boolean;
  isError: boolean;
  onRetry?: () => void;
}

export function AnalysisList({
  analyses,
  isLoading,
  isError,
  onRetry,
}: AnalysisListProps) {
  const { t } = useTranslation("analysis");

  if (isLoading) {
    return (
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {Array.from({ length: 3 }).map((_, i) => (
          <div
            key={i}
            className="h-48 rounded-lg border bg-muted/50 animate-pulse"
          />
        ))}
      </div>
    );
  }

  if (isError) {
    return (
      <div className="flex flex-col items-center justify-center py-12 text-center">
        <p className="text-destructive mb-4">{t("common.error")}</p>
        {onRetry && (
          <Button variant="outline" onClick={onRetry}>
            {t("common.retry")}
          </Button>
        )}
      </div>
    );
  }

  if (!analyses?.length) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-center">
        <FileText className="h-12 w-12 text-muted-foreground/50 mb-4" />
        <h3 className="text-lg font-medium mb-1">
          {t("list.empty.title")}
        </h3>
        <p className="text-sm text-muted-foreground max-w-sm mb-4">
          {t("list.empty.description")}
        </p>
        <Link to="/analysis/create">
          <Button>{t("list.newAnalysis")}</Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {analyses.map((analysis) => (
        <AnalysisCard key={analysis.id} analysis={analysis} />
      ))}
    </div>
  );
}
