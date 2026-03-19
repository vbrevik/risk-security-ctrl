# Section 02 Code Review Interview

## Auto-fixes Applied

### 1. Framework ID prefix delimiter guard [AUTO-FIX]
**Change:** `starts_with(fw_id)` → `starts_with(&format!("{}-", fw_id)) || cid == fw_id`
**Rationale:** Prevents "nist" from matching "nist-csf-*" concept IDs.

### 2. Deduplicate matched concept_ids [AUTO-FIX]
**Change:** `Vec<&str>` → `HashSet<&str>` for matched_concept_ids
**Rationale:** Prevents score inflation from overlapping topics.

### 3. Fix flaky ordering test [AUTO-FIX]
**Change:** Added more concept_ids (3 for iso31000) and added "management" to doc_keywords so iso31000 scores 5.0 vs nist-csf's 2.0.
**Rationale:** Original test had both frameworks scoring 2.0, making order non-deterministic.

### 4. Borrow frameworks param [AUTO-FIX]
**Change:** `frameworks: Vec<(String, String)>` → `frameworks: &[(String, String)]`
**Rationale:** Function only borrows, no need to take ownership.

## Let Go

- Abbreviation extraction (works via tokenizer already, 3-char minimum is fine)
- Flat direct name bonus (plan specifies fixed bonus)
- Topic overlap weighting (binary match sufficient for MVP)
- Debug logging in detect_frameworks
