import { useState } from "react";
import { useTranslation } from "react-i18next";
import { FileDown, Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { exportAnalysis } from "../api";
import type { AnalysisStatus } from "../types";

interface ExportButtonsProps {
  analysisId: string;
  analysisName: string;
  status: AnalysisStatus;
}

export function ExportButtons({
  analysisId,
  analysisName,
  status,
}: ExportButtonsProps) {
  const { t } = useTranslation("analysis");
  const [loadingFormat, setLoadingFormat] = useState<string | null>(null);

  const isCompleted = status === "completed";
  const isExporting = loadingFormat !== null;

  async function handleExport(format: string) {
    setLoadingFormat(format);
    try {
      await exportAnalysis(analysisId, format, analysisName || undefined);
    } catch {
      alert(t("export.error"));
    } finally {
      setLoadingFormat(null);
    }
  }

  return (
    <div className="flex gap-2">
      <Button
        variant="outline"
        size="sm"
        onClick={() => handleExport("pdf")}
        disabled={!isCompleted || isExporting}
        title={!isCompleted ? t("export.disabled") : undefined}
      >
        {loadingFormat === "pdf" ? (
          <Loader2 className="h-4 w-4 animate-spin mr-1" />
        ) : (
          <FileDown className="h-4 w-4 mr-1" />
        )}
        {loadingFormat === "pdf" ? t("export.downloading") : t("export.pdf")}
      </Button>
      <Button
        variant="outline"
        size="sm"
        onClick={() => handleExport("docx")}
        disabled={!isCompleted || isExporting}
        title={!isCompleted ? t("export.disabled") : undefined}
      >
        {loadingFormat === "docx" ? (
          <Loader2 className="h-4 w-4 animate-spin mr-1" />
        ) : (
          <FileDown className="h-4 w-4 mr-1" />
        )}
        {loadingFormat === "docx" ? t("export.downloading") : t("export.docx")}
      </Button>
    </div>
  );
}
