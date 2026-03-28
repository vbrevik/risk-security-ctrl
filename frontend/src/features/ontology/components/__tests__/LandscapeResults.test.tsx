import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { LandscapeResults } from "../LandscapeResults";
import type { Framework } from "../../types";

const FRAMEWORKS: Framework[] = [
  { id: "iso31000", name: "ISO 31000", version: null, description: "Risk management", source_url: null, created_at: "", updated_at: "", verification_status: null, verification_date: null, verification_source: null, verification_notes: null },
  { id: "gdpr", name: "GDPR", version: null, description: "Data protection", source_url: null, created_at: "", updated_at: "", verification_status: null, verification_date: null, verification_source: null, verification_notes: null },
  { id: "dora", name: "DORA", version: null, description: null, source_url: null, created_at: "", updated_at: "", verification_status: null, verification_date: null, verification_source: null, verification_notes: null },
];

const CONCEPT_COUNT = new Map([["iso31000", 42], ["gdpr", 30], ["dora", 10]]);
const CONCEPT_TO_FW = new Map([["c1", "iso31000"], ["c2", "gdpr"], ["c3", "dora"]]);

describe("LandscapeResults", () => {
  it("applicable frameworks shown with full styling", () => {
    const { container } = render(
      <LandscapeResults
        applicableFrameworkIds={["iso31000", "gdpr"]}
        frameworks={FRAMEWORKS}
        relationships={[]}
        conceptCountMap={CONCEPT_COUNT}
        conceptToFramework={CONCEPT_TO_FW}
      />
    );
    // Applicable frameworks should be in .feature-card elements
    const cards = container.querySelectorAll(".feature-card");
    // Summary banner + 2 applicable frameworks = 3 feature-cards
    expect(cards.length).toBeGreaterThanOrEqual(2);
    expect(screen.getByText("ISO 31000")).toBeInTheDocument();
    expect(screen.getByText("GDPR")).toBeInTheDocument();
  });

  it("non-applicable frameworks shown faded", () => {
    const { container } = render(
      <LandscapeResults
        applicableFrameworkIds={["iso31000"]}
        frameworks={FRAMEWORKS}
        relationships={[]}
        conceptCountMap={CONCEPT_COUNT}
        conceptToFramework={CONCEPT_TO_FW}
      />
    );
    const fadedSection = container.querySelector(".opacity-40");
    expect(fadedSection).toBeInTheDocument();
    // GDPR and DORA should be in the faded section
    expect(fadedSection?.textContent).toContain("GDPR");
    expect(fadedSection?.textContent).toContain("DORA");
  });

  it("summary banner shows correct counts", () => {
    render(
      <LandscapeResults
        applicableFrameworkIds={["iso31000", "gdpr"]}
        frameworks={FRAMEWORKS}
        relationships={[]}
        conceptCountMap={CONCEPT_COUNT}
        conceptToFramework={CONCEPT_TO_FW}
      />
    );
    const banner = screen.getByTestId("summary-banner");
    // 2 frameworks
    expect(banner.textContent).toContain("2");
    // 42 + 30 = 72 concepts
    expect(banner.textContent).toContain("72");
  });
});
