import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { api } from "@/lib/api";
import type {
  Assessment,
  ComplianceScore,
  CreateAssessmentRequest,
  AssessmentFilters,
  PaginatedResponse,
} from "../types";

// Query keys
export const complianceKeys = {
  all: ["compliance"] as const,
  assessments: (filters?: AssessmentFilters) =>
    [...complianceKeys.all, "assessments", filters] as const,
  assessment: (id: string) =>
    [...complianceKeys.all, "assessment", id] as const,
  score: (id: string) =>
    [...complianceKeys.all, "assessment", id, "score"] as const,
};

// Fetch assessments with optional filters
export function useAssessments(filters?: AssessmentFilters) {
  return useQuery({
    queryKey: complianceKeys.assessments(filters),
    queryFn: async () => {
      const params = new URLSearchParams();
      if (filters?.page) params.set("page", String(filters.page));
      if (filters?.limit) params.set("limit", String(filters.limit));
      if (filters?.framework_id)
        params.set("framework_id", filters.framework_id);
      if (filters?.status) params.set("status", filters.status);
      const query = params.toString();
      const { data } = await api.get<PaginatedResponse<Assessment>>(
        `/compliance/assessments${query ? `?${query}` : ""}`
      );
      return data;
    },
    staleTime: 1000 * 60, // 1 minute
  });
}

// Fetch compliance score for an assessment
export function useAssessmentScore(id: string) {
  return useQuery({
    queryKey: complianceKeys.score(id),
    queryFn: async () => {
      const { data } = await api.get<ComplianceScore>(
        `/compliance/assessments/${id}/score`
      );
      return data;
    },
    staleTime: 1000 * 60, // 1 minute
    enabled: !!id,
  });
}

// Create assessment mutation
export function useCreateAssessment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (request: CreateAssessmentRequest) => {
      const { data } = await api.post<Assessment>(
        "/compliance/assessments",
        request
      );
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: complianceKeys.all,
      });
    },
  });
}

// Delete assessment mutation
export function useDeleteAssessment() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async (id: string) => {
      await api.delete(`/compliance/assessments/${id}`);
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: complianceKeys.all,
      });
    },
  });
}
