import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import type {
  Framework,
  Concept,
  Relationship,
  ConceptWithRelationships,
  PaginatedResponse,
  Topic,
} from "../types";

// Query keys
export const ontologyKeys = {
  all: ["ontology"] as const,
  frameworks: () => [...ontologyKeys.all, "frameworks"] as const,
  framework: (id: string) => [...ontologyKeys.frameworks(), id] as const,
  concepts: (frameworkId?: string) =>
    [...ontologyKeys.all, "concepts", { frameworkId }] as const,
  concept: (id: string) => [...ontologyKeys.all, "concept", id] as const,
  conceptRelationships: (id: string) =>
    [...ontologyKeys.concept(id), "relationships"] as const,
  relationships: () => [...ontologyKeys.all, "relationships"] as const,
  search: (query: string, frameworkId?: string) =>
    [...ontologyKeys.all, "search", { query, frameworkId }] as const,
  topics: () => [...ontologyKeys.all, "topics"] as const,
};

// Fetch all frameworks
export function useFrameworks() {
  return useQuery({
    queryKey: ontologyKeys.frameworks(),
    queryFn: async () => {
      const { data } = await api.get<Framework[]>("/ontology/frameworks");
      return data;
    },
    staleTime: Infinity,
  });
}

// Fetch single framework
export function useFramework(id: string) {
  return useQuery({
    queryKey: ontologyKeys.framework(id),
    queryFn: async () => {
      const { data } = await api.get<Framework>(`/ontology/frameworks/${id}`);
      return data;
    },
    staleTime: Infinity,
    enabled: !!id,
  });
}

// Fetch concepts for a framework
export function useConcepts(frameworkId?: string) {
  return useQuery({
    queryKey: ontologyKeys.concepts(frameworkId),
    queryFn: async () => {
      const params = new URLSearchParams();
      if (frameworkId) params.set("framework_id", frameworkId);
      params.set("limit", "500"); // Fetch all concepts for tree building
      const { data } = await api.get<PaginatedResponse<Concept>>(
        `/ontology/concepts?${params}`
      );
      return data.data;
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    enabled: !!frameworkId,
  });
}

// Fetch single concept
export function useConcept(id: string) {
  return useQuery({
    queryKey: ontologyKeys.concept(id),
    queryFn: async () => {
      const { data } = await api.get<Concept>(`/ontology/concepts/${id}`);
      return data;
    },
    staleTime: 1000 * 60 * 5,
    enabled: !!id,
  });
}

// Fetch concept with relationships
export function useConceptRelationships(id: string) {
  return useQuery({
    queryKey: ontologyKeys.conceptRelationships(id),
    queryFn: async () => {
      const { data } = await api.get<ConceptWithRelationships>(
        `/ontology/concepts/${id}/relationships`
      );
      return data;
    },
    staleTime: 1000 * 60 * 5,
    enabled: !!id,
  });
}

// Fetch all relationships
export function useRelationships() {
  return useQuery({
    queryKey: ontologyKeys.relationships(),
    queryFn: async () => {
      const { data } = await api.get<Relationship[]>("/ontology/relationships");
      return data;
    },
    staleTime: 1000 * 60 * 5,
  });
}

// Fetch topic tags for cross-cutting theme filtering
export function useTopics() {
  return useQuery({
    queryKey: ontologyKeys.topics(),
    queryFn: async () => {
      const { data } = await api.get<Topic[]>("/ontology/topics");
      return data;
    },
    staleTime: Infinity,
  });
}

// Search concepts
export function useSearchConcepts(query: string, frameworkId?: string) {
  return useQuery({
    queryKey: ontologyKeys.search(query, frameworkId),
    queryFn: async () => {
      const params = new URLSearchParams({ q: query });
      if (frameworkId) params.set("framework_id", frameworkId);
      const { data } = await api.get<PaginatedResponse<Concept>>(
        `/ontology/concepts/search?${params}`
      );
      return data.data;
    },
    staleTime: 1000 * 30, // 30 seconds
    enabled: query.length >= 2,
  });
}
