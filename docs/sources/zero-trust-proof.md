# Zero Trust Architecture (NIST SP 800-207) — Verification Proof

**Source:** https://www.nist.gov/publications/zero-trust-architecture
**Reference:** NIST SP 800-207 — Zero Trust Architecture (August 2020)
**Verified:** 2026-03-28
**Status:** verified (corrections applied)

## How to Verify

1. **NIST publication page:** https://csrc.nist.gov/pubs/sp/800/207/final
2. **Full PDF:** https://nvlpubs.nist.gov/nistpubs/specialpublications/NIST.SP.800-207.pdf — Section 2.1 for tenets, Section 3 for components
3. **Microsoft ZT principles (for comparison):** https://learn.microsoft.com/en-us/security/zero-trust/zero-trust-overview

## Verification Results

### Issues Found — Corrections Applied
1. **CRITICAL: Microsoft principles misattributed to NIST.** "Verify explicitly," "Least privileged access," and "Assume breach" are Microsoft's Zero Trust pillars, NOT NIST SP 800-207 tenets. These used Microsoft-specific language (JIT/JEA, risk-based adaptive policies). **Removed** all 4 concepts (3 principles + 1 category).
2. **MAJOR: All 7 tenet names corrected** to match exact NIST SP 800-207 Section 2.1 wording.
3. **MAJOR: Component names corrected.** "Policy Decision Point" → "Policy Engine" (PE); "Policy Administration Point" → "Policy Administrator" (PA) per NIST SP 800-207 Section 3.

### Issues Documented (Not Corrected — Low Priority)
4. **Missing NIST components:** Figure 2 of SP 800-207 shows additional supporting components (Data Access Policies, Enterprise PKI, ID Management System, Industry Compliance System, Activity Logs). Could be added in future.

### Concept Count
15 concepts (reduced from 19) — 2 categories + 7 tenets + 6 components. 4 fabricated Microsoft concepts removed.
