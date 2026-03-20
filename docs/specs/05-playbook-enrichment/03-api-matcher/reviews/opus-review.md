# Opus Plan Review: 03-api-matcher

## Recommended Changes

1. **Reconcile Sections 3 and 5** — use `actions_text` from ConceptCandidate in recommendations instead of a separate DB query in synchronous `classify_findings`
2. **Clarify FTS dedup strategy** — spec says "keep best score" but BM25 scores from different FTS tables aren't directly comparable; document why first-occurrence is pragmatic
3. **Gap candidates: skip guidance fields** — don't LEFT JOIN guidance for gap candidates (hundreds of rows, zero scoring benefit)
4. **Fix GROUP_CONCAT ordering** — use nested subquery for guaranteed sort order
5. **Add test: guidance row with empty children** (zero actions/questions/references)
6. **Mention tokio::join!** for parallel guidance queries in handler
7. **Update existing ConceptCandidate test fixtures** for new optional fields

## Architecture Concerns

- N+1 query pattern in handler (6 sequential queries) — mitigate with tokio::join! for independence
- Scoring bias toward guidance-rich concepts — acknowledged as acceptable (richer vocabulary = correct behavior)
- classify_findings is sync — cannot add DB queries without refactor; use pre-fetched actions_text instead

## Security

All STIG controls correctly identified. No new attack surface.
