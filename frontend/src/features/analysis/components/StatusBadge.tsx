import { useTranslation } from "react-i18next";
import { Badge } from "@/components/ui/badge";
import type { AnalysisStatus } from "../types";

const statusStyles: Record<AnalysisStatus, string> = {
  pending: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
  processing: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300",
  completed: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300",
  failed: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
  deleted: "bg-muted text-muted-foreground",
};

interface StatusBadgeProps {
  status: AnalysisStatus;
}

export function StatusBadge({ status }: StatusBadgeProps) {
  const { t } = useTranslation("analysis");

  const badge = (
    <Badge variant="outline" className={statusStyles[status]}>
      {t(`status.${status}`)}
    </Badge>
  );

  if (status === "processing") {
    return <span className="animate-pulse">{badge}</span>;
  }

  return badge;
}
