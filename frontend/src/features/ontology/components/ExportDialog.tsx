import { useState } from "react";
import { X, Download } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";

interface ExportDialogProps {
  isOpen: boolean;
  onClose: () => void;
}

export function ExportDialog({ isOpen, onClose }: ExportDialogProps) {
  const { t } = useTranslation("ontology");
  const [format, setFormat] = useState<"png" | "svg">("png");
  const [size, setSize] = useState<"current" | "full">("current");
  const [includeLegend, setIncludeLegend] = useState(true);
  const [includeTitle, setIncludeTitle] = useState(true);
  const [isExporting, setIsExporting] = useState(false);

  if (!isOpen) return null;

  const handleExport = async () => {
    // Find the SVG element in the graph view
    const svgElement = document.querySelector(".bg-muted\\/20 svg") as SVGSVGElement | null;
    if (!svgElement) {
      console.error("No SVG element found for export");
      return;
    }
    setIsExporting(true);

    try {
      if (format === "svg") {
        // Export as SVG
        const serializer = new XMLSerializer();
        const svgString = serializer.serializeToString(svgElement);
        const blob = new Blob([svgString], { type: "image/svg+xml" });
        const url = URL.createObjectURL(blob);

        const link = document.createElement("a");
        link.href = url;
        link.download = "ontology-graph.svg";
        link.click();
        URL.revokeObjectURL(url);
      } else {
        // Export as PNG using html2canvas
        const html2canvas = (await import("html2canvas")).default;
        const canvas = await html2canvas(svgElement as unknown as HTMLElement, {
          backgroundColor: "#ffffff",
          scale: 2,
        });

        const link = document.createElement("a");
        link.href = canvas.toDataURL("image/png");
        link.download = "ontology-graph.png";
        link.click();
      }

      onClose();
    } catch (error) {
      console.error("Export failed:", error);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/50" onClick={onClose} />
      <Card className="relative z-10 w-80">
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-lg">{t("export.title")}</CardTitle>
          <button onClick={onClose} className="p-1 hover:bg-accent rounded">
            <X className="h-4 w-4" />
          </button>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Format */}
          <div>
            <label className="text-sm font-medium">{t("export.format")}</label>
            <div className="flex gap-4 mt-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="format"
                  checked={format === "png"}
                  onChange={() => setFormat("png")}
                  className="w-4 h-4"
                />
                <span className="text-sm">PNG</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="format"
                  checked={format === "svg"}
                  onChange={() => setFormat("svg")}
                  className="w-4 h-4"
                />
                <span className="text-sm">SVG</span>
              </label>
            </div>
          </div>

          {/* Size */}
          <div>
            <label className="text-sm font-medium">{t("export.size")}</label>
            <div className="flex gap-4 mt-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="size"
                  checked={size === "current"}
                  onChange={() => setSize("current")}
                  className="w-4 h-4"
                />
                <span className="text-sm">{t("export.currentView")}</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="size"
                  checked={size === "full"}
                  onChange={() => setSize("full")}
                  className="w-4 h-4"
                />
                <span className="text-sm">{t("export.fullGraph")}</span>
              </label>
            </div>
          </div>

          {/* Include options */}
          <div>
            <label className="text-sm font-medium">{t("export.include")}</label>
            <div className="space-y-2 mt-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={includeLegend}
                  onChange={(e) => setIncludeLegend(e.target.checked)}
                  className="w-4 h-4 rounded"
                />
                <span className="text-sm">{t("export.legend")}</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={includeTitle}
                  onChange={(e) => setIncludeTitle(e.target.checked)}
                  className="w-4 h-4 rounded"
                />
                <span className="text-sm">{t("export.exportTitle")}</span>
              </label>
            </div>
          </div>

          {/* Actions */}
          <div className="flex justify-end gap-2 pt-2">
            <Button variant="outline" onClick={onClose}>
              {t("export.cancel")}
            </Button>
            <Button onClick={handleExport} disabled={isExporting}>
              <Download className="h-4 w-4 mr-2" />
              {isExporting ? t("export.exporting") : t("export.export")}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
