import { useTranslation } from "react-i18next";
import {
  CheckCircle2,
  AlertTriangle,
  Info,
  Circle,
  XCircle,
} from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { toVerificationStatus } from "../types";
import type { VerificationStatus } from "../types";

interface VerificationBadgeProps {
  status: string | null;
}

type BadgeConfig = {
  colorClasses: string;
  Icon: React.ComponentType<{ className?: string; "aria-hidden"?: boolean }>;
  i18nKey: string;
  label: string;
};

const BADGE_CONFIG: Record<VerificationStatus | "unknown", BadgeConfig> = {
  verified: {
    colorClasses:
      "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
    Icon: CheckCircle2,
    i18nKey: "proof.status.verified",
    label: "Verified",
  },
  corrected: {
    colorClasses:
      "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
    Icon: CheckCircle2,
    i18nKey: "proof.status.corrected",
    label: "Corrected",
  },
  "partially-verified": {
    colorClasses:
      "bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200",
    Icon: AlertTriangle,
    i18nKey: "proof.status.partially-verified",
    label: "Partially Verified",
  },
  "structure-verified": {
    colorClasses:
      "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
    Icon: Info,
    i18nKey: "proof.status.structure-verified",
    label: "Structure Verified",
  },
  unverified: {
    colorClasses:
      "bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400",
    Icon: Circle,
    i18nKey: "proof.status.unverified",
    label: "Unverified",
  },
  "needs-correction": {
    colorClasses: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
    Icon: XCircle,
    i18nKey: "proof.status.needs-correction",
    label: "Needs Correction",
  },
  unknown: {
    colorClasses:
      "bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400",
    Icon: Circle,
    i18nKey: "proof.status.unknown",
    label: "Unknown",
  },
};

export function VerificationBadge({ status }: VerificationBadgeProps) {
  const { t } = useTranslation("ontology");
  const normalized = toVerificationStatus(status);
  const config = BADGE_CONFIG[normalized];
  const { Icon } = config;

  return (
    <Badge
      variant="outline"
      className={`inline-flex items-center gap-1.5 ${config.colorClasses}`}
      aria-label={t(config.i18nKey, config.label)}
    >
      <Icon className="h-3.5 w-3.5" aria-hidden />
      <span>{t(config.i18nKey, config.label)}</span>
    </Badge>
  );
}
