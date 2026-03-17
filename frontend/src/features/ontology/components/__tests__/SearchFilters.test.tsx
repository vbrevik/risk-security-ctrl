import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { SearchFilters } from "../SearchFilters";

const FRAMEWORK_FACETS = [
  { id: "gdpr", name: "GDPR", count: 10 },
  { id: "nis2", name: "NIS2", count: 5 },
  { id: "dora", name: "DORA", count: 0 },
];

const TYPE_FACETS = [
  { type: "control", count: 8 },
  { type: "principle", count: 3 },
];

describe("SearchFilters", () => {
  it("renders framework checkboxes with counts", () => {
    render(
      <SearchFilters
        frameworkFacets={FRAMEWORK_FACETS}
        typeFacets={TYPE_FACETS}
        activeFrameworks={[]}
        activeTypes={[]}
        onToggleFramework={vi.fn()}
        onToggleType={vi.fn()}
      />
    );
    expect(screen.getByText("GDPR")).toBeInTheDocument();
    expect(screen.getByText("(10)")).toBeInTheDocument();
    expect(screen.getByText("NIS2")).toBeInTheDocument();
    expect(screen.getByText("(5)")).toBeInTheDocument();
  });

  it("only shows frameworks with results (count > 0)", () => {
    render(
      <SearchFilters
        frameworkFacets={FRAMEWORK_FACETS}
        typeFacets={TYPE_FACETS}
        activeFrameworks={[]}
        activeTypes={[]}
        onToggleFramework={vi.fn()}
        onToggleType={vi.fn()}
      />
    );
    expect(screen.queryByText("DORA")).not.toBeInTheDocument();
  });

  it("toggling a checkbox calls the filter callback", async () => {
    const onToggle = vi.fn();
    const user = userEvent.setup();
    render(
      <SearchFilters
        frameworkFacets={FRAMEWORK_FACETS}
        typeFacets={TYPE_FACETS}
        activeFrameworks={[]}
        activeTypes={[]}
        onToggleFramework={onToggle}
        onToggleType={vi.fn()}
      />
    );
    await user.click(screen.getByText("GDPR"));
    expect(onToggle).toHaveBeenCalledWith("gdpr");
  });

  it("renders concept type checkboxes with counts", () => {
    render(
      <SearchFilters
        frameworkFacets={FRAMEWORK_FACETS}
        typeFacets={TYPE_FACETS}
        activeFrameworks={[]}
        activeTypes={[]}
        onToggleFramework={vi.fn()}
        onToggleType={vi.fn()}
      />
    );
    expect(screen.getByText("control")).toBeInTheDocument();
    expect(screen.getByText("(8)")).toBeInTheDocument();
    expect(screen.getByText("principle")).toBeInTheDocument();
  });
});
