# Section 11 Code Review Interview

## Finding 1: GET silently masks corrupt config (Auto-fix)
- **Action:** Auto-fixed
- **Change:** Changed `unwrap_or_else` to `map_err` returning `AppError::Internal` when config file exists but has invalid JSON
- **Rationale:** Silent fallback to defaults is indistinguishable from "no config set" for the client

## Finding 2: Orphaned `.tmp` on rename failure (Auto-fix)
- **Action:** Auto-fixed
- **Change:** Added `std::fs::remove_file(&tmp_path)` in the rename error path (best-effort)
- **Rationale:** Persistent rename failures would leave `.tmp` file and never update actual config
