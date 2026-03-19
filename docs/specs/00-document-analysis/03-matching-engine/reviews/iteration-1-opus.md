# Opus Review - 03-matching-engine

## Key findings (11 items)

1. **FTS5 injection** - Need to sanitize FTS5 reserved words (NEAR, NOT, AND, OR) and quote terms
2. **Gap candidate performance** - Skip scoring for unmatched concepts, directly classify as Gap
3. **detect_frameworks contradictory data source** - Pick either passed-in or DB query
4. **Framework-to-topic mapping unspecified** - Need concrete query
5. **Matcher lifecycle** - Topics stored in struct, per-request construction
6. **Evidence extraction performance** - Pre-compute sentence keywords once
7. **Cap ordering bug** - Assign priorities before applying max_findings cap
8. **No test strategy** - Covered in TDD plan
9. **NotApplicable never produced** - Documented as deferred to LLM Phase 2
10. **Token count arbitrary** - Store word count, label as estimate
11. **Dynamic SQL IN clause** - Use json_each() approach (already in section-03)
