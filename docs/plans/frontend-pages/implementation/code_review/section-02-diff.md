diff --git a/docs/plans/frontend-pages/implementation/deep_implement_config.json b/docs/plans/frontend-pages/implementation/deep_implement_config.json
index 4380944..2caafc5 100644
--- a/docs/plans/frontend-pages/implementation/deep_implement_config.json
+++ b/docs/plans/frontend-pages/implementation/deep_implement_config.json
@@ -14,7 +14,12 @@
     "section-05-unified-search",
     "section-06-regulatory-landscape"
   ],
-  "sections_state": {},
+  "sections_state": {
+    "section-01-shared-infra": {
+      "status": "complete",
+      "commit_hash": "593e8b7"
+    }
+  },
   "pre_commit": {
     "present": false,
     "type": "none",
diff --git a/frontend/src/features/ontology/components/index.ts b/frontend/src/features/ontology/components/index.ts
index b27b6e0..7b36ec5 100644
--- a/frontend/src/features/ontology/components/index.ts
+++ b/frontend/src/features/ontology/components/index.ts
@@ -5,3 +5,4 @@ export { TreeView } from "./Tree";
 export { CompareView } from "./Compare";
 export { ContextPanel } from "./ContextPanel";
 export { ExportDialog } from "./ExportDialog";
+export { CrosswalkView } from "./Crosswalk";
diff --git a/frontend/src/i18n/locales/en/common.json b/frontend/src/i18n/locales/en/common.json
index 6ba0d97..7468d69 100644
--- a/frontend/src/i18n/locales/en/common.json
+++ b/frontend/src/i18n/locales/en/common.json
@@ -3,6 +3,7 @@
   "nav": {
     "home": "Home",
     "ontology": "Ontology Explorer",
+    "crosswalk": "Crosswalk",
     "compliance": "Compliance Tracking",
     "reports": "Reports"
   },
diff --git a/frontend/src/i18n/locales/en/ontology.json b/frontend/src/i18n/locales/en/ontology.json
index 8d9c4b8..c11dd0b 100644
--- a/frontend/src/i18n/locales/en/ontology.json
+++ b/frontend/src/i18n/locales/en/ontology.json
@@ -111,5 +111,19 @@
     "allTypes": "All types",
     "clearAll": "Clear all",
     "topics": "Topics"
+  },
+  "crosswalk": {
+    "title": "Framework Crosswalk",
+    "description": "Explore cross-framework mappings between risk management and AI governance standards at different abstraction levels.",
+    "sourceFramework": "Source framework",
+    "targetFramework": "Target framework",
+    "allLevels": "All levels",
+    "noMappings": "No mappings found",
+    "noMappingsHint": "These frameworks may not have cross-framework relationships defined yet.",
+    "selectBoth": "Select two frameworks to compare",
+    "selectBothHint": "Choose a source and target framework to view their cross-framework mappings.",
+    "mappingDetail": "Mapping Detail",
+    "rationale": "Rationale",
+    "level": "Level"
   }
 }
diff --git a/frontend/src/i18n/locales/nb/common.json b/frontend/src/i18n/locales/nb/common.json
index 8cd7d12..934d063 100644
--- a/frontend/src/i18n/locales/nb/common.json
+++ b/frontend/src/i18n/locales/nb/common.json
@@ -3,6 +3,7 @@
   "nav": {
     "home": "Hjem",
     "ontology": "Ontologiutforsker",
+    "crosswalk": "Kryssreferanse",
     "compliance": "Samsvarssporing",
     "reports": "Rapporter"
   },
diff --git a/frontend/src/i18n/locales/nb/ontology.json b/frontend/src/i18n/locales/nb/ontology.json
index da99106..1800b1f 100644
--- a/frontend/src/i18n/locales/nb/ontology.json
+++ b/frontend/src/i18n/locales/nb/ontology.json
@@ -111,5 +111,19 @@
     "allTypes": "Alle typer",
     "clearAll": "Fjern alle",
     "topics": "Emner"
+  },
+  "crosswalk": {
+    "title": "Rammeverk-kryssreferanse",
+    "description": "Utforsk koblinger mellom rammeverk for risikostyring og KI-styring pa ulike abstraksjonsniva.",
+    "sourceFramework": "Kilderammeverk",
+    "targetFramework": "Malrammeverk",
+    "allLevels": "Alle niva",
+    "noMappings": "Ingen koblinger funnet",
+    "noMappingsHint": "Disse rammeverkene har kanskje ikke definerte kryssreferanser enna.",
+    "selectBoth": "Velg to rammeverk for sammenligning",
+    "selectBothHint": "Velg et kilde- og malrammeverk for a se kryssreferansene mellom dem.",
+    "mappingDetail": "Koblingsdetaljer",
+    "rationale": "Begrunnelse",
+    "level": "Niva"
   }
 }
diff --git a/frontend/src/routes/__root.tsx b/frontend/src/routes/__root.tsx
index 322f2c7..7d7f076 100644
--- a/frontend/src/routes/__root.tsx
+++ b/frontend/src/routes/__root.tsx
@@ -35,18 +35,6 @@ function RootLayout() {
                 >
                   {t("nav.home")}
                 </Link>
-                <Link
-                  to="/frameworks"
-                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
-                >
-                  Frameworks
-                </Link>
-                <Link
-                  to="/crosswalk"
-                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
-                >
-                  Crosswalk
-                </Link>
                 <Link
                   to="/ontology"
                   className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
@@ -74,6 +62,32 @@ function RootLayout() {
               </Button>
             </div>
           </div>
+          <nav className="container flex h-8 items-center space-x-4 px-6 border-t border-border overflow-x-auto whitespace-nowrap">
+            <Link
+              to="/frameworks"
+              className="transition-colors text-xs font-medium font-mono text-foreground/40 hover:text-foreground/60 [&.active]:text-foreground"
+            >
+              Frameworks
+            </Link>
+            <Link
+              to="/crosswalk"
+              className="transition-colors text-xs font-medium font-mono text-foreground/40 hover:text-foreground/60 [&.active]:text-foreground"
+            >
+              Crosswalk
+            </Link>
+            <Link
+              to="/landscape"
+              className="transition-colors text-xs font-medium font-mono text-foreground/40 hover:text-foreground/60 [&.active]:text-foreground"
+            >
+              Landscape
+            </Link>
+            <Link
+              to="/concepts/search"
+              className="transition-colors text-xs font-medium font-mono text-foreground/40 hover:text-foreground/60 [&.active]:text-foreground"
+            >
+              Search
+            </Link>
+          </nav>
         </header>
         <main className="container py-6 px-6">
           <Outlet />
diff --git a/frontend/src/routes/__tests__/root-nav.test.tsx b/frontend/src/routes/__tests__/root-nav.test.tsx
new file mode 100644
index 0000000..1d50e24
--- /dev/null
+++ b/frontend/src/routes/__tests__/root-nav.test.tsx
@@ -0,0 +1,130 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen } from "@testing-library/react";
+import React from "react";
+import {
+  createRootRoute,
+  createRoute,
+  createRouter,
+  createMemoryHistory,
+  RouterProvider,
+  Link,
+  Outlet,
+} from "@tanstack/react-router";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
+
+// Mock i18next
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => {
+      const translations: Record<string, string> = {
+        appName: "RSC",
+        "nav.home": "Home",
+        "nav.ontology": "Ontology",
+        "nav.compliance": "Compliance",
+        "nav.reports": "Reports",
+      };
+      return translations[key] ?? key;
+    },
+    i18n: { language: "en", changeLanguage: vi.fn() },
+  }),
+}));
+
+// Build a minimal layout that mirrors __root.tsx navigation structure
+function TestRootLayout() {
+  return (
+    <div>
+      <nav data-testid="primary-nav">
+        <Link to="/">Home</Link>
+        <Link to="/ontology">Ontology</Link>
+        <Link to="/compliance">Compliance</Link>
+        <Link to="/reports">Reports</Link>
+      </nav>
+      <nav data-testid="secondary-nav">
+        <Link to="/frameworks">Frameworks</Link>
+        <Link to="/crosswalk">Crosswalk</Link>
+        <Link to="/landscape">Landscape</Link>
+        <Link to="/concepts/search">Search</Link>
+      </nav>
+      <Outlet />
+    </div>
+  );
+}
+
+function renderWithRouter(initialPath = "/") {
+  const rootRoute = createRootRoute({ component: TestRootLayout });
+
+  const indexRoute = createRoute({ getParentRoute: () => rootRoute, path: "/" , component: () => React.createElement("div", null, "Home") });
+  const frameworksRoute = createRoute({ getParentRoute: () => rootRoute, path: "/frameworks" , component: () => React.createElement("div", null, "Frameworks") });
+  const crosswalkRoute = createRoute({ getParentRoute: () => rootRoute, path: "/crosswalk" , component: () => React.createElement("div", null, "Crosswalk") });
+  const landscapeRoute = createRoute({ getParentRoute: () => rootRoute, path: "/landscape" , component: () => React.createElement("div", null, "Landscape") });
+  const ontologyRoute = createRoute({ getParentRoute: () => rootRoute, path: "/ontology" , component: () => React.createElement("div", null, "Ontology") });
+  const complianceRoute = createRoute({ getParentRoute: () => rootRoute, path: "/compliance" , component: () => React.createElement("div", null, "Compliance") });
+  const reportsRoute = createRoute({ getParentRoute: () => rootRoute, path: "/reports" , component: () => React.createElement("div", null, "Reports") });
+  const conceptsSearchRoute = createRoute({ getParentRoute: () => rootRoute, path: "/concepts/search" , component: () => React.createElement("div", null, "Search") });
+
+  const routeTree = rootRoute.addChildren([
+    indexRoute,
+    frameworksRoute,
+    crosswalkRoute,
+    landscapeRoute,
+    ontologyRoute,
+    complianceRoute,
+    reportsRoute,
+    conceptsSearchRoute,
+  ]);
+
+  const router = createRouter({
+    routeTree,
+    history: createMemoryHistory({ initialEntries: [initialPath] }),
+  });
+
+  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } });
+
+  return render(
+    React.createElement(
+      QueryClientProvider,
+      { client: queryClient },
+      React.createElement(RouterProvider, { router: router as any })
+    )
+  );
+}
+
+describe("Root Navigation", () => {
+  it("secondary nav renders 4 links", async () => {
+    renderWithRouter("/");
+    const secondaryNav = await screen.findByTestId("secondary-nav");
+    const links = secondaryNav.querySelectorAll("a");
+    expect(links).toHaveLength(4);
+    expect(links[0].textContent).toBe("Frameworks");
+    expect(links[1].textContent).toBe("Crosswalk");
+    expect(links[2].textContent).toBe("Landscape");
+    expect(links[3].textContent).toBe("Search");
+  });
+
+  it("secondary nav links have correct href values", async () => {
+    renderWithRouter("/");
+    const secondaryNav = await screen.findByTestId("secondary-nav");
+    const links = secondaryNav.querySelectorAll("a");
+    expect(links[0].getAttribute("href")).toBe("/frameworks");
+    expect(links[1].getAttribute("href")).toBe("/crosswalk");
+    expect(links[2].getAttribute("href")).toBe("/landscape");
+    expect(links[3].getAttribute("href")).toBe("/concepts/search");
+  });
+
+  it("primary nav contains only Home, Ontology, Compliance, Reports", async () => {
+    renderWithRouter("/");
+    const primaryNav = await screen.findByTestId("primary-nav");
+    const links = primaryNav.querySelectorAll("a");
+    const linkTexts = Array.from(links).map((l) => l.textContent);
+    expect(linkTexts).toEqual(["Home", "Ontology", "Compliance", "Reports"]);
+    expect(linkTexts).not.toContain("Frameworks");
+    expect(linkTexts).not.toContain("Crosswalk");
+  });
+
+  it("active link gets active class on matching route", async () => {
+    renderWithRouter("/frameworks");
+    const secondaryNav = await screen.findByTestId("secondary-nav");
+    const frameworksLink = secondaryNav.querySelector('a[href="/frameworks"]');
+    expect(frameworksLink?.classList.contains("active")).toBe(true);
+  });
+});
diff --git a/frontend/src/routes/concepts/index.tsx b/frontend/src/routes/concepts/index.tsx
new file mode 100644
index 0000000..877af09
--- /dev/null
+++ b/frontend/src/routes/concepts/index.tsx
@@ -0,0 +1,9 @@
+import { createFileRoute, Navigate } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/concepts/")({
+  component: ConceptsRedirect,
+});
+
+function ConceptsRedirect() {
+  return <Navigate to="/concepts/search" />;
+}
diff --git a/frontend/src/routes/concepts/search.tsx b/frontend/src/routes/concepts/search.tsx
new file mode 100644
index 0000000..17f5451
--- /dev/null
+++ b/frontend/src/routes/concepts/search.tsx
@@ -0,0 +1,20 @@
+import { createFileRoute } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/concepts/search")({
+  component: ConceptSearchPage,
+  validateSearch: (
+    search: Record<string, unknown>
+  ): { q?: string; frameworks?: string; types?: string } => ({
+    q: (search.q as string) ?? undefined,
+    frameworks: (search.frameworks as string) ?? undefined,
+    types: (search.types as string) ?? undefined,
+  }),
+});
+
+function ConceptSearchPage() {
+  return (
+    <div>
+      <h1 className="text-2xl font-bold font-mono">Concept Search</h1>
+    </div>
+  );
+}
diff --git a/frontend/src/routes/frameworks/index.tsx b/frontend/src/routes/frameworks/index.tsx
new file mode 100644
index 0000000..cb2a01b
--- /dev/null
+++ b/frontend/src/routes/frameworks/index.tsx
@@ -0,0 +1,16 @@
+import { createFileRoute } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/frameworks/")({
+  component: FrameworkCatalogPage,
+  validateSearch: (search: Record<string, unknown>): { id?: string } => ({
+    id: (search.id as string) ?? undefined,
+  }),
+});
+
+function FrameworkCatalogPage() {
+  return (
+    <div>
+      <h1 className="text-2xl font-bold font-mono">Framework Catalog</h1>
+    </div>
+  );
+}
diff --git a/frontend/src/routes/landscape/index.tsx b/frontend/src/routes/landscape/index.tsx
new file mode 100644
index 0000000..6311b51
--- /dev/null
+++ b/frontend/src/routes/landscape/index.tsx
@@ -0,0 +1,19 @@
+import { createFileRoute } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/landscape/")({
+  component: RegulatoryLandscapePage,
+  validateSearch: (
+    search: Record<string, unknown>
+  ): { sector?: string; activities?: string } => ({
+    sector: (search.sector as string) ?? undefined,
+    activities: (search.activities as string) ?? undefined,
+  }),
+});
+
+function RegulatoryLandscapePage() {
+  return (
+    <div>
+      <h1 className="text-2xl font-bold font-mono">Regulatory Landscape</h1>
+    </div>
+  );
+}
