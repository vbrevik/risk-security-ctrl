# Split 01: EU Legislation Verification

## Scope

Verify 5 EU legislative frameworks against EUR-Lex authoritative sources.

| Framework | File | Concepts | EUR-Lex Reference |
|-----------|------|----------|-------------------|
| CER Directive | cer-directive.json | 42 | Directive (EU) 2022/2557 |
| DORA | dora.json | 32 | Regulation (EU) 2022/2554 |
| EU AI Act | eu-ai-act.json | 50 | Regulation (EU) 2024/1689 |
| GDPR | gdpr.json | 60 | Regulation (EU) 2016/679 |
| NIS2 | nis2.json | 52 | Directive (EU) 2022/2555 |

## Verification Methodology

For each framework:

1. **Fetch official text** from EUR-Lex (https://eur-lex.europa.eu/)
2. **Extract structure** — chapters, articles, sections, recitals as applicable
3. **Compare** against ontology JSON file:
   - Verify chapter/article numbering matches
   - Verify concept names match official article titles
   - Check for fabricated or missing entries
   - Verify version/date metadata
4. **Document findings** in proof file
5. **If errors found:** Rebuild JSON from verified source
6. **If correct:** Mark as verified

## Proof File Format

Each proof file in `docs/sources/` follows the established template:
```
# {Framework Name} — Verified Structure

**Source:** {EUR-Lex URL}
**Reference:** {Directive/Regulation number}
**Enacted:** {date}
**Extracted:** {date}

## {Chapter/Title structure}
- § / Art. {number}. {title}
...
```

## Deliverables

- [ ] 5 verified/rebuilt ontology JSON files
- [ ] 5 proof files in `docs/sources/`
- [ ] Verification status annotation for each framework
- [ ] `cargo test` passes after any rebuilds

## Verification Status Values

- `verified` — Full structure matches authoritative source
- `corrected` — Errors found and fixed from authoritative source
- `partially-verified` — Some aspects verified, others inaccessible
- `unverified` — Not yet checked against source

## Pass/Fail Criteria

A framework **passes** if:
- All chapter/article numbers match the official text
- All concept names correspond to actual article titles or section headings
- No fabricated entries exist
- Version metadata is correct

A framework **fails** if any of the above are violated, triggering a rebuild.
