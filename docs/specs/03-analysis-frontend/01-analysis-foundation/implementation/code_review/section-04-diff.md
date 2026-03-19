diff --git a/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json b/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
index e1060e9..a779162 100644
--- a/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
+++ b/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
@@ -22,6 +22,10 @@
     "section-02-i18n-and-navigation": {
       "status": "complete",
       "commit_hash": "7c8c19e"
+    },
+    "section-03-shadcn-components": {
+      "status": "complete",
+      "commit_hash": "46ab818"
     }
   },
   "pre_commit": {
diff --git a/frontend/src/features/analysis/components/AnalysisCard.tsx b/frontend/src/features/analysis/components/AnalysisCard.tsx
new file mode 100644
index 0000000..23bf93d
--- /dev/null
+++ b/frontend/src/features/analysis/components/AnalysisCard.tsx
@@ -0,0 +1,62 @@
+import { Link } from "@tanstack/react-router";
+import {
+  Card,
+  CardHeader,
+  CardTitle,
+  CardContent,
+  CardFooter,
+} from "@/components/ui/card";
+import { StatusBadge } from "./StatusBadge";
+import type { AnalysisListItem } from "../types";
+
+interface AnalysisCardProps {
+  analysis: AnalysisListItem;
+}
+
+export function AnalysisCard({ analysis }: AnalysisCardProps) {
+  const createdDate = new Date(analysis.created_at).toLocaleDateString();
+
+  return (
+    <Link
+      to="/analysis/$id"
+      params={{ id: analysis.id }}
+      className="block"
+    >
+      <Card className="transition-colors hover:border-primary/50">
+        <CardHeader className="pb-2">
+          <div className="flex items-start justify-between gap-2">
+            <CardTitle className="text-base leading-tight">
+              {analysis.name}
+            </CardTitle>
+            <StatusBadge status={analysis.status} />
+          </div>
+        </CardHeader>
+        <CardContent className="pb-2">
+          {analysis.description && (
+            <p className="text-sm text-muted-foreground line-clamp-2">
+              {analysis.description}
+            </p>
+          )}
+          <div className="flex items-center gap-2 mt-2">
+            <span className="text-xs px-1.5 py-0.5 rounded bg-muted font-mono">
+              {analysis.input_type}
+            </span>
+            {analysis.processing_time_ms != null && (
+              <span className="text-xs text-muted-foreground">
+                {(analysis.processing_time_ms / 1000).toFixed(1)}s
+              </span>
+            )}
+          </div>
+        </CardContent>
+        <CardFooter className="text-xs text-muted-foreground pt-0">
+          <span>{createdDate}</span>
+          {analysis.error_message && (
+            <span className="ml-auto text-destructive truncate max-w-[200px]">
+              {analysis.error_message}
+            </span>
+          )}
+        </CardFooter>
+      </Card>
+    </Link>
+  );
+}
diff --git a/frontend/src/features/analysis/components/AnalysisList.tsx b/frontend/src/features/analysis/components/AnalysisList.tsx
new file mode 100644
index 0000000..89f1521
--- /dev/null
+++ b/frontend/src/features/analysis/components/AnalysisList.tsx
@@ -0,0 +1,73 @@
+import { useTranslation } from "react-i18next";
+import { Link } from "@tanstack/react-router";
+import { FileText } from "lucide-react";
+import { Button } from "@/components/ui/button";
+import { AnalysisCard } from "./AnalysisCard";
+import type { AnalysisListItem } from "../types";
+
+interface AnalysisListProps {
+  analyses: AnalysisListItem[] | undefined;
+  isLoading: boolean;
+  isError: boolean;
+  onRetry?: () => void;
+}
+
+export function AnalysisList({
+  analyses,
+  isLoading,
+  isError,
+  onRetry,
+}: AnalysisListProps) {
+  const { t } = useTranslation("analysis");
+
+  if (isLoading) {
+    return (
+      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
+        {Array.from({ length: 3 }).map((_, i) => (
+          <div
+            key={i}
+            className="h-48 rounded-lg border bg-muted/50 animate-pulse"
+          />
+        ))}
+      </div>
+    );
+  }
+
+  if (isError) {
+    return (
+      <div className="flex flex-col items-center justify-center py-12 text-center">
+        <p className="text-destructive mb-4">{t("common.error")}</p>
+        {onRetry && (
+          <Button variant="outline" onClick={onRetry}>
+            {t("common.back")}
+          </Button>
+        )}
+      </div>
+    );
+  }
+
+  if (!analyses?.length) {
+    return (
+      <div className="flex flex-col items-center justify-center py-16 text-center">
+        <FileText className="h-12 w-12 text-muted-foreground/50 mb-4" />
+        <h3 className="text-lg font-medium mb-1">
+          {t("list.empty.title")}
+        </h3>
+        <p className="text-sm text-muted-foreground max-w-sm mb-4">
+          {t("list.empty.description")}
+        </p>
+        <Link to="/analysis/create">
+          <Button>{t("list.newAnalysis")}</Button>
+        </Link>
+      </div>
+    );
+  }
+
+  return (
+    <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
+      {analyses.map((analysis) => (
+        <AnalysisCard key={analysis.id} analysis={analysis} />
+      ))}
+    </div>
+  );
+}
diff --git a/frontend/src/features/analysis/components/StatusBadge.tsx b/frontend/src/features/analysis/components/StatusBadge.tsx
new file mode 100644
index 0000000..4567d92
--- /dev/null
+++ b/frontend/src/features/analysis/components/StatusBadge.tsx
@@ -0,0 +1,31 @@
+import { useTranslation } from "react-i18next";
+import { Badge } from "@/components/ui/badge";
+import type { AnalysisStatus } from "../types";
+
+const statusStyles: Record<AnalysisStatus, string> = {
+  pending: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
+  processing: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300",
+  completed: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300",
+  failed: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
+  deleted: "bg-muted text-muted-foreground",
+};
+
+interface StatusBadgeProps {
+  status: AnalysisStatus;
+}
+
+export function StatusBadge({ status }: StatusBadgeProps) {
+  const { t } = useTranslation("analysis");
+
+  const badge = (
+    <Badge variant="outline" className={statusStyles[status]}>
+      {t(`status.${status}`)}
+    </Badge>
+  );
+
+  if (status === "processing") {
+    return <span className="animate-pulse">{badge}</span>;
+  }
+
+  return badge;
+}
diff --git a/frontend/src/features/analysis/components/__tests__/AnalysisCard.test.tsx b/frontend/src/features/analysis/components/__tests__/AnalysisCard.test.tsx
new file mode 100644
index 0000000..ff9f1d2
--- /dev/null
+++ b/frontend/src/features/analysis/components/__tests__/AnalysisCard.test.tsx
@@ -0,0 +1,79 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen } from "@testing-library/react";
+import React from "react";
+import {
+  createRootRoute,
+  createRoute,
+  createRouter,
+  createMemoryHistory,
+  RouterProvider,
+  Outlet,
+} from "@tanstack/react-router";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
+import { AnalysisCard } from "../AnalysisCard";
+import type { AnalysisListItem } from "../../types";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => key,
+  }),
+}));
+
+function renderWithRouter(ui: React.ReactElement) {
+  const rootRoute = createRootRoute({
+    component: () => React.createElement("div", null, React.createElement(Outlet)),
+  });
+
+  const indexRoute = createRoute({
+    getParentRoute: () => rootRoute,
+    path: "/",
+    component: () => ui,
+  });
+
+  const analysisRoute = createRoute({
+    getParentRoute: () => rootRoute,
+    path: "/analysis/$id",
+    component: () => React.createElement("div", null, "Detail"),
+  });
+
+  const router = createRouter({
+    routeTree: rootRoute.addChildren([indexRoute, analysisRoute]),
+    history: createMemoryHistory({ initialEntries: ["/"] }),
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
+const mockItem: AnalysisListItem = {
+  id: "abc-123",
+  name: "Test Analysis",
+  description: "A test description",
+  input_type: "text",
+  status: "completed",
+  error_message: null,
+  processing_time_ms: 2300,
+  created_at: "2025-01-15T10:00:00Z",
+  updated_at: "2025-01-15T10:05:00Z",
+};
+
+describe("AnalysisCard", () => {
+  it("renders analysis name and status badge", async () => {
+    renderWithRouter(<AnalysisCard analysis={mockItem} />);
+    expect(await screen.findByText("Test Analysis")).toBeDefined();
+    expect(screen.getByText("status.completed")).toBeDefined();
+  });
+
+  it("links to /analysis/{id}", async () => {
+    renderWithRouter(<AnalysisCard analysis={mockItem} />);
+    const link = await screen.findByRole("link");
+    expect(link.getAttribute("href")).toBe("/analysis/abc-123");
+  });
+});
diff --git a/frontend/src/features/analysis/components/__tests__/AnalysisList.test.tsx b/frontend/src/features/analysis/components/__tests__/AnalysisList.test.tsx
new file mode 100644
index 0000000..488f2f9
--- /dev/null
+++ b/frontend/src/features/analysis/components/__tests__/AnalysisList.test.tsx
@@ -0,0 +1,111 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen, fireEvent } from "@testing-library/react";
+import React from "react";
+import {
+  createRootRoute,
+  createRoute,
+  createRouter,
+  createMemoryHistory,
+  RouterProvider,
+  Outlet,
+} from "@tanstack/react-router";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
+import { AnalysisList } from "../AnalysisList";
+import type { AnalysisListItem } from "../../types";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => key,
+  }),
+}));
+
+function renderWithRouter(ui: React.ReactElement) {
+  const rootRoute = createRootRoute({
+    component: () => React.createElement("div", null, React.createElement(Outlet)),
+  });
+
+  const indexRoute = createRoute({
+    getParentRoute: () => rootRoute,
+    path: "/",
+    component: () => ui,
+  });
+
+  const analysisRoute = createRoute({
+    getParentRoute: () => rootRoute,
+    path: "/analysis/$id",
+    component: () => React.createElement("div", null, "Detail"),
+  });
+
+  const createRoute2 = createRoute({
+    getParentRoute: () => rootRoute,
+    path: "/analysis/create",
+    component: () => React.createElement("div", null, "Create"),
+  });
+
+  const router = createRouter({
+    routeTree: rootRoute.addChildren([indexRoute, analysisRoute, createRoute2]),
+    history: createMemoryHistory({ initialEntries: ["/"] }),
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
+const mockItems: AnalysisListItem[] = [
+  {
+    id: "a1", name: "First Analysis", description: null, input_type: "text",
+    status: "completed", error_message: null, processing_time_ms: 100,
+    created_at: "2025-01-01", updated_at: "2025-01-01",
+  },
+  {
+    id: "a2", name: "Second Analysis", description: null, input_type: "pdf",
+    status: "processing", error_message: null, processing_time_ms: null,
+    created_at: "2025-01-02", updated_at: "2025-01-02",
+  },
+];
+
+describe("AnalysisList", () => {
+  it("renders loading skeleton while fetching", async () => {
+    renderWithRouter(
+      <AnalysisList analyses={undefined} isLoading={true} isError={false} />
+    );
+    // Skeleton divs have animate-pulse in their className attribute
+    await vi.waitFor(() => {
+      const el = document.querySelector('[class*="animate-pulse"]');
+      expect(el).not.toBeNull();
+    });
+  });
+
+  it("renders analysis cards when data loads", async () => {
+    renderWithRouter(
+      <AnalysisList analyses={mockItems} isLoading={false} isError={false} />
+    );
+    expect(await screen.findByText("First Analysis")).toBeDefined();
+    expect(screen.getByText("Second Analysis")).toBeDefined();
+  });
+
+  it("renders empty state when no analyses exist", async () => {
+    renderWithRouter(
+      <AnalysisList analyses={[]} isLoading={false} isError={false} />
+    );
+    expect(await screen.findByText("list.empty.title")).toBeDefined();
+  });
+
+  it("renders error state with retry button on error", async () => {
+    const onRetry = vi.fn();
+    renderWithRouter(
+      <AnalysisList analyses={undefined} isLoading={false} isError={true} onRetry={onRetry} />
+    );
+    expect(await screen.findByText("common.error")).toBeDefined();
+    const retryButton = screen.getByRole("button");
+    fireEvent.click(retryButton);
+    expect(onRetry).toHaveBeenCalledOnce();
+  });
+});
diff --git a/frontend/src/features/analysis/components/__tests__/StatusBadge.test.tsx b/frontend/src/features/analysis/components/__tests__/StatusBadge.test.tsx
new file mode 100644
index 0000000..f34969d
--- /dev/null
+++ b/frontend/src/features/analysis/components/__tests__/StatusBadge.test.tsx
@@ -0,0 +1,35 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen } from "@testing-library/react";
+import { StatusBadge } from "../StatusBadge";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => key,
+  }),
+}));
+
+describe("StatusBadge", () => {
+  it("renders green badge for completed status", () => {
+    const { container } = render(<StatusBadge status="completed" />);
+    expect(screen.getByText("status.completed")).toBeDefined();
+    expect(container.innerHTML).toMatch(/bg-green/);
+  });
+
+  it("renders yellow badge with pulse for processing status", () => {
+    const { container } = render(<StatusBadge status="processing" />);
+    expect(screen.getByText("status.processing")).toBeDefined();
+    expect(container.innerHTML).toMatch(/animate-pulse/);
+  });
+
+  it("renders red badge for failed status", () => {
+    const { container } = render(<StatusBadge status="failed" />);
+    expect(screen.getByText("status.failed")).toBeDefined();
+    expect(container.innerHTML).toMatch(/bg-red/);
+  });
+
+  it("renders blue badge for pending status", () => {
+    const { container } = render(<StatusBadge status="pending" />);
+    expect(screen.getByText("status.pending")).toBeDefined();
+    expect(container.innerHTML).toMatch(/bg-blue/);
+  });
+});
diff --git a/frontend/src/features/analysis/index.ts b/frontend/src/features/analysis/index.ts
index 9a18f25..891d295 100644
--- a/frontend/src/features/analysis/index.ts
+++ b/frontend/src/features/analysis/index.ts
@@ -1,2 +1,5 @@
 export * from "./types";
 export * from "./api";
+export { StatusBadge } from "./components/StatusBadge";
+export { AnalysisCard } from "./components/AnalysisCard";
+export { AnalysisList } from "./components/AnalysisList";
diff --git a/frontend/src/routes/analysis/index.tsx b/frontend/src/routes/analysis/index.tsx
index 1a71147..c88fc54 100644
--- a/frontend/src/routes/analysis/index.tsx
+++ b/frontend/src/routes/analysis/index.tsx
@@ -1,9 +1,106 @@
-import { createFileRoute } from "@tanstack/react-router";
+import { createFileRoute, Link, useNavigate } from "@tanstack/react-router";
+import { useTranslation } from "react-i18next";
+import { Settings } from "lucide-react";
+import { Button } from "@/components/ui/button";
+import {
+  Select,
+  SelectContent,
+  SelectItem,
+  SelectTrigger,
+  SelectValue,
+} from "@/components/ui/select";
+import { useAnalyses } from "@/features/analysis/api";
+import { AnalysisList } from "@/features/analysis/components/AnalysisList";
 
 export const Route = createFileRoute("/analysis/")({
   component: AnalysisListPage,
+  validateSearch: (search: Record<string, unknown>) => ({
+    page: Number(search.page) || 1,
+    status: (search.status as string) || undefined,
+  }),
 });
 
 function AnalysisListPage() {
-  return <div>Analysis list page — implemented in section-04</div>;
+  const { t } = useTranslation("analysis");
+  const { page, status } = Route.useSearch();
+  const navigate = useNavigate();
+
+  const { data, isLoading, isError, refetch } = useAnalyses({
+    page,
+    limit: 12,
+    status: status as "pending" | "processing" | "completed" | "failed" | undefined,
+  });
+
+  return (
+    <div className="space-y-6">
+      <div className="flex items-center justify-between">
+        <h1 className="text-2xl font-bold tracking-tight">{t("list.title")}</h1>
+        <div className="flex gap-2">
+          <Link to="/analysis/settings">
+            <Button variant="outline" size="icon">
+              <Settings className="h-4 w-4" />
+            </Button>
+          </Link>
+          <Link to="/analysis/create">
+            <Button>{t("list.newAnalysis")}</Button>
+          </Link>
+        </div>
+      </div>
+
+      <Select
+        value={status ?? ""}
+        onValueChange={(value) =>
+          navigate({
+            search: { status: value || undefined, page: 1 },
+          })
+        }
+      >
+        <SelectTrigger className="w-[180px]">
+          <SelectValue placeholder={t("list.filters.all")} />
+        </SelectTrigger>
+        <SelectContent>
+          <SelectItem value="">{t("list.filters.all")}</SelectItem>
+          <SelectItem value="pending">{t("status.pending")}</SelectItem>
+          <SelectItem value="processing">{t("status.processing")}</SelectItem>
+          <SelectItem value="completed">{t("status.completed")}</SelectItem>
+          <SelectItem value="failed">{t("status.failed")}</SelectItem>
+        </SelectContent>
+      </Select>
+
+      <AnalysisList
+        analyses={data?.data}
+        isLoading={isLoading}
+        isError={isError}
+        onRetry={refetch}
+      />
+
+      {data && data.total_pages > 1 && (
+        <div className="flex items-center justify-center gap-4">
+          <Button
+            variant="outline"
+            size="sm"
+            disabled={page <= 1}
+            onClick={() =>
+              navigate({ search: { page: page - 1, status } })
+            }
+          >
+            Previous
+          </Button>
+          <span className="text-sm text-muted-foreground">
+            Page {page} of {data.total_pages}
+          </span>
+          <Button
+            variant="outline"
+            size="sm"
+            disabled={page >= data.total_pages}
+            onClick={() =>
+              navigate({ search: { page: page + 1, status } })
+            }
+          >
+            Next
+          </Button>
+        </div>
+      )}
+    </div>
+  );
 }
