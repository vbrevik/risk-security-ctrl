diff --git a/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json b/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
index 403cc5b..ae631bb 100644
--- a/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
+++ b/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
@@ -14,7 +14,12 @@
     "section-05-create-page",
     "section-06-settings-page"
   ],
-  "sections_state": {},
+  "sections_state": {
+    "section-01-types-and-hooks": {
+      "status": "complete",
+      "commit_hash": "00275d6"
+    }
+  },
   "pre_commit": {
     "present": false,
     "type": "none",
diff --git a/frontend/src/i18n/__tests__/analysis-namespace.test.ts b/frontend/src/i18n/__tests__/analysis-namespace.test.ts
new file mode 100644
index 0000000..3c2a8bd
--- /dev/null
+++ b/frontend/src/i18n/__tests__/analysis-namespace.test.ts
@@ -0,0 +1,22 @@
+import { describe, it, expect } from "vitest";
+import i18n from "../index";
+
+describe("analysis i18n namespace", () => {
+  it("analysis namespace is registered and loadable", () => {
+    const bundle = i18n.getResourceBundle("en", "analysis");
+    expect(bundle).toBeDefined();
+    expect(bundle.title).toBeTruthy();
+  });
+
+  it("key access returns translated string, not the key itself", () => {
+    const result = i18n.t("title", { ns: "analysis", lng: "en" });
+    expect(result).not.toBe("title");
+    expect(result).toBe("Document Analysis");
+  });
+
+  it("nb locale has analysis namespace", () => {
+    const bundle = i18n.getResourceBundle("nb", "analysis");
+    expect(bundle).toBeDefined();
+    expect(bundle.title).toBeTruthy();
+  });
+});
diff --git a/frontend/src/i18n/index.ts b/frontend/src/i18n/index.ts
index 2a1e866..28ab532 100644
--- a/frontend/src/i18n/index.ts
+++ b/frontend/src/i18n/index.ts
@@ -6,11 +6,13 @@ import enCommon from "./locales/en/common.json";
 import enOntology from "./locales/en/ontology.json";
 import enCompliance from "./locales/en/compliance.json";
 import enReports from "./locales/en/reports.json";
+import enAnalysis from "./locales/en/analysis.json";
 
 import nbCommon from "./locales/nb/common.json";
 import nbOntology from "./locales/nb/ontology.json";
 import nbCompliance from "./locales/nb/compliance.json";
 import nbReports from "./locales/nb/reports.json";
+import nbAnalysis from "./locales/nb/analysis.json";
 
 const resources = {
   en: {
@@ -18,12 +20,14 @@ const resources = {
     ontology: enOntology,
     compliance: enCompliance,
     reports: enReports,
+    analysis: enAnalysis,
   },
   nb: {
     common: nbCommon,
     ontology: nbOntology,
     compliance: nbCompliance,
     reports: nbReports,
+    analysis: nbAnalysis,
   },
 };
 
diff --git a/frontend/src/i18n/locales/en/analysis.json b/frontend/src/i18n/locales/en/analysis.json
new file mode 100644
index 0000000..b45a6be
--- /dev/null
+++ b/frontend/src/i18n/locales/en/analysis.json
@@ -0,0 +1,62 @@
+{
+  "title": "Document Analysis",
+  "list": {
+    "title": "Analyses",
+    "newAnalysis": "New Analysis",
+    "empty": {
+      "title": "No analyses yet",
+      "description": "Create your first analysis to get started."
+    },
+    "filters": {
+      "status": "Status",
+      "all": "All"
+    }
+  },
+  "status": {
+    "pending": "Pending",
+    "processing": "Processing",
+    "completed": "Completed",
+    "failed": "Failed"
+  },
+  "create": {
+    "title": "New Analysis",
+    "nameLabel": "Name",
+    "namePlaceholder": "Enter analysis name",
+    "descriptionLabel": "Description",
+    "textTab": "Text Input",
+    "uploadTab": "File Upload",
+    "textPlaceholder": "Paste or type the document text to analyze...",
+    "dropzoneText": "Drag and drop a file here, or",
+    "dropzoneBrowse": "browse",
+    "uploading": "Uploading...",
+    "submit": "Create Analysis",
+    "maxFileSize": "Maximum file size: 25MB",
+    "invalidFileType": "Invalid file type. Supported formats: PDF, DOCX",
+    "fileTooLarge": "File exceeds the maximum size of 25MB",
+    "success": "Analysis created successfully"
+  },
+  "settings": {
+    "title": "Matcher Configuration",
+    "thresholds": "Thresholds",
+    "minConfidence": "Minimum Confidence",
+    "addressedThreshold": "Addressed Threshold",
+    "partialThreshold": "Partial Threshold",
+    "maxFindings": "Max Findings per Framework",
+    "includeAddressed": "Include Addressed Findings",
+    "boostTerms": "Boost Terms",
+    "termLabel": "Term",
+    "weightLabel": "Weight",
+    "addTerm": "Add Term",
+    "save": "Save Settings",
+    "saved": "Settings saved",
+    "resetDefaults": "Reset to Defaults",
+    "resetConfirm": "Reset all settings to defaults?"
+  },
+  "common": {
+    "back": "Back",
+    "delete": "Delete",
+    "deleteConfirm": "Are you sure you want to delete this analysis?",
+    "cancel": "Cancel",
+    "error": "An error occurred"
+  }
+}
diff --git a/frontend/src/i18n/locales/en/common.json b/frontend/src/i18n/locales/en/common.json
index 6f1fea2..95e2185 100644
--- a/frontend/src/i18n/locales/en/common.json
+++ b/frontend/src/i18n/locales/en/common.json
@@ -8,7 +8,8 @@
     "reports": "Reports",
     "frameworks": "Frameworks",
     "landscape": "Landscape",
-    "search": "Search"
+    "search": "Search",
+    "analysis": "Analysis"
   },
   "actions": {
     "save": "Save",
diff --git a/frontend/src/i18n/locales/nb/analysis.json b/frontend/src/i18n/locales/nb/analysis.json
new file mode 100644
index 0000000..828e7b1
--- /dev/null
+++ b/frontend/src/i18n/locales/nb/analysis.json
@@ -0,0 +1,62 @@
+{
+  "title": "Dokumentanalyse",
+  "list": {
+    "title": "Analyser",
+    "newAnalysis": "Ny analyse",
+    "empty": {
+      "title": "Ingen analyser ennå",
+      "description": "Opprett din første analyse for å komme i gang."
+    },
+    "filters": {
+      "status": "Status",
+      "all": "Alle"
+    }
+  },
+  "status": {
+    "pending": "Venter",
+    "processing": "Behandler",
+    "completed": "Fullført",
+    "failed": "Feilet"
+  },
+  "create": {
+    "title": "Ny analyse",
+    "nameLabel": "Navn",
+    "namePlaceholder": "Skriv inn analysenavn",
+    "descriptionLabel": "Beskrivelse",
+    "textTab": "Tekstinntasting",
+    "uploadTab": "Filopplasting",
+    "textPlaceholder": "Lim inn eller skriv dokumentteksten som skal analyseres...",
+    "dropzoneText": "Dra og slipp en fil her, eller",
+    "dropzoneBrowse": "bla gjennom",
+    "uploading": "Laster opp...",
+    "submit": "Opprett analyse",
+    "maxFileSize": "Maksimal filstørrelse: 25 MB",
+    "invalidFileType": "Ugyldig filtype. Støttede formater: PDF, DOCX",
+    "fileTooLarge": "Filen overskrider maksimal størrelse på 25 MB",
+    "success": "Analyse opprettet"
+  },
+  "settings": {
+    "title": "Matcherkonfigurasjon",
+    "thresholds": "Terskelverdier",
+    "minConfidence": "Minimum konfidens",
+    "addressedThreshold": "Terskel for adressert",
+    "partialThreshold": "Terskel for delvis",
+    "maxFindings": "Maks funn per rammeverk",
+    "includeAddressed": "Inkluder adresserte funn",
+    "boostTerms": "Forsterkningstermer",
+    "termLabel": "Term",
+    "weightLabel": "Vekt",
+    "addTerm": "Legg til term",
+    "save": "Lagre innstillinger",
+    "saved": "Innstillinger lagret",
+    "resetDefaults": "Tilbakestill til standard",
+    "resetConfirm": "Tilbakestille alle innstillinger til standard?"
+  },
+  "common": {
+    "back": "Tilbake",
+    "delete": "Slett",
+    "deleteConfirm": "Er du sikker på at du vil slette denne analysen?",
+    "cancel": "Avbryt",
+    "error": "Det oppstod en feil"
+  }
+}
diff --git a/frontend/src/i18n/locales/nb/common.json b/frontend/src/i18n/locales/nb/common.json
index a9595a9..287f667 100644
--- a/frontend/src/i18n/locales/nb/common.json
+++ b/frontend/src/i18n/locales/nb/common.json
@@ -8,7 +8,8 @@
     "reports": "Rapporter",
     "frameworks": "Rammeverk",
     "landscape": "Landskap",
-    "search": "Søk"
+    "search": "Søk",
+    "analysis": "Analyse"
   },
   "actions": {
     "save": "Lagre",
diff --git a/frontend/src/routes/__root.tsx b/frontend/src/routes/__root.tsx
index a482028..d1e8fdb 100644
--- a/frontend/src/routes/__root.tsx
+++ b/frontend/src/routes/__root.tsx
@@ -66,6 +66,12 @@ function RootLayout() {
                 {t("nav.search")}
               </Link>
               <span className="text-border mx-1.5">·</span>
+              <Link
+                to="/analysis"
+                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
+              >
+                {t("nav.analysis")}
+              </Link>
               <Link
                 to="/compliance"
                 className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
diff --git a/frontend/src/routes/__tests__/analysis-nav.test.tsx b/frontend/src/routes/__tests__/analysis-nav.test.tsx
new file mode 100644
index 0000000..dede374
--- /dev/null
+++ b/frontend/src/routes/__tests__/analysis-nav.test.tsx
@@ -0,0 +1,103 @@
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
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => {
+      const translations: Record<string, string> = {
+        appName: "RSC",
+        "nav.home": "Home",
+        "nav.ontology": "Ontology Explorer",
+        "nav.compliance": "Compliance",
+        "nav.reports": "Reports",
+        "nav.frameworks": "Frameworks",
+        "nav.crosswalk": "Crosswalk",
+        "nav.landscape": "Landscape",
+        "nav.search": "Search",
+        "nav.analysis": "Analysis",
+      };
+      return translations[key] ?? key;
+    },
+    i18n: { language: "en", changeLanguage: vi.fn() },
+  }),
+}));
+
+function TestRootLayout() {
+  return (
+    <div>
+      <nav data-testid="main-nav">
+        <Link to="/">Home</Link>
+        <Link to="/ontology">Ontology Explorer</Link>
+        <Link to="/frameworks">Frameworks</Link>
+        <Link to="/crosswalk">Crosswalk</Link>
+        <Link to="/landscape">Landscape</Link>
+        <Link to="/concepts/search">Search</Link>
+        <Link to="/analysis">Analysis</Link>
+        <Link to="/compliance">Compliance</Link>
+        <Link to="/reports">Reports</Link>
+      </nav>
+      <Outlet />
+    </div>
+  );
+}
+
+function renderWithRouter(initialPath = "/") {
+  const rootRoute = createRootRoute({ component: TestRootLayout });
+
+  const routes = [
+    createRoute({ getParentRoute: () => rootRoute, path: "/", component: () => React.createElement("div", null, "Home") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/frameworks", component: () => React.createElement("div", null, "Frameworks") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/crosswalk", component: () => React.createElement("div", null, "Crosswalk") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/landscape", component: () => React.createElement("div", null, "Landscape") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/ontology", component: () => React.createElement("div", null, "Ontology") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/compliance", component: () => React.createElement("div", null, "Compliance") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/reports", component: () => React.createElement("div", null, "Reports") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/concepts/search", component: () => React.createElement("div", null, "Search") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/analysis", component: () => React.createElement("div", null, "Analysis") }),
+  ];
+
+  const router = createRouter({
+    routeTree: rootRoute.addChildren(routes),
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
+describe("Analysis navigation link", () => {
+  it("renders Analysis link in the navigation", async () => {
+    renderWithRouter("/");
+    const nav = await screen.findByTestId("main-nav");
+    const links = Array.from(nav.querySelectorAll("a")).map((a) => a.textContent);
+    expect(links).toContain("Analysis");
+  });
+
+  it("Analysis link points to /analysis", async () => {
+    renderWithRouter("/");
+    const nav = await screen.findByTestId("main-nav");
+    const analysisLink = Array.from(nav.querySelectorAll("a")).find(
+      (a) => a.textContent === "Analysis"
+    );
+    expect(analysisLink).toBeDefined();
+    expect(analysisLink?.getAttribute("href")).toBe("/analysis");
+  });
+});
diff --git a/frontend/src/routes/__tests__/root-nav.test.tsx b/frontend/src/routes/__tests__/root-nav.test.tsx
index a86ff2f..01c0be5 100644
--- a/frontend/src/routes/__tests__/root-nav.test.tsx
+++ b/frontend/src/routes/__tests__/root-nav.test.tsx
@@ -26,6 +26,7 @@ vi.mock("react-i18next", () => ({
         "nav.crosswalk": "Crosswalk",
         "nav.landscape": "Landscape",
         "nav.search": "Search",
+        "nav.analysis": "Analysis",
       };
       return translations[key] ?? key;
     },
@@ -44,6 +45,7 @@ function TestRootLayout() {
         <Link to="/crosswalk">Crosswalk</Link>
         <Link to="/landscape">Landscape</Link>
         <Link to="/concepts/search">Search</Link>
+        <Link to="/analysis">Analysis</Link>
         <Link to="/compliance">Compliance</Link>
         <Link to="/reports">Reports</Link>
       </nav>
@@ -64,6 +66,7 @@ function renderWithRouter(initialPath = "/") {
     createRoute({ getParentRoute: () => rootRoute, path: "/compliance", component: () => React.createElement("div", null, "Compliance") }),
     createRoute({ getParentRoute: () => rootRoute, path: "/reports", component: () => React.createElement("div", null, "Reports") }),
     createRoute({ getParentRoute: () => rootRoute, path: "/concepts/search", component: () => React.createElement("div", null, "Search") }),
+    createRoute({ getParentRoute: () => rootRoute, path: "/analysis", component: () => React.createElement("div", null, "Analysis") }),
   ];
 
   const router = createRouter({
@@ -83,11 +86,11 @@ function renderWithRouter(initialPath = "/") {
 }
 
 describe("Root Navigation (Single Bar)", () => {
-  it("renders all 8 navigation links", async () => {
+  it("renders all 9 navigation links", async () => {
     renderWithRouter("/");
     const nav = await screen.findByTestId("main-nav");
     const links = nav.querySelectorAll("a");
-    expect(links).toHaveLength(8);
+    expect(links).toHaveLength(9);
   });
 
   it("contains all expected link targets", async () => {
@@ -101,6 +104,7 @@ describe("Root Navigation (Single Bar)", () => {
       "/crosswalk",
       "/landscape",
       "/concepts/search",
+      "/analysis",
       "/compliance",
       "/reports",
     ]);
diff --git a/frontend/src/routes/analysis/$id.tsx b/frontend/src/routes/analysis/$id.tsx
new file mode 100644
index 0000000..b2921f8
--- /dev/null
+++ b/frontend/src/routes/analysis/$id.tsx
@@ -0,0 +1,15 @@
+import { createFileRoute, Link } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/analysis/$id")({
+  component: AnalysisDetailPage,
+});
+
+function AnalysisDetailPage() {
+  const { id } = Route.useParams();
+  return (
+    <div>
+      <Link to="/analysis">&larr; Back</Link>
+      <p>Analysis detail page for {id} — coming in split 02</p>
+    </div>
+  );
+}
diff --git a/frontend/src/routes/analysis/create.tsx b/frontend/src/routes/analysis/create.tsx
new file mode 100644
index 0000000..6853f02
--- /dev/null
+++ b/frontend/src/routes/analysis/create.tsx
@@ -0,0 +1,9 @@
+import { createFileRoute } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/analysis/create")({
+  component: CreateAnalysisPage,
+});
+
+function CreateAnalysisPage() {
+  return <div>Create analysis page — implemented in section-05</div>;
+}
diff --git a/frontend/src/routes/analysis/index.tsx b/frontend/src/routes/analysis/index.tsx
new file mode 100644
index 0000000..1a71147
--- /dev/null
+++ b/frontend/src/routes/analysis/index.tsx
@@ -0,0 +1,9 @@
+import { createFileRoute } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/analysis/")({
+  component: AnalysisListPage,
+});
+
+function AnalysisListPage() {
+  return <div>Analysis list page — implemented in section-04</div>;
+}
diff --git a/frontend/src/routes/analysis/settings.tsx b/frontend/src/routes/analysis/settings.tsx
new file mode 100644
index 0000000..f52d772
--- /dev/null
+++ b/frontend/src/routes/analysis/settings.tsx
@@ -0,0 +1,9 @@
+import { createFileRoute } from "@tanstack/react-router";
+
+export const Route = createFileRoute("/analysis/settings")({
+  component: AnalysisSettingsPage,
+});
+
+function AnalysisSettingsPage() {
+  return <div>Settings page — implemented in section-06</div>;
+}
