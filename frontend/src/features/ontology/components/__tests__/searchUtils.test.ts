import { describe, it, expect } from "vitest";
import { computeFacets } from "../../../../routes/concepts/search";
import type { Concept } from "../../types";

function makeConcept(id: string, frameworkId: string, type: string): Concept {
  return {
    id,
    framework_id: frameworkId,
    parent_id: null,
    concept_type: type,
    code: null,
    name_en: id,
    name_nb: null,
    definition_en: null,
    definition_nb: null,
    source_reference: null,
    sort_order: null,
    created_at: "",
    updated_at: "",
  };
}

describe("computeFacets", () => {
  it("counts concepts per framework", () => {
    const concepts = [
      makeConcept("c1", "gdpr", "control"),
      makeConcept("c2", "gdpr", "principle"),
      makeConcept("c3", "nis2", "requirement"),
    ];
    const { frameworks } = computeFacets(concepts);
    expect(frameworks.get("gdpr")).toBe(2);
    expect(frameworks.get("nis2")).toBe(1);
  });

  it("counts concepts per type", () => {
    const concepts = [
      makeConcept("c1", "gdpr", "control"),
      makeConcept("c2", "nis2", "control"),
      makeConcept("c3", "gdpr", "principle"),
    ];
    const { types } = computeFacets(concepts);
    expect(types.get("control")).toBe(2);
    expect(types.get("principle")).toBe(1);
  });

  it("handles empty results", () => {
    const { frameworks, types } = computeFacets([]);
    expect(frameworks.size).toBe(0);
    expect(types.size).toBe(0);
  });
});
