# Integration Notes: Opus Review Feedback

## Integrating

1. **Reconcile Sections 3 and 5** — Section 5 will use `actions_text` from ConceptCandidate instead of a separate DB query. This avoids making `classify_findings` async.

2. **Gap candidates skip guidance fields** — The gap candidate query (stage 3) won't LEFT JOIN guidance tables. Only FTS and exact-match candidates get guidance fields.

3. **Fix GROUP_CONCAT ordering** — Use nested subquery pattern for guaranteed order.

4. **Add missing test case** — Concept with guidance row but empty child tables.

5. **tokio::join! for parallel queries** — Section 2 will run the four guidance queries concurrently.

6. **Mention existing fixture updates** — Section 3 testing strategy notes that existing `ConceptCandidate` test literals need updating.

## NOT Integrating

1. **Clarify FTS dedup strategy** — The reviewer noted BM25 scores aren't comparable across tables. I'll keep first-occurrence dedup (existing HashSet pattern) since the purpose of the union is broader recall, not score comparison. The TF-IDF re-scoring in Section 4 handles the final ranking. Added a note to the plan.

2. **Scoring bias concern** — Acknowledged as acceptable behavior. NIST AI RMF concepts have more specific vocabulary, so higher scores are correct. Not adding normalization caps.

3. **serde(rename) bidirectional concern** — Only serialization is needed; deserialization won't happen. Not changing.

4. **Performance benchmark** — Out of scope for this plan. SQLite is local; 6 queries on indexed columns is sub-millisecond.
