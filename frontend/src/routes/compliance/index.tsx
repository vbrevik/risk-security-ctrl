import { useState } from "react";
import { createFileRoute } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { useAssessments } from "@/features/compliance/api";
import {
  AssessmentList,
  AssessmentFilters,
  CreateAssessmentDialog,
} from "@/features/compliance/components";
import type { AssessmentStatus } from "@/features/compliance/types";

export const Route = createFileRoute("/compliance/")({
  component: CompliancePage,
});

function CompliancePage() {
  const { t } = useTranslation("compliance");
  const [dialogOpen, setDialogOpen] = useState(false);
  const [statusFilter, setStatusFilter] = useState<string | undefined>();
  const [frameworkFilter, setFrameworkFilter] = useState<string | undefined>();

  const { data, isLoading, isError } = useAssessments({
    status: statusFilter as AssessmentStatus | undefined,
    framework_id: frameworkFilter,
  });

  return (
    <div className="space-y-6 p-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">{t("title")}</h1>
        <Button onClick={() => setDialogOpen(true)}>
          <Plus className="h-4 w-4 mr-2" />
          {t("assessments.new")}
        </Button>
      </div>

      <AssessmentFilters
        status={statusFilter}
        frameworkId={frameworkFilter}
        onStatusChange={setStatusFilter}
        onFrameworkChange={setFrameworkFilter}
      />

      <AssessmentList
        assessments={data?.data}
        isLoading={isLoading}
        isError={isError}
      />

      <CreateAssessmentDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
      />
    </div>
  );
}
