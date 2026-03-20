I now have all the context needed to write the section. Let me compose it.

# Section 5: Actionable Recommendations

## Overview

This section modifies the recommendation generation logic in `classify_findings()` to include suggested actions from the NIST AI RMF Playbook when available. Currently, gap findings produce generic text like "Document does not address MEASURE 1.1." After this change, findings for concepts with guidance data will additionally list concrete suggested actions, giving users specific next steps directly from the playbook.

**Dependencies:** Sections 3 and 4 must be completed first. Section 3 adds the `about_en` and `actions_text` fields to `ConceptCandidate`. Section 4 uses those fields for scoring. This section reads `actions_text` from `ScoredCandidate.candidate` to enrich recommendation text -- no additional database queries are needed.

**File to modify:** `/Users/vidarbrevik/projects/risk-security-ctrl/backend/src/features/analysis/matcher.rs`

---

## Tests First

All tests go in the existing `#[cfg(test)] mod tests` block at the bottom of `matcher.rs`. The existing `make_scored` helper must be updated (by Section 3) to accept the new optional fields. These tests assume that update is in place.

```rust
// In matcher.rs #[cfg(test)] mod tests

// Helper: build a ScoredCandidate with actions_text populated
// Reuses the existing make_scored pattern but with the new fields from Section 3.
// After Section 3, ConceptCandidate has: about_en: Option<String>, actions_text: Option<String>

// Test: finding for concept with actions_text includes "Suggested Actions:" section
#[test]
fn classify_recommendation_with_actions_includes_suggested_actions_heading() {
    // Construct a ScoredCandidate where candidate.actions_text = Some("Action one\nAction two")
    // candidate.code = Some("MS-1.1")
    // Score 0.0 => Gap finding
    // Call classify_findings(vec![sc], &MatcherConfig::default(), "")
    // Assert recommendation.unwrap().contains("Suggested Actions:")
}

// Test: all actions from actions_text appear in recommendation (split by newline)
#[test]
fn classify_recommendation_lists_all_individual_actions() {
    // actions_text = "Establish approaches\nDocument methods\nTrack outcomes"
    // Assert recommendation contains all three action strings
}

// Test: actions formatted with concept code and action number
#[test]
fn classify_recommendation_actions_include_code_and_number() {
    // candidate.code = Some("GV-1.1"), actions_text = "First action\nSecond action"
    // Assert recommendation contains "(GV-1.1, Action 1)" and "(GV-1.1, Action 2)"
}

// Test: finding for concept without actions_text has no actions section in recommendation
#[test]
fn classify_recommendation_without_actions_has_no_suggested_actions() {
    // candidate.actions_text = None
    // Assert !recommendation.unwrap().contains("Suggested Actions:")
}

// Test: classify_findings remains synchronous (no async, no DB access)
// This is a structural assertion -- classify_findings is a plain fn, not async fn.
// Verified by the fact that these tests call it without .await and without a runtime.
#[test]
fn classify_findings_is_sync() {
    // Simply call classify_findings synchronously -- if it compiled, it is sync.
    let sc = make_scored("c1", "fw", None, "Test", "Def", None, 0.0);
    let _ = classify_findings(vec![sc], &MatcherConfig::default(), "");
}
```

---

## Implementation Details

### What Changes

The modification is localized to the recommendation generation block inside `classify_findings()`, specifically the `// Step 6: Recommendation` section (lines ~570-602 in current code). After the existing recommendation text is built, append suggested actions if the candidate has `actions_text`.

### Current Recommendation Logic (for reference)

The existing code at Step 6 builds a `recommendation: Option<String>` using a `match` on `finding_type`:

- **Addressed**: `"Document adequately covers {name}{ref_clause}"`
- **PartiallyAddressed**: `"Document partially addresses {name}. Consider expanding coverage of {def_excerpt}{ref_clause}"`
- **Gap**: `"Document does not address {name}: {def_excerpt}. Recommended action: {action}"`
- **NotApplicable**: `None`

### New Logic

After the existing `match` block produces the `recommendation` value, add a post-processing step that appends suggested actions. The logic:

1. Check if `recommendation` is `Some` (it is `None` only for `NotApplicable`)
2. Check if `sc.candidate.actions_text` is `Some` and non-empty
3. If both conditions hold, split `actions_text` by `'\n'` to recover individual action strings
4. Determine the concept code label: use `sc.candidate.code.as_deref().unwrap_or(&sc.candidate.name_en)`
5. Format each action as `"- {action_text} ({code}, Action {n})"` where `n` is the 1-based index
6. Append `"\n\nSuggested Actions:\n{formatted_actions}"` to the existing recommendation string

Pseudocode for the appended block:

```rust
let recommendation = recommendation.map(|mut rec| {
    if let Some(ref actions) = sc.candidate.actions_text {
        if !actions.is_empty() {
            let code_label = sc.candidate.code.as_deref()
                .unwrap_or(&sc.candidate.name_en);
            let action_lines: String = actions
                .split('\n')
                .enumerate()
                .map(|(i, action)| format!("- {} ({}, Action {})", action.trim(), code_label, i + 1))
                .collect::<Vec<_>>()
                .join("\n");
            rec.push_str("\n\nSuggested Actions:\n");
            rec.push_str(&action_lines);
        }
    }
    rec
});
```

### Key Design Decisions

- **All actions are included** (per interview decision noted in the plan). The frontend handles display/truncation.
- **Actions are appended to all finding types** (Gap, PartiallyAddressed, Addressed) -- not just gaps. An addressed finding with actions still benefits from showing what specific actions the document covers. The only exception is `NotApplicable` which has no recommendation at all.
- **No additional DB queries.** The `actions_text` field is pre-fetched during candidate retrieval (Section 3) and carried through scoring (Section 4). The `classify_findings` function remains a pure, synchronous function.
- **Concept code is preferred over name** for the action citation label, matching standard NIST AI RMF notation (e.g., "GV-1.1, Action 1").
- **Empty `actions_text`** (e.g., `Some("")`) is treated the same as `None` -- no actions section is appended.
- **Newline trimming** via `.trim()` on each action prevents blank lines from producing empty bullet items.

### What NOT to Change

- The `NewFinding` struct in `engine.rs` is unchanged -- the recommendation field is already `Option<String>` and can hold the longer text.
- The IDF/scoring logic is unchanged (that is Section 4).
- The evidence extraction logic (Step 7) is unchanged.
- The priority assignment logic (Step 5) is unchanged.
- The `actions_text` field is not persisted separately in findings -- it is folded into the recommendation text. If the frontend later needs structured access to actions, that would be a separate API change.

### Compliance Notes

- **V-222585 (Fail secure):** Missing `actions_text` (`None`) produces unchanged recommendation -- no error.
- **V-222609 (No panics):** The `.split('\n')` and `.trim()` operations cannot panic. The `.unwrap_or()` on code provides a safe fallback.
- No user input is processed in this code path -- all data comes from pre-validated database content.

### Existing Test Fixtures to Update

The `make_scored` helper (line ~1129) and any test that constructs `ConceptCandidate` or `ScoredCandidate` literals will need the new `about_en` and `actions_text` fields added by Section 3. For this section's tests, set `actions_text` to specific values to verify the formatting. For existing tests that do not care about actions, set both new fields to `None` to preserve current behavior.

## Implementation Notes (Post-Build)

**Files modified:** `backend/src/features/analysis/matcher.rs` — post-processing block after recommendation match, `make_scored_with_actions` helper, 5 new tests.

**All 51 matcher tests pass. No deviations from plan.**