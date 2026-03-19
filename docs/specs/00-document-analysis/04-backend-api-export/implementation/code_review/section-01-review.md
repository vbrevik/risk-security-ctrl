# Section 01 Code Review

## Findings

1. **DUPLICATED TOPIC-LOADING LOGIC (Medium)** - Same ~25-line block copy-pasted in main.rs and tests/common/mod.rs. Should extract to shared function.

2. **RELATIVE PATH FRAGILITY (Medium)** - Both files hardcode `../ontology-data/topic-tags.json`. Silently loads empty if CWD differs.

3. **SILENT DATA LOSS VIA filter_map (Low-Medium)** - Malformed topic entries silently dropped without logging.

4. **MISSING DEDICATED TEST (Low)** - Plan specifies `test_topics_loaded_in_appstate` test stub, not implemented.

5. **NO genpdf/image VERSION ALIGNMENT CHECK (Low)** - genpdf 0.2 depends on image 0.24.x; adding image 0.25 may cause duplicate crate versions.

6. **NO ANALYSIS ROUTE REGISTERED (Informational)** - Expected in later section.
