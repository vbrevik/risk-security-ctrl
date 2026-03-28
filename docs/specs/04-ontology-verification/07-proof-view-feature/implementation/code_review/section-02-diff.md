diff --git a/frontend/src/features/ontology/api/__tests__/hooks.test.ts b/frontend/src/features/ontology/api/__tests__/hooks.test.ts
index 3db8372..5e64636 100644
--- a/frontend/src/features/ontology/api/__tests__/hooks.test.ts
+++ b/frontend/src/features/ontology/api/__tests__/hooks.test.ts
@@ -2,7 +2,7 @@ import { describe, it, expect, vi, beforeEach } from "vitest";
 import { renderHook, waitFor } from "@testing-library/react";
 import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
 import React from "react";
-import { useAllConcepts, useFrameworkStats } from "../index";
+import { useAllConcepts, useFrameworkStats, useFrameworkProof } from "../index";
 
 // Mock the api module
 vi.mock("@/lib/api", () => ({
@@ -30,8 +30,9 @@ function createWrapper() {
   };
 }
 
-const FW_A = { id: "fw-a", name: "FW A", version: null, description: null, source_url: null, created_at: "", updated_at: "" };
-const FW_B = { id: "fw-b", name: "FW B", version: null, description: null, source_url: null, created_at: "", updated_at: "" };
+const VERIFICATION_FIELDS = { verification_status: null, verification_date: null, verification_source: null, verification_notes: null };
+const FW_A = { id: "fw-a", name: "FW A", version: null, description: null, source_url: null, created_at: "", updated_at: "", ...VERIFICATION_FIELDS };
+const FW_B = { id: "fw-b", name: "FW B", version: null, description: null, source_url: null, created_at: "", updated_at: "", ...VERIFICATION_FIELDS };
 
 function makeConcept(id: string, frameworkId: string, type = "control") {
   return {
@@ -188,3 +189,96 @@ describe("useFrameworkStats", () => {
     expect(statsB!.relationshipCount).toBe(1);
   });
 });
+
+const MOCK_PROOF = {
+  framework_id: "nist-csf",
+  verification_status: "verified",
+  verification_date: "2025-01-15",
+  verification_source: "https://example.com/nist-csf",
+  verification_notes: "Verified against official publication",
+  proof_content: "# NIST CSF Proof\n\nVerification details...",
+};
+
+describe("useFrameworkProof", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("fetches GET /api/ontology/frameworks/{id}/proof when frameworkId is provided", async () => {
+    mockedApi.get.mockResolvedValueOnce({ data: MOCK_PROOF });
+
+    const { result } = renderHook(() => useFrameworkProof("nist-csf"), {
+      wrapper: createWrapper(),
+    });
+
+    await waitFor(() => {
+      expect(result.current.isLoading).toBe(false);
+    });
+
+    expect(result.current.data?.framework_id).toBe("nist-csf");
+    expect(mockedApi.get).toHaveBeenCalledWith(
+      "/ontology/frameworks/nist-csf/proof"
+    );
+  });
+
+  it("does NOT call api.get when frameworkId is null (skipToken)", async () => {
+    const { result } = renderHook(() => useFrameworkProof(null), {
+      wrapper: createWrapper(),
+    });
+
+    // Short wait — no fetch should fire
+    await new Promise((r) => setTimeout(r, 50));
+
+    expect(mockedApi.get).not.toHaveBeenCalled();
+    expect(result.current.data).toBeUndefined();
+  });
+
+  it("passes through null proof_content from API response", async () => {
+    mockedApi.get.mockResolvedValueOnce({
+      data: { ...MOCK_PROOF, proof_content: null },
+    });
+
+    const { result } = renderHook(() => useFrameworkProof("nist-csf"), {
+      wrapper: createWrapper(),
+    });
+
+    await waitFor(() => {
+      expect(result.current.isLoading).toBe(false);
+    });
+
+    expect(result.current.data?.proof_content).toBeNull();
+  });
+
+  it("passes through null verification_status from API response", async () => {
+    mockedApi.get.mockResolvedValueOnce({
+      data: { ...MOCK_PROOF, verification_status: null },
+    });
+
+    const { result } = renderHook(() => useFrameworkProof("nist-csf"), {
+      wrapper: createWrapper(),
+    });
+
+    await waitFor(() => {
+      expect(result.current.isLoading).toBe(false);
+    });
+
+    expect(result.current.data?.verification_status).toBeNull();
+  });
+
+  it("isLoading is true while fetch is in flight, false after resolution", async () => {
+    mockedApi.get.mockResolvedValueOnce({ data: MOCK_PROOF });
+
+    const { result } = renderHook(() => useFrameworkProof("nist-csf"), {
+      wrapper: createWrapper(),
+    });
+
+    // Initially loading
+    expect(result.current.isLoading).toBe(true);
+
+    await waitFor(() => {
+      expect(result.current.isLoading).toBe(false);
+    });
+
+    expect(result.current.data).toEqual(MOCK_PROOF);
+  });
+});
diff --git a/frontend/src/features/ontology/api/index.ts b/frontend/src/features/ontology/api/index.ts
index ba12e52..18eb27a 100644
--- a/frontend/src/features/ontology/api/index.ts
+++ b/frontend/src/features/ontology/api/index.ts
@@ -1,4 +1,4 @@
-import { useQuery, useQueries } from "@tanstack/react-query";
+import { useQuery, useQueries, skipToken } from "@tanstack/react-query";
 import { useMemo } from "react";
 import { api } from "@/lib/api";
 import type {
@@ -9,6 +9,7 @@ import type {
   PaginatedResponse,
   Topic,
   FrameworkStats,
+  FrameworkProof,
 } from "../types";
 
 // Query keys
@@ -25,6 +26,7 @@ export const ontologyKeys = {
   search: (query: string, frameworkId?: string) =>
     [...ontologyKeys.all, "search", { query, frameworkId }] as const,
   topics: () => [...ontologyKeys.all, "topics"] as const,
+  proof: (id: string) => [...ontologyKeys.framework(id), "proof"] as const,
 };
 
 // Fetch all frameworks
@@ -259,3 +261,24 @@ export function useFrameworkStats(): {
 
   return { data, isLoading };
 }
+
+/**
+ * Lazily fetches proof and verification metadata for a framework.
+ * Only fires when frameworkId is non-null (user has opened the proof panel).
+ * Uses skipToken (TanStack Query v5) for type-safe conditional fetching.
+ * staleTime: Infinity because proof files are static verification artifacts.
+ */
+export function useFrameworkProof(frameworkId: string | null) {
+  return useQuery({
+    queryKey: frameworkId ? ontologyKeys.proof(frameworkId) : [],
+    queryFn: frameworkId
+      ? async () => {
+          const { data } = await api.get<FrameworkProof>(
+            `/ontology/frameworks/${frameworkId}/proof`
+          );
+          return data;
+        }
+      : skipToken,
+    staleTime: Infinity,
+  });
+}
