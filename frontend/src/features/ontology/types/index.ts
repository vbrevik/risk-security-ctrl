// API Response Types (match backend models)
export interface Framework {
  id: string;
  name: string;
  version: string | null;
  description: string | null;
  source_url: string | null;
  created_at: string;
  updated_at: string;
}

export interface Concept {
  id: string;
  framework_id: string;
  parent_id: string | null;
  concept_type: string;
  code: string | null;
  name_en: string;
  name_nb: string | null;
  definition_en: string | null;
  definition_nb: string | null;
  source_reference: string | null;
  sort_order: number | null;
  created_at: string;
  updated_at: string;
}

export interface Relationship {
  id: string;
  source_concept_id: string;
  target_concept_id: string;
  relationship_type: string;
  description: string | null;
  created_at: string | null;
}

export interface RelatedConcept {
  relationship_id: string;
  relationship_type: string;
  concept_id: string;
  concept_name_en: string;
  concept_name_nb: string | null;
  direction: "incoming" | "outgoing";
}

export interface ConceptWithRelationships extends Concept {
  related_concepts: RelatedConcept[];
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}

// Tree Types
export interface TreeNode {
  id: string;
  name: string;
  type: string;
  code: string | null;
  children: TreeNode[];
  concept: Concept;
  isExpanded: boolean;
}

export interface FrameworkTreeData {
  framework: Framework;
  tree: TreeNode[];
  isExpanded: boolean;
  isLoading: boolean;
}

// Graph Types
export interface GraphNode {
  id: string;
  name: string;
  type: string;
  frameworkId: string;
  code: string | null;
  x?: number;
  y?: number;
  fx?: number | null;
  fy?: number | null;
}

export interface GraphEdge {
  id: string;
  source: string | GraphNode;
  target: string | GraphNode;
  type: string;
  label: string;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

// UI State Types
export type ViewMode = "graph" | "tree" | "detail" | "compare";

export interface ExplorerState {
  selectedConceptId: string | null;
  selectedConcepts: string[];
  viewMode: ViewMode;
  sidebarCollapsed: boolean;
  compareFrameworks: [string | null, string | null];
  activeFrameworks: string[];
  activeConceptType: string | null;
  searchHighlightIds: string[];
}

// Layout Persistence Types
export interface NodePosition {
  x: number;
  y: number;
}

export interface SavedLayout {
  positions: Record<string, NodePosition>;
  zoom: number;
  panX: number;
  panY: number;
  expandedNodes: string[];
  timestamp: number;
}
