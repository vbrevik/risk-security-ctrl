import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ConceptDrawer } from "../ConceptDrawer";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({ t: (key: string) => key }),
}));

const mockRefetch = vi.fn();

vi.mock("@/features/ontology/api", () => ({
  useConceptRelationships: vi.fn(),
}));

import { useConceptRelationships } from "@/features/ontology/api";
const mockedHook = vi.mocked(useConceptRelationships);

const mockConcept = {
  id: "test-concept",
  framework_id: "iso-31000",
  parent_id: null,
  concept_type: "control",
  code: "C-001",
  name_en: "Test Control",
  name_nb: null,
  definition_en: "A test definition for this concept.",
  definition_nb: null,
  source_reference: null,
  sort_order: 1,
  created_at: "2025-01-01",
  updated_at: "2025-01-01",
  related_concepts: [
    {
      relationship_id: "r1",
      relationship_type: "maps_to",
      concept_id: "c2",
      concept_framework_id: "nist-csf",
      concept_name_en: "Related NIST Control",
      concept_name_nb: null,
      direction: "outgoing" as const,
    },
  ],
};

describe("ConceptDrawer", () => {
  it("renders nothing when conceptId is null", () => {
    mockedHook.mockReturnValue({
      data: undefined,
      isLoading: false,
      isError: false,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId={null} onClose={vi.fn()} />);
    expect(screen.queryByText("detail.conceptPanel.title")).not.toBeInTheDocument();
  });

  it("renders Sheet content when conceptId is non-null", () => {
    mockedHook.mockReturnValue({
      data: mockConcept,
      isLoading: false,
      isError: false,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId="test-concept" onClose={vi.fn()} />);
    expect(screen.getByText("detail.conceptPanel.title")).toBeInTheDocument();
  });

  it("displays concept name and type from fetched data", () => {
    mockedHook.mockReturnValue({
      data: mockConcept,
      isLoading: false,
      isError: false,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId="test-concept" onClose={vi.fn()} />);
    expect(screen.getByText("Test Control")).toBeInTheDocument();
    expect(screen.getByText("control")).toBeInTheDocument();
    expect(screen.getByText("iso-31000")).toBeInTheDocument();
  });

  it("displays concept definition", () => {
    mockedHook.mockReturnValue({
      data: mockConcept,
      isLoading: false,
      isError: false,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId="test-concept" onClose={vi.fn()} />);
    expect(screen.getByText("A test definition for this concept.")).toBeInTheDocument();
  });

  it("shows loading skeleton when data is loading", () => {
    mockedHook.mockReturnValue({
      data: undefined,
      isLoading: true,
      isError: false,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId="test-concept" onClose={vi.fn()} />);
    const loader = screen.getByLabelText("detail.conceptPanel.loading");
    expect(loader).toBeInTheDocument();
  });

  it("shows error state with retry button when fetch fails", () => {
    mockedHook.mockReturnValue({
      data: undefined,
      isLoading: false,
      isError: true,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId="test-concept" onClose={vi.fn()} />);
    expect(screen.getByText("detail.conceptPanel.error")).toBeInTheDocument();
    fireEvent.click(screen.getByText("detail.conceptPanel.retry"));
    expect(mockRefetch).toHaveBeenCalled();
  });

  it("Open in Explorer link has correct href and target", () => {
    mockedHook.mockReturnValue({
      data: mockConcept,
      isLoading: false,
      isError: false,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId="test-concept" onClose={vi.fn()} />);
    const link = screen.getByText("detail.conceptPanel.openInExplorer").closest("a");
    expect(link?.getAttribute("href")).toBe("/ontology?concept=test-concept");
    expect(link?.getAttribute("target")).toBe("_blank");
  });

  it("shows cross-framework mappings", () => {
    mockedHook.mockReturnValue({
      data: mockConcept,
      isLoading: false,
      isError: false,
      refetch: mockRefetch,
    } as ReturnType<typeof useConceptRelationships>);

    render(<ConceptDrawer conceptId="test-concept" onClose={vi.fn()} />);
    expect(screen.getByText("Related NIST Control")).toBeInTheDocument();
  });
});
