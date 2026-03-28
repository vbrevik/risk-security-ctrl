# NIST SP 800-53 Rev 5 — Verification Proof

**Source:** https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final
**Reference:** NIST SP 800-53 Rev 5 (September 2020, updated December 2020)
**Verified:** 2026-03-28
**Status:** corrected

## How to Verify

1. **Official PDF:** https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-53r5.pdf
2. **OSCAL Machine-Readable Catalog (JSON):** https://github.com/usnistgov/oscal-content/blob/main/nist.gov/SP800-53/rev5/json/NIST_SP-800-53_rev5_catalog.json
3. **Interactive Control Catalog:** https://csrc.nist.gov/projects/cprt/catalog#/cprt/framework/version/SP_800_53_5_1_0/home

### Verify Control Families
Open the interactive catalog or PDF Appendix A. The 20 families are:
AC, AT, AU, CA, CM, CP, IA, IR, MA, MP, PE, PL, PM, PS, PT, RA, SA, SC, SI, SR

### Verify Individual Controls
Use the interactive catalog to search by control ID (e.g., "SA-22", "AT-5"). Each control shows official name, description, and supplemental guidance.

### Verify Corrections
- **SA-22:** Search "SA-22" in catalog → should show "Unsupported System Components"
- **AT-5:** Search "AT-5" → should show "Contacts with Security Groups and Associations"
- **SA-24:** Search "SA-24" → should show "Design for Cyber Resiliency"

## Corrections Applied

1. **SA-22** corrected from "Developer Security and Privacy Architecture" to "Unsupported System Components"
2. **AT-5** corrected from "Communications Security Awareness" to "Contacts with Security Groups and Associations"
3. **SA-24** added: "Design for Cyber Resiliency" (was missing)
4. **SI-18** title corrected to "Personally Identifiable Information Quality Operations"
5. **PM-25** title expanded to full official name

## Concept Count
345 concepts (was 344, +1 for SA-24)
