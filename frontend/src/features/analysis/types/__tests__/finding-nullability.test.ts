import { describe, it, expect } from "vitest";
import type { AnalysisFinding } from "../../types";

describe("AnalysisFinding nullability", () => {
  it("accepts null for optional concept and evidence fields", () => {
    const finding: AnalysisFinding = {
      id: "f1",
      concept_id: "c1",
      framework_id: "fw1",
      finding_type: "gap",
      confidence_score: 0.85,
      evidence_text: null,
      recommendation: null,
      priority: 1,
      sort_order: 1,
      concept_code: null,
      concept_name: null,
      concept_definition: null,
    };
    expect(finding.evidence_text).toBeNull();
    expect(finding.recommendation).toBeNull();
    expect(finding.concept_code).toBeNull();
    expect(finding.concept_name).toBeNull();
    expect(finding.concept_definition).toBeNull();
  });
});
