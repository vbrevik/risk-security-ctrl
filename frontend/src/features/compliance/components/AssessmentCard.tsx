import { useTranslation } from "react-i18next";
import { Link } from "@tanstack/react-router";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import { useAssessmentScore } from "../api";
import type { Assessment, AssessmentStatus } from "../types";

const statusVariant: Record<
  AssessmentStatus,
  "default" | "secondary" | "destructive" | "outline"
> = {
  draft: "secondary",
  in_progress: "default",
  under_review: "outline",
  completed: "default",
  archived: "secondary",
};

interface AssessmentCardProps {
  assessment: Assessment;
  frameworkName?: string;
}

export function AssessmentCard({
  assessment,
  frameworkName,
}: AssessmentCardProps) {
  const { t } = useTranslation("compliance");
  const { data: score } = useAssessmentScore(assessment.id);

  const createdDate = new Date(assessment.created_at).toLocaleDateString();
  const dueDate = assessment.due_date
    ? new Date(assessment.due_date).toLocaleDateString()
    : null;

  return (
    <Link
      to="/compliance/$assessmentId"
      params={{ assessmentId: assessment.id }}
      className="block"
    >
      <Card className="transition-colors hover:border-primary/50">
        <CardHeader className="pb-2">
          <div className="flex items-start justify-between gap-2">
            <CardTitle className="text-base leading-tight">
              {assessment.name}
            </CardTitle>
            <Badge variant={statusVariant[assessment.status]}>
              {t(`assessments.status.${assessment.status}`)}
            </Badge>
          </div>
          {frameworkName && (
            <p className="text-sm text-muted-foreground">{frameworkName}</p>
          )}
        </CardHeader>
        <CardContent className="pb-2">
          {assessment.description && (
            <p className="text-sm text-muted-foreground line-clamp-2">
              {assessment.description}
            </p>
          )}
          {score && (
            <div className="mt-3">
              <div className="flex items-center justify-between text-sm mb-1">
                <span className="text-muted-foreground">
                  {t("assessments.score")}
                </span>
                <span
                  className={cn(
                    "font-medium",
                    score.overall_compliance_percentage >= 80
                      ? "text-green-600"
                      : score.overall_compliance_percentage >= 50
                        ? "text-yellow-600"
                        : "text-red-600"
                  )}
                >
                  {Math.round(score.overall_compliance_percentage)}%
                </span>
              </div>
              <div className="h-2 rounded-full bg-muted overflow-hidden">
                <div
                  className={cn(
                    "h-full rounded-full transition-all",
                    score.overall_compliance_percentage >= 80
                      ? "bg-green-500"
                      : score.overall_compliance_percentage >= 50
                        ? "bg-yellow-500"
                        : "bg-red-500"
                  )}
                  style={{
                    width: `${Math.round(score.overall_compliance_percentage)}%`,
                  }}
                />
              </div>
            </div>
          )}
        </CardContent>
        <CardFooter className="text-xs text-muted-foreground pt-0">
          <div className="flex gap-4">
            <span>
              {t("assessments.created")}: {createdDate}
            </span>
            {dueDate && (
              <span>
                {t("assessments.due")}: {dueDate}
              </span>
            )}
          </div>
        </CardFooter>
      </Card>
    </Link>
  );
}
