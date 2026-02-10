import { useTranslation } from "react-i18next";
import { ClipboardList } from "lucide-react";
import { useFrameworks } from "@/features/ontology/api";
import { AssessmentCard } from "./AssessmentCard";
import type { Assessment } from "../types";

interface AssessmentListProps {
  assessments: Assessment[] | undefined;
  isLoading: boolean;
  isError: boolean;
}

export function AssessmentList({
  assessments,
  isLoading,
  isError,
}: AssessmentListProps) {
  const { t } = useTranslation("compliance");
  const { data: frameworks } = useFrameworks();

  const frameworkMap = new Map(frameworks?.map((fw) => [fw.id, fw.name]));

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
        <p className="text-destructive">{t("assessments.error")}</p>
      </div>
    );
  }

  if (!assessments?.length) {
    return (
      <div className="flex flex-col items-center justify-center py-16 text-center">
        <ClipboardList className="h-12 w-12 text-muted-foreground/50 mb-4" />
        <h3 className="text-lg font-medium mb-1">
          {t("assessments.emptyTitle")}
        </h3>
        <p className="text-sm text-muted-foreground max-w-sm">
          {t("assessments.emptyDescription")}
        </p>
      </div>
    );
  }

  return (
    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
      {assessments.map((assessment) => (
        <AssessmentCard
          key={assessment.id}
          assessment={assessment}
          frameworkName={frameworkMap.get(assessment.framework_id)}
        />
      ))}
    </div>
  );
}
