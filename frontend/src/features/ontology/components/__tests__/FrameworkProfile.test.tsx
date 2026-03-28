import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { FrameworkProfile } from "../FrameworkProfile";
import type { Framework, Concept, Relationship, FrameworkStats } from "../../types";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string, fallback?: string) => fallback ?? key,
  }),
}));

vi.mock("../../api", () => ({
  useFrameworkProof: vi.fn(() => ({ isLoading: true, isError: false, data: undefined })),
}));

const FW: Framework = {
  id: "iso31000",
  name: "ISO 31000",
  version: "2018",
  description: "Risk management guidelines",
  source_url: "https://iso.org/31000",
  created_at: "",
  updated_at: "",
  verification_status: "verified",
  verification_date: "2025-01-15",
  verification_source: "https://example.com/proof",
  verification_notes: null,
};

const FW_B: Framework = {
  id: "nist-csf",
  name: "NIST CSF",
  version: "2.0",
  description: null,
  source_url: null,
  created_at: "",
  updated_at: "",
  verification_status: null,
  verification_date: null,
  verification_source: null,
  verification_notes: null,
};

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return React.createElement(QueryClientProvider, { client: queryClient }, children);
  };
}

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

const DEFAULT_PROPS = {
  concepts: CONCEPTS,
  relationships: RELATIONSHIPS,
  stats: STATS,
  frameworks: [FW, FW_B],
  conceptToFramework: new Map([["c1", "iso31000"], ["c2", "iso31000"], ["c3", "iso31000"], ["c1-1", "iso31000"], ["ext1", "nist-csf"]]),
  isLoading: false,
};

describe("FrameworkProfile – verification UI", () => {
  it("renders VerificationBadge when verification_status is non-null", () => {
    render(
      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
      { wrapper: createWrapper() }
    );
    // FW has verification_status: "verified" — badge should be present
    const badge = document.querySelector("[aria-label]");
    expect(badge).not.toBeNull();
  });

  it("renders VerificationBadge in fallback style when verification_status is null", () => {
    render(
      <FrameworkProfile framework={FW_B} {...DEFAULT_PROPS} />,
      { wrapper: createWrapper() }
    );
    // FW_B has verification_status: null — badge renders unknown/fallback
    const badge = document.querySelector("[aria-label]");
    expect(badge).not.toBeNull();
  });

  it("renders View Proof button when verification_status is non-null", () => {
    render(
      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
      { wrapper: createWrapper() }
    );
    expect(screen.getByRole("button", { name: /view proof/i })).toBeInTheDocument();
  });

  it("does not render View Proof button when verification_status is null", () => {
    render(
      <FrameworkProfile framework={FW_B} {...DEFAULT_PROPS} />,
      { wrapper: createWrapper() }
    );
    expect(screen.queryByRole("button", { name: /view proof/i })).toBeNull();
  });

  it("mounts ProofPanel after clicking View Proof button", () => {
    render(
      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
      { wrapper: createWrapper() }
    );
    fireEvent.click(screen.getByRole("button", { name: /view proof/i }));
    // ProofPanel renders loading skeletons (mocked useFrameworkProof returns isLoading: true)
    const skeletons = document.querySelectorAll(".animate-pulse");
    expect(skeletons.length).toBeGreaterThanOrEqual(1);
  });

  it("hides ProofPanel when switching to a framework with no verification status", () => {
    const { rerender } = render(
      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
      { wrapper: createWrapper() }
    );
    fireEvent.click(screen.getByRole("button", { name: /view proof/i }));
    expect(document.querySelectorAll(".animate-pulse").length).toBeGreaterThanOrEqual(1);

    // Switch to FW_B (verification_status: null)
    rerender(
      <QueryClientProvider client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}>
        <FrameworkProfile framework={FW_B} {...DEFAULT_PROPS} />
      </QueryClientProvider>
    );
    expect(screen.queryByRole("button", { name: /view proof/i })).toBeNull();
    expect(screen.queryByRole("button", { name: /hide proof/i })).toBeNull();
  });

  it("resets proof panel when switching between two verified frameworks (useEffect reset)", () => {
    const FW_C: Framework = {
      ...FW_B,
      id: "iso27001",
      name: "ISO 27001",
      verification_status: "structure-verified",
    };
    const { rerender } = render(
      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
      { wrapper: createWrapper() }
    );
    fireEvent.click(screen.getByRole("button", { name: /view proof/i }));
    // Panel is open — "Hide Proof" button is visible
    expect(screen.getByRole("button", { name: /hide proof/i })).toBeInTheDocument();

    // Switch to FW_C (also has verification_status, but different id)
    rerender(
      <QueryClientProvider client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}>
        <FrameworkProfile framework={FW_C} {...{ ...DEFAULT_PROPS, frameworks: [FW, FW_B, FW_C] }} />
      </QueryClientProvider>
    );
    // showProof should have reset — "View Proof" visible, not "Hide Proof"
    expect(screen.getByRole("button", { name: /view proof/i })).toBeInTheDocument();
    expect(screen.queryByRole("button", { name: /hide proof/i })).toBeNull();
  });
});
