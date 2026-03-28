# CER Directive (EU) 2022/2557 — Verification Proof

**Source:** https://eur-lex.europa.eu/eli/dir/2022/2557/oj
**Reference:** Directive (EU) 2022/2557 of 14 December 2022, Official Journal L 333, 27.12.2022
**Verified:** 2026-03-28
**Status:** corrected

## How to Verify

1. **EUR-Lex Full Text (EN):** https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022L2557
2. **EUR-Lex PDF:** https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:32022L2557
3. **Table of Contents:** Open the EUR-Lex HTML version, expand "Document information" to see the article-by-article structure.
4. **Annex (11 sectors):** Scroll to the bottom of the full text or search for "ANNEX" in the PDF.

Each article can be verified by searching for "Article X" in the full text. The sector list is in the Annex at the end of the directive.

## Official Structure

7 Chapters, 27 Articles, 1 Annex (11 sectors)

### Chapters
- Chapter I: General provisions (Articles 1-3)
- Chapter II: National frameworks (Articles 4-8)
- Chapter III: Competent authorities and cooperation (Articles 9-11)
- Chapter IV: Resilience of critical entities (Articles 12-16)
- Chapter V: Critical entities of particular European significance (Articles 17-18)
- Chapter VI: Supervision and enforcement (Articles 19-22)
- Chapter VII: Final provisions (Articles 23-27)

### Annex — 11 Sectors
1. Energy (electricity, district heating/cooling, oil, gas, hydrogen)
2. Transport (air, rail, water, road)
3. Banking
4. Financial market infrastructures
5. Health
6. Drinking water
7. Waste water
8. Digital infrastructure
9. Public administration
10. Space
11. Production, processing and distribution of food

## Verification Results

### Confirmed Correct
- All 11 sectors present and accurately described
- All article source_reference fields accurate
- Category groupings match directive chapters
- Article 13(1)(a)-(e) resilience measures all correctly identified
- 24-hour notification deadline (Art 15) correct
- 4-year cycle for risk assessments (Art 5) and strategy (Art 4) correct
- 9-month entity risk assessment deadline (Art 12) correct
- Hydrogen inclusion in energy sector correct

### Issues Found — All Resolved (2026-03-28)

1. **Wrong codes on 3 concepts** — Three concepts under Governance used sequential ART-X codes that did not match their actual article references. Fixed:
   - `cer-background-checks`: ART-8 → ART-14 ("Background Checks", Chapter IV). Also moved to Resilience Obligations parent.
   - `cer-supervisory-powers`: ART-10 → ART-21, name corrected to "Supervision and Enforcement"
   - `cer-sector-equivalence`: ART-14 → ART-8, name corrected to "Critical Entities in Banking, Financial Market Infrastructure and Digital Infrastructure Sectors"

2. **Missing Art 2 (Definitions)** — Added `cer-definitions` under Governance and Oversight. Key terms defined: critical entity, resilience, incident, critical infrastructure, essential service, risk, risk assessment, public administration entity.

3. **Missing Art 7 (Significant disruptive effect criteria)** — Added `cer-significant-disruptive-effect` under Risk Assessment. Documents the 6 criteria (a–f) used by Member States to determine whether a disruption qualifies as "significant" for the purpose of identifying critical entities.

4. **Duplicate Art 12 coverage** — Both `cer-entity-risk-assessment` (RA2, Risk Assessment category) and `cer-entity-risk-assessment-obligation` (RO1, Resilience Obligations) covered Article 12. Removed RA2; RO1 provides the canonical coverage.

### Concept Count
43 concepts — all legitimate, zero fabricated. (Was 42; removed 1 duplicate, added 2 new.)

### Intentionally Excluded
- Arts 1, 3: General scope and relationship with other Union acts
- Arts 4, 5, 6: National strategy, risk assessment, and identification process (covered by GO1, RA1, RA3 rollup concepts)
- Arts 9-11: Competent authority designation details (covered by GO2-GO3 rollup)
- Arts 15-16: Incident notification details (covered by IN1-IN3)
- Arts 17-18: Advisory missions and enhanced cooperation mechanics (covered by ES2-ES3)
- Arts 19-22: Enforcement details beyond supervisory powers (covered by ART-21 rollup)
- Arts 23-27: Final provisions, transposition, entry into force
