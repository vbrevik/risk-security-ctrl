# Section 03 Code Review

## Critical Fixes Applied
- H1: Char offsets from normalized text applied to raw text — re-find headers in raw text
- H2: Hardcoded CARGO_MANIFEST_DIR — moved ontology_path to ExtractionConfig
- M1: Transparency subsection match over-broad — tightened to "Transparency Questions"
- M2: Organizations can document not line-anchored — changed to starts_with
- M3: Hyphen rejoining ignoring trailing whitespace — trim before check

## Let Go
- M4: Test indentation masking design gap
- M5: Double space in normalization (regex compensates)
