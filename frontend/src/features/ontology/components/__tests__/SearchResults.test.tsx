import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import { SearchResults } from "../SearchResults";
import type { Framework, Concept } from "../../types";
import {
  createRootRoute,
  createRoute,
  createRouter,
  createMemoryHistory,
  RouterProvider,
  Outlet,
} from "@tanstack/react-router";

const FRAMEWORKS: Framework[] = [
  { id: "gdpr", name: "GDPR", version: null, description: null, source_url: null, created_at: "", updated_at: "" },
  { id: "nis2", name: "NIS2", version: null, description: null, source_url: null, created_at: "", updated_at: "" },
];

function makeConcept(id: string, fwId: string, type: string): Concept {
  return {
    id, framework_id: fwId, parent_id: null, concept_type: type,
    code: "CODE-1", name_en: `Concept ${id}`, name_nb: null,
    definition_en: "A test definition for this concept that is fairly long to test truncation behavior",
    definition_nb: null, source_reference: null, sort_order: null, created_at: "", updated_at: "",
  };
}

function renderWithRouter(component: () => React.ReactElement) {
  const rootRoute = createRootRoute({ component: () => React.createElement(Outlet) });
  const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component,
  });
  const router = createRouter({
    routeTree: rootRoute.addChildren([indexRoute]),
    history: createMemoryHistory({ initialEntries: ["/"] }),
  });
  return render(React.createElement(RouterProvider, { router: router as any }));
}

describe("SearchResults", () => {
  it("groups results by framework", async () => {
    const grouped = [
      { frameworkId: "gdpr", frameworkName: "GDPR", concepts: [makeConcept("c1", "gdpr", "control")] },
      { frameworkId: "nis2", frameworkName: "NIS2", concepts: [makeConcept("c2", "nis2", "requirement")] },
    ];
    renderWithRouter(() => (
      <SearchResults groupedResults={grouped} query="test" frameworks={FRAMEWORKS} totalCount={2} />
    ));
    expect(await screen.findByText("GDPR")).toBeInTheDocument();
    expect(screen.getByText("NIS2")).toBeInTheDocument();
  });

  it("each result shows code, name, type pill", async () => {
    const grouped = [
      { frameworkId: "gdpr", frameworkName: "GDPR", concepts: [makeConcept("c1", "gdpr", "control")] },
    ];
    renderWithRouter(() => (
      <SearchResults groupedResults={grouped} query="concept" frameworks={FRAMEWORKS} totalCount={1} />
    ));
    expect(await screen.findByText("CODE-1")).toBeInTheDocument();
    expect(screen.getByText("control")).toBeInTheDocument();
  });

  it("shows empty state when no query", async () => {
    renderWithRouter(() => (
      <SearchResults groupedResults={[]} query="" frameworks={FRAMEWORKS} totalCount={0} />
    ));
    expect(await screen.findByText(/search across/i)).toBeInTheDocument();
  });

  it("shows no results message when query has no matches", async () => {
    renderWithRouter(() => (
      <SearchResults groupedResults={[]} query="xyznonexistent" frameworks={FRAMEWORKS} totalCount={0} />
    ));
    expect(await screen.findByText(/no concepts matching/i)).toBeInTheDocument();
  });
});
