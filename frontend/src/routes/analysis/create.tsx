import { createFileRoute, Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { CreateAnalysisForm } from "@/features/analysis/components/CreateAnalysisForm";

export const Route = createFileRoute("/analysis/create")({
  component: CreateAnalysisPage,
});

function CreateAnalysisPage() {
  const { t } = useTranslation("analysis");

  return (
    <div className="max-w-2xl mx-auto p-6 space-y-6">
      <Link
        to="/analysis"
        className="text-sm text-muted-foreground hover:text-foreground transition-colors"
      >
        &larr; {t("common.back")}
      </Link>
      <h1 className="text-2xl font-bold tracking-tight">{t("create.title")}</h1>
      <CreateAnalysisForm />
    </div>
  );
}
