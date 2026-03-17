import { SECTORS, ACTIVITIES } from "../utils/landscapeMapping";

interface LandscapeSelectorProps {
  sector: string | undefined;
  activities: string[];
  onSectorChange: (sector: string) => void;
  onActivitiesChange: (activities: string[]) => void;
}

export function LandscapeSelector({
  sector,
  activities,
  onSectorChange,
  onActivitiesChange,
}: LandscapeSelectorProps) {
  const toggleActivity = (key: string) => {
    if (activities.includes(key)) {
      onActivitiesChange(activities.filter((a) => a !== key));
    } else {
      onActivitiesChange([...activities, key]);
    }
  };

  return (
    <div className="space-y-6">
      {/* Sector selector */}
      <div>
        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-3">
          Organization Sector
        </h3>
        <div className="space-y-1.5">
          {SECTORS.map((s) => (
            <label
              key={s.key}
              className={`flex items-start gap-2 p-2 rounded cursor-pointer transition-colors ${
                sector === s.key
                  ? "bg-accent/10 border border-accent/30"
                  : "hover:bg-muted/50 border border-transparent"
              }`}
            >
              <input
                type="radio"
                name="sector"
                value={s.key}
                checked={sector === s.key}
                onChange={() => onSectorChange(s.key)}
                className="mt-0.5"
              />
              <div>
                <div className="text-sm font-medium">{s.label}</div>
                <div className="text-[10px] text-foreground/40">{s.description}</div>
              </div>
            </label>
          ))}
        </div>
      </div>

      {/* Activity selector */}
      <div>
        <div className="flex items-center justify-between mb-3">
          <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50">
            Activities
          </h3>
          {activities.length > 0 && (
            <button
              onClick={() => onActivitiesChange([])}
              className="text-[10px] text-foreground/40 hover:text-foreground/70"
            >
              Clear all
            </button>
          )}
        </div>
        <div className="space-y-1.5">
          {ACTIVITIES.map((a) => (
            <label
              key={a.key}
              className="flex items-center gap-2 p-2 rounded cursor-pointer hover:bg-muted/50 transition-colors"
            >
              <input
                type="checkbox"
                checked={activities.includes(a.key)}
                onChange={() => toggleActivity(a.key)}
                className="rounded border-border"
              />
              <span className="text-sm">{a.label}</span>
            </label>
          ))}
        </div>
      </div>
    </div>
  );
}
