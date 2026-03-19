import { useMemo } from "react";
import type { AnalysisFinding } from "../types";

export interface ChartData {
  frameworkCoverage: Array<{
    frameworkId: string;
    total: number;
    addressed: number;
    percentage: number;
  }>;
  priorityCounts: Array<{
    priority: number;
    count: number;
  }>;
  typeCounts: {
    addressed: number;
    partiallyAddressed: number;
    gap: number;
    notApplicable: number;
    total: number;
  };
  radarData: Array<{
    frameworkId: string;
    values: { addressed: number; partial: number; gap: number; notApplicable: number };
    total: number;
  }>;
}

const EMPTY_TYPE_COUNTS: ChartData["typeCounts"] = {
  addressed: 0,
  partiallyAddressed: 0,
  gap: 0,
  notApplicable: 0,
  total: 0,
};

const EMPTY_CHART_DATA: ChartData = {
  frameworkCoverage: [],
  priorityCounts: [],
  typeCounts: EMPTY_TYPE_COUNTS,
  radarData: [],
};

export function useChartData(
  findings: AnalysisFinding[] | undefined
): ChartData {
  return useMemo(() => {
    if (!findings || findings.length === 0) {
      return EMPTY_CHART_DATA;
    }

    // Type counts
    const typeCounts = {
      addressed: 0,
      partiallyAddressed: 0,
      gap: 0,
      notApplicable: 0,
      total: findings.length,
    };
    for (const f of findings) {
      switch (f.finding_type) {
        case "addressed":
          typeCounts.addressed++;
          break;
        case "partially_addressed":
          typeCounts.partiallyAddressed++;
          break;
        case "gap":
          typeCounts.gap++;
          break;
        case "not_applicable":
          typeCounts.notApplicable++;
          break;
      }
    }

    // Framework coverage
    const fwMap = new Map<
      string,
      { total: number; addressed: number }
    >();
    for (const f of findings) {
      const entry = fwMap.get(f.framework_id) ?? { total: 0, addressed: 0 };
      entry.total++;
      if (f.finding_type === "addressed") {
        entry.addressed++;
      }
      fwMap.set(f.framework_id, entry);
    }
    const frameworkCoverage = Array.from(fwMap.entries())
      .map(([frameworkId, { total, addressed }]) => ({
        frameworkId,
        total,
        addressed,
        percentage: (addressed / total) * 100,
      }))
      .sort((a, b) => a.frameworkId.localeCompare(b.frameworkId));

    // Priority counts
    const prioMap = new Map<number, number>();
    for (const f of findings) {
      prioMap.set(f.priority, (prioMap.get(f.priority) ?? 0) + 1);
    }
    const priorityCounts = Array.from(prioMap.entries())
      .map(([priority, count]) => ({ priority, count }))
      .sort((a, b) => a.priority - b.priority);

    // Radar data: normalized percentages per framework by finding type
    const radarMap = new Map<string, { addressed: number; partial: number; gap: number; notApplicable: number; total: number }>();
    for (const f of findings) {
      const entry = radarMap.get(f.framework_id) ?? { addressed: 0, partial: 0, gap: 0, notApplicable: 0, total: 0 };
      entry.total++;
      switch (f.finding_type) {
        case "addressed": entry.addressed++; break;
        case "partially_addressed": entry.partial++; break;
        case "gap": entry.gap++; break;
        case "not_applicable": entry.notApplicable++; break;
      }
      radarMap.set(f.framework_id, entry);
    }
    const radarData = Array.from(radarMap.entries())
      .map(([frameworkId, counts]) => ({
        frameworkId,
        values: {
          addressed: (counts.addressed / counts.total) * 100,
          partial: (counts.partial / counts.total) * 100,
          gap: (counts.gap / counts.total) * 100,
          notApplicable: (counts.notApplicable / counts.total) * 100,
        },
        total: counts.total,
      }))
      .sort((a, b) => a.frameworkId.localeCompare(b.frameworkId));

    return { frameworkCoverage, priorityCounts, typeCounts, radarData };
  }, [findings]);
}
