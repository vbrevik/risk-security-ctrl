import { createFileRoute, Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { SettingsForm } from "@/features/analysis/components/SettingsForm";

export const Route = createFileRoute("/_authenticated/analysis/settings")({
  component: AnalysisSettingsPage,
});

function AnalysisSettingsPage() {
  const { t } = useTranslation("analysis");

  return (
    <div className="max-w-2xl mx-auto space-y-6 p-6">
      <Link
        to="/analysis"
        className="text-sm text-muted-foreground hover:text-foreground transition-colors"
      >
        &larr; {t("common.back")}
      </Link>
      <h1 className="text-2xl font-bold tracking-tight">{t("settings.title")}</h1>
      <SettingsForm />
    </div>
  );
}
