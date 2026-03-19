import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
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
import { AnalysisList } from "../AnalysisList";
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

  const createRoute2 = createRoute({
    getParentRoute: () => rootRoute,
    path: "/analysis/create",
    component: () => React.createElement("div", null, "Create"),
  });

  const router = createRouter({
    routeTree: rootRoute.addChildren([indexRoute, analysisRoute, createRoute2]),
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

const mockItems: AnalysisListItem[] = [
  {
    id: "a1", name: "First Analysis", description: null, input_type: "text",
    status: "completed", error_message: null, processing_time_ms: 100,
    created_at: "2025-01-01", updated_at: "2025-01-01",
  },
  {
    id: "a2", name: "Second Analysis", description: null, input_type: "pdf",
    status: "processing", error_message: null, processing_time_ms: null,
    created_at: "2025-01-02", updated_at: "2025-01-02",
  },
];

describe("AnalysisList", () => {
  it("renders loading skeleton while fetching", async () => {
    renderWithRouter(
      <AnalysisList analyses={undefined} isLoading={true} isError={false} />
    );
    // Skeleton divs have animate-pulse in their className attribute
    await vi.waitFor(() => {
      const el = document.querySelector('[class*="animate-pulse"]');
      expect(el).not.toBeNull();
    });
  });

  it("renders analysis cards when data loads", async () => {
    renderWithRouter(
      <AnalysisList analyses={mockItems} isLoading={false} isError={false} />
    );
    expect(await screen.findByText("First Analysis")).toBeDefined();
    expect(screen.getByText("Second Analysis")).toBeDefined();
  });

  it("renders empty state when no analyses exist", async () => {
    renderWithRouter(
      <AnalysisList analyses={[]} isLoading={false} isError={false} />
    );
    expect(await screen.findByText("list.empty.title")).toBeDefined();
  });

  it("renders error state with retry button on error", async () => {
    const onRetry = vi.fn();
    renderWithRouter(
      <AnalysisList analyses={undefined} isLoading={false} isError={true} onRetry={onRetry} />
    );
    expect(await screen.findByText("common.error")).toBeDefined();
    const retryButton = screen.getByRole("button");
    fireEvent.click(retryButton);
    expect(onRetry).toHaveBeenCalledOnce();
  });
});
