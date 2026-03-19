import { useTranslation } from "react-i18next";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { Analysis } from "../types";
import type { ChartData } from "../hooks/useChartData";

interface SummaryStatsProps {
  analysis: Analysis;
  chartData: ChartData;
  isLoading?: boolean;
  overrideTypeCounts?: ChartData["typeCounts"];
}

function formatProcessingTime(ms: number | null): string {
  if (ms == null) return "\u2014";
  return `${(ms / 1000).toFixed(1)}s`;
}

function formatTokenCount(count: number | null): string {
  if (count == null) return "\u2014";
  return new Intl.NumberFormat("en-US").format(count);
}

function calcPercent(count: number, total: number): string {
  if (total === 0) return "0%";
  return `${Math.round((count / total) * 100)}%`;
}

export function SummaryStats({ analysis, chartData, isLoading, overrideTypeCounts }: SummaryStatsProps) {
  const { t } = useTranslation("analysis");

  const typeCounts = overrideTypeCounts ?? chartData.typeCounts;

  const cards = [
    {
      label: t("stats.totalFindings"),
      value: String(typeCounts.total),
    },
    {
      label: t("stats.addressed"),
      value: String(typeCounts.addressed),
      secondary: calcPercent(typeCounts.addressed, typeCounts.total),
    },
    {
      label: t("stats.gaps"),
      value: String(typeCounts.gap),
      secondary: calcPercent(typeCounts.gap, typeCounts.total),
    },
    {
      label: t("stats.frameworks"),
      value: String(analysis.matched_framework_ids.length),
    },
    {
      label: t("stats.processingTime"),
      value: formatProcessingTime(analysis.processing_time_ms),
    },
    {
      label: t("stats.tokenCount"),
      value: formatTokenCount(analysis.token_count),
    },
  ];

  return (
    <div
      className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4"
      data-testid="summary-stats"
    >
      {cards.map((card) => (
        <Card key={card.label} data-testid="stat-card">
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-muted-foreground">
              {card.label}
            </CardTitle>
          </CardHeader>
          <CardContent>
            {isLoading ? (
              <div className="animate-pulse bg-muted rounded h-8 w-20" />
            ) : (
              <>
                <p className="text-2xl font-bold">{card.value}</p>
                {card.secondary && (
                  <p className="text-xs text-muted-foreground">{card.secondary}</p>
                )}
              </>
            )}
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
