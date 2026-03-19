import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import {
  createRootRoute,
  createRoute,
  createRouter,
  createMemoryHistory,
  RouterProvider,
  Outlet,
} from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { AnalysisCard } from "../AnalysisCard";
import type { AnalysisListItem } from "../../types";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

function renderWithRouter(ui: React.ReactElement) {
  const rootRoute = createRootRoute({
    component: () => React.createElement("div", null, React.createElement(Outlet)),
  });

  const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component: () => ui,
  });

  const analysisRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/analysis/$id",
    component: () => React.createElement("div", null, "Detail"),
  });

  const router = createRouter({
    routeTree: rootRoute.addChildren([indexRoute, analysisRoute]),
    history: createMemoryHistory({ initialEntries: ["/"] }),
  });

  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } });

  return render(
    React.createElement(
      QueryClientProvider,
      { client: queryClient },
      React.createElement(RouterProvider, { router: router as any })
    )
  );
}

const mockItem: AnalysisListItem = {
  id: "abc-123",
  name: "Test Analysis",
  description: "A test description",
  input_type: "text",
  status: "completed",
  error_message: null,
  processing_time_ms: 2300,
  created_at: "2025-01-15T10:00:00Z",
  updated_at: "2025-01-15T10:05:00Z",
};

describe("AnalysisCard", () => {
  it("renders analysis name and status badge", async () => {
    renderWithRouter(<AnalysisCard analysis={mockItem} />);
    expect(await screen.findByText("Test Analysis")).toBeDefined();
    expect(screen.getByText("status.completed")).toBeDefined();
  });

  it("links to /analysis/{id}", async () => {
    renderWithRouter(<AnalysisCard analysis={mockItem} />);
    const link = await screen.findByRole("link");
    expect(link.getAttribute("href")).toBe("/analysis/abc-123");
  });
});
