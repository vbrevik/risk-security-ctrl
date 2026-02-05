import { createFileRoute } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { FileText, Download } from "lucide-react";

export const Route = createFileRoute("/reports/")({
  component: ReportsPage,
});

function ReportsPage() {
  const { t } = useTranslation("reports");

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <h1 className="text-3xl font-bold">{t("title")}</h1>
        <Button>
          <FileText className="h-4 w-4 mr-2" />
          {t("generate")}
        </Button>
      </div>

      <div className="grid gap-6 md:grid-cols-3">
        <Card>
          <CardHeader>
            <CardTitle>{t("types.complianceSummary")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-sm text-muted-foreground">
              Overview of compliance status across all frameworks and
              assessments.
            </p>
            <Button variant="outline" className="w-full" disabled>
              <Download className="h-4 w-4 mr-2" />
              {t("export.pdf")}
            </Button>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("types.riskAssessment")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-sm text-muted-foreground">
              Detailed risk assessment findings and recommendations.
            </p>
            <Button variant="outline" className="w-full" disabled>
              <Download className="h-4 w-4 mr-2" />
              {t("export.pdf")}
            </Button>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>{t("types.auditTrail")}</CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <p className="text-sm text-muted-foreground">
              Complete history of changes and actions for audit purposes.
            </p>
            <Button variant="outline" className="w-full" disabled>
              <Download className="h-4 w-4 mr-2" />
              {t("export.csv")}
            </Button>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
