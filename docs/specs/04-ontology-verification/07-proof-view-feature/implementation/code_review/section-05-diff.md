diff --git a/frontend/src/features/ontology/components/FrameworkProfile.tsx b/frontend/src/features/ontology/components/FrameworkProfile.tsx
index 90fe9bf..19ceb9c 100644
--- a/frontend/src/features/ontology/components/FrameworkProfile.tsx
+++ b/frontend/src/features/ontology/components/FrameworkProfile.tsx
@@ -1,7 +1,10 @@
 import { useState, useEffect } from "react";
-import { BookOpen, ExternalLink, ChevronRight, ChevronDown } from "lucide-react";
+import { BookOpen, ExternalLink, ChevronRight, ChevronDown, ShieldCheck } from "lucide-react";
+import { useTranslation } from "react-i18next";
 import type { Framework, Concept, Relationship, FrameworkStats } from "../types";
 import { getFrameworkColor } from "../utils/graphTransform";
+import { VerificationBadge } from "./VerificationBadge";
+import { ProofPanel } from "./ProofPanel";
 
 interface FrameworkProfileProps {
   framework: Framework | null;
@@ -36,12 +39,19 @@ export function FrameworkProfile({
   isLoading,
 }: FrameworkProfileProps) {
   const [expanded, setExpanded] = useState<Set<string>>(new Set());
+  const [showProof, setShowProof] = useState(false);
+  const { t } = useTranslation("ontology");
 
   // Reset expanded state when framework changes
   useEffect(() => {
     setExpanded(new Set());
   }, [framework?.id]);
 
+  // Reset proof panel when framework changes
+  useEffect(() => {
+    setShowProof(false);
+  }, [framework?.id]);
+
   if (!framework) {
     return (
       <div className="flex-1 flex flex-col items-center justify-center text-foreground/40">
@@ -113,23 +123,38 @@ export function FrameworkProfile({
           {framework.version && (
             <span className="tech-badge">{framework.version}</span>
           )}
+          <VerificationBadge status={framework.verification_status} />
         </div>
         {framework.description && (
           <p className="text-sm text-foreground/70 mb-2">{framework.description}</p>
         )}
-        {framework.source_url && (
-          <a
-            href={framework.source_url}
-            target="_blank"
-            rel="noopener noreferrer"
-            className="inline-flex items-center gap-1 text-xs text-foreground/50 hover:text-foreground/80 transition-colors"
-          >
-            <ExternalLink className="w-3 h-3" />
-            Source
-          </a>
-        )}
+        <div className="flex flex-wrap items-center gap-3">
+          {framework.source_url && (
+            <a
+              href={framework.source_url}
+              target="_blank"
+              rel="noopener noreferrer"
+              className="inline-flex items-center gap-1 text-xs text-foreground/50 hover:text-foreground/80 transition-colors"
+            >
+              <ExternalLink className="w-3 h-3" />
+              Source
+            </a>
+          )}
+          {framework.verification_status !== null && (
+            <button
+              onClick={() => setShowProof((prev) => !prev)}
+              className="inline-flex items-center gap-1 text-xs text-foreground/50 hover:text-foreground/80 transition-colors"
+            >
+              <ShieldCheck className="w-3 h-3" />
+              {showProof ? t("proof.hideProof", "Hide Proof") : t("proof.viewProof", "View Proof")}
+            </button>
+          )}
+        </div>
       </div>
 
+      {/* Proof Panel */}
+      {showProof && <ProofPanel frameworkId={framework.id} />}
+
       {/* Stats Strip */}
       {stats && (
         <div className="grid grid-cols-4 gap-3">
@@ -203,12 +228,12 @@ export function FrameworkProfile({
                 />
                 <span className="flex-1 font-mono text-xs">{conn.name}</span>
                 <span className="tech-badge text-[10px]">{conn.count}</span>
-                {conn.types.map((t) => (
+                {conn.types.map((relType) => (
                   <span
-                    key={t}
-                    className={`text-[9px] px-1.5 py-0.5 rounded-full ${REL_TYPE_COLORS[t] ?? "bg-gray-500/20 text-gray-700"}`}
+                    key={relType}
+                    className={`text-[9px] px-1.5 py-0.5 rounded-full ${REL_TYPE_COLORS[relType] ?? "bg-gray-500/20 text-gray-700"}`}
                   >
-                    {t.replace(/_/g, " ")}
+                    {relType.replace(/_/g, " ")}
                   </span>
                 ))}
               </div>
diff --git a/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx b/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx
index 24073c2..f9368b3 100644
--- a/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx
+++ b/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx
@@ -1,9 +1,20 @@
-import { describe, it, expect } from "vitest";
-import { render, screen } from "@testing-library/react";
+import { describe, it, expect, vi } from "vitest";
+import { render, screen, fireEvent } from "@testing-library/react";
 import React from "react";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
 import { FrameworkProfile } from "../FrameworkProfile";
 import type { Framework, Concept, Relationship, FrameworkStats } from "../../types";
 
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string, fallback?: string) => fallback ?? key,
+  }),
+}));
+
+vi.mock("../../api", () => ({
+  useFrameworkProof: vi.fn(() => ({ isLoading: true, isError: false, data: undefined })),
+}));
+
 const FW: Framework = {
   id: "iso31000",
   name: "ISO 31000",
@@ -12,6 +23,10 @@ const FW: Framework = {
   source_url: "https://iso.org/31000",
   created_at: "",
   updated_at: "",
+  verification_status: "verified",
+  verification_date: "2025-01-15",
+  verification_source: "https://example.com/proof",
+  verification_notes: null,
 };
 
 const FW_B: Framework = {
@@ -22,8 +37,21 @@ const FW_B: Framework = {
   source_url: null,
   created_at: "",
   updated_at: "",
+  verification_status: null,
+  verification_date: null,
+  verification_source: null,
+  verification_notes: null,
 };
 
+function createWrapper() {
+  const queryClient = new QueryClient({
+    defaultOptions: { queries: { retry: false } },
+  });
+  return function Wrapper({ children }: { children: React.ReactNode }) {
+    return React.createElement(QueryClientProvider, { client: queryClient }, children);
+  };
+}
+
 function makeConcept(id: string, type: string, parentId: string | null = null): Concept {
   return {
     id,
@@ -138,3 +166,81 @@ describe("FrameworkProfile", () => {
     expect(screen.getByText(/select a framework/i)).toBeInTheDocument();
   });
 });
+
+const DEFAULT_PROPS = {
+  concepts: CONCEPTS,
+  relationships: RELATIONSHIPS,
+  stats: STATS,
+  frameworks: [FW, FW_B],
+  conceptToFramework: new Map([["c1", "iso31000"], ["c2", "iso31000"], ["c3", "iso31000"], ["c1-1", "iso31000"], ["ext1", "nist-csf"]]),
+  isLoading: false,
+};
+
+describe("FrameworkProfile – verification UI", () => {
+  it("renders VerificationBadge when verification_status is non-null", () => {
+    render(
+      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
+      { wrapper: createWrapper() }
+    );
+    // FW has verification_status: "verified" — badge should be present
+    const badge = document.querySelector("[aria-label]");
+    expect(badge).not.toBeNull();
+  });
+
+  it("renders VerificationBadge in fallback style when verification_status is null", () => {
+    render(
+      <FrameworkProfile framework={FW_B} {...DEFAULT_PROPS} />,
+      { wrapper: createWrapper() }
+    );
+    // FW_B has verification_status: null — badge renders unknown/fallback
+    const badge = document.querySelector("[aria-label]");
+    expect(badge).not.toBeNull();
+  });
+
+  it("renders View Proof button when verification_status is non-null", () => {
+    render(
+      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
+      { wrapper: createWrapper() }
+    );
+    expect(screen.getByRole("button", { name: /view proof/i })).toBeInTheDocument();
+  });
+
+  it("does not render View Proof button when verification_status is null", () => {
+    render(
+      <FrameworkProfile framework={FW_B} {...DEFAULT_PROPS} />,
+      { wrapper: createWrapper() }
+    );
+    expect(screen.queryByRole("button", { name: /view proof/i })).toBeNull();
+  });
+
+  it("mounts ProofPanel after clicking View Proof button", () => {
+    render(
+      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
+      { wrapper: createWrapper() }
+    );
+    fireEvent.click(screen.getByRole("button", { name: /view proof/i }));
+    // ProofPanel renders loading skeletons (mocked useFrameworkProof returns isLoading: true)
+    const skeletons = document.querySelectorAll(".animate-pulse");
+    expect(skeletons.length).toBeGreaterThanOrEqual(1);
+  });
+
+  it("hides ProofPanel when framework changes", () => {
+    const { rerender } = render(
+      <FrameworkProfile framework={FW} {...DEFAULT_PROPS} />,
+      { wrapper: createWrapper() }
+    );
+    fireEvent.click(screen.getByRole("button", { name: /view proof/i }));
+    // Panel is visible — skeletons are present
+    expect(document.querySelectorAll(".animate-pulse").length).toBeGreaterThanOrEqual(1);
+
+    // Switch to FW_B (no verification_status)
+    rerender(
+      <QueryClientProvider client={new QueryClient({ defaultOptions: { queries: { retry: false } } })}>
+        <FrameworkProfile framework={FW_B} {...DEFAULT_PROPS} />
+      </QueryClientProvider>
+    );
+    // Proof panel should be hidden — no loading skeletons from ProofPanel
+    expect(screen.queryByRole("button", { name: /view proof/i })).toBeNull();
+    expect(screen.queryByRole("button", { name: /hide proof/i })).toBeNull();
+  });
+});
