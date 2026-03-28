# NIST CSF 2.0 — Verification Proof

**Source:** https://www.nist.gov/cyberframework
**Reference:** NIST Cybersecurity Framework 2.0 (February 2024)
**Verified:** 2026-03-28
**Status:** partially-verified

## How to Verify

1. **Official PDF:** https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.29.pdf
2. **CSF 2.0 Reference Tool (interactive):** https://csrc.nist.gov/Projects/Cybersecurity-Framework/Filters#/csf/filters
3. **CSF 2.0 Excel export:** https://csrc.nist.gov/extensions/nudp/services/json/csf/download?olirIds=all (use the Reference Tool's export)

### Verify Functions (Section 3 of PDF, p. 9)
Open the PDF and confirm 6 functions: Govern (GV), Identify (ID), Protect (PR), Detect (DE), Respond (RS), Recover (RC).

### Verify Categories (Appendix A of PDF, pp. 16-33)
The Reference Tool at the CSRC link above lets you browse all 22 categories interactively. Each category code (e.g., GV.OC, GV.RM, ID.AM) is clickable.

### Verify Subcategories
Use the Reference Tool to expand each category and see all subcategories. The tool shows the official code, outcome statement, and implementation examples for each.

## Verification Results

### Confirmed Correct
- All 6 Functions present with correct codes
- All 22 Categories present with correct codes and names
- 21 subcategories present — all correctly coded and defined
- Zero fabricated entries

### Incomplete Coverage
- **85 of 106 official subcategories missing** (~20% coverage)
- Most notable gaps: GV.SC (0/10), ID.RA (2/10), PR.DS (2/10), DE.CM (2/9)

### Concept Count
50 concepts (1 root + 6 functions + 22 categories + 21 subcategories)
