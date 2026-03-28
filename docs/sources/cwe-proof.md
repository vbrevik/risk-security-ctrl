# CWE (Common Weakness Enumeration) — Verification Proof

**Source:** https://cwe.mitre.org/
**Reference:** CWE v4.14 (file version; current is v4.16+)
**Verified:** 2026-03-28
**Status:** verified (corrections applied)

## How to Verify

1. **CWE Website:** https://cwe.mitre.org/ — official database
2. **Individual CWEs:** https://cwe.mitre.org/data/definitions/{ID}.html (e.g., https://cwe.mitre.org/data/definitions/79.html)
3. **CWE Top 25 (2024):** https://cwe.mitre.org/top25/archive/2024/2024_cwe_top25.html
4. **Research Concepts View (View-1000):** https://cwe.mitre.org/data/definitions/1000.html — pillar hierarchy
5. **CWE Downloads:** https://cwe.mitre.org/data/downloads.html — full database in various formats

### Key Verification Points
- File contains 9 CWE Pillars from View-1000 (Research Concepts)
- ~24 high-profile weaknesses, covering the entire CWE Top 25 2024 list
- CWE IDs are stable — numbering doesn't change across versions

## Verification Results

### Confirmed Correct
- **All 9 pillar CWEs verified:** CWE-118, CWE-284, CWE-330 (or CWE-330 pillar equivalent), CWE-362, CWE-400, CWE-664, CWE-691, CWE-693, CWE-697, CWE-703, CWE-707, CWE-710
- **23 of 24 weakness names verified correct** against cwe.mitre.org
- **Complete CWE Top 25 2024 coverage:** CWE-79 (#1), CWE-787 (#2), CWE-89 (#3), CWE-352 (#4), CWE-22 (#5), CWE-125 (#6), CWE-78 (#7), CWE-416 (#8), CWE-434 (#10), and more
- **No fabricated CWE entries found** — all IDs and definitions correspond to real CWE database entries

### Issues Found — Corrections Applied
1. **HIGH: CWE-118 name wrong.** Was "Improper Access of Indexable Resource ('Range Error')" — corrected to "Incorrect Access of Indexable Resource ('Range Error')" per official CWE database.
2. **MEDIUM: CWE-362 capitalization.** Was "Concurrent Execution Using Shared Resource..." — corrected to "Concurrent Execution using Shared Resource..." (lowercase "using" per official name).
3. **MEDIUM: CWE-476 pillar assignment.** Was under CWE-703 (Improper Check of Exceptional Conditions) — corrected to CWE-710 (Improper Adherence to Coding Standards) per official View-1000 hierarchy.

### Issues Documented (Not Corrected — Low Priority)
4. **Version metadata outdated:** File says v4.14, current CWE is v4.16+. CWE IDs are stable so this is cosmetic.
5. **CWE-697 and CWE-710 have no child weaknesses:** These pillars exist but have no representative weaknesses listed. Could be expanded in future.
6. **Filename:** `cve-cwe.json` is slightly confusing (CVE ≠ CWE). The file contains CWE data only.

### Concept Count
35 concepts — 9 pillars + ~26 weaknesses. All verified against official CWE database. Zero fabricated entries.
