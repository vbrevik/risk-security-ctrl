# NIS2 Directive (EU) 2022/2555 — Verification Proof

**Source:** https://eur-lex.europa.eu/eli/dir/2022/2555/oj
**Reference:** Directive (EU) 2022/2555 of 14 December 2022
**Verified:** 2026-03-28
**Status:** verified
## How to Verify

1. **EUR-Lex Full Text (EN):** https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022L2555
2. **EUR-Lex PDF:** https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX:32022L2555
3. **Annex I (11 high-criticality sectors):** Search "ANNEX I" in the text
4. **Annex II (7 other critical sectors):** Search "ANNEX II"

### Key Verification Points
- **Article 21(2)(a)-(j):** The 10 risk management measures — search "Article 21" and verify all 10 sub-paragraphs
- **Article 23:** Multi-stage incident reporting (24h early warning, 72h notification, 1 month final report)
- **Article 34(4)-(5):** Administrative fines — EUR 10M/2% for essential, EUR 7M/1.4% for important

## Verification Results

### All Sections Verified
- **Entity Classification (Art 3):** Essential and important entities correctly defined
- **Sectors:** All 11 Annex I (high criticality) and 7 Annex II (other critical) sectors present and correct
- **Risk Management (Art 21):** All 10 sub-measures (a)-(j) present and accurately described
- **Management Accountability (Art 20):** Correctly included
- **Incident Reporting (Art 23):** Multi-stage reporting (24h, 72h, intermediate, 1 month) all correct
- **Governance (Arts 7-16):** CSIRTs, Cooperation Group, EU-CyCLONe all correctly modeled
- **Supervision (Arts 32-34):** Essential vs important entity supervision correctly differentiated
- **Fines:** EUR 10M/2% essential, EUR 7M/1.4% important — correct per Art 34(4)-(5)
- **Vulnerability Disclosure (Arts 12-13):** Correctly modeled
- **Transposition deadline:** 17 Oct 2024 correct

### Issues Found
1. **Wrong code on important entities:** `nis2-important-entities` has code ART-4, but Art 4 is "Definitions". Should be ART-3-2 (important entities defined in Art 3(2)). The `source_reference` field correctly says Art 3(2).
2. **Governance source_reference overlap:** Category references "Articles 7-16, 31-37" but supervision is separate category
3. **Art 20 parenting:** Placed under Art 21 risk management; Art 20 is standalone "Governance" article

### Concept Count
52 concepts — all legitimate, zero fabricated. High structural accuracy.
