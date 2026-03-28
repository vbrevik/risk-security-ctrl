# CISA Zero Trust Maturity Model v2.0 — Verification Proof

**Source:** https://www.cisa.gov/zero-trust-maturity-model
**Reference:** CISA Zero Trust Maturity Model v2.0 (April 2023)
**Verified:** 2026-03-28
**Status:** verified (corrections applied)

## How to Verify

1. **CISA resource page:** https://www.cisa.gov/resources-tools/resources/zero-trust-maturity-model
2. **ZTMM v2.0 PDF:** https://www.cisa.gov/sites/default/files/2023-04/zero_trust_maturity_model_v2_508.pdf
3. **Microsoft per-pillar guides:** https://learn.microsoft.com/en-us/security/zero-trust/cisa-zero-trust-maturity-model-intro

## Verification Results

### Issues Found — Corrections Applied
1. **CRITICAL: Devices pillar — all 3 function names wrong.** Corrected: "Device Compliance" → "Policy Enforcement and Compliance Monitoring"; "Asset Management" → "Asset and Supply Chain Risk Management"; "Endpoint Detection and Response" → "Device Threat Protection".
2. **CRITICAL: Apps pillar — 2 wrong names.** Corrected: "Application Security Testing" → "Application Threat Protections"; "Workload Security" → "Secure Application Development and Deployment Workflow".
3. **CRITICAL: Data pillar — wrong names.** Corrected: "Data Inventory & Classification" → "Data Inventory Management"; "Data Protection" → "Data Encryption".
4. **CRITICAL: Missing functions added.** Added: Network Traffic Management (P3.2), Accessible Applications (P4.3), Data Categorization (P5.2), Data Availability (P5.4).
5. **CRITICAL: Fabricated function removed.** "Identity Stores" (P1.2) is not an official ZTMM function — removed.
6. **MAJOR: Description fixed.** "three maturity levels" → "four maturity stages".

### Confirmed Correct
- **5 pillars** (Identity, Devices, Networks, Applications & Workloads, Data) — correct
- **4 maturity stages** (Traditional, Initial, Advanced, Optimal) — correct
- **3 cross-cutting capabilities** (Visibility & Analytics, Automation & Orchestration, Governance) — correct

### Concept Count
34 concepts (increased from 31) — 3 categories + 5 pillars + 19 functions + 4 maturity stages + 3 capabilities. 1 fabricated removed, 4 missing added.
