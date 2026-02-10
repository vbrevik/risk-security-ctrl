import type { PaginatedResponse } from "@/features/ontology/types";

export type { PaginatedResponse };

// Assessment status enum matching backend
export type AssessmentStatus =
  | "draft"
  | "in_progress"
  | "under_review"
  | "completed"
  | "archived";

export interface Assessment {
  id: string;
  framework_id: string;
  name: string;
  description: string | null;
  status: AssessmentStatus;
  owner_id: string | null;
  due_date: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateAssessmentRequest {
  framework_id: string;
  name: string;
  description?: string;
  owner_id?: string;
  due_date?: string;
}

export interface UpdateAssessmentRequest {
  name?: string;
  description?: string;
  status?: AssessmentStatus;
  owner_id?: string;
  due_date?: string;
}

// Compliance item status
export type ComplianceStatus =
  | "not_assessed"
  | "compliant"
  | "partially_compliant"
  | "non_compliant"
  | "not_applicable";

export interface ComplianceItemWithConcept {
  id: string;
  assessment_id: string;
  concept_id: string;
  status: ComplianceStatus;
  notes: string | null;
  assessed_by: string | null;
  assessed_at: string | null;
  created_at: string;
  updated_at: string;
  concept_code: string | null;
  concept_name_en: string;
  concept_name_nb: string | null;
  concept_type: string;
  concept_definition_en: string | null;
}

// Evidence
export type EvidenceType =
  | "document"
  | "link"
  | "screenshot"
  | "note"
  | "other";

export interface Evidence {
  id: string;
  compliance_item_id: string;
  evidence_type: EvidenceType;
  title: string;
  description: string | null;
  file_path: string | null;
  url: string | null;
  uploaded_by: string | null;
  created_at: string;
  updated_at: string;
}

// Compliance score
export interface SectionScore {
  section_id: string;
  section_name: string;
  total_items: number;
  compliant: number;
  partially_compliant: number;
  non_compliant: number;
  not_assessed: number;
  not_applicable: number;
  compliance_percentage: number;
}

export interface ComplianceScore {
  assessment_id: string;
  total_items: number;
  compliant: number;
  partially_compliant: number;
  non_compliant: number;
  not_assessed: number;
  not_applicable: number;
  overall_compliance_percentage: number;
  sections: SectionScore[];
}

// Filter params for assessment list
export interface AssessmentFilters {
  page?: number;
  limit?: number;
  framework_id?: string;
  status?: AssessmentStatus;
}
