import { useTranslation } from "react-i18next";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useFrameworks } from "@/features/ontology/api";
import type { AssessmentStatus } from "../types";

const ALL_STATUSES: AssessmentStatus[] = [
  "draft",
  "in_progress",
  "under_review",
  "completed",
  "archived",
];

interface AssessmentFiltersProps {
  status: string | undefined;
  frameworkId: string | undefined;
  onStatusChange: (status: string | undefined) => void;
  onFrameworkChange: (frameworkId: string | undefined) => void;
}

export function AssessmentFilters({
  status,
  frameworkId,
  onStatusChange,
  onFrameworkChange,
}: AssessmentFiltersProps) {
  const { t } = useTranslation("compliance");
  const { data: frameworks } = useFrameworks();

  return (
    <div className="flex gap-3 flex-wrap">
      <Select
        value={status ?? "all"}
        onValueChange={(v) => onStatusChange(v === "all" ? undefined : v)}
      >
        <SelectTrigger className="w-[180px]">
          <SelectValue placeholder={t("filters.status")} />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">{t("filters.allStatuses")}</SelectItem>
          {ALL_STATUSES.map((s) => (
            <SelectItem key={s} value={s}>
              {t(`assessments.status.${s}`)}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      <Select
        value={frameworkId ?? "all"}
        onValueChange={(v) => onFrameworkChange(v === "all" ? undefined : v)}
      >
        <SelectTrigger className="w-[220px]">
          <SelectValue placeholder={t("filters.framework")} />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">{t("filters.allFrameworks")}</SelectItem>
          {frameworks?.map((fw) => (
            <SelectItem key={fw.id} value={fw.id}>
              {fw.name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}
