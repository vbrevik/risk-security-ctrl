import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import { FrameworkProfile } from "../FrameworkProfile";
import type { Framework, Concept, Relationship, FrameworkStats } from "../../types";

const FW: Framework = {
  id: "iso31000",
  name: "ISO 31000",
  version: "2018",
  description: "Risk management guidelines",
  source_url: "https://iso.org/31000",
  created_at: "",
  updated_at: "",
};

const FW_B: Framework = {
  id: "nist-csf",
  name: "NIST CSF",
  version: "2.0",
  description: null,
  source_url: null,
  created_at: "",
  updated_at: "",
};

function makeConcept(id: string, type: string, parentId: string | null = null): Concept {
  return {
    id,
    framework_id: "iso31000",
    parent_id: parentId,
    concept_type: type,
    code: id.toUpperCase(),
    name_en: `Concept ${id}`,
    name_nb: null,
    definition_en: null,
    definition_nb: null,
    source_reference: null,
    sort_order: null,
    created_at: "",
    updated_at: "",
  };
}

const CONCEPTS: Concept[] = [
  makeConcept("c1", "principle"),
  makeConcept("c2", "principle"),
  makeConcept("c3", "process"),
  makeConcept("c1-1", "guideline", "c1"),
];

const RELATIONSHIPS: Relationship[] = [
  { id: "r1", source_concept_id: "c1", target_concept_id: "ext1", relationship_type: "maps_to", description: null, created_at: null },
  { id: "r2", source_concept_id: "c2", target_concept_id: "ext1", relationship_type: "related_to", description: null, created_at: null },
];

const STATS: FrameworkStats = {
  conceptCount: 4,
  conceptTypes: { principle: 2, process: 1, guideline: 1 },
  connectedFrameworks: 1,
  relationshipCount: 2,
};

describe("FrameworkProfile", () => {
  it("renders framework name, version, description, source link", () => {
    render(
      <FrameworkProfile
        framework={FW}
        concepts={CONCEPTS}
        relationships={RELATIONSHIPS}
        stats={STATS}
        frameworks={[FW, FW_B]}
        conceptToFramework={new Map([["c1", "iso31000"], ["c2", "iso31000"], ["c3", "iso31000"], ["c1-1", "iso31000"], ["ext1", "nist-csf"]])}
        isLoading={false}
      />
    );
    expect(screen.getByText("ISO 31000")).toBeInTheDocument();
    expect(screen.getByText("2018")).toBeInTheDocument();
    expect(screen.getByText("Risk management guidelines")).toBeInTheDocument();
    const sourceLink = screen.getByRole("link", { name: /source/i });
    expect(sourceLink).toHaveAttribute("href", "https://iso.org/31000");
    expect(sourceLink).toHaveAttribute("target", "_blank");
  });

  it("renders 4 stat boxes with correct values", () => {
    const { container } = render(
      <FrameworkProfile
        framework={FW}
        concepts={CONCEPTS}
        relationships={RELATIONSHIPS}
        stats={STATS}
        frameworks={[FW, FW_B]}
        conceptToFramework={new Map([["c1", "iso31000"], ["c2", "iso31000"], ["c3", "iso31000"], ["c1-1", "iso31000"], ["ext1", "nist-csf"]])}
        isLoading={false}
      />
    );
    const statBoxes = container.querySelectorAll(".feature-card");
    expect(statBoxes).toHaveLength(4);
    const values = Array.from(statBoxes).map(
      (box) => box.querySelector(".stat-number")?.textContent
    );
    expect(values).toEqual(["4", "3", "1", "2"]);
  });

  it("renders concept type breakdown bar", () => {
    const { container } = render(
      <FrameworkProfile
        framework={FW}
        concepts={CONCEPTS}
        relationships={RELATIONSHIPS}
        stats={STATS}
        frameworks={[FW, FW_B]}
        conceptToFramework={new Map([["c1", "iso31000"], ["c2", "iso31000"], ["c3", "iso31000"], ["c1-1", "iso31000"], ["ext1", "nist-csf"]])}
        isLoading={false}
      />
    );
    // Legend items for each type (use getAllByText since types appear in both legend and badges)
    expect(screen.getAllByText(/principle/i).length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText(/process/i).length).toBeGreaterThanOrEqual(1);
    expect(screen.getAllByText(/guideline/i).length).toBeGreaterThanOrEqual(1);
    // Stacked bar should exist
    const bar = container.querySelector("[data-testid='type-breakdown-bar']");
    expect(bar).toBeInTheDocument();
  });

  it("renders empty state when no framework selected", () => {
    render(
      <FrameworkProfile
        framework={null}
        concepts={[]}
        relationships={[]}
        stats={null}
        frameworks={[]}
        conceptToFramework={new Map()}
        isLoading={false}
      />
    );
    expect(screen.getByText(/select a framework/i)).toBeInTheDocument();
  });
});
