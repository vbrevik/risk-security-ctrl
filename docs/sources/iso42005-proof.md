# ISO/IEC 42005:2025 — Verification Proof

**Source:** https://www.iso.org/standard/80623.html
**Reference:** ISO/IEC 42005:2025 Information technology — Artificial intelligence — AI system impact assessment
**Verified:** 2026-03-28
**Status:** corrected

## How to Verify

1. **ISO Catalogue Page:** https://www.iso.org/standard/80623.html
2. **iTeh Standards Preview PDF:** https://standards.iteh.ai/catalog/standards/iso/80623
3. **IEC Webstore:** https://webstore.iec.ch/en/publication/80623

The full standard requires purchase. The complete table of contents was extracted from the iTeh Standards preview PDF using pdfminer.

## Official Structure

ISO/IEC 42005:2025 is a **guidance document** (not a management system standard with HLS/Annex SL structure). It provides guidance on conducting AI system impact assessments (AIIA).

| Clause | Title |
|--------|-------|
| 1 | Scope |
| 2 | Normative references |
| 3 | Terms and definitions |
| 4 | Abbreviated terms |
| **5** | **Developing and implementing an AI system impact assessment process** |
| **6** | **Documenting the AI system impact assessment** |
| Annex A | Relationship with other standards (informative) |
| Annex B | Considerations for determining AI system impact assessment threshold (informative) |
| **Annex C** | **Harms and benefits taxonomy (informative)** |
| Annex D | Relationship with other impact assessments (informative) |
| Annex E | Template for AI system impact assessment (informative) |

**Clause 5 sub-clauses (AI system impact assessment process):**
- 5.1 General
- 5.2 Understanding the context of the AI system
- 5.3 Identifying the responsibilities for the AIIA
- 5.4 Determining the AIIA threshold
- 5.5 Understanding the AI system
- 5.6 Identifying potential harms and benefits
- 5.7 Analysing identified harms and benefits
- 5.8 Evaluating identified harms and benefits
- 5.9 Treating identified harms and benefits
- 5.10 Monitoring and review
- 5.11 Communication and consultation
- 5.12 Documentation

**Clause 6 sub-clauses (AIIA documentation):**
- 6.1 General
- 6.2 Basic information
- 6.3 AI system information
- 6.4 Responsible parties
- 6.5 Context of the AI system
- 6.6 AIIA process documentation
- 6.7 Harms and benefits analysis results
- 6.8 AIIA outcomes
- 6.9 Review and update information

**Annex C — Harms and benefits taxonomy:**
- C.1 Human rights and civil liberties
- C.2 Health and safety
- C.3 Societal wellbeing
- C.4 Economic
- C.5 Environmental
- C.6 Governance

Each dimension contains 3–6 specific impact areas (sub-sub-clauses C.x.1, C.x.2, etc.).

## Verification Results

### Issues Found — All Resolved (2026-03-28)

The original ontology had completely fabricated internal codes and structure that did not correspond to the actual standard.

1. **Fabricated IAP codes (IAP.1–IAP.7)** — Seven "Impact Assessment Process" codes mapping concepts to non-existent clause numbers. E.g. "Establishing Context" mapped to "Clause 5.2", but real Clause 5.2 is "Understanding the context of the AI system". Real Clause 5 has 12 subclauses (5.1–5.12).

2. **Fabricated ID codes (ID.1–ID.6)** — Six "Impact Dimensions" codes not derived from standard clause structure. Themes correspond to Annex C dimensions (C.1–C.6) but the codes were invented and source_reference fields were empty.

3. **Fabricated SE codes (SE.1–SE.3)** — Three "Stakeholder Engagement" concepts with no clause basis. Stakeholder engagement is covered within Clause 5.11 Communication and consultation, not a separate concept group.

4. **Fabricated LC codes (LC.1–LC.3)** — Three "Lifecycle" concepts with no clause basis.

5. **Wrong source_url** — Was `https://www.iso.org/standard/44545.html` (incorrect catalogue ID). Corrected to `https://www.iso.org/standard/80623.html`.

### Rebuild Approach

Complete rebuild from scratch using actual clause references extracted from the iTeh Standards PDF preview:
- Clause 5 process requirements (11 sub-clauses: 5.2–5.12) → type `requirement`
- Clause 6 documentation requirements (7 sub-clauses: 6.3–6.9) → type `requirement`
- Annex C dimensions (6 top-level: C.1–C.6) → type `dimension`
- Annex C impact areas (26 sub-items) → type `impact-area`
- 3 category nodes as parents: iso42005-process (Clause 5), iso42005-documentation (Clause 6), iso42005-impact-taxonomy (Annex C)

Codes now use actual clause references (5.2, 5.3, C.1, C.1.1, etc.) instead of fabricated identifiers.

### Concept Count
53 concepts — 3 categories + 11 process requirements + 7 documentation requirements + 6 dimensions + 26 impact areas. All mapped to real ISO/IEC 42005:2025 clause numbers and titles, zero fabricated.
(Was 49; rebuild replaced 49 concepts with 53 correctly-structured ones.)

### Intentionally Excluded
- Clauses 1–4: Scope, normative references, terms, abbreviated terms
- 5.1 General: Introductory sub-clause with no independent substantive content
- 6.1–6.2: General and basic information introductory sub-clauses
- Annexes A, B, D, E: Relationships with other standards, threshold considerations, cross-assessment comparisons, template — editorial/reference material not modelled as concepts
