# Split 04: MITRE Databases Verification

## Scope

Verify 3 MITRE frameworks against their official public databases.

| Framework | File | Concepts | Source |
|-----------|------|----------|--------|
| MITRE ATT&CK | mitre-attack.json | 74 | attack.mitre.org |
| MITRE ATLAS | mitre-atlas.json | 55 | atlas.mitre.org |
| CWE | cve-cwe.json | 35 | cwe.mitre.org |

## Verification Methodology

MITRE databases are fully public with structured data exports:

1. **MITRE ATT&CK** — verify tactic/technique structure against STIX/TAXII data or the ATT&CK website matrix view
2. **MITRE ATLAS** — verify against the ATLAS matrix at atlas.mitre.org
3. **CWE** — verify top-level CWE categories and hierarchy against cwe.mitre.org

For each:
1. Fetch the official structure (tactics, techniques, categories)
2. Compare concept codes, names, and hierarchy against JSON
3. Check version metadata matches current published version
4. Document in proof file

## Special Considerations

- ATT&CK has a very large taxonomy; our 74 concepts likely represent tactics + top-level techniques only — verify this subset is correct
- ATLAS is smaller and AI-focused — verify all technique IDs match
- CWE with 35 concepts likely represents top-level categories — verify which CWE views are represented

## Deliverables

- [ ] 3 verified/rebuilt ontology JSON files
- [ ] 3 proof files in `docs/sources/`
- [ ] `cargo test` passes
