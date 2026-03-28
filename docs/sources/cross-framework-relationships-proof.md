# Cross-Framework Relationship Validation Report

**Verified:** 2026-03-28
**Status:** verified (all references valid)

## Scope

Validated all 30 relationship JSON files (`ontology-data/relationships-*.json`) against the full concept pool from all framework JSON files.

## Results

- **585 relationships** checked across **30 files**
- **0 broken references** — all `source_concept_id` and `target_concept_id` values resolve to valid concepts
- **1805 unique concept IDs** in the concept pool

### Relationship Type Distribution

| Type | Count | Notes |
|------|-------|-------|
| maps_to | 202 | Cross-framework equivalence |
| related_to | 169 | Loose association |
| implements | 143 | One concept implements another |
| supports | 39 | Supporting relationship |
| mitigated_by | 12 | Threat/vulnerability mitigation |
| exploited_by | 8 | Attack vector relationships |
| informs | 4 | Advisory/informational |
| extends | 4 | Extension relationship |
| exploits | 2 | Attack exploits weakness |
| threatens | 1 | Threat relationship |
| detected_by | 1 | Detection relationship |

### Fixes Applied in Prior Splits

During verification of individual frameworks (Splits 01-05), the following relationship fixes were applied:

1. **relationships-mitre-atlas.json** — 9 ID mappings updated after ATLAS rebuild (old → new concept IDs)
2. **relationships-iso10015.json** — 3 broken refs removed (concepts deleted in HLS rewrite)
3. **relationships-nist-ai-genai.json** — 2 broken refs removed (non-existent concept IDs)
4. **relationships-cisa-ztmm.json** — 1 broken ref removed (zt-p1-verify-explicitly deleted)
5. **relationships-mitre-attack.json** — 2 broken refs removed (zt-p3-assume-breach, zt-p1-verify-explicitly)
6. **relationships-nist-sp-800-53.json** — 2 broken refs removed (zt-p2-least-privilege, zt-p1-verify-explicitly)
7. **relationships-zero-trust.json** — 3 broken refs removed (3 Microsoft ZT concepts removed from zero-trust.json)

### Semantic Review

All relationship types are semantically appropriate:
- `maps_to` / `implements` used for cross-standard equivalences (e.g., ISO 27001 control → NIST 800-53 control)
- `related_to` used for loose thematic connections
- `mitigated_by` / `exploited_by` / `exploits` / `threatens` / `detected_by` used for security-specific threat-control relationships (CWE, MITRE ATT&CK)
- `supports` / `informs` / `extends` used for hierarchical/advisory relationships

No relationship type constraint exists in the database schema (`TEXT NOT NULL`, no CHECK constraint), so all types are accepted.

## Validation Method

Python script loaded all concept IDs from all framework JSON files (excluding relationship files and topic-tags.json), then checked every `source_concept_id` and `target_concept_id` in every relationship file. Script at `/tmp/validate_relationships.py`.
