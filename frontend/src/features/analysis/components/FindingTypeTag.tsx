import { useTranslation } from "react-i18next";
import { Badge } from "@/components/ui/badge";
import type { FindingType } from "../types";

interface FindingTypeTagProps {
  type: FindingType;
}

const TYPE_STYLES: Record<FindingType, string> = {
  addressed: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  partially_addressed:
    "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
  gap: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
  not_applicable:
    "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200",
};

export function FindingTypeTag({ type }: FindingTypeTagProps) {
  const { t } = useTranslation("analysis");

  return (
    <Badge variant="outline" className={TYPE_STYLES[type]}>
      {t(`findings.type.${type}`)}
    </Badge>
  );
}
