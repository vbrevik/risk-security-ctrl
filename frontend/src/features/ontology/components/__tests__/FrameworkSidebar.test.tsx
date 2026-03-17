import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import React from "react";
import { FrameworkSidebar } from "../FrameworkSidebar";
import type { Framework, FrameworkStats } from "../../types";

function makeFramework(id: string, name: string): Framework {
  return { id, name, version: "1.0", description: null, source_url: null, created_at: "", updated_at: "" };
}

const FRAMEWORKS: Framework[] = [
  makeFramework("iso31000", "ISO 31000"),
  makeFramework("iso31010", "ISO 31010"),
  makeFramework("eu-ai-act", "EU AI Act"),
  makeFramework("gdpr", "GDPR"),
  makeFramework("zero-trust", "Zero Trust"),
];

const STATS = new Map<string, FrameworkStats>([
  ["iso31000", { conceptCount: 42, conceptTypes: { principle: 10, process: 32 }, connectedFrameworks: 3, relationshipCount: 15 }],
  ["iso31010", { conceptCount: 28, conceptTypes: { technique: 28 }, connectedFrameworks: 2, relationshipCount: 8 }],
  ["eu-ai-act", { conceptCount: 55, conceptTypes: { requirement: 55 }, connectedFrameworks: 4, relationshipCount: 20 }],
  ["gdpr", { conceptCount: 30, conceptTypes: { article: 30 }, connectedFrameworks: 1, relationshipCount: 5 }],
  ["zero-trust", { conceptCount: 18, conceptTypes: { pillar: 18 }, connectedFrameworks: 2, relationshipCount: 10 }],
]);

describe("FrameworkSidebar", () => {
  it("renders all frameworks grouped by domain", () => {
    render(
      <FrameworkSidebar
        frameworks={FRAMEWORKS}
        stats={STATS}
        selectedId={null}
        onSelect={vi.fn()}
        isLoading={false}
      />
    );
    // Should render domain headings
    expect(screen.getByText(/Risk & Security/i)).toBeInTheDocument();
    expect(screen.getByText(/AI Governance/i)).toBeInTheDocument();
    expect(screen.getByText(/EU Regulations/i)).toBeInTheDocument();
    expect(screen.getByText(/Architecture/i)).toBeInTheDocument();

    // Should render framework names
    expect(screen.getByText("ISO 31000")).toBeInTheDocument();
    expect(screen.getByText("EU AI Act")).toBeInTheDocument();
    expect(screen.getByText("GDPR")).toBeInTheDocument();
    expect(screen.getByText("Zero Trust")).toBeInTheDocument();
  });

  it("each framework shows concept count", () => {
    render(
      <FrameworkSidebar
        frameworks={FRAMEWORKS}
        stats={STATS}
        selectedId={null}
        onSelect={vi.fn()}
        isLoading={false}
      />
    );
    expect(screen.getByText("42")).toBeInTheDocument();
    expect(screen.getByText("55")).toBeInTheDocument();
  });

  it("clicking a framework calls the selection callback", async () => {
    const onSelect = vi.fn();
    const user = userEvent.setup();
    render(
      <FrameworkSidebar
        frameworks={FRAMEWORKS}
        stats={STATS}
        selectedId={null}
        onSelect={onSelect}
        isLoading={false}
      />
    );
    await user.click(screen.getByText("GDPR"));
    expect(onSelect).toHaveBeenCalledWith("gdpr");
  });

  it("active framework is visually highlighted", () => {
    const { container } = render(
      <FrameworkSidebar
        frameworks={FRAMEWORKS}
        stats={STATS}
        selectedId="iso31000"
        onSelect={vi.fn()}
        isLoading={false}
      />
    );
    const activeItem = container.querySelector("[data-active='true']");
    expect(activeItem).toBeInTheDocument();
    expect(activeItem?.textContent).toContain("ISO 31000");
  });

  it("renders loading skeleton when data is pending", () => {
    const { container } = render(
      <FrameworkSidebar
        frameworks={[]}
        stats={new Map()}
        selectedId={null}
        onSelect={vi.fn()}
        isLoading={true}
      />
    );
    const skeletons = container.querySelectorAll(".animate-pulse");
    expect(skeletons.length).toBeGreaterThan(0);
  });
});
