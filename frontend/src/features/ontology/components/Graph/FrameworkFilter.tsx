import { useState, useRef, useEffect } from "react";
import { Filter, Check } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cn } from "@/lib/utils";
import { useExplorer } from "../../context";
import { getFrameworkColor } from "../../utils/graphTransform";
import type { Framework } from "../../types";

interface FrameworkFilterProps {
  frameworks: Framework[];
}

export function FrameworkFilter({ frameworks }: FrameworkFilterProps) {
  const { t } = useTranslation("ontology");
  const { state, toggleFramework, setActiveFrameworks } = useExplorer();
  const [open, setOpen] = useState(false);
  const ref = useRef<HTMLDivElement>(null);

  const allActive = state.activeFrameworks.length === 0;
  const activeCount = allActive ? frameworks.length : state.activeFrameworks.length;

  // Close on outside click
  useEffect(() => {
    if (!open) return;
    const handleClick = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClick);
    return () => document.removeEventListener("mousedown", handleClick);
  }, [open]);

  const handleToggle = (fwId: string) => {
    if (allActive) {
      // First toggle when all are shown — select all except this one
      const others = frameworks.filter((f) => f.id !== fwId).map((f) => f.id);
      setActiveFrameworks(others);
    } else {
      toggleFramework(fwId);
    }
  };

  const handleShowAll = () => {
    setActiveFrameworks([]);
  };

  const handleShowOnly = (fwId: string) => {
    setActiveFrameworks([fwId]);
  };

  return (
    <div ref={ref} className="absolute top-4 left-4 z-10">
      <button
        onClick={() => setOpen(!open)}
        className={cn(
          "flex items-center gap-2 px-3 py-2 bg-card border rounded-md shadow-sm text-sm",
          "hover:bg-accent transition-colors",
          !allActive && "border-primary"
        )}
      >
        <Filter className="h-4 w-4" />
        <span>{t("filters.frameworks")}</span>
        {!allActive && (
          <span className="bg-primary text-primary-foreground text-xs px-1.5 py-0.5 rounded-full">
            {activeCount}
          </span>
        )}
      </button>

      {open && (
        <div className="mt-1 w-72 max-h-96 overflow-y-auto bg-card border rounded-md shadow-lg">
          <div className="p-2 border-b">
            <button
              onClick={handleShowAll}
              className="text-xs text-primary hover:underline"
            >
              {t("filters.clearAll")}
            </button>
          </div>
          <div className="p-1">
            {frameworks.map((fw) => {
              const isActive = allActive || state.activeFrameworks.includes(fw.id);
              return (
                <div
                  key={fw.id}
                  className="flex items-center gap-2 px-2 py-1.5 rounded hover:bg-accent/50 group"
                >
                  <button
                    onClick={() => handleToggle(fw.id)}
                    className="flex items-center gap-2 flex-1 text-left text-sm"
                  >
                    <span
                      className={cn(
                        "w-4 h-4 rounded border flex items-center justify-center flex-shrink-0",
                        isActive ? "bg-primary border-primary" : "border-muted-foreground"
                      )}
                    >
                      {isActive && <Check className="h-3 w-3 text-primary-foreground" />}
                    </span>
                    <span
                      className="w-2.5 h-2.5 rounded-full flex-shrink-0"
                      style={{ backgroundColor: getFrameworkColor(fw.id) }}
                    />
                    <span className="truncate">{fw.name}</span>
                  </button>
                  <button
                    onClick={() => handleShowOnly(fw.id)}
                    className="text-[10px] text-muted-foreground opacity-0 group-hover:opacity-100 hover:text-foreground flex-shrink-0"
                  >
                    only
                  </button>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
