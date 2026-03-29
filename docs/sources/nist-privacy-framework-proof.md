# NIST Privacy Framework Version 1.0 — Verification Proof

**Source:** https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.01162020.pdf
**Reference:** NIST Privacy Framework: A Tool For Improving Privacy Through Enterprise Risk Management, Version 1.0
**Verified:** 2026-03-28
**Status:** verified

## How to Verify

1. **NIST official page:** https://www.nist.gov/privacy-framework
2. **Full document PDF:** https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.01162020.pdf
3. **Core table:** Appendix A of the document contains the complete Core table (Functions, Categories, Subcategories)
4. **Structure check:** Compare Function codes (ID-P, GV-P, CT-P, CM-P, PR-P), Category codes (e.g. ID.IM-P, GV.PO-P), and subcategory codes (e.g. ID.IM-P1) against the Core table

## Version Note

NIST Privacy Framework **Version 1.0** (January 2020) is the current stable standard. Version 1.1 was released as an Initial Public Draft on April 14, 2025 (comment deadline June 13, 2025; final release expected 2026). This ontology implements Version 1.0.

## Official Structure

NIST Privacy Framework v1.0 has **5 Functions**, **18 Categories**, and **100 Subcategories**:

| Function | Code | Categories | Subcategories |
|----------|------|-----------|---------------|
| Identify-P | ID-P | ID.IM-P, ID.BE-P, ID.RA-P, ID.DE-P | 21 |
| Govern-P | GV-P | GV.PO-P, GV.RM-P, GV.AT-P, GV.MT-P | 20 |
| Control-P | CT-P | CT.PO-P, CT.DM-P, CT.DP-P | 19 |
| Communicate-P | CM-P | CM.PO-P, CM.AW-P | 10 |
| Protect-P | PR-P | PR.PO-P, PR.AC-P, PR.DS-P, PR.MA-P, PR.PT-P | 30 |

### Subcategory counts per Category

| Category | Count |
|----------|-------|
| ID.IM-P  | 8 |
| ID.BE-P  | 3 |
| ID.RA-P  | 5 |
| ID.DE-P  | 5 |
| GV.PO-P  | 6 |
| GV.RM-P  | 3 |
| GV.AT-P  | 4 |
| GV.MT-P  | 7 |
| CT.PO-P  | 4 |
| CT.DM-P  | 10 |
| CT.DP-P  | 5 |
| CM.PO-P  | 2 |
| CM.AW-P  | 8 |
| PR.PO-P  | 10 |
| PR.AC-P  | 6 |
| PR.DS-P  | 8 |
| PR.MA-P  | 2 |
| PR.PT-P  | 4 |
| **Total** | **100** |

## Verification Results

### Confirmed Correct

- 5 Function names, codes, and ordering match the official document
- All 18 Category names and codes verified against Appendix A of NIST CSWP.01162020
- All 100 Subcategory codes and descriptions match the Core table verbatim
- Total: 124 concepts (1 root container + 5 functions + 18 categories + 100 subcategories)
- FK ordering: all parent concepts appear before children in the concepts array
- All concept IDs are unique

### Issues Found

None. Structure is clean and complete from initial build.

## Concept Count

124 concepts — all codes, titles, and definitions accurate against NIST Privacy Framework Version 1.0.
