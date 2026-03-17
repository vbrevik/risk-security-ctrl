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

// Mock i18next
vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => {
      const translations: Record<string, string> = {
        appName: "RSC",
        "nav.home": "Home",
        "nav.ontology": "Ontology",
        "nav.compliance": "Compliance",
        "nav.reports": "Reports",
      };
      return translations[key] ?? key;
    },
    i18n: { language: "en", changeLanguage: vi.fn() },
  }),
}));

// Build a minimal layout that mirrors __root.tsx navigation structure
function TestRootLayout() {
  return (
    <div>
      <nav data-testid="primary-nav">
        <Link to="/">Home</Link>
        <Link to="/ontology">Ontology</Link>
        <Link to="/compliance">Compliance</Link>
        <Link to="/reports">Reports</Link>
      </nav>
      <nav data-testid="secondary-nav">
        <Link to="/frameworks">Frameworks</Link>
        <Link to="/crosswalk">Crosswalk</Link>
        <Link to="/landscape">Landscape</Link>
        <Link to="/concepts/search">Search</Link>
      </nav>
      <Outlet />
    </div>
  );
}

function renderWithRouter(initialPath = "/") {
  const rootRoute = createRootRoute({ component: TestRootLayout });

  const indexRoute = createRoute({ getParentRoute: () => rootRoute, path: "/" , component: () => React.createElement("div", null, "Home") });
  const frameworksRoute = createRoute({ getParentRoute: () => rootRoute, path: "/frameworks" , component: () => React.createElement("div", null, "Frameworks") });
  const crosswalkRoute = createRoute({ getParentRoute: () => rootRoute, path: "/crosswalk" , component: () => React.createElement("div", null, "Crosswalk") });
  const landscapeRoute = createRoute({ getParentRoute: () => rootRoute, path: "/landscape" , component: () => React.createElement("div", null, "Landscape") });
  const ontologyRoute = createRoute({ getParentRoute: () => rootRoute, path: "/ontology" , component: () => React.createElement("div", null, "Ontology") });
  const complianceRoute = createRoute({ getParentRoute: () => rootRoute, path: "/compliance" , component: () => React.createElement("div", null, "Compliance") });
  const reportsRoute = createRoute({ getParentRoute: () => rootRoute, path: "/reports" , component: () => React.createElement("div", null, "Reports") });
  const conceptsSearchRoute = createRoute({ getParentRoute: () => rootRoute, path: "/concepts/search" , component: () => React.createElement("div", null, "Search") });

  const routeTree = rootRoute.addChildren([
    indexRoute,
    frameworksRoute,
    crosswalkRoute,
    landscapeRoute,
    ontologyRoute,
    complianceRoute,
    reportsRoute,
    conceptsSearchRoute,
  ]);

  const router = createRouter({
    routeTree,
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

describe("Root Navigation", () => {
  it("secondary nav renders 4 links", async () => {
    renderWithRouter("/");
    const secondaryNav = await screen.findByTestId("secondary-nav");
    const links = secondaryNav.querySelectorAll("a");
    expect(links).toHaveLength(4);
    expect(links[0].textContent).toBe("Frameworks");
    expect(links[1].textContent).toBe("Crosswalk");
    expect(links[2].textContent).toBe("Landscape");
    expect(links[3].textContent).toBe("Search");
  });

  it("secondary nav links have correct href values", async () => {
    renderWithRouter("/");
    const secondaryNav = await screen.findByTestId("secondary-nav");
    const links = secondaryNav.querySelectorAll("a");
    expect(links[0].getAttribute("href")).toBe("/frameworks");
    expect(links[1].getAttribute("href")).toBe("/crosswalk");
    expect(links[2].getAttribute("href")).toBe("/landscape");
    expect(links[3].getAttribute("href")).toBe("/concepts/search");
  });

  it("primary nav contains only Home, Ontology, Compliance, Reports", async () => {
    renderWithRouter("/");
    const primaryNav = await screen.findByTestId("primary-nav");
    const links = primaryNav.querySelectorAll("a");
    const linkTexts = Array.from(links).map((l) => l.textContent);
    expect(linkTexts).toEqual(["Home", "Ontology", "Compliance", "Reports"]);
    expect(linkTexts).not.toContain("Frameworks");
    expect(linkTexts).not.toContain("Crosswalk");
  });

  it("active link gets active class on matching route", async () => {
    renderWithRouter("/frameworks");
    const secondaryNav = await screen.findByTestId("secondary-nav");
    const frameworksLink = secondaryNav.querySelector('a[href="/frameworks"]');
    expect(frameworksLink?.classList.contains("active")).toBe(true);
  });
});
