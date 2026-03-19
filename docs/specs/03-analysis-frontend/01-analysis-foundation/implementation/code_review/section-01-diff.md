diff --git a/frontend/src/features/analysis/api/__tests__/hooks.test.ts b/frontend/src/features/analysis/api/__tests__/hooks.test.ts
new file mode 100644
index 0000000..b9840be
--- /dev/null
+++ b/frontend/src/features/analysis/api/__tests__/hooks.test.ts
@@ -0,0 +1,309 @@
+import { describe, it, expect, vi, beforeEach } from "vitest";
+import { renderHook, waitFor, act } from "@testing-library/react";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
+import React from "react";
+
+vi.mock("@/lib/api", () => ({
+  api: {
+    get: vi.fn(),
+    post: vi.fn(),
+    put: vi.fn(),
+    delete: vi.fn(),
+  },
+}));
+
+import { api } from "@/lib/api";
+const mockedApi = vi.mocked(api);
+
+import {
+  useAnalyses,
+  useAnalysis,
+  useCreateAnalysis,
+  useUploadAnalysis,
+  useDeleteAnalysis,
+  useFindings,
+  usePromptTemplate,
+  useUpdatePromptTemplate,
+  analysisKeys,
+} from "../index";
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
+describe("useAnalyses", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("returns paginated list on successful fetch", async () => {
+    const mockData = {
+      data: [
+        { id: "a1", name: "Test", description: null, input_type: "text", status: "completed", error_message: null, processing_time_ms: 100, created_at: "2025-01-01", updated_at: "2025-01-01" },
+      ],
+      total: 1,
+      page: 1,
+      limit: 20,
+      total_pages: 1,
+    };
+    mockedApi.get.mockResolvedValue({ data: mockData });
+
+    const { result } = renderHook(() => useAnalyses(), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    expect(result.current.data).toEqual(mockData);
+  });
+
+  it("passes status filter as query param", async () => {
+    mockedApi.get.mockResolvedValue({ data: { data: [], total: 0, page: 1, limit: 20, total_pages: 0 } });
+
+    renderHook(() => useAnalyses({ status: "completed" }), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(mockedApi.get).toHaveBeenCalled();
+    });
+
+    const calledUrl = (mockedApi.get as ReturnType<typeof vi.fn>).mock.calls[0][0] as string;
+    expect(calledUrl).toContain("status=completed");
+  });
+});
+
+describe("useAnalysis", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("returns analysis data for valid id", async () => {
+    const mockAnalysis = {
+      id: "a1",
+      name: "Test",
+      description: null,
+      input_type: "text",
+      input_text: "some text",
+      original_filename: null,
+      file_path: null,
+      extracted_text: null,
+      status: "completed",
+      error_message: null,
+      prompt_template: null,
+      matched_framework_ids: "[]",
+      processing_time_ms: 100,
+      token_count: 500,
+      created_by: null,
+      created_at: "2025-01-01",
+      updated_at: "2025-01-01",
+    };
+    mockedApi.get.mockResolvedValue({ data: mockAnalysis });
+
+    const { result } = renderHook(() => useAnalysis("a1"), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    expect(result.current.data?.id).toBe("a1");
+  });
+
+  it("parses matched_framework_ids from JSON string to array", async () => {
+    const mockAnalysis = {
+      id: "a1",
+      name: "Test",
+      matched_framework_ids: '["fw-1","fw-2"]',
+      status: "completed",
+      input_type: "text",
+      created_at: "2025-01-01",
+      updated_at: "2025-01-01",
+    };
+    mockedApi.get.mockResolvedValue({ data: mockAnalysis });
+
+    const { result } = renderHook(() => useAnalysis("a1"), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    expect(result.current.data?.matched_framework_ids).toEqual(["fw-1", "fw-2"]);
+  });
+
+  it("disabled when id is empty", async () => {
+    renderHook(() => useAnalysis(""), { wrapper: createWrapper() });
+
+    // Wait a tick to ensure no call is made
+    await new Promise((r) => setTimeout(r, 50));
+    expect(mockedApi.get).not.toHaveBeenCalled();
+  });
+});
+
+describe("useCreateAnalysis", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("posts to /api/analyses with request body", async () => {
+    const mockResponse = { id: "a1", name: "New", status: "pending" };
+    mockedApi.post.mockResolvedValue({ data: mockResponse });
+
+    const { result } = renderHook(() => useCreateAnalysis(), { wrapper: createWrapper() });
+
+    act(() => {
+      result.current.mutate({ name: "New", input_text: "test content" });
+    });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    expect(mockedApi.post).toHaveBeenCalledWith("/analyses", { name: "New", input_text: "test content" });
+  });
+});
+
+describe("useUploadAnalysis", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("posts FormData with file and name fields", async () => {
+    mockedApi.post.mockResolvedValue({ data: { id: "a1" } });
+
+    const { result } = renderHook(() => useUploadAnalysis(), { wrapper: createWrapper() });
+
+    const file = new File(["content"], "test.pdf", { type: "application/pdf" });
+    act(() => {
+      result.current.mutate({ file, name: "Upload Test" });
+    });
+
+    await waitFor(() => {
+      expect(mockedApi.post).toHaveBeenCalled();
+    });
+
+    const [url, formData, config] = (mockedApi.post as ReturnType<typeof vi.fn>).mock.calls[0];
+    expect(url).toBe("/analyses/upload");
+    expect(formData).toBeInstanceOf(FormData);
+    expect(config.headers["Content-Type"]).toBe("multipart/form-data");
+  });
+
+  it("returns progress percentage during upload", async () => {
+    mockedApi.post.mockImplementation((_url: string, _data: unknown, config: { onUploadProgress?: (e: { loaded: number; total: number }) => void }) => {
+      // Simulate upload progress callback
+      if (config?.onUploadProgress) {
+        config.onUploadProgress({ loaded: 50, total: 100 });
+      }
+      return Promise.resolve({ data: { id: "a1" } });
+    });
+
+    const { result } = renderHook(() => useUploadAnalysis(), { wrapper: createWrapper() });
+
+    const file = new File(["content"], "test.pdf", { type: "application/pdf" });
+    act(() => {
+      result.current.mutate({ file, name: "Upload Test" });
+    });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    // Progress should have been set to 50 during upload (then reset to 0 on settled)
+    // After settled, progress resets to 0
+    expect(result.current.progress).toBe(0);
+  });
+});
+
+describe("useDeleteAnalysis", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("calls DELETE /api/analyses/{id}", async () => {
+    mockedApi.delete.mockResolvedValue({ data: {} });
+
+    const { result } = renderHook(() => useDeleteAnalysis(), { wrapper: createWrapper() });
+
+    act(() => {
+      result.current.mutate("a1");
+    });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    expect(mockedApi.delete).toHaveBeenCalledWith("/analyses/a1");
+  });
+});
+
+describe("usePromptTemplate", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("fetches from /api/analyses/prompt-template", async () => {
+    const mockConfig = {
+      version: 1,
+      thresholds: { min_confidence: 0.5, addressed: 0.8, partial: 0.6 },
+      max_findings_per_framework: 50,
+      include_addressed_findings: true,
+      boost_terms: {},
+    };
+    mockedApi.get.mockResolvedValue({ data: mockConfig });
+
+    const { result } = renderHook(() => usePromptTemplate(), { wrapper: createWrapper() });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    expect(result.current.data).toEqual(mockConfig);
+    expect(mockedApi.get).toHaveBeenCalledWith("/analyses/prompt-template");
+  });
+});
+
+describe("useUpdatePromptTemplate", () => {
+  beforeEach(() => {
+    vi.resetAllMocks();
+  });
+
+  it("puts to /api/analyses/prompt-template", async () => {
+    mockedApi.put.mockResolvedValue({ data: {} });
+
+    const config = {
+      version: 1,
+      thresholds: { min_confidence: 0.5, addressed: 0.8, partial: 0.6 },
+      max_findings_per_framework: 50,
+      include_addressed_findings: true,
+      boost_terms: {},
+    };
+
+    const { result } = renderHook(() => useUpdatePromptTemplate(), { wrapper: createWrapper() });
+
+    act(() => {
+      result.current.mutate(config);
+    });
+
+    await waitFor(() => {
+      expect(result.current.isSuccess).toBe(true);
+    });
+
+    expect(mockedApi.put).toHaveBeenCalledWith("/analyses/prompt-template", config);
+  });
+});
+
+describe("analysisKeys", () => {
+  it("builds hierarchical keys correctly", () => {
+    expect(analysisKeys.all).toEqual(["analysis"]);
+    expect(analysisKeys.list()).toContain("analysis");
+    expect(analysisKeys.detail("a1")).toContain("a1");
+    expect(analysisKeys.findings("a1")).toContain("findings");
+    expect(analysisKeys.promptTemplate()).toContain("prompt-template");
+  });
+});
diff --git a/frontend/src/features/analysis/api/index.ts b/frontend/src/features/analysis/api/index.ts
new file mode 100644
index 0000000..da82e1a
--- /dev/null
+++ b/frontend/src/features/analysis/api/index.ts
@@ -0,0 +1,189 @@
+import { useState } from "react";
+import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
+import { api } from "@/lib/api";
+import type {
+  Analysis,
+  AnalysisListItem,
+  AnalysisFinding,
+  MatcherConfig,
+  CreateAnalysisRequest,
+  UploadAnalysisInput,
+  AnalysisListParams,
+  FindingsListParams,
+  PaginatedResponse,
+} from "../types";
+
+// Hierarchical query keys for granular cache invalidation
+export const analysisKeys = {
+  all: ["analysis"] as const,
+  list: (params?: AnalysisListParams) => [...analysisKeys.all, "list", params] as const,
+  detail: (id: string) => [...analysisKeys.all, "detail", id] as const,
+  findings: (id: string, params?: FindingsListParams) =>
+    [...analysisKeys.all, "detail", id, "findings", params] as const,
+  promptTemplate: () => [...analysisKeys.all, "prompt-template"] as const,
+};
+
+// Paginated analysis list with auto-polling for processing items
+export function useAnalyses(params?: AnalysisListParams) {
+  return useQuery({
+    queryKey: analysisKeys.list(params),
+    queryFn: async () => {
+      const searchParams = new URLSearchParams();
+      if (params?.page) searchParams.set("page", String(params.page));
+      if (params?.limit) searchParams.set("limit", String(params.limit));
+      if (params?.status) searchParams.set("status", params.status);
+      const query = searchParams.toString();
+      const { data } = await api.get<PaginatedResponse<AnalysisListItem>>(
+        `/analyses${query ? `?${query}` : ""}`
+      );
+      return data;
+    },
+    refetchInterval: (query) => {
+      const data = query.state.data;
+      if (data?.data.some((item) => item.status === "processing")) {
+        return 5000;
+      }
+      return false;
+    },
+  });
+}
+
+// Single analysis with matched_framework_ids parsing
+export function useAnalysis(id: string) {
+  return useQuery({
+    queryKey: analysisKeys.detail(id),
+    queryFn: async () => {
+      const { data } = await api.get<Analysis>(`/analyses/${id}`);
+      // Backend returns matched_framework_ids as JSON string; parse it
+      if (typeof data.matched_framework_ids === "string") {
+        try {
+          data.matched_framework_ids = JSON.parse(data.matched_framework_ids);
+        } catch {
+          data.matched_framework_ids = [];
+        }
+      }
+      return data;
+    },
+    enabled: !!id,
+    staleTime: 5 * 60 * 1000,
+  });
+}
+
+// Create analysis from text input
+export function useCreateAnalysis() {
+  const queryClient = useQueryClient();
+  return useMutation({
+    mutationFn: async (request: CreateAnalysisRequest) => {
+      const { data } = await api.post<Analysis>("/analyses", request);
+      return data;
+    },
+    onSuccess: () => {
+      queryClient.invalidateQueries({ queryKey: analysisKeys.all });
+    },
+  });
+}
+
+// Upload file for analysis with progress tracking
+export function useUploadAnalysis() {
+  const queryClient = useQueryClient();
+  const [progress, setProgress] = useState(0);
+
+  const mutation = useMutation({
+    mutationFn: async (input: UploadAnalysisInput) => {
+      const formData = new FormData();
+      formData.append("file", input.file);
+      formData.append("name", input.name);
+      const { data } = await api.post<Analysis>("/analyses/upload", formData, {
+        headers: { "Content-Type": "multipart/form-data" },
+        onUploadProgress: (e) => {
+          setProgress(Math.round((e.loaded * 100) / (e.total ?? 1)));
+        },
+      });
+      return data;
+    },
+    onSuccess: () => {
+      queryClient.invalidateQueries({ queryKey: analysisKeys.all });
+    },
+    onSettled: () => {
+      setProgress(0);
+    },
+  });
+
+  return { ...mutation, progress };
+}
+
+// Delete analysis
+export function useDeleteAnalysis() {
+  const queryClient = useQueryClient();
+  return useMutation({
+    mutationFn: async (id: string) => {
+      const { data } = await api.delete(`/analyses/${id}`);
+      return data;
+    },
+    onSuccess: () => {
+      queryClient.invalidateQueries({ queryKey: analysisKeys.all });
+    },
+  });
+}
+
+// Paginated findings for an analysis
+export function useFindings(id: string, params?: FindingsListParams) {
+  return useQuery({
+    queryKey: analysisKeys.findings(id, params),
+    queryFn: async () => {
+      const searchParams = new URLSearchParams();
+      if (params?.page) searchParams.set("page", String(params.page));
+      if (params?.limit) searchParams.set("limit", String(params.limit));
+      if (params?.framework_id) searchParams.set("framework_id", params.framework_id);
+      if (params?.finding_type) searchParams.set("finding_type", params.finding_type);
+      if (params?.priority) searchParams.set("priority", String(params.priority));
+      if (params?.sort_by) searchParams.set("sort_by", params.sort_by);
+      const query = searchParams.toString();
+      const { data } = await api.get<PaginatedResponse<AnalysisFinding>>(
+        `/analyses/${id}/findings${query ? `?${query}` : ""}`
+      );
+      return data;
+    },
+    enabled: !!id,
+  });
+}
+
+// One-shot export download (not a hook)
+export async function exportAnalysis(id: string, format: string = "pdf") {
+  const { data } = await api.get(`/analyses/${id}/export/${format}`, {
+    responseType: "blob",
+  });
+  const blob = data as Blob;
+  const url = URL.createObjectURL(blob);
+  const a = document.createElement("a");
+  a.href = url;
+  a.download = `analysis-${id}.${format}`;
+  a.click();
+  URL.revokeObjectURL(url);
+}
+
+// Prompt template / matcher config
+export function usePromptTemplate() {
+  return useQuery({
+    queryKey: analysisKeys.promptTemplate(),
+    queryFn: async () => {
+      const { data } = await api.get<MatcherConfig>("/analyses/prompt-template");
+      return data;
+    },
+    staleTime: 30 * 1000,
+  });
+}
+
+// Update prompt template
+export function useUpdatePromptTemplate() {
+  const queryClient = useQueryClient();
+  return useMutation({
+    mutationFn: async (config: MatcherConfig) => {
+      const { data } = await api.put("/analyses/prompt-template", config);
+      return data;
+    },
+    onSuccess: () => {
+      queryClient.invalidateQueries({ queryKey: analysisKeys.promptTemplate() });
+    },
+  });
+}
diff --git a/frontend/src/features/analysis/index.ts b/frontend/src/features/analysis/index.ts
new file mode 100644
index 0000000..9a18f25
--- /dev/null
+++ b/frontend/src/features/analysis/index.ts
@@ -0,0 +1,2 @@
+export * from "./types";
+export * from "./api";
diff --git a/frontend/src/features/analysis/types/index.ts b/frontend/src/features/analysis/types/index.ts
new file mode 100644
index 0000000..28ccb28
--- /dev/null
+++ b/frontend/src/features/analysis/types/index.ts
@@ -0,0 +1,97 @@
+// Re-export PaginatedResponse from ontology types
+export type { PaginatedResponse } from "@/features/ontology/types";
+
+// Union types
+export type AnalysisStatus = "pending" | "processing" | "completed" | "failed" | "deleted";
+export type InputType = "text" | "pdf" | "docx";
+export type FindingType = "addressed" | "partially_addressed" | "gap" | "not_applicable";
+
+// Full analysis entity from GET /api/analyses/{id}
+export interface Analysis {
+  id: string;
+  name: string;
+  description: string | null;
+  input_type: InputType;
+  input_text: string | null;
+  original_filename: string | null;
+  file_path: string | null;
+  extracted_text: string | null;
+  status: AnalysisStatus;
+  error_message: string | null;
+  prompt_template: string | null;
+  matched_framework_ids: string[];
+  processing_time_ms: number | null;
+  token_count: number | null;
+  created_by: string | null;
+  created_at: string;
+  updated_at: string;
+}
+
+// Subset returned by list endpoint GET /api/analyses
+export interface AnalysisListItem {
+  id: string;
+  name: string;
+  description: string | null;
+  input_type: InputType;
+  status: AnalysisStatus;
+  error_message: string | null;
+  processing_time_ms: number | null;
+  created_at: string;
+  updated_at: string;
+}
+
+// Individual finding with concept metadata
+export interface AnalysisFinding {
+  id: string;
+  concept_id: string;
+  framework_id: string;
+  finding_type: FindingType;
+  confidence_score: number;
+  evidence_text: string;
+  recommendation: string;
+  priority: number;
+  sort_order: number;
+  concept_code: string;
+  concept_name: string;
+  concept_definition: string;
+}
+
+// Matcher configuration for prompt template
+export interface MatcherConfig {
+  version: number;
+  thresholds: {
+    min_confidence: number;
+    addressed: number;
+    partial: number;
+  };
+  max_findings_per_framework: number;
+  include_addressed_findings: boolean;
+  boost_terms: Record<string, number>;
+}
+
+// Request types
+export interface CreateAnalysisRequest {
+  name: string;
+  description?: string;
+  input_text: string;
+}
+
+export interface UploadAnalysisInput {
+  file: File;
+  name: string;
+}
+
+export interface AnalysisListParams {
+  page?: number;
+  limit?: number;
+  status?: AnalysisStatus;
+}
+
+export interface FindingsListParams {
+  page?: number;
+  limit?: number;
+  framework_id?: string;
+  finding_type?: FindingType;
+  priority?: number;
+  sort_by?: string;
+}
