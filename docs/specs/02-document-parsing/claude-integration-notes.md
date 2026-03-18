# Integration Notes: Opus Review

## Integrating

1. **File size check** — Add `std::fs::metadata()` check at top of `parse()` before reading file. High priority.
2. **extract_text_by_pages verification** — Must verify API exists. Fallback: use `extract_text()` and produce single section.
3. **parse() vs parse_text() API** — Document that `parse()` is file-only, `parse_text()` is separate path. Split 04 calls the right one based on input_type.
4. **DocumentParser as unit struct** — Make it `pub struct DocumentParser;` with associated functions.
5. **DOCX w:t concatenation** — Concatenate without spaces within paragraph. Document headers/footers/tables as known limitations.
6. **spawn_blocking note** — Add to plan as split 04 concern. Parser functions stay sync.
7. **Tracing** — Add tracing::info/warn for parse start/complete/fail.

## Not Integrating

- **Test plan** — Will be covered in claude-plan-tdd.md (next step).
- **Norwegian stopwords** — Spec says English only for MVP. Documented.
- **Sentence splitting fragility** — Acceptable for MVP. Matching engine tolerates imperfect boundaries.
- **regex dependency** — Use manual string scanning, no regex crate.
- **Deserialize on ParsedDocument** — Response-only type, intentionally omitted.
- **Token count accuracy** — Documented as rough estimate.
