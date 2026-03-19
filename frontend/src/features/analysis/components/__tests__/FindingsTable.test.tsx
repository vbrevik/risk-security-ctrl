import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { FindingsTable } from "../FindingsTable";
import type { FindingType } from "../../types";
import { makeFinding } from "../../test-utils/factories";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

const defaultProps = {
  findings: [
    makeFinding({ id: "f1" }),
    makeFinding({ id: "f2", concept_code: null, concept_name: null }),
  ],
  expandedIds: new Set<string>(),
  onToggleExpand: vi.fn(),
  frameworkIds: ["iso-31000", "nist-csf"],
  filters: {} as { framework_id?: string; finding_type?: FindingType; priority?: number },
  onFilterChange: vi.fn(),
  page: 1,
  totalPages: 3,
  onPageChange: vi.fn(),
};

describe("FindingsTable", () => {
  it("renders table with correct column headers", () => {
    render(<FindingsTable {...defaultProps} />);
    expect(screen.getByText("findings.columns.conceptCode")).toBeInTheDocument();
    expect(screen.getByText("findings.columns.conceptName")).toBeInTheDocument();
    expect(screen.getByText("findings.columns.framework")).toBeInTheDocument();
    expect(screen.getByText("findings.columns.type")).toBeInTheDocument();
    expect(screen.getByText("findings.columns.priority")).toBeInTheDocument();
    expect(screen.getByText("findings.columns.confidence")).toBeInTheDocument();
  });

  it("renders a row for each finding", () => {
    const { container } = render(<FindingsTable {...defaultProps} />);
    // Each finding has a TableRow in tbody (not counting expanded rows)
    const rows = container.querySelectorAll("tbody tr");
    expect(rows.length).toBe(2);
  });

  it("displays concept_code or dash fallback when null", () => {
    render(<FindingsTable {...defaultProps} />);
    expect(screen.getByText("C-001")).toBeInTheDocument();
    expect(screen.getAllByText("\u2014").length).toBeGreaterThan(0);
  });

  it("displays confidence as percentage", () => {
    render(<FindingsTable {...defaultProps} />);
    expect(screen.getAllByText("85%").length).toBeGreaterThan(0);
  });

  it("expand button toggles row expansion", () => {
    const onToggle = vi.fn();
    render(<FindingsTable {...defaultProps} onToggleExpand={onToggle} />);
    const expandButtons = screen.getAllByRole("button", {
      name: "findings.expand",
    });
    fireEvent.click(expandButtons[0]);
    expect(onToggle).toHaveBeenCalledWith("f1");
  });

  it("expanded row shows evidence and recommendation", () => {
    render(
      <FindingsTable
        {...defaultProps}
        expandedIds={new Set(["f1"])}
      />
    );
    expect(screen.getByText("Some evidence")).toBeInTheDocument();
    expect(screen.getByText("Fix this")).toBeInTheDocument();
    expect(screen.getByText("Definition of control")).toBeInTheDocument();
  });

  it("expand button has aria-expanded attribute", () => {
    render(
      <FindingsTable
        {...defaultProps}
        expandedIds={new Set(["f1"])}
      />
    );
    const expandedBtn = screen.getByRole("button", {
      name: "findings.collapse",
    });
    expect(expandedBtn.getAttribute("aria-expanded")).toBe("true");
  });

  it("previous button disabled on page 1", () => {
    render(<FindingsTable {...defaultProps} page={1} />);
    const prevBtn = screen.getByText("list.pagination.previous");
    expect(prevBtn.closest("button")).toBeDisabled();
  });

  it("next button disabled on last page", () => {
    render(<FindingsTable {...defaultProps} page={3} totalPages={3} />);
    const nextBtn = screen.getByText("list.pagination.next");
    expect(nextBtn.closest("button")).toBeDisabled();
  });

  it("clicking Next calls onPageChange with page + 1", () => {
    const onPage = vi.fn();
    render(<FindingsTable {...defaultProps} onPageChange={onPage} />);
    fireEvent.click(screen.getByText("list.pagination.next"));
    expect(onPage).toHaveBeenCalledWith(2);
  });

  it("clicking Previous calls onPageChange with page - 1", () => {
    const onPage = vi.fn();
    render(<FindingsTable {...defaultProps} page={2} onPageChange={onPage} />);
    fireEvent.click(screen.getByText("list.pagination.previous"));
    expect(onPage).toHaveBeenCalledWith(1);
  });

  it("concept name cell is clickable when onConceptClick is provided", () => {
    const onClick = vi.fn();
    render(<FindingsTable {...defaultProps} onConceptClick={onClick} />);
    const btn = screen.getByRole("button", { name: "Control One" });
    expect(btn).toBeInTheDocument();
  });

  it("clicking concept name fires onConceptClick with concept_id", () => {
    const onClick = vi.fn();
    render(<FindingsTable {...defaultProps} onConceptClick={onClick} />);
    fireEvent.click(screen.getByRole("button", { name: "Control One" }));
    expect(onClick).toHaveBeenCalledWith("c1");
  });

  it("concept code cell shows dash when concept_code is null", () => {
    const onClick = vi.fn();
    render(<FindingsTable {...defaultProps} onConceptClick={onClick} />);
    // Second finding has concept_code: null — should show dash, not a button
    const dashes = screen.getAllByText("\u2014");
    expect(dashes.length).toBeGreaterThan(0);
  });

  it("no error when onConceptClick is not provided", () => {
    expect(() => render(<FindingsTable {...defaultProps} />)).not.toThrow();
  });
});
