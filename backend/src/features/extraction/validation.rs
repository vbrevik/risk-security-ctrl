use serde::{Deserialize, Serialize};

/// Report from validating extracted data against the ontology.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub total_expected: usize,
    pub total_extracted: usize,
    pub missing_concepts: Vec<String>,
    pub unmatched_sections: Vec<String>,
    pub warnings: Vec<String>,
}
