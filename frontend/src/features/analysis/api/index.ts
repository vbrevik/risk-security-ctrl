import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import type {
  Analysis,
  AnalysisListItem,
  AnalysisFinding,
  MatcherConfig,
  CreateAnalysisRequest,
  UploadAnalysisInput,
  AnalysisListParams,
  FindingsListParams,
  PaginatedResponse,
} from "../types";

// Hierarchical query keys for granular cache invalidation
export const analysisKeys = {
  all: ["analysis"] as const,
  list: (params?: AnalysisListParams) => [...analysisKeys.all, "list", params] as const,
  detail: (id: string) => [...analysisKeys.all, "detail", id] as const,
  findings: (id: string, params?: FindingsListParams) =>
    [...analysisKeys.all, "detail", id, "findings", params] as const,
  promptTemplate: () => [...analysisKeys.all, "prompt-template"] as const,
};

// Paginated analysis list with auto-polling for processing items
export function useAnalyses(params?: AnalysisListParams) {
  return useQuery({
    queryKey: analysisKeys.list(params),
    queryFn: async () => {
      const searchParams = new URLSearchParams();
      if (params?.page != null) searchParams.set("page", String(params.page));
      if (params?.limit != null) searchParams.set("limit", String(params.limit));
      if (params?.status) searchParams.set("status", params.status);
      const query = searchParams.toString();
      const { data } = await api.get<PaginatedResponse<AnalysisListItem>>(
        `/analyses${query ? `?${query}` : ""}`
      );
      return data;
    },
    refetchInterval: (query) => {
      const data = query.state.data;
      if (data?.items.some((item) => item.status === "processing")) {
        return 5000;
      }
      return false;
    },
  });
}

// Single analysis with matched_framework_ids parsing
export function useAnalysis(id: string) {
  return useQuery({
    queryKey: analysisKeys.detail(id),
    queryFn: async () => {
      const { data } = await api.get<Analysis>(`/analyses/${id}`);
      // Backend returns matched_framework_ids as JSON string; parse it
      if (typeof data.matched_framework_ids === "string") {
        try {
          data.matched_framework_ids = JSON.parse(data.matched_framework_ids);
        } catch {
          data.matched_framework_ids = [];
        }
      }
      return data;
    },
    enabled: !!id,
    staleTime: 5 * 60 * 1000,
    refetchOnMount: "always" as const,
    refetchInterval: (query) => {
      if (query.state.data?.status === "processing") {
        return 5000;
      }
      return false;
    },
  });
}

// Create analysis from text input
export function useCreateAnalysis() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (request: CreateAnalysisRequest) => {
      const { data } = await api.post<Analysis>("/analyses", request);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: analysisKeys.all });
    },
  });
}

// Upload file for analysis with progress tracking
export function useUploadAnalysis() {
  const queryClient = useQueryClient();
  const [progress, setProgress] = useState(0);

  const mutation = useMutation({
    mutationFn: async (input: UploadAnalysisInput) => {
      const formData = new FormData();
      formData.append("file", input.file);
      formData.append("name", input.name);
      const { data } = await api.post<Analysis>("/analyses/upload", formData, {
        headers: { "Content-Type": "multipart/form-data" },
        onUploadProgress: (e) => {
          setProgress(Math.round((e.loaded * 100) / (e.total ?? 1)));
        },
      });
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: analysisKeys.all });
    },
    onSettled: () => {
      setProgress(0);
    },
  });

  return { ...mutation, progress };
}

// Delete analysis
export function useDeleteAnalysis() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (id: string) => {
      const { data } = await api.delete(`/analyses/${id}`);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: analysisKeys.all });
    },
  });
}

// Paginated findings for an analysis
export function useFindings(id: string, params?: FindingsListParams) {
  return useQuery({
    queryKey: analysisKeys.findings(id, params),
    queryFn: async () => {
      const searchParams = new URLSearchParams();
      if (params?.page != null) searchParams.set("page", String(params.page));
      if (params?.limit != null) searchParams.set("limit", String(params.limit));
      if (params?.framework_id) searchParams.set("framework_id", params.framework_id);
      if (params?.finding_type) searchParams.set("finding_type", params.finding_type);
      if (params?.priority != null) searchParams.set("priority", String(params.priority));
      if (params?.sort_by) searchParams.set("sort_by", params.sort_by);
      const query = searchParams.toString();
      const { data } = await api.get<PaginatedResponse<AnalysisFinding>>(
        `/analyses/${id}/findings${query ? `?${query}` : ""}`
      );
      return data;
    },
    enabled: !!id,
  });
}

// One-shot export download (not a hook)
export async function exportAnalysis(id: string, format: string = "pdf") {
  const { data } = await api.get(`/analyses/${id}/export/${format}`, {
    responseType: "blob",
  });
  const blob = data as Blob;
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = `analysis-${id}.${format}`;
  a.click();
  URL.revokeObjectURL(url);
}

// Prompt template / matcher config
export function usePromptTemplate() {
  return useQuery({
    queryKey: analysisKeys.promptTemplate(),
    queryFn: async () => {
      const { data } = await api.get<MatcherConfig>("/analyses/prompt-template");
      return data;
    },
    staleTime: 30 * 1000,
  });
}

// Update prompt template
export function useUpdatePromptTemplate() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (config: MatcherConfig) => {
      const { data } = await api.put("/analyses/prompt-template", config);
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: analysisKeys.promptTemplate() });
    },
  });
}
