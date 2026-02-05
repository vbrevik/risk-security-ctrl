import { createFileRoute } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";

export const Route = createFileRoute("/compliance/")({
  component: CompliancePage,
});

function CompliancePage() {
  const { t } = useTranslation("compliance");

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">{t("title")}</h1>
        <Button>
          <Plus className="h-4 w-4 mr-2" />
          {t("assessments.new")}
        </Button>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>{t("assessments.title")}</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="h-64 flex items-center justify-center border rounded-lg bg-muted/50">
            <p className="text-muted-foreground">
              Assessment list will be implemented in Sprint 2
            </p>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
