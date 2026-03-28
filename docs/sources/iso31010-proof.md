# IEC 31010:2019 — Verification Proof

**Source:** https://www.iso.org/standard/72140.html
**Reference:** IEC 31010:2019 Risk management — Risk assessment techniques
**Verified:** 2026-03-28
**Status:** corrected

## How to Verify

1. **ISO Catalogue Page:** https://www.iso.org/standard/72140.html
2. **IEC Webstore:** https://webstore.iec.ch/en/publication/59809
3. **Wikipedia:** https://en.wikipedia.org/wiki/ISO_31010 — lists techniques from the standard
4. **Full standard PDF (public copy):** https://certgroup.org/wp-content/uploads/2022/08/iso_31010_2019.pdf

### Key Verification Points
- Standard is officially **IEC 31010:2019** (transferred from ISO to IEC in this edition)
- Annex B describes **41 techniques** across 10 clause groups (B.1–B.10)
- Table A.1 shows technique applicability to risk assessment stages

## Official Annex B Structure (IEC 31010:2019)

| Clause | Group | Techniques |
|--------|-------|------------|
| B.1 | Eliciting views from stakeholders and experts | Brainstorming, Delphi, Nominal group, Structured interviews, Surveys |
| B.2 | Identifying risk | Checklists, FMEA/FMECA, HAZOP, Scenario analysis, SWIFT |
| B.3 | Determining sources, causes and drivers | Cindynic approach, Ishikawa analysis |
| B.4 | Analysing controls | Bow-tie, HACCP, LOPA |
| B.5 | Understanding consequences and likelihood | Bayesian analysis, Bayesian networks, BIA, Cause-consequence, ETA, FTA, HRA, Markov, Monte Carlo, PIA/DPIA |
| B.6 | Analysing dependencies and interactions | Causal mapping, Cross impact analysis |
| B.7 | Measuring risk | Toxicological risk assessment, VaR, CVaR/ES |
| B.8 | Evaluating significance | ALARP/SFAIRP, F-N diagrams, Pareto charts, RCM, Risk indices |
| B.9 | Selecting between options | CBA, Decision tree, Game theory, Multi-criteria analysis |
| B.10 | Recording and reporting | Risk registers, Risk matrix, S-curves |

## Verification Results

### Confirmed Correct
- **Framework metadata:** source_url correct, version 2019 correct
- **Core techniques present:** Brainstorming, Delphi, FMEA/FMECA, HAZOP, HACCP, LOPA, Bow-tie, Markov, Monte Carlo, Bayesian, FTA, ETA, SWIFT, Risk Matrix, HRA, Ishikawa — all correctly named and mapped to Annex B references
- All 35 techniques in the post-correction ontology are legitimate risk assessment methods

### Issues Found — All Resolved (2026-03-28)

1. **T07 Root Cause Analysis — removed from IEC 31010:2019.** RCA was present in ISO 31010:2009 but was not included as a standalone technique in the 2019 revision. It is now covered by IEC 62740:2015 (Root cause analysis). Updated T07 definition to document this change. Causal analysis in IEC 31010:2019 is distributed across Ishikawa (B.3.3), cause-consequence analysis (B.5.5), and causal mapping (B.6.1).

2. **18 techniques from Annex B missing.** Added the 7 most relevant for security risk management practice:
   - **Nominal group technique** (B.1.4) — structured expert elicitation
   - **Bayesian networks / influence diagrams** (B.5.3) — probabilistic risk modelling
   - **PIA/DPIA** (B.5.11) — privacy impact analysis, required under GDPR for high-risk processing
   - **Causal mapping** (B.6.1) — systemic dependencies and feedback loops
   - **ALARP / SFAIRP** (B.8.2) — tolerable risk criteria, foundational to safety and resilience frameworks
   - **F-N diagrams** (B.8.3) — cumulative frequency-consequence plots
   - **Risk indices** (B.8.6) — composite risk scoring and prioritisation

3. **Category groupings are editorial.** The original categories (Look-up methods, Supporting methods, etc.) do not match the official Annex B clause structure. Three new categories added with official B.X codes: B.3, B.5–B.6, B.8.

### Intentionally Excluded Techniques
Techniques from Annex B not added due to limited relevance for security/IT risk management:
- Surveys (B.1.6) — generic data collection, not risk-specific
- Cindynic approach (B.3.2) — French safety science framework, rarely used in security contexts
- Cross impact analysis (B.6.2) — primarily used in futures/foresight
- Toxicological risk assessment (B.7.1) — environmental/chemical domain
- Value at Risk (B.7.2), CVaR/ES (B.7.3) — financial domain
- Game theory (B.9.4) — theoretical, limited practical security use
- Risk registers (B.10.2) — a management tool, not a risk assessment technique
- Pareto charts (B.8.4), S-curves (B.10.4) — data visualisation tools
- Reliability centred maintenance (B.8.5) — physical asset maintenance domain

### Note on Framework Name
The standard is officially **IEC** 31010:2019 (transferred from ISO/IEC joint committee to IEC). Our file uses "ISO 31010:2019" which is cosmetically inaccurate but ISO still hosts the catalogue page. Not corrected.

### Concept Count
48 concepts — 35 techniques + 13 categories. All legitimate, zero fabricated.
(Was 38; added 7 techniques + 3 new category nodes.)
