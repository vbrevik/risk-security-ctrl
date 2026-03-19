import type { AnalysisFinding } from "../types";

export function makeFinding(overrides: Partial<AnalysisFinding> = {}): AnalysisFinding {
  return {
    id: "f1",
    concept_id: "c1",
    framework_id: "fw1",
    finding_type: "gap",
    confidence_score: 0.85,
    evidence_text: "Some evidence",
    recommendation: "Fix this",
    priority: 1,
    sort_order: 1,
    concept_code: "C-001",
    concept_name: "Control One",
    concept_definition: "Definition of control",
    ...overrides,
  };
}
