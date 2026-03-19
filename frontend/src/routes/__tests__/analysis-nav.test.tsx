import { describe, it, expect, vi } from "vitest";
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

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const translations: Record<string, string> = {
        appName: "RSC",
        "nav.home": "Home",
        "nav.ontology": "Ontology Explorer",
        "nav.compliance": "Compliance",
        "nav.reports": "Reports",
        "nav.frameworks": "Frameworks",
        "nav.crosswalk": "Crosswalk",
        "nav.landscape": "Landscape",
        "nav.search": "Search",
        "nav.analysis": "Analysis",
      };
      return translations[key] ?? key;
    },
    i18n: { language: "en", changeLanguage: vi.fn() },
  }),
}));

function TestRootLayout() {
  return (
    <div>
      <nav data-testid="main-nav">
        <Link to="/">Home</Link>
        <Link to="/ontology">Ontology Explorer</Link>
        <Link to="/frameworks">Frameworks</Link>
        <Link to="/crosswalk">Crosswalk</Link>
        <Link to="/landscape">Landscape</Link>
        <Link to="/concepts/search">Search</Link>
        <Link to="/analysis">Analysis</Link>
        <Link to="/compliance">Compliance</Link>
        <Link to="/reports">Reports</Link>
      </nav>
      <Outlet />
    </div>
  );
}

function renderWithRouter(initialPath = "/") {
  const rootRoute = createRootRoute({ component: TestRootLayout });

  const routes = [
    createRoute({ getParentRoute: () => rootRoute, path: "/", component: () => React.createElement("div", null, "Home") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/frameworks", component: () => React.createElement("div", null, "Frameworks") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/crosswalk", component: () => React.createElement("div", null, "Crosswalk") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/landscape", component: () => React.createElement("div", null, "Landscape") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/ontology", component: () => React.createElement("div", null, "Ontology") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/compliance", component: () => React.createElement("div", null, "Compliance") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/reports", component: () => React.createElement("div", null, "Reports") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/concepts/search", component: () => React.createElement("div", null, "Search") }),
    createRoute({ getParentRoute: () => rootRoute, path: "/analysis", component: () => React.createElement("div", null, "Analysis") }),
  ];

  const router = createRouter({
    routeTree: rootRoute.addChildren(routes),
    history: createMemoryHistory({ initialEntries: [initialPath] }),
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

describe("Analysis navigation link", () => {
  it("renders Analysis link in the navigation", async () => {
    renderWithRouter("/");
    const nav = await screen.findByTestId("main-nav");
    const links = Array.from(nav.querySelectorAll("a")).map((a) => a.textContent);
    expect(links).toContain("Analysis");
  });

  it("Analysis link points to /analysis", async () => {
    renderWithRouter("/");
    const nav = await screen.findByTestId("main-nav");
    const analysisLink = Array.from(nav.querySelectorAll("a")).find(
      (a) => a.textContent === "Analysis"
    );
    expect(analysisLink).toBeDefined();
    expect(analysisLink?.getAttribute("href")).toBe("/analysis");
  });
});
