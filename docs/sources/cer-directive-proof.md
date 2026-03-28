# CER Directive (EU) 2022/2557 — Verification Proof

**Source:** https://eur-lex.europa.eu/eli/dir/2022/2557/oj
**Reference:** Directive (EU) 2022/2557 of 14 December 2022, Official Journal L 333, 27.12.2022
**Verified:** 2026-03-28
**Status:** partially-verified
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

### Issues Found
1. **Misleading codes:** `cer-background-checks` has code ART-8 but references Art 14; `cer-supervisory-powers` has code ART-10 but references Art 21; `cer-sector-equivalence` has code ART-14 but references Art 8. Codes are sequential within category, not article references — ART- prefix is misleading.
2. **Missing Art 2** (Definitions) — substantive article with key terms
3. **Missing Art 7** (Significant disruptive effect criteria)
4. **Duplicate coverage:** Both `cer-entity-risk-assessment` and `cer-entity-risk-assessment-obligation` cover Art 12

### Concept Count
42 concepts — all legitimate, zero fabricated
