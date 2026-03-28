diff --git a/frontend/src/features/ontology/components/ProofPanel.tsx b/frontend/src/features/ontology/components/ProofPanel.tsx
new file mode 100644
index 0000000..7994e85
--- /dev/null
+++ b/frontend/src/features/ontology/components/ProofPanel.tsx
@@ -0,0 +1,100 @@
+import { useMemo } from "react";
+import ReactMarkdown from "react-markdown";
+import remarkGfm from "remark-gfm";
+import { ExternalLink } from "lucide-react";
+import { useTranslation } from "react-i18next";
+import { useFrameworkProof } from "../api";
+import { VerificationBadge } from "./VerificationBadge";
+
+interface ProofPanelProps {
+  frameworkId: string;
+}
+
+/**
+ * Displays verification provenance for a framework.
+ * Fetches proof data lazily when mounted.
+ * Renders: loading skeleton | error message | metadata + optional markdown.
+ */
+export function ProofPanel({ frameworkId }: ProofPanelProps) {
+  const { t } = useTranslation("ontology");
+  const { data, isLoading, isError } = useFrameworkProof(frameworkId);
+
+  if (isLoading) {
+    return (
+      <div className="space-y-2 p-4">
+        <div className="h-4 w-full bg-muted rounded animate-pulse" />
+        <div className="h-4 w-3/4 bg-muted rounded animate-pulse" />
+        <div className="h-4 w-1/2 bg-muted rounded animate-pulse" />
+      </div>
+    );
+  }
+
+  if (isError || !data) {
+    return (
+      <div className="p-4 text-sm text-muted-foreground">
+        {t("proof.error", "Could not load proof document.")}
+      </div>
+    );
+  }
+
+  const formattedDate = data.verification_date
+    ? new Date(data.verification_date).toLocaleDateString()
+    : null;
+
+  return (
+    <div className="p-4 space-y-3 text-sm">
+      {/* Metadata row */}
+      <div className="flex flex-wrap items-center gap-3">
+        <VerificationBadge status={data.verification_status} />
+        {formattedDate && (
+          <span className="text-muted-foreground">
+            {t("proof.date", "Verified")}: {formattedDate}
+          </span>
+        )}
+        {data.verification_source && (
+          <a
+            href={data.verification_source}
+            target="_blank"
+            rel="noopener noreferrer"
+            className="inline-flex items-center gap-1 text-muted-foreground hover:text-foreground"
+          >
+            <ExternalLink className="w-3 h-3" />
+            {t("proof.source", "Source")}
+          </a>
+        )}
+      </div>
+      {data.verification_notes && (
+        <p className="text-muted-foreground">
+          <span className="font-medium">{t("proof.notes", "Notes")}: </span>
+          {data.verification_notes}
+        </p>
+      )}
+
+      {data.proof_content ? (
+        <>
+          <hr className="border-border" />
+          <MarkdownContent content={data.proof_content} />
+        </>
+      ) : (
+        <p className="text-muted-foreground italic">
+          {t("proof.noProof", "No proof document available")}
+        </p>
+      )}
+    </div>
+  );
+}
+
+function MarkdownContent({ content }: { content: string }) {
+  const rendered = useMemo(
+    () => (
+      <ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown>
+    ),
+    [content]
+  );
+
+  return (
+    <div className="max-h-96 overflow-y-auto prose prose-sm dark:prose-invert">
+      {rendered}
+    </div>
+  );
+}
diff --git a/frontend/src/features/ontology/components/__tests__/ProofPanel.test.tsx b/frontend/src/features/ontology/components/__tests__/ProofPanel.test.tsx
new file mode 100644
index 0000000..97c0f9d
--- /dev/null
+++ b/frontend/src/features/ontology/components/__tests__/ProofPanel.test.tsx
@@ -0,0 +1,157 @@
+import { describe, it, expect, vi, beforeEach } from "vitest";
+import { render, screen } from "@testing-library/react";
+import React from "react";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
+import { ProofPanel } from "../ProofPanel";
+import type { FrameworkProof } from "../../types";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string, fallback?: string) => fallback ?? key,
+  }),
+}));
+
+vi.mock("../../api", () => ({
+  useFrameworkProof: vi.fn(),
+}));
+
+vi.mock("../VerificationBadge", () => ({
+  VerificationBadge: ({ status }: { status: string | null }) => (
+    <div data-testid="verification-badge">{status ?? "unknown"}</div>
+  ),
+}));
+
+import { useFrameworkProof } from "../../api";
+const mockedHook = vi.mocked(useFrameworkProof);
+
+function createWrapper() {
+  const queryClient = new QueryClient({
+    defaultOptions: { queries: { retry: false } },
+  });
+  return function Wrapper({ children }: { children: React.ReactNode }) {
+    return React.createElement(
+      QueryClientProvider,
+      { client: queryClient },
+      children
+    );
+  };
+}
+
+function makeProof(overrides: Partial<FrameworkProof> = {}): FrameworkProof {
+  return {
+    framework_id: "nist-csf",
+    verification_status: "verified",
+    verification_date: "2025-01-15",
+    verification_source: null,
+    verification_notes: null,
+    proof_content: null,
+    ...overrides,
+  };
+}
+
+describe("ProofPanel", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("renders skeleton elements while loading", () => {
+    mockedHook.mockReturnValue({
+      isLoading: true,
+      isError: false,
+      data: undefined,
+    } as ReturnType<typeof useFrameworkProof>);
+
+    const { container } = render(
+      <ProofPanel frameworkId="nist-csf" />,
+      { wrapper: createWrapper() }
+    );
+
+    const skeletons = container.querySelectorAll(".animate-pulse");
+    expect(skeletons.length).toBeGreaterThanOrEqual(3);
+  });
+
+  it("renders generic error message without internal paths on error", () => {
+    mockedHook.mockReturnValue({
+      isLoading: false,
+      isError: true,
+      data: undefined,
+    } as ReturnType<typeof useFrameworkProof>);
+
+    const { container } = render(
+      <ProofPanel frameworkId="nist-csf" />,
+      { wrapper: createWrapper() }
+    );
+
+    // Should show an error message
+    expect(container.textContent).toBeTruthy();
+    // Should NOT expose internal paths or API details
+    expect(container.innerHTML).not.toMatch(/docs\/sources/);
+    expect(container.innerHTML).not.toMatch(/api\/ontology/);
+  });
+
+  it("renders markdown content heading when proof_content is non-null", async () => {
+    mockedHook.mockReturnValue({
+      isLoading: false,
+      isError: false,
+      data: makeProof({ proof_content: "# Verification\n\nSome text here." }),
+    } as ReturnType<typeof useFrameworkProof>);
+
+    render(
+      <ProofPanel frameworkId="nist-csf" />,
+      { wrapper: createWrapper() }
+    );
+
+    expect(screen.getByRole("heading", { name: /verification/i })).toBeInTheDocument();
+  });
+
+  it("renders no-proof message when proof_content is null", () => {
+    mockedHook.mockReturnValue({
+      isLoading: false,
+      isError: false,
+      data: makeProof({ proof_content: null }),
+    } as ReturnType<typeof useFrameworkProof>);
+
+    render(
+      <ProofPanel frameworkId="nist-csf" />,
+      { wrapper: createWrapper() }
+    );
+
+    expect(
+      screen.getByText("No proof document available")
+    ).toBeInTheDocument();
+  });
+
+  it("source link has rel=noopener noreferrer when verification_source is present", () => {
+    mockedHook.mockReturnValue({
+      isLoading: false,
+      isError: false,
+      data: makeProof({
+        verification_source: "https://example.com/nist-csf-proof",
+      }),
+    } as ReturnType<typeof useFrameworkProof>);
+
+    render(
+      <ProofPanel frameworkId="nist-csf" />,
+      { wrapper: createWrapper() }
+    );
+
+    const link = screen.getByRole("link");
+    expect(link).toHaveAttribute("rel", "noopener noreferrer");
+    expect(link).toHaveAttribute("target", "_blank");
+  });
+
+  it("no external link rendered when verification_source is null", () => {
+    mockedHook.mockReturnValue({
+      isLoading: false,
+      isError: false,
+      data: makeProof({ verification_source: null }),
+    } as ReturnType<typeof useFrameworkProof>);
+
+    render(
+      <ProofPanel frameworkId="nist-csf" />,
+      { wrapper: createWrapper() }
+    );
+
+    expect(screen.queryByRole("link")).toBeNull();
+  });
+});
