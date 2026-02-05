import type { Concept, Relationship, GraphNode, GraphEdge, GraphData } from "../types";

export function conceptsToGraphNodes(concepts: Concept[], language: "en" | "nb" = "en"): GraphNode[] {
  return concepts.map((concept) => ({
    id: concept.id,
    name: language === "nb" && concept.name_nb ? concept.name_nb : concept.name_en,
    type: concept.concept_type,
    frameworkId: concept.framework_id,
    code: concept.code,
  }));
}

export function relationshipsToGraphEdges(
  relationships: Relationship[],
  nodeIds: Set<string>
): GraphEdge[] {
  return relationships
    .filter(
      (rel) => nodeIds.has(rel.source_concept_id) && nodeIds.has(rel.target_concept_id)
    )
    .map((rel) => ({
      id: rel.id,
      source: rel.source_concept_id,
      target: rel.target_concept_id,
      type: rel.relationship_type,
      label: formatRelationshipLabel(rel.relationship_type),
    }));
}

export function formatRelationshipLabel(type: string): string {
  return type
    .replace(/_/g, " ")
    .replace(/([A-Z])/g, " $1")
    .trim()
    .toLowerCase();
}

export function buildGraphData(
  concepts: Concept[],
  relationships: Relationship[],
  language: "en" | "nb" = "en"
): GraphData {
  const nodes = conceptsToGraphNodes(concepts, language);
  const nodeIds = new Set(nodes.map((n) => n.id));
  const edges = relationshipsToGraphEdges(relationships, nodeIds);
  return { nodes, edges };
}

// Framework colors for consistent theming
export const frameworkColors: Record<string, string> = {
  iso31000: "#3b82f6", // blue
  iso31010: "#22c55e", // green
  "nist-csf": "#f97316", // orange
  iso27000: "#8b5cf6", // purple
  iso9000: "#ec4899", // pink
  "zero-trust": "#14b8a6", // teal
  "data-centric": "#eab308", // yellow
  fmn: "#6366f1", // indigo
};

export function getFrameworkColor(frameworkId: string): string {
  return frameworkColors[frameworkId] ?? "#64748b"; // slate fallback
}
