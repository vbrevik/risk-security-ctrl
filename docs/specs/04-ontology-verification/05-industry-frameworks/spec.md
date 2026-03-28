# Split 05: Industry & Other Frameworks Verification

## Scope

Verify 6 remaining frameworks against published source material.

| Framework | File | Concepts | Source Type |
|-----------|------|----------|-------------|
| Google SAIF | google-saif.json | 25 | Google whitepaper |
| Data-Centric Security | data-centric.json | 13 | Industry concept compilation |
| XAI DataOps | xai-dataops.json | 66 | Industry framework |
| Zero Trust | zero-trust.json | 19 | NIST SP 800-207 |
| CISA ZTMM | cisa-ztmm.json | 31 | CISA Zero Trust Maturity Model v2.0 |
| NATO FMN | fmn.json | 20 | Federated Mission Networking |

## Verification Methodology

These vary significantly in source accessibility:

### Freely verifiable
- **Zero Trust (NIST SP 800-207)** — verify against nist.gov publication
- **CISA ZTMM** — verify against cisa.gov published maturity model
- **Google SAIF** — verify against Google's published SAIF framework page

### Partially verifiable
- **NATO FMN** — limited public documentation; verify what's available
- **XAI DataOps** — verify against published whitepapers/documentation
- **Data-Centric Security** — conceptual compilation; verify source attributions

For each:
1. Locate the authoritative source
2. Extract structure
3. Compare against JSON
4. Document verification depth in proof file

## Deliverables

- [ ] 6 verified ontology JSON files (with appropriate verification status)
- [ ] 6 proof files documenting sources and verification depth
- [ ] `cargo test` passes
