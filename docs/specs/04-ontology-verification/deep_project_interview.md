# Deep Project Interview — Ontology Source Verification

**Date:** 2026-03-28
**Interviewer:** Claude
**Interviewee:** Vidar Brevik (project owner)

## Context

Two ontology files (NSL Sikkerhetsloven, NSM Grunnprinsipper) were discovered to contain AI-hallucinated data. Both were rebuilt from official sources. The user wants the same verification procedure applied to all 28 remaining frameworks, plus a "proof view" feature showing source provenance in the UI.

## Key Decisions

### Q1: ISO Standards Behind Paywall
**Decision:** Verify from publicly available TOC + preview pages on iso.org. Document what could vs. could not be verified. Mark each framework's verification status explicitly (verified/partially-verified/unverifiable).

### Q2: Proof View Feature Scope
**Decision:** Dedicated proof page (new route) showing:
- Full proof file content
- Verification history
- Links or screenshots to the source material
- Must be possible to verify WHY a claim is raised — traceable back to authoritative source

### Q3: Project Splitting Strategy
**Decision:** Split by source type:
- EU legislation (CER, DORA, EU AI Act, GDPR, NIS2) — EUR-Lex
- NIST publications (CSF, RMF, 800-53, AI RMF, GenAI) — nist.gov
- ISO standards (27000, 31000, 31010, 42001, 42005, 9000, 10015, 23894, 24028) — iso.org
- MITRE databases (ATT&CK, ATLAS, CWE) — mitre.org
- Industry/other frameworks (Google SAIF, Data-Centric, XAI DataOps, Zero Trust, CISA ZTMM, FMN)
- Proof view feature (UI + backend)

### Q4: Verification Evidence Storage
**Decision:** Database fields + proof files:
- Add verification metadata columns to frameworks table (status, verified_date, source_url, verification_notes)
- Keep detailed markdown proof files in docs/sources/
- UI reads both for the proof view

### Q5: Skill Application (/zachman-spec, /stig-compliance)
**Decision:** Both stages:
- /zachman-spec during spec generation (structure the proof view feature spec)
- /stig-compliance during both spec and deep-plan phases
