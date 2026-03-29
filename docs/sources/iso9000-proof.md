# ISO 9000/9001:2015 — Verification Proof

**Source:** https://www.iso.org/standard/45481.html (ISO 9000:2015), https://www.iso.org/standard/62085.html (ISO 9001:2015)
**Reference:** ISO 9000:2015 Quality management systems — Fundamentals and vocabulary; ISO 9001:2015 Quality management systems — Requirements
**Verified:** 2026-03-28
**Status:** corrected

## How to Verify

1. **ISO 9000:2015 Catalogue:** https://www.iso.org/standard/45481.html
2. **ISO 9001:2015 Catalogue:** https://www.iso.org/standard/62085.html
3. **ISO Quality Management Principles (free PDF):** https://www.iso.org/publication/PUB100080.html — all 7 QMPs with official descriptions. Direct download: https://www.iso.org/files/live/sites/isoorg/files/store/en/PUB100080.pdf
4. **ISO 9001:2015 table of contents:** Visible via ISO Online Browsing Platform preview at https://www.iso.org/obp/ui/#iso:std:iso:9001:ed-5:v1:en

## Standard Coverage

The ontology file models both standards together under the `iso9000` framework ID:

| Standard | What it contains | Ontology coverage |
|----------|-----------------|-------------------|
| ISO 9000:2015 | 7 Quality Management Principles (Clause 2.3.1–2.3.7), vocabulary (Clause 3) | 7 QMP concepts + iso9000-principles category |
| ISO 9001:2015 | QMS requirements (Clauses 4–10) | 7 clause-level requirement concepts + iso9000-qms category |

This bundling is intentional: the two standards are designed to be used together, ISO 9000 providing the principles and vocabulary while ISO 9001 provides the requirements. The `source_reference` fields on each concept correctly cite the respective standard.

## Official Structure

**ISO 9000:2015 — Quality Management Principles (Clause 2.3):**

| Code | Clause | Title |
|------|--------|-------|
| QMP1 | 2.3.1 | Customer focus |
| QMP2 | 2.3.2 | Leadership |
| QMP3 | 2.3.3 | Engagement of people |
| QMP4 | 2.3.4 | Process approach |
| QMP5 | 2.3.5 | Improvement |
| QMP6 | 2.3.6 | Evidence-based decision making |
| QMP7 | 2.3.7 | Relationship management |

**ISO 9001:2015 — Requirements (HLS structure Clauses 4–10):**

| Code | Clause | Title |
|------|--------|-------|
| 4 | Clause 4 | Context of the organization |
| 5 | Clause 5 | Leadership |
| 6 | Clause 6 | Planning |
| 7 | Clause 7 | Support |
| 8 | Clause 8 | Operation |
| 9 | Clause 9 | Performance evaluation |
| 10 | Clause 10 | Improvement |

## Verification Results

### Confirmed Correct
- All 7 QMPs present with correct clause references (2.3.1–2.3.7)
- **QMP definitions verified against PUB100080.pdf (fetched 2026-03-29):** QMP1–QMP6 are exact matches to the official Statement text in the free ISO QMP publication. QMP7 uses "providers" where the pamphlet says "suppliers" — see note below.
- All 7 ISO 9001:2015 top-level clauses present
- source_reference fields correctly distinguish the two standards

### Issues Found — All Resolved (2026-03-28)

1. **Clause 5 name incorrect** — "Leadership and commitment" is the title of sub-clause 5.1, not Clause 5. Official Clause 5 title is "Leadership". **Fixed.**

2. **Clause 6 name incorrect** — "Planning for the QMS" is not the official title. Official Clause 6 title is "Planning". **Fixed.**

3. **Clause 10 name incorrect** — "Continual improvement" is the title of sub-clause 10.3, not Clause 10. Official Clause 10 title is "Improvement". **Fixed.**

4. **Framework name updated** — Changed from "ISO 9000:2015" to "ISO 9000/9001:2015" to accurately reflect that the file covers both standards.

### Known Discrepancy — QMP7 "providers" vs "suppliers"

The free ISO QMP pamphlet (PUB100080.pdf) states QMP7 as: *"...such as suppliers."*

The ontology uses *"...such as providers."*

**Why "providers" is correct for ISO 9000:2015:** The 2015 revision of ISO 9000 replaced the term "supplier" with "provider" throughout the standard (defined as term 3.2.5: "organization that provides a product or service"). The free QMP pamphlet is an older publication that has not been updated to reflect this vocabulary change. The standard text itself (Clause 2.3.7 in ISO 9000:2015) uses "provider". The ontology is consistent with the 2015 standard vocabulary.

### Previously Fixed (prior session)
- Norwegian typo in performance evaluation: "Ytelsesevvaluering" → "Ytelsesevaluering"

### Concept Count
16 concepts — 7 QMP principles + 7 ISO 9001 requirement clauses + 2 category nodes. All legitimate, zero fabricated.

### Intentionally Excluded
- Clause 3 (vocabulary/terms and definitions): ~200+ defined terms not modelled individually
- Sub-clauses of Clauses 4–10: individual sub-clause requirements (e.g., 4.1, 4.2, 5.1.1) not modelled
- Clauses 1–2: Scope and normative references
