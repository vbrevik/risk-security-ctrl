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

    // Single-pass aggregation
    const typeCounts = {
      addressed: 0,
      partiallyAddressed: 0,
      gap: 0,
      notApplicable: 0,
      total: findings.length,
    };
    const fwMap = new Map<string, { total: number; addressed: number; partial: number; gap: number; notApplicable: number }>();
    const prioMap = new Map<number, number>();

    for (const f of findings) {
      // Type counts
      switch (f.finding_type) {
        case "addressed": typeCounts.addressed++; break;
        case "partially_addressed": typeCounts.partiallyAddressed++; break;
        case "gap": typeCounts.gap++; break;
        case "not_applicable": typeCounts.notApplicable++; break;
      }

      // Framework coverage + radar data
      const fw = fwMap.get(f.framework_id) ?? { total: 0, addressed: 0, partial: 0, gap: 0, notApplicable: 0 };
      fw.total++;
      switch (f.finding_type) {
        case "addressed": fw.addressed++; break;
        case "partially_addressed": fw.partial++; break;
        case "gap": fw.gap++; break;
        case "not_applicable": fw.notApplicable++; break;
      }
      fwMap.set(f.framework_id, fw);

      // Priority counts
      prioMap.set(f.priority, (prioMap.get(f.priority) ?? 0) + 1);
    }

    const frameworkCoverage = Array.from(fwMap.entries())
      .map(([frameworkId, fw]) => ({
        frameworkId,
        total: fw.total,
        addressed: fw.addressed,
        percentage: (fw.addressed / fw.total) * 100,
      }))
      .sort((a, b) => a.frameworkId.localeCompare(b.frameworkId));

    const priorityCounts = Array.from(prioMap.entries())
      .map(([priority, count]) => ({ priority, count }))
      .sort((a, b) => a.priority - b.priority);

    const radarData = Array.from(fwMap.entries())
      .map(([frameworkId, fw]) => ({
        frameworkId,
        values: {
          addressed: (fw.addressed / fw.total) * 100,
          partial: (fw.partial / fw.total) * 100,
          gap: (fw.gap / fw.total) * 100,
          notApplicable: (fw.notApplicable / fw.total) * 100,
        },
        total: fw.total,
      }))
      .sort((a, b) => a.frameworkId.localeCompare(b.frameworkId));

    return { frameworkCoverage, priorityCounts, typeCounts, radarData };
  }, [findings]);
}
