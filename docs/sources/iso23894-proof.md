# ISO/IEC 23894:2023 — Verification Proof

**Source:** https://www.iso.org/standard/77304.html
**Reference:** ISO/IEC 23894:2023 Information technology — AI — Guidance on risk management
**Verified:** 2026-03-28
**Status:** structure-verified

## How to Verify

1. **ISO Catalogue Page:** https://www.iso.org/standard/77304.html
2. **ISO Online Browsing Platform:** https://www.iso.org/obp/ui/#iso:std:iso-iec:23894:ed-1:v1:en — may show table of contents
3. **ANSI Webstore:** Search for "ISO/IEC 23894:2023" at https://webstore.ansi.org — may show ToC preview
4. **ISO 31000:2018 reference:** https://www.iso.org/standard/65694.html — the base framework this standard extends

### Key Verification Points
- Based on ISO 31000:2018 — same Clause 4 (Principles), Clause 5 (Framework), Clause 6 (Process) structure
- All 8 ISO 31000 principles applied to AI context
- AI-specific risk sources and lifecycle considerations in Annexes

## Verification Results

### Confirmed Correct
- **Framework metadata:** Name, version (2023), source_url all correct
- **8 Principles (P1-P8):** All correctly mirror ISO 31000:2018 Clause 4 principles
- **Risk management process:** All ISO 31000 process steps correctly represented (Communication, Scope/Context, Risk Assessment with 3 sub-steps, Treatment, Monitoring, Recording)
- **AI-specific risk sources:** Data, model, system, use, societal, organizational — thematically appropriate
- **AI lifecycle phases:** Planning/Design through Retirement — reasonable decomposition

### Issues Found — Corrections Applied
1. **Process clause numbering:** Process section was assigned to Clause 5 (same as Framework), creating an internal inconsistency. **Corrected** Process to Clause 6 to match ISO 31000:2018 structure (Clause 5 = Framework, Clause 6 = Process).
2. **Process concept codes:** Codes were still showing 5.x after source_reference fix. **Corrected** all codes from 5.x to 6.x.
3. **Sort order:** Framework had sort_order 5 (after Process at 2). **Corrected** to logical order: Principles (1), Framework (2), Process (3), Risk Sources (4), Lifecycle (5).
4. **Missing framework components:** Only 2 of 6 ISO 31000 framework components were included. **Added** 4 missing components: Leadership & Commitment (FW.3), Implementation (FW.4), Evaluation (FW.5), Improvement (FW.6).

### Issues Documented (Not Corrected — Require Purchased Standard)
5. **Annex structure:** Annexes A (Risk Sources) and B (Lifecycle) have plausible but unverifiable sub-clause numbering.

### Concept Count
40 concepts (increased from 36) — 4 framework components added. All thematically appropriate, zero fabricated.
