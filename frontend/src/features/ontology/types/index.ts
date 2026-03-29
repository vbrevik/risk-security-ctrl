// API Response Types (match backend models)
export interface Framework {
  id: string;
  name: string;
  version: string | null;
  description: string | null;
  source_url: string | null;
  created_at: string;
  updated_at: string;
  // Verification provenance (returned by the API, added in split 07)
  verification_status: string | null;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
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
  concept_framework_id: string;
  concept_name_en: string;
  concept_name_nb: string | null;
  direction: "incoming" | "outgoing";
}

export interface ActionResponse {
  sort_order: number;
  text_en: string;
  text_nb: string | null;
}

export interface QuestionResponse {
  sort_order: number;
  text_en: string;
  text_nb: string | null;
}

export interface ReferenceResponse {
  type: string;
  title: string;
  authors: string | null;
  year: number | null;
  venue: string | null;
  url: string | null;
}

export interface ConceptGuidanceResponse {
  source_pdf: string;
  source_page: number;
  about_en: string | null;
  about_nb: string | null;
  suggested_actions: ActionResponse[];
  transparency_questions: QuestionResponse[];
  references: ReferenceResponse[];
}

export interface ConceptWithRelationships extends Concept {
  related_concepts: RelatedConcept[];
  guidance?: ConceptGuidanceResponse;
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

// Topic tag for cross-cutting theme filtering
export interface Topic {
  id: string;
  name_en: string;
  name_nb: string;
  description_en: string;
  description_nb: string;
  concept_ids: string[];
}

// UI State Types
export type ViewMode = "graph" | "tree" | "compare";

export interface ExplorerState {
  selectedConceptId: string | null;
  selectedConcepts: string[];
  viewMode: ViewMode;
  sidebarCollapsed: boolean;
  compareFrameworks: [string | null, string | null];
  activeFrameworks: string[];
  activeTopics: string[];
  activeConceptType: string | null;
  searchHighlightIds: string[];
  navigationHistory: string[];
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

// Framework Explorer Page Types
export interface FrameworkStats {
  conceptCount: number;
  conceptTypes: Record<string, number>;
  connectedFrameworks: number;
  relationshipCount: number;
}

export interface CrosswalkCell {
  sourceFrameworkId: string;
  targetFrameworkId: string;
  count: number;
  relationships: Relationship[];
}

export interface LandscapeProfile {
  sector: string;
  activities: string[];
  applicableFrameworks: string[];
}

// Verification Provenance Types (split 07)
export type VerificationStatus =
  | "verified"
  | "partially-verified"
  | "structure-verified"
  | "corrected"
  | "unverified"
  | "needs-correction";

const KNOWN_STATUSES: ReadonlySet<string> = new Set<VerificationStatus>([
  "verified",
  "partially-verified",
  "structure-verified",
  "corrected",
  "unverified",
  "needs-correction",
]);

/**
 * Normalizes a raw backend verification_status string to a typed
 * VerificationStatus or "unknown" for null/unrecognized values.
 * Used by VerificationBadge and ProofPanel for safe style mapping.
 */
export function toVerificationStatus(
  value: string | null,
): VerificationStatus | "unknown" {
  if (value !== null && KNOWN_STATUSES.has(value)) {
    return value as VerificationStatus;
  }
  return "unknown";
}

/**
 * Response shape of GET /api/ontology/frameworks/{id}/proof
 * Used by useFrameworkProof hook and ProofPanel component.
 */
export interface FrameworkProof {
  framework_id: string;
  verification_status: string | null;
  verification_date: string | null;
  verification_source: string | null;
  verification_notes: string | null;
  source_trust_tier: number | null; // 1 = primary official, 2 = secondary, 3 = unofficial
  proof_content: string | null; // raw markdown; null if no proof file exists
}
