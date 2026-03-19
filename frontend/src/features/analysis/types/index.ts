// Re-export PaginatedResponse from ontology types
export type { PaginatedResponse } from "@/features/ontology/types";

// Union types
export type AnalysisStatus = "pending" | "processing" | "completed" | "failed" | "deleted";
export type InputType = "text" | "pdf" | "docx";
export type FindingType = "addressed" | "partially_addressed" | "gap" | "not_applicable";

// Full analysis entity from GET /api/analyses/{id}
export interface Analysis {
  id: string;
  name: string;
  description: string | null;
  input_type: InputType;
  input_text: string | null;
  original_filename: string | null;
  file_path: string | null;
  extracted_text: string | null;
  status: AnalysisStatus;
  error_message: string | null;
  prompt_template: string | null;
  matched_framework_ids: string[];
  processing_time_ms: number | null;
  token_count: number | null;
  created_by: string | null;
  created_at: string;
  updated_at: string;
}

// Subset returned by list endpoint GET /api/analyses
export interface AnalysisListItem {
  id: string;
  name: string;
  description: string | null;
  input_type: InputType;
  status: AnalysisStatus;
  error_message: string | null;
  processing_time_ms: number | null;
  created_at: string;
  updated_at: string;
}

// Individual finding with concept metadata
export interface AnalysisFinding {
  id: string;
  concept_id: string;
  framework_id: string;
  finding_type: FindingType;
  confidence_score: number;
  evidence_text: string | null;
  recommendation: string | null;
  priority: number;
  sort_order: number;
  concept_code: string | null;
  concept_name: string | null;
  concept_definition: string | null;
}

// Matcher configuration for prompt template
export interface MatcherConfig {
  version: number;
  thresholds: {
    min_confidence: number;
    addressed: number;
    partial: number;
  };
  max_findings_per_framework: number;
  include_addressed_findings: boolean;
  boost_terms: Record<string, number>;
}

// Request types
export interface CreateAnalysisRequest {
  name: string;
  description?: string;
  input_text: string;
}

export interface UploadAnalysisInput {
  file: File;
  name: string;
}

export interface AnalysisListParams {
  page?: number;
  limit?: number;
  status?: AnalysisStatus;
}

export interface FindingsListParams {
  page?: number;
  limit?: number;
  framework_id?: string;
  finding_type?: FindingType;
  priority?: number;
  sort_by?: string;
}
