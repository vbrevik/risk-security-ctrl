# Split 02: NIST Publications Verification

## Scope

Verify 5 NIST frameworks against nist.gov authoritative sources.

| Framework | File | Concepts | NIST Reference |
|-----------|------|----------|----------------|
| NIST CSF 2.0 | nist-csf.json | 50 | Cybersecurity Framework 2.0 |
| NIST RMF | nist-rmf.json | 55 | SP 800-37 Rev. 2 |
| NIST SP 800-53 | nist-sp-800-53.json | 344 | SP 800-53 Rev. 5 |
| NIST AI RMF | nist-ai-rmf.json | 129 | AI 100-1 |
| NIST AI GenAI | nist-ai-genai-profile.json | 18 | AI 600-1 |

## Verification Methodology

For each framework:

1. **Fetch official source** from nist.gov (NIST publications are freely available)
2. **Extract structure** — functions, categories, subcategories, control families
3. **Compare** against ontology JSON:
   - Verify function/category/subcategory codes match (e.g., GV, ID, PR, DE, RS, RC for CSF)
   - Verify control family codes for 800-53 (AC, AT, AU, etc.)
   - Check concept names match official titles
   - Verify AI RMF subcategory numbering (GV-1, MP-2, etc.)
4. **Document findings** in proof file
5. **Rebuild if errors found**

## Special Considerations

- **NIST SP 800-53** has 344 concepts — verify at control family level, spot-check individual controls
- **NIST AI RMF** uses abbreviated prefixes (gv-, mp-, ms-, mg-) per CLAUDE.md gotcha — verify these match
- **NIST CSF 2.0** was updated from 1.1; verify version 2.0 structure (6 functions including Govern)

## Deliverables

- [ ] 5 verified/rebuilt ontology JSON files
- [ ] 5 proof files in `docs/sources/`
- [ ] Verification status for each
- [ ] `cargo test` passes
