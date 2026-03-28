# Split 03: ISO Standards Verification

## Scope

Verify 9 ISO standards against publicly available structure information.

| Framework | File | Concepts | Standard Reference |
|-----------|------|----------|--------------------|
| ISO/IEC 27000 series | iso27000.json | 106 | ISO/IEC 27001:2022, 27002:2022 |
| ISO 31000 | iso31000.json | 26 | ISO 31000:2018 |
| IEC 31010 | iso31010.json | 38 | IEC 31010:2019 |
| ISO/IEC 42001 | iso42001.json | 86 | ISO/IEC 42001:2023 |
| ISO/IEC 42005 | iso42005.json | 49 | ISO/IEC 42005:2025 |
| ISO 9000 | iso9000.json | 16 | ISO 9000:2015 |
| ISO 10015 | iso10015.json | 27 | ISO 10015:2019 |
| ISO/IEC 23894 | iso23894.json | 36 | ISO/IEC 23894:2023 |
| ISO/IEC 24028 | iso24028.json | 28 | ISO/IEC 24028:2020 |

## Verification Methodology

ISO standards are behind paywall. Verification uses publicly available information:

1. **ISO catalogue page** (iso.org) — verify title, scope, edition year
2. **Table of contents / preview** — verify clause numbering and titles where available
3. **Published annexes** — ISO 27001 Annex A controls are widely documented
4. **Secondary authoritative sources** — official ISO summaries, national standards bodies

For each framework:
1. Fetch ISO catalogue page for official metadata
2. Compare clause structure against JSON where publicly visible
3. Document what could and could not be verified
4. Mark verification status accordingly

## Verification Status

Each framework gets one of:
- `verified` — Full structure confirmed from public sources
- `partially-verified` — TOC/high-level structure confirmed; individual clause content behind paywall
- `structure-verified` — Clause numbering and titles confirmed; definitions not independently verifiable

## Special Considerations

- **ISO 27001 Annex A** controls are widely published and can be fully verified
- **ISO 31000** has a simple, well-known structure that's publicly documented
- **ISO 42001** (AI Management System) is newer and less publicly documented
- **ISO 42005:2025** may have very limited public information given its recency

## Deliverables

- [ ] 9 verified ontology JSON files (with verification status annotations)
- [ ] 9 proof files documenting what was verified and what was not
- [ ] `cargo test` passes
