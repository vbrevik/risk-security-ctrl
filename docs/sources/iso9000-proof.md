# ISO 9000:2015 — Verification Proof

**Source:** https://www.iso.org/standard/45481.html
**Reference:** ISO 9000:2015 Quality management systems — Fundamentals and vocabulary
**Verified:** 2026-03-28
**Status:** partially-verified

## How to Verify

1. **ISO Catalogue Page:** https://www.iso.org/standard/45481.html
2. **ISO 9001:2015 Catalogue:** https://www.iso.org/standard/62085.html
3. **ISO Quality Management Principles (free PDF):** https://www.iso.org/publication/PUB100080.html — all 7 QMPs
4. **ISO 9000 family overview:** https://www.iso.org/iso-9001-quality-management.html

### Key Verification Points
- 7 Quality Management Principles (Clause 2.3): Customer focus, Leadership, Engagement of people, Process approach, Improvement, Evidence-based decision making, Relationship management
- ISO 9000:2015 is vocabulary/fundamentals; ISO 9001:2015 is requirements (distinct standards)

## Verification Results

### Confirmed Correct
- **Framework metadata:** Name, version (2015), source_url all correct
- **7 QMPs (all 7/7):** All correctly identified with accurate clause references (2.3.1-2.3.7)
- **Principle definitions:** Accurate paraphrases of official ISO text
- **QMS definition:** Correctly references Clause 3.5.4

### Issues Found — Corrections Applied
1. **Norwegian typo:** `name_nb` for performance evaluation was "Ytelsesevvaluering" (double v). **Corrected** to "Ytelsesevaluering".
2. **QMP7 wording:** Used "suppliers" but ISO 9000:2015 prefers "providers" (term 3.2.5). **Corrected** definition_en to use "providers".

### Issues Documented (Not Corrected — Design Decision)
3. **ISO 9001 clauses in ISO 9000 file:** Clauses 4-10 (Context, Leadership, Planning, Support, Operation, Performance evaluation, Improvement) are from ISO 9001:2015, not ISO 9000:2015. The source_reference fields correctly say "ISO 9001:2015 Clause X". This appears to be an intentional bundling of the 9000-family content.

### Concept Count
16 concepts — 7 principles + 7 ISO 9001 requirement clauses + 2 categories. All legitimate, zero fabricated.
