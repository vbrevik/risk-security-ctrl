# Section 11 Code Review: Prompt Template Endpoints

## Critical
No critical issues found.

## Important

### 1. GET silently masks corrupt config file (82% confidence)
- `unwrap_or_else` returns defaults when config file exists but has invalid JSON. Client can't tell "no config set" from "config corrupted."
- Fix: Return `AppError::Internal` for deserialization failure instead of silent fallback.

### 2. Orphaned `.tmp` file on rename failure (83% confidence)
- No cleanup of `.tmp` if rename fails. Persistent rename failures leave every PUT writing to `.tmp` and returning 500.
- Fix: Best-effort `std::fs::remove_file(&tmp_path)` in rename error path.

## No Issues Found
- Path traversal: PROMPT_TEMPLATE_PATH is a compile-time constant
- Relative path: consistent with existing "uploads" pattern
- Atomic write: tmp + rename on same filesystem is POSIX-safe
- Double deserialization in PUT: intentional for consistent error format
- Audit log user_id = NULL: consistent with export handler
- serde(default) accepting partial input: documented MatcherConfig behavior
