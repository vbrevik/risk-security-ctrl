import { ZoomIn, ZoomOut, RotateCcw, Maximize, Map } from "lucide-react";
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
  const buttonClass = cn(
    "p-2 bg-card border rounded-md shadow-sm",
    "hover:bg-accent transition-colors",
    "focus:outline-none focus:ring-2 focus:ring-ring"
  );

  return (
    <div className="absolute bottom-4 right-4 flex flex-col gap-1">
      <div className="flex gap-1">
        <button onClick={onZoomIn} className={buttonClass} title="Zoom in (+)">
          <ZoomIn className="h-4 w-4" />
        </button>
        <button onClick={onZoomOut} className={buttonClass} title="Zoom out (-)">
          <ZoomOut className="h-4 w-4" />
        </button>
        <button onClick={onResetView} className={buttonClass} title="Reset view (0)">
          <RotateCcw className="h-4 w-4" />
        </button>
      </div>
      <div className="flex gap-1">
        <button onClick={onFitToScreen} className={buttonClass} title="Fit to screen">
          <Maximize className="h-4 w-4" />
        </button>
        <button
          onClick={onToggleMinimap}
          className={cn(buttonClass, minimapVisible && "bg-accent")}
          title="Toggle minimap"
        >
          <Map className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
}
