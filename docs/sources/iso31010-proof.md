# IEC 31010:2019 — Verification Proof

**Source:** https://www.iso.org/standard/72140.html
**Reference:** IEC 31010:2019 Risk management — Risk assessment techniques
**Verified:** 2026-03-28
**Status:** partially-verified

## How to Verify

1. **ISO Catalogue Page:** https://www.iso.org/standard/72140.html
2. **IEC Webstore:** https://webstore.iec.ch/en/publication/59809
3. **Wikipedia:** https://en.wikipedia.org/wiki/ISO_31010 — lists techniques from the standard
4. **BSI Shop:** https://shop.bsigroup.com/products/risk-management-risk-assessment-techniques — shows table of contents

### Key Verification Points
- Standard is officially **IEC 31010:2019** (transferred from ISO to IEC)
- Annex B describes ~31 risk assessment techniques
- Table A.1 shows technique applicability to risk assessment stages

## Verification Results

### Confirmed Correct
- **Framework metadata:** source_url correct, version 2019 correct
- **Core techniques present:** Brainstorming, Delphi, FMEA/FMECA, HAZOP, HACCP, LOPA, Bow-tie, Markov, Monte Carlo, Bayesian, FTA, ETA, SWIFT, Risk Matrix, HRA, Ishikawa — all correctly named
- **28 techniques total** in the file

### Issues Found
1. **Framework name:** File uses "ISO 31010:2019" but official designation is "IEC 31010:2019" (transferred to IEC). Not corrected — cosmetic, ISO still hosts the catalogue page.
2. **Category names are editorial additions:** Groupings like "Look-up methods", "Supporting methods", "Scenario analysis" etc. are not from the standard. IEC 31010 lists techniques sequentially in Annex B without named sub-groups. Acceptable for ontology usability but noted as non-standard.
3. **Annex B clause numbers may not match:** Sequential B.1-B.28 mapping was assumed — actual standard may order techniques differently.
4. **Root cause analysis placement:** Under "Scenario analysis" but it's a causal analysis method, not a scenario technique.

### Missing Techniques
Several techniques from Annex B may be missing, including: Reliability centred maintenance (RCM), Sneak circuit analysis, FN curves, Risk indices, Influence diagrams. Exact list requires the purchased standard.

### Concept Count
38 concepts — 28 techniques + 10 categories. All techniques are legitimate risk assessment methods, zero fabricated. Category names are editorial additions.
