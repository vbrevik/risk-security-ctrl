# Opus Review - 02-document-parsing

Key issues identified (see full review in agent output).

**High:** extract_text_by_pages may not exist, no file size check
**Medium:** No test plan, DOCX w:t concatenation, parse() can't reach parse_text(), no spawn_blocking note
**Low:** Sentence splitting fragility, English-only stopwords, DocumentParser type, no tracing
