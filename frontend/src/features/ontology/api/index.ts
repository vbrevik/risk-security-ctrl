import { useQuery, useQueries, skipToken } from "@tanstack/react-query";
import { useMemo } from "react";
import { api } from "@/lib/api";
import type {
  Framework,
  Concept,
  Relationship,
  ConceptWithRelationships,
  PaginatedResponse,
  Topic,
  FrameworkStats,
  FrameworkProof,
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
  proof: (id: string) => [...ontologyKeys.framework(id), "proof"] as const,
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
      const params = new URLSearchParams({ q: query, limit: "500" });
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

// Fetch all pages of concepts for a single framework
async function fetchAllConceptsForFramework(
  frameworkId: string
): Promise<Concept[]> {
  const allConcepts: Concept[] = [];
  let page = 1;

  while (true) {
    const params = new URLSearchParams();
    params.set("framework_id", frameworkId);
    params.set("limit", "500");
    params.set("page", String(page));
    const { data } = await api.get<PaginatedResponse<Concept>>(
      `/ontology/concepts?${params}`
    );
    allConcepts.push(...data.data);
    if (page >= data.total_pages) break;
    page++;
  }

  return allConcepts;
}

// Fetch all concepts across all frameworks
export function useAllConcepts(): {
  data: Concept[];
  conceptToFramework: Map<string, string>;
  isLoading: boolean;
  errors: Error[];
} {
  const { data: frameworks, isLoading: fwsLoading } = useFrameworks();

  const queries = useQueries({
    queries: (frameworks ?? []).map((fw) => ({
      queryKey: [...ontologyKeys.concepts(fw.id), "all"],
      queryFn: () => fetchAllConceptsForFramework(fw.id),
      staleTime: 1000 * 60 * 5,
    })),
  });

  const queryDataKeys = queries.map((q) => q.dataUpdatedAt).join(",");

  const data = useMemo(
    () => queries.flatMap((q) => q.data ?? []),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [queryDataKeys]
  );

  const conceptToFramework = useMemo(() => {
    const map = new Map<string, string>();
    for (const concept of data) {
      map.set(concept.id, concept.framework_id);
    }
    return map;
  }, [data]);

  const errors = useMemo(
    () =>
      queries
        .filter((q) => q.error != null)
        .map((q) => q.error as Error),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [queryDataKeys]
  );

  const isLoading = fwsLoading || queries.some((q) => q.isPending);

  return { data, conceptToFramework, isLoading, errors };
}

// Compute per-framework statistics from concepts and relationships
export function useFrameworkStats(): {
  data: Map<string, FrameworkStats>;
  isLoading: boolean;
} {
  const { data: frameworks, isLoading: fwLoading } = useFrameworks();
  const { data: allConcepts, conceptToFramework, isLoading: conceptsLoading } = useAllConcepts();
  const { data: relationships, isLoading: relsLoading } = useRelationships();

  const isLoading = fwLoading || conceptsLoading || relsLoading;

  const data = useMemo(() => {
    const stats = new Map<string, FrameworkStats>();
    if (!frameworks) return stats;

    for (const fw of frameworks) {
      const fwConcepts = allConcepts.filter((c) => c.framework_id === fw.id);
      const conceptTypes: Record<string, number> = {};
      for (const c of fwConcepts) {
        conceptTypes[c.concept_type] = (conceptTypes[c.concept_type] || 0) + 1;
      }

      const fwConceptIds = new Set(fwConcepts.map((c) => c.id));
      const fwRelationships = (relationships ?? []).filter(
        (r) =>
          fwConceptIds.has(r.source_concept_id) ||
          fwConceptIds.has(r.target_concept_id)
      );

      const connectedFws = new Set<string>();
      for (const rel of fwRelationships) {
        const sourceFw = conceptToFramework.get(rel.source_concept_id);
        const targetFw = conceptToFramework.get(rel.target_concept_id);
        if (sourceFw && sourceFw !== fw.id) connectedFws.add(sourceFw);
        if (targetFw && targetFw !== fw.id) connectedFws.add(targetFw);
      }

      stats.set(fw.id, {
        conceptCount: fwConcepts.length,
        conceptTypes,
        connectedFrameworks: connectedFws.size,
        relationshipCount: fwRelationships.length,
      });
    }

    return stats;
  }, [frameworks, allConcepts, relationships, conceptToFramework]);

  return { data, isLoading };
}

/**
 * Lazily fetches proof and verification metadata for a framework.
 * Only fires when frameworkId is non-null (user has opened the proof panel).
 * Uses skipToken (TanStack Query v5) for type-safe conditional fetching.
 * staleTime: Infinity because proof files are static verification artifacts.
 */
export function useFrameworkProof(frameworkId: string | null) {
  return useQuery({
    queryKey: frameworkId ? ontologyKeys.proof(frameworkId) : ["__disabled__"],
    queryFn: frameworkId
      ? async () => {
          const { data } = await api.get<FrameworkProof>(
            `/ontology/frameworks/${frameworkId}/proof`
          );
          return data;
        }
      : skipToken,
    staleTime: Infinity,
  });
}
