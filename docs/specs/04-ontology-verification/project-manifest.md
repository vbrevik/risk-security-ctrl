# Project Manifest — Ontology Source Verification & Proof View

**Created:** 2026-03-28
**Status:** Draft — pending user confirmation

## Project Summary

Verify all 28 remaining ontology framework JSON files against their official online sources, create proof files documenting verification evidence, and build a proof view feature that enables users to trace any framework claim back to its authoritative source.

## Splits

### 01-eu-legislation
**Scope:** Verify 5 EU legislative frameworks against EUR-Lex
**Frameworks:** CER Directive (2022/2557), DORA (2022/2554), EU AI Act (2024/1689), GDPR (2016/679), NIS2 (2022/2555)
**Deliverables:** Verified/rebuilt JSON files, 5 proof files in docs/sources/
**Complexity:** Medium (252 concepts total, well-structured legislative text)
**Dependencies:** None

### 02-nist-publications
**Scope:** Verify 5 NIST frameworks against nist.gov
**Frameworks:** NIST CSF 2.0, NIST RMF (SP 800-37), NIST SP 800-53 Rev 5, NIST AI RMF (AI 100-1), NIST AI GenAI Profile (AI 600-1)
**Deliverables:** Verified/rebuilt JSON files, 5 proof files
**Complexity:** Medium-High (596 concepts total, 800-53 alone has 344)
**Dependencies:** None

### 03-iso-standards
**Scope:** Verify 9 ISO standards against publicly available structure data
**Frameworks:** ISO 27000, ISO 31000, ISO 31010, ISO 42001, ISO 42005, ISO 9000, ISO 10015, ISO 23894, ISO 24028
**Deliverables:** Verified/rebuilt JSON files, 9 proof files with verification status annotations
**Complexity:** High (412 concepts total, paywall limits verification depth)
**Dependencies:** None

### 04-mitre-databases
**Scope:** Verify 3 MITRE frameworks against mitre.org/attack.mitre.org
**Frameworks:** MITRE ATT&CK, MITRE ATLAS, CWE
**Deliverables:** Verified/rebuilt JSON files, 3 proof files
**Complexity:** Low-Medium (164 concepts total, well-structured public databases)
**Dependencies:** None

### 05-industry-frameworks
**Scope:** Verify 6 industry/other frameworks against published sources
**Frameworks:** Google SAIF, Data-Centric Security, XAI DataOps, Zero Trust (NIST SP 800-207), CISA ZTMM, NATO FMN
**Deliverables:** Verified/rebuilt JSON files, 6 proof files
**Complexity:** Medium (174 concepts total, varied source accessibility)
**Dependencies:** None

### 06-relationship-validation
**Scope:** Validate all cross-framework relationship files after framework verification
**Deliverables:** Updated relationship JSON files with valid concept IDs
**Complexity:** Medium (concept ID changes propagate to relationship files)
**Dependencies:** 01-eu-legislation, 02-nist-publications, 03-iso-standards, 04-mitre-databases, 05-industry-frameworks

### 07-proof-view-feature
**Scope:** Build UI + backend for proof view feature
**Deliverables:** Database schema migration, API endpoints, dedicated proof page route, verification metadata display
**Key decisions:**
- Database fields (verification status, date, source_url) on frameworks table
- Markdown proof files served via API
- Links/screenshots to source material for claim traceability
- /zachman-spec applied to feature architecture
- /stig-compliance applied at both spec and plan phases
**Complexity:** High (schema change, new API endpoints, new frontend route)
**Dependencies:** 01-eu-legislation, 02-nist-publications, 03-iso-standards, 04-mitre-databases, 05-industry-frameworks

## Execution Order

```
Phase 1 (parallel):  01-eu-legislation
                     02-nist-publications
                     03-iso-standards
                     04-mitre-databases
                     05-industry-frameworks

Phase 2 (after Phase 1): 06-relationship-validation
                          07-proof-view-feature
```

## Dependency Graph

```
01-eu-legislation ──────────┐
02-nist-publications ───────┤
03-iso-standards ───────────┼──► 06-relationship-validation
04-mitre-databases ─────────┤
05-industry-frameworks ─────┤
                            └──► 07-proof-view-feature
```

<!-- SPLIT_MANIFEST
01-eu-legislation
02-nist-publications
03-iso-standards
04-mitre-databases
05-industry-frameworks
06-relationship-validation
07-proof-view-feature
END_MANIFEST -->
