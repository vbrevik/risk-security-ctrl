import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import {
  createRootRoute,
  createRoute,
  createRouter,
  createMemoryHistory,
  RouterProvider,
  Link,
  Outlet,
} from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { useAnalyses } from "@/features/analysis/api";
import { AnalysisList } from "@/features/analysis/components/AnalysisList";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

vi.mock("@/features/analysis/api", () => ({
  useAnalyses: vi.fn(),
}));

const mockedUseAnalyses = vi.mocked(useAnalyses);

/**
 * Simplified page component that mirrors the route's behavior
 * without needing TanStack Router's Route.useSearch().
 */
function TestAnalysisListPage() {
  const { data, isLoading, isError, refetch } = useAnalyses({ page: 1, limit: 12 });
  return (
    <div>
      <h1>list.title</h1>
      <Link to="/analysis/create">list.newAnalysis</Link>
      <AnalysisList
        analyses={data?.data}
        isLoading={isLoading}
        isError={isError}
        onRetry={refetch}
      />
    </div>
  );
}

function renderWithRouter() {
  const rootRoute = createRootRoute({
    component: () => React.createElement("div", null, React.createElement(Outlet)),
  });

  const pageRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/analysis",
    component: TestAnalysisListPage,
  });

  const createRoute2 = createRoute({
    getParentRoute: () => rootRoute,
    path: "/analysis/create",
    component: () => React.createElement("div", null, "Create"),
  });

  const detailRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/analysis/$id",
    component: () => React.createElement("div", null, "Detail"),
  });

  const router = createRouter({
    routeTree: rootRoute.addChildren([pageRoute, createRoute2, detailRoute]),
    history: createMemoryHistory({ initialEntries: ["/analysis"] }),
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

describe("Analysis list page", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("renders page title and New Analysis button", async () => {
    mockedUseAnalyses.mockReturnValue({
      data: { data: [], total: 0, page: 1, limit: 12, total_pages: 0 },
      isLoading: false,
      isError: false,
      refetch: vi.fn(),
    } as any);

    renderWithRouter();
    expect(await screen.findByText("list.title")).toBeDefined();
    const newButtons = screen.getAllByText("list.newAnalysis");
    expect(newButtons.length).toBeGreaterThan(0);
  });

  it("renders empty state when no analyses", async () => {
    mockedUseAnalyses.mockReturnValue({
      data: { data: [], total: 0, page: 1, limit: 12, total_pages: 0 },
      isLoading: false,
      isError: false,
      refetch: vi.fn(),
    } as any);

    renderWithRouter();
    expect(await screen.findByText("list.empty.title")).toBeDefined();
  });
});
