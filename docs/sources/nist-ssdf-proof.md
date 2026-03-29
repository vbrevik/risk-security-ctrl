# NIST SP 800-218 Secure Software Development Framework (SSDF) v1.1 — Verification Proof

**Source:** https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-218.pdf
**Reference:** NIST Special Publication 800-218, Secure Software Development Framework (SSDF) Version 1.1: Recommendations for Mitigating the Risk of Software Vulnerabilities
**Verified:** 2026-03-28
**Status:** verified

## How to Verify

1. **NIST CSRC page:** https://csrc.nist.gov/publications/detail/sp/800-218/final
2. **Full document PDF:** https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-218.pdf
3. **Excel table:** https://csrc.nist.gov/files/pubs/sp/800/218/final/docs/nist.sp.800-218.ssdf-table.xlsx (definitive structured table)
4. **Structure check:** Table 1 in the document lists all practice groups, practices (with codes), and tasks

## Official Structure

NIST SP 800-218 v1.1 (February 2022) has **4 Practice Groups**, **19 Practices**, and **42 Tasks**:

| Practice Group | Code | Practices | Tasks |
|---------------|------|-----------|-------|
| Prepare the Organization | PO | PO.1–PO.5 (5 practices) | 13 |
| Protect the Software | PS | PS.1–PS.3 (3 practices) | 4 |
| Produce Well-Secured Software | PW | PW.1, PW.2, PW.4–PW.9 (8 practices) | 16 |
| Respond to Vulnerabilities | RV | RV.1–RV.3 (3 practices) | 9 |

**Note:** PW.3 does not exist in v1.1 (was merged into PW.4 from v1.0). PW.4.3 was also removed.

### Task counts per practice

| Practice | Tasks |
|----------|-------|
| PO.1 | 3 |
| PO.2 | 3 |
| PO.3 | 3 |
| PO.4 | 2 |
| PO.5 | 2 |
| PS.1 | 1 |
| PS.2 | 1 |
| PS.3 | 2 |
| PW.1 | 3 |
| PW.2 | 1 |
| PW.4 | 3 (PW.4.1, PW.4.2, PW.4.4 — PW.4.3 removed) |
| PW.5 | 1 |
| PW.6 | 2 |
| PW.7 | 2 |
| PW.8 | 2 |
| PW.9 | 2 |
| RV.1 | 3 |
| RV.2 | 2 |
| RV.3 | 4 |
| **Total** | **42** |

## Verification Results

### Confirmed Correct

- 4 practice group names and codes match the official document
- All 19 practice codes and titles verified against NIST SP 800-218 Table 1
- All 42 task codes and descriptions verified against the SSDF table
- PW.3 gap and PW.4.3 removal correctly reflected
- Total: 66 concepts (1 root + 4 groups + 19 practices + 42 tasks)
- FK ordering: all parents before children

### Issues Found

None. Structure built directly from the official February 2022 publication.

## Concept Count

66 concepts — all codes, titles, and descriptions accurate against NIST SP 800-218 Version 1.1.
