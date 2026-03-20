# Section 04 Contract: TF-IDF Scoring Enhancement

## GOAL
Include about_en and actions_text in keyword extraction for TF-IDF scoring.

## CONSTRAINTS
- Change only the candidate_keywords text concatenation in score_candidates()
- unwrap_or("") for None guidance fields (STIG V-222609)
- No new SQL queries
- Existing tests must pass unchanged

## FAILURE CONDITIONS
- SHALL NOT change scoring formula, IDF, or normalization
- SHALL NOT break existing scoring tests
- SHALL NOT skip tests
