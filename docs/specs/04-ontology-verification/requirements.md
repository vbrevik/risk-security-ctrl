# Ontology Source Verification & Proof View

## Background

The project's ontology-first architecture relies on JSON files in `ontology-data/` defining 30 security/risk management frameworks. Two files (NSL Sikkerhetsloven, NSM Grunnprinsipper) were found to contain hallucinated/incorrect data when verified against official sources. Both have been rebuilt from verified sources.

The remaining 28 frameworks need the same verification procedure, and a new "proof view" UI feature should display source provenance to users.

## Requirements

### R1: Source Verification of All Ontology Frameworks

For each of the 28 remaining frameworks:
1. Fetch the official source (standard body website, legislation portal, etc.)
2. Compare the JSON file's structure against the authoritative source
3. Document any discrepancies (wrong names, missing sections, fabricated entries)
4. If errors found: rebuild the JSON from verified data
5. If correct: document verification status

### R2: Proof Files for Each Framework

For each verified framework, create a proof file in `docs/sources/` containing:
- Official source URL
- Version/date information
- Extracted structure (chapters, sections, controls)
- Extraction date
- Any notes about verification methodology

Two proof files already exist as templates:
- `docs/sources/sikkerhetsloven-lovdata-extract.md`
- `docs/sources/nsm-grunnprinsipper-v21-extract.md`

### R3: Cross-Framework Relationship Validation

After verifying framework files, validate all relationship files:
- Check that all source and target concept IDs exist
- Verify relationship types are semantically correct
- Remove or fix relationships referencing invalid concepts

### R4: Proof View Feature

Design a UI feature that shows source provenance for each framework:
- Display verified source URL
- Show extraction/verification date
- Indicate verification status (verified/unverified/needs-update)
- Link to the proof file content
- Integrate into the existing ontology explorer UI

## Frameworks to Verify (28 remaining)

| # | ID | Name | Concepts | Source Type |
|---|-----|------|----------|-------------|
| 1 | cer-directive | CER Directive (EU) 2022/2557 | 42 | EU legislation |
| 2 | cisa-ztmm | CISA Zero Trust Maturity Model | 31 | US gov publication |
| 3 | cwe | CWE (Common Weakness Enumeration) | 35 | MITRE database |
| 4 | data-centric | Data-Centric Security | 13 | Industry concept |
| 5 | dora | DORA (EU) 2022/2554 | 32 | EU legislation |
| 6 | eu-ai-act | EU AI Act 2024/1689 | 50 | EU legislation |
| 7 | fmn | Federated Mission Networking | 20 | NATO publication |
| 8 | gdpr | GDPR (EU) 2016/679 | 60 | EU legislation |
| 9 | google-saif | Google SAIF | 25 | Industry framework |
| 10 | iso10015 | ISO 10015:2019 | 27 | ISO standard |
| 11 | iso23894 | ISO/IEC 23894:2023 | 36 | ISO standard |
| 12 | iso24028 | ISO/IEC 24028:2020 | 28 | ISO standard |
| 13 | iso27000 | ISO/IEC 27000 series | 106 | ISO standard |
| 14 | iso31000 | ISO 31000:2018 | 26 | ISO standard |
| 15 | iso31010 | IEC 31010:2019 | 38 | ISO/IEC standard |
| 16 | iso42001 | ISO/IEC 42001:2023 | 86 | ISO standard |
| 17 | iso42005 | ISO/IEC 42005:2025 | 49 | ISO standard |
| 18 | iso9000 | ISO 9000:2015 | 16 | ISO standard |
| 19 | mitre-atlas | MITRE ATLAS | 55 | MITRE database |
| 20 | mitre-attack | MITRE ATT&CK | 74 | MITRE database |
| 21 | nis2 | NIS2 Directive (EU) 2022/2555 | 52 | EU legislation |
| 22 | nist-ai-genai | NIST AI 600-1 GenAI Profile | 18 | NIST publication |
| 23 | nist-ai-rmf | NIST AI RMF (AI 100-1) | 129 | NIST publication |
| 24 | nist-csf | NIST CSF 2.0 | 50 | NIST publication |
| 25 | nist-rmf | NIST RMF (SP 800-37) | 55 | NIST publication |
| 26 | nist-800-53 | NIST SP 800-53 Rev 5 | 344 | NIST publication |
| 27 | xai-dataops | XAI DataOps | 66 | Industry concept |
| 28 | zero-trust | Zero Trust (NIST SP 800-207) | 19 | NIST publication |

## Verification Priority

1. **High priority** — Frameworks with official online sources that can be fetched and compared:
   - EU legislation (CER, DORA, EU AI Act, GDPR, NIS2) via EUR-Lex
   - NIST publications (CSF, RMF, 800-53, AI RMF, GenAI) via nist.gov
   - MITRE databases (ATT&CK, ATLAS, CWE) via mitre.org/attack.mitre.org
   - CISA publications via cisa.gov

2. **Medium priority** — Frameworks with published but harder-to-access sources:
   - ISO standards (behind paywall, verify structure from public TOC/previews)
   - NATO FMN (limited public availability)

3. **Lower priority** — Industry/conceptual frameworks:
   - Google SAIF, Data-Centric, XAI DataOps, Zero Trust (verify against whitepapers)

## Constraints

- ISO standard full text is behind paywall; verify from publicly available table of contents, preview pages, and official scope descriptions
- Each verification must be independently traceable — no batching of proof files
- Backend tests must pass after any rebuild (`cargo test`)
- Relationship files must be updated when concept IDs change
- Existing concept IDs referenced in guidance files must remain stable or guidance must be updated
