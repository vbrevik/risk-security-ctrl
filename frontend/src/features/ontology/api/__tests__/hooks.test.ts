import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, waitFor } from "@testing-library/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import React from "react";
import { useAllConcepts, useFrameworkStats } from "../index";

// Mock the api module
vi.mock("@/lib/api", () => ({
  api: {
    get: vi.fn(),
  },
}));

import { api } from "@/lib/api";

const mockedApi = vi.mocked(api);

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: {
      queries: { retry: false },
    },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return React.createElement(
      QueryClientProvider,
      { client: queryClient },
      children
    );
  };
}

const FW_A = { id: "fw-a", name: "FW A", version: null, description: null, source_url: null, created_at: "", updated_at: "" };
const FW_B = { id: "fw-b", name: "FW B", version: null, description: null, source_url: null, created_at: "", updated_at: "" };

function makeConcept(id: string, frameworkId: string, type = "control") {
  return {
    id,
    framework_id: frameworkId,
    parent_id: null,
    concept_type: type,
    code: null,
    name_en: id,
    name_nb: null,
    definition_en: null,
    definition_nb: null,
    source_reference: null,
    sort_order: null,
    created_at: "",
    updated_at: "",
  };
}

describe("useAllConcepts", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("returns combined concepts from multiple frameworks", async () => {
    mockedApi.get.mockImplementation(async (url: string) => {
      if (url === "/ontology/frameworks") {
        return { data: [FW_A, FW_B] };
      }
      if (url.includes("framework_id=fw-a")) {
        return { data: { items: [makeConcept("c1", "fw-a")], total: 1, page: 1, limit: 500, total_pages: 1 } };
      }
      if (url.includes("framework_id=fw-b")) {
        return { data: { items: [makeConcept("c2", "fw-b")], total: 1, page: 1, limit: 500, total_pages: 1 } };
      }
      return { data: [] };
    });

    const { result } = renderHook(() => useAllConcepts(), { wrapper: createWrapper() });

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.data).toHaveLength(2);
    expect(result.current.data.map((c) => c.id).sort()).toEqual(["c1", "c2"]);
  });

  it("returns errors array when some queries fail", async () => {
    mockedApi.get.mockImplementation(async (url: string) => {
      if (url === "/ontology/frameworks") {
        return { data: [FW_A, FW_B] };
      }
      if (url.includes("framework_id=fw-a")) {
        return { data: { items: [makeConcept("c1", "fw-a")], total: 1, page: 1, limit: 500, total_pages: 1 } };
      }
      if (url.includes("framework_id=fw-b")) {
        throw new Error("Network error");
      }
      return { data: [] };
    });

    const { result } = renderHook(() => useAllConcepts(), { wrapper: createWrapper() });

    await waitFor(() => {
      expect(result.current.errors.length).toBeGreaterThan(0);
    });

    // Successful framework's concepts should still be available
    expect(result.current.data.some((c) => c.id === "c1")).toBe(true);
  });

  it("builds correct concept-to-framework Map", async () => {
    mockedApi.get.mockImplementation(async (url: string) => {
      if (url === "/ontology/frameworks") {
        return { data: [FW_A, FW_B] };
      }
      if (url.includes("framework_id=fw-a")) {
        return { data: { items: [makeConcept("c1", "fw-a")], total: 1, page: 1, limit: 500, total_pages: 1 } };
      }
      if (url.includes("framework_id=fw-b")) {
        return { data: { items: [makeConcept("c2", "fw-b")], total: 1, page: 1, limit: 500, total_pages: 1 } };
      }
      return { data: [] };
    });

    const { result } = renderHook(() => useAllConcepts(), { wrapper: createWrapper() });

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    expect(result.current.conceptToFramework.get("c1")).toBe("fw-a");
    expect(result.current.conceptToFramework.get("c2")).toBe("fw-b");
  });
});

describe("useFrameworkStats", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("returns correct stats per framework", async () => {
    mockedApi.get.mockImplementation(async (url: string) => {
      if (url === "/ontology/frameworks") {
        return { data: [FW_A, FW_B] };
      }
      if (url === "/ontology/relationships") {
        return {
          data: [
            { id: "r1", source_concept_id: "c1", target_concept_id: "c3", relationship_type: "maps_to", description: null, created_at: null },
          ],
        };
      }
      if (url.includes("framework_id=fw-a")) {
        return {
          data: {
            items: [
              makeConcept("c1", "fw-a", "control"),
              makeConcept("c2", "fw-a", "principle"),
            ],
            total: 2, page: 1, limit: 500, total_pages: 1,
          },
        };
      }
      if (url.includes("framework_id=fw-b")) {
        return {
          data: {
            items: [makeConcept("c3", "fw-b", "control")],
            total: 1, page: 1, limit: 500, total_pages: 1,
          },
        };
      }
      return { data: [] };
    });

    const { result } = renderHook(() => useFrameworkStats(), { wrapper: createWrapper() });

    await waitFor(() => {
      expect(result.current.isLoading).toBe(false);
    });

    const statsA = result.current.data.get("fw-a");
    expect(statsA).toBeDefined();
    expect(statsA!.conceptCount).toBe(2);
    expect(statsA!.conceptTypes).toEqual({ control: 1, principle: 1 });
    expect(statsA!.connectedFrameworks).toBe(1); // connected to fw-b via r1
    expect(statsA!.relationshipCount).toBe(1);

    const statsB = result.current.data.get("fw-b");
    expect(statsB).toBeDefined();
    expect(statsB!.conceptCount).toBe(1);
    expect(statsB!.connectedFrameworks).toBe(1); // connected to fw-a via r1
    expect(statsB!.relationshipCount).toBe(1);
  });
});
