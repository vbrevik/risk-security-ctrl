# NIST SP 800-161 Rev. 1 Cybersecurity Supply Chain Risk Management — Verification Proof

**Source:** https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-161r1.pdf
**Reference:** NIST Special Publication 800-161 Revision 1, Cybersecurity Supply Chain Risk Management Practices for Systems and Organizations (May 2022, updated November 2024)
**Verified:** 2026-03-28
**Status:** verified

## How to Verify

1. **NIST CSRC page:** https://csrc.nist.gov/pubs/sp/800/161/r1/upd1/final
2. **Full document PDF:** https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-161r1.pdf
3. **Organizational levels:** Chapter 2 of the document describes the three-tier hierarchy
4. **Practice tiers:** Chapter 3, Sections 3.4.1 (Foundational), 3.4.2 (Sustaining), 3.4.3 (Enhancing)
5. **Key practices:** NIST IR 8276 — https://csrc.nist.gov/pubs/ir/8276/final

## Framework Nature

NIST SP 800-161 Rev. 1 is a **guidance document** rather than a structured control catalog. It does not define its own control codes — instead it references the SR (Supply Chain Risk Management) control family from NIST SP 800-53 Rev. 5. The ontology therefore models the framework's structural concepts: organizational levels, maturity practice tiers, and key practices.

## Official Structure

| Component | Source | Count |
|-----------|--------|-------|
| Organizational Levels | Chapter 2 | 3 |
| Foundational Practices | Section 3.4.1 | 9 |
| Sustaining Practices | Section 3.4.2 | 6 |
| Enhancing Practices | Section 3.4.3 | 2 |
| Key C-SCRM Practices | NIST IR 8276 | 8 |

**Total ontology concepts:** 35 (10 categories + 25 subcategories)

### Organizational Levels
- **L1** — Organization Level (strategic governance)
- **L2** — Mission/Business Process Level (operational translation)
- **L3** — System/Operational Level (tactical implementation)

### Practice Tiers
**Foundational (FP.1–FP.9):** Executive endorsement, risk hierarchy, criticality measurement, acquisition integration, supplier assessments, embedded software monitoring, quality assurance, compliance checks, incident response plans.

**Sustaining (SP.1–SP.6):** Certification-based assessments, continuous monitoring, contract requirements, supplier IR integration, collaborative improvement, metrics collection.

**Enhancing (EP.1–EP.2):** C-SCRM automation, probabilistic quantitative risk analysis.

### Key C-SCRM Practices (NIST IR 8276)
KP.1–KP.8: Integrate across organization, establish formal program, know critical suppliers, understand supply chain, collaborate with suppliers, include in resilience activities, assess throughout relationship, plan for full lifecycle.

## Verification Results

### Confirmed Correct
- 3 organizational level descriptions match Chapter 2 of SP 800-161r1
- Foundational/Sustaining/Enhancing practice tiers match Chapter 3 Sections 3.4.1-3.4.3
- 8 key practices match NIST IR 8276 exactly
- All parent-child relationships topologically ordered

### Issues Found
None. Built directly from official publication structure.

## Concept Count
35 concepts — organizational levels, maturity practice tiers, and key practices accurately represent NIST SP 800-161 Rev. 1 structure.
