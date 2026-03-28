# DORA (EU) 2022/2554 — Verification Proof

**Source:** https://eur-lex.europa.eu/eli/reg/2022/2554/oj
**Reference:** Regulation (EU) 2022/2554 of 14 December 2022
**Verified:** 2026-03-28
**Status:** corrected

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
- Chapter II: All 12 articles present (Art 5-16) with correct titles and definitions
- Chapter III: All 7 articles present (Art 17-23), correct titles and definitions
- Chapter IV: All 4 articles present and correct (Art 24-27)
- Chapter V: Key articles (28-31) present, Art 28-3 models the register obligation; Arts 32-44 folded into the oversight rollup (Art 31)
- Chapter VI: All 3 sub-paragraphs of Art 45 present

### Issues Found — All Resolved (2026-03-28)

1. **Wrong code on Art 29 concept** — `dora-third-party-assessment` used code `ART-28-4` but mapped to Art 29 content. **Fixed:** code changed to `ART-29`.

2. **Missing Art 15** — "Further Harmonisation of ICT Risk Management Tools, Methods, Processes and Policies". **Fixed:** added as `dora-ict-harmonisation` under ICT Risk Management.

3. **Missing Art 16** — "Simplified ICT Risk Management Framework" (proportionate regime for microenterprises, exempted payment/e-money institutions, small IORPs). **Fixed:** added as `dora-ict-simplified-framework`.

4. **Missing Art 20** — "Harmonisation of Reporting Content, Formats and Templates" (ESA mandate for standard incident reporting templates). **Fixed:** added as `dora-incident-reporting-templates`.

5. **Missing Art 23** — "Operational or Security Payment-Related Incidents" (extends Chapter III to payment-related incidents for credit institutions, payment institutions, AISPs, and e-money institutions). **Fixed:** added as `dora-payment-incident-reporting`.

6. **Chapter III title truncated** — Was "ICT-Related Incident Management"; official title is "ICT-related incident management, classification and reporting". **Fixed.**

7. **Art 12 title truncated** — Was "Backup Policies and Recovery Methods"; official title is "Backup policies and procedures, restoration and recovery procedures and methods". **Fixed.**

### Concept Count
36 concepts — all legitimate, zero fabricated. (Was 32 before corrections; 4 concepts added.)

### Intentionally Excluded (scope/editorial)
- Arts 1-4: General provisions (definitions, scope, proportionality)
- Arts 32-44: Detailed oversight framework mechanics (designation, cooperation, fees, penalties for ICT TPPs) — covered by the Art 31 rollup concept
- Arts 46-56: Competent authority powers and cooperation
- Arts 58-64: Transitional and final provisions
