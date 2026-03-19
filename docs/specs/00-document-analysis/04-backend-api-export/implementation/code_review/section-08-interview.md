# Section 08 Code Review Interview

## Finding #1: Hardcoded `./fonts` relative path
- **Decision:** Keep current `./fonts` approach
- **Rationale:** Works when CWD is `backend/`, which matches dev and expected Docker WORKDIR

## Finding #2: Charts overflow A4 without scaling
- **Decision:** Auto-fixed
- **Action:** Added `CHART_SCALE = 0.5` constant and `.with_scale()` in `embed_png`

## Finding #3: Byte len vs char count mismatch
- **Decision:** Auto-fixed
- **Action:** Changed `text.len() > 2000` to `text.chars().count() > 2000`

## Finding #4: Test searches raw PDF binary bytes
- **Decision:** Let go
- **Rationale:** Test passes, acceptable fragility for now

## Finding #5: Chart failures leave dangling heading
- **Decision:** Auto-fixed
- **Action:** Added `push_chart_fallback()` with "[Chart could not be rendered]" placeholder
