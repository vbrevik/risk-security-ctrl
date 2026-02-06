import { ZoomIn, ZoomOut, RotateCcw, Maximize, Map } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cn } from "@/lib/utils";

interface GraphControlsProps {
  onZoomIn: () => void;
  onZoomOut: () => void;
  onResetView: () => void;
  onFitToScreen: () => void;
  onToggleMinimap: () => void;
  minimapVisible: boolean;
}

export function GraphControls({
  onZoomIn,
  onZoomOut,
  onResetView,
  onFitToScreen,
  onToggleMinimap,
  minimapVisible,
}: GraphControlsProps) {
  const { t } = useTranslation("ontology");
  const buttonClass = cn(
    "p-2 bg-card border rounded-md shadow-sm",
    "hover:bg-accent transition-colors",
    "focus:outline-none focus:ring-2 focus:ring-ring"
  );

  return (
    <div className="absolute bottom-4 right-4 flex flex-col gap-1">
      <div className="flex gap-1">
        <button onClick={onZoomIn} className={buttonClass} title={`${t("graph.zoomIn")} (+)`}>
          <ZoomIn className="h-4 w-4" />
        </button>
        <button onClick={onZoomOut} className={buttonClass} title={`${t("graph.zoomOut")} (-)`}>
          <ZoomOut className="h-4 w-4" />
        </button>
        <button onClick={onResetView} className={buttonClass} title={`${t("graph.resetView")} (0)`}>
          <RotateCcw className="h-4 w-4" />
        </button>
      </div>
      <div className="flex gap-1">
        <button onClick={onFitToScreen} className={buttonClass} title={t("graph.fitToScreen")}>
          <Maximize className="h-4 w-4" />
        </button>
        <button
          onClick={onToggleMinimap}
          className={cn(buttonClass, minimapVisible && "bg-accent")}
          title={t("graph.toggleMinimap")}
        >
          <Map className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
}
