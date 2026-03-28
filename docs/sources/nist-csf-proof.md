# NIST CSF 2.0 — Verification Proof

**Source:** https://www.nist.gov/cyberframework
**Reference:** NIST Cybersecurity Framework 2.0 (CSWP.29, February 2024)
**Verified:** 2026-03-28
**Status:** verified

## How to Verify

1. **Official PDF:** https://nvlpubs.nist.gov/nistpubs/CSWP/NIST.CSWP.29.pdf
2. **CSF 2.0 Reference Tool (interactive):** https://csrc.nist.gov/Projects/Cybersecurity-Framework/Filters#/csf/filters
3. **CSF 2.0 JSON export:** https://csrc.nist.gov/extensions/nudp/services/json/csf/download?olirIds=all

### Verify Functions (Section 3 of PDF, p. 9)
Open the PDF and confirm 6 functions: Govern (GV), Identify (ID), Protect (PR), Detect (DE), Respond (RS), Recover (RC).

### Verify Categories (Appendix A of PDF, pp. 16–33)
The Reference Tool at the CSRC link above lets you browse all 22 categories interactively. Each category code (e.g., GV.OC, GV.RM, ID.AM) is clickable.

### Verify Subcategories
Use the Reference Tool to expand each category and see all subcategories. The tool shows the official code, outcome statement, and implementation examples for each.

## Verification Results

### Confirmed Correct
- All 6 Functions present with correct codes and outcome statements
- All 22 Categories present with correct codes and names
- All 106 subcategories present with correct codes and official outcome statement text

### Non-Sequential Numbering (by design)
NIST CSF 2.0 intentionally uses non-contiguous numbering due to retirement and renumbering of subcategories from CSF 1.1:

| Category | Present | Skip |
|----------|---------|------|
| ID.AM | 01–05, 07, 08 | 06 retired |
| PR.DS | 01, 02, 10, 11 | 03–09 retired |
| DE.CM | 01, 02, 03, 06, 09 | 04, 05, 07, 08 retired |
| DE.AE | 02, 03, 04, 06, 07, 08 | 01, 05 retired |
| RS.AN | 03, 06, 07, 08 | 01, 02, 04, 05 retired |
| RS.CO | 02, 03 | 01 retired |
| RS.MI | 01, 02 | 03 retired |
| RC.CO | 03, 04 | 01, 02 retired |

### Erroneous Entries Removed (2026-03-28)
Three codes from CSF 1.x or internal drafts were present and removed:
- `RS.COM-01`, `RS.COM-02` — non-existent codes (confusion with RS.CO)
- `RC.IMPROVE-01` — non-existent code (no RC.IMPROVE category in CSF 2.0)

### Concept Count
135 concepts total: 1 root node + 6 functions + 22 categories + 106 subcategories
