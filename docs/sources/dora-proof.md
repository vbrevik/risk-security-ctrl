# DORA (EU) 2022/2554 — Verification Proof

**Source:** https://eur-lex.europa.eu/eli/reg/2022/2554/oj
**Reference:** Regulation (EU) 2022/2554 of 14 December 2022
**Verified:** 2026-03-28
**Status:** partially-verified
## How to Verify

1. **EUR-Lex Full Text (EN):** https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022R2554
2. **EUR-Lex PDF:** https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:32022R2554
3. **Structured Mirror:** https://www.digital-operational-resilience-act.com/DORA_Articles.html

Search for "Article X" in the EUR-Lex text to verify each concept. The chapter structure is visible in the table of contents at the top of the HTML version.

## Official Structure

9 Chapters, 64 Articles

### Substantive Chapters (modeled in ontology)
- Chapter II: ICT risk management (Articles 5-16)
- Chapter III: ICT-related incident management, classification and reporting (Articles 17-23)
- Chapter IV: Digital operational resilience testing (Articles 24-27)
- Chapter V: Managing of ICT third-party risk (Articles 28-44)
- Chapter VI: Information-sharing arrangements (Article 45)

### Excluded Chapters (administrative/procedural)
- Chapter I: General provisions (Articles 1-4)
- Chapter VII: Competent authorities (Articles 46-56)
- Chapter VIII: Delegated acts (Article 57)
- Chapter IX: Transitional and final provisions (Articles 58-64)

## Verification Results

### Confirmed Correct
- All 5 substantive chapter categories correctly mapped
- Chapter II: 10 of 12 articles present (Art 5-14), all titles accurate
- Chapter III: 4 of 7 articles present, all correct
- Chapter IV: All 4 articles present and correct (Art 24-27)
- Chapter V: Key articles (28, 30) plus oversight rollup (31-44) present
- Chapter VI: All 3 sub-paragraphs of Art 45 present

### Issues Found
1. **Wrong code:** `dora-third-party-assessment` uses code ART-28-4 but maps to Art 29 content (standalone article, not paragraph of Art 28)
2. **Missing Art 15:** Further harmonisation of ICT risk management tools/methods/processes
3. **Missing Art 16:** Simplified ICT risk management framework (proportionate regime for smaller entities)
4. **Missing Art 20:** Harmonisation of reporting content and templates
5. **Missing Art 23:** Operational/security payment-related incidents for credit/payment institutions
6. **Minor title abbreviations:** Ch III name omits "classification and reporting"; Art 12 title shortened

### Concept Count
32 concepts — all legitimate, zero fabricated
