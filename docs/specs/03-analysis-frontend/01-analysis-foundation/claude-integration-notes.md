# Integration Notes: Opus Plan Review

## Integrating

### H1: Analysis type missing backend fields
**Integrating.** Add all backend fields to the `Analysis` type (input_text, original_filename, file_path, extracted_text, prompt_template, created_by) as optional/nullable. Split 02 will need extracted_text for the appendix display.

### H3: List endpoint returns subset of fields
**Integrating.** Define `AnalysisListItem` type matching the list endpoint's actual response (id, name, description, input_type, status, error_message, processing_time_ms, created_at, updated_at). Drop "frameworks count" from the card — it's not available in the list response.

### H4: matched_framework_ids returned as string
**Integrating.** Add client-side JSON.parse in `useAnalysis` hook's transform. This is a backend quirk that should be fixed later but we handle it in the frontend for now.

### M1: Upload progress not reset on error
**Integrating.** Use `onSettled` instead of `onSuccess` for progress reset.

### M3: Upload mutation input type unclear
**Integrating.** Define explicit `UploadAnalysisInput { file: File; name: string }` type.

### M5: Navigation placement imprecise
**Integrating.** Specify exact position: after the second separator, before Compliance link.

### L1: Description not supported in upload endpoint
**Integrating.** Hide description field when upload tab is active (backend doesn't accept it for uploads).

### L5: Tab state interaction underspecified
**Integrating.** Specify: preserve both tab states, only submit active tab's content.

## NOT Integrating

### H2: Finding type locale mismatch
**Not integrating.** The findings endpoint already returns flat `concept_name` and `concept_definition` (from `name_en` and `definition_en`). The frontend type matches the actual API response. If locale-aware names are needed later, that's a backend change + new frontend work.

### M2: CreateAnalysisRequest missing prompt_template
**Not integrating.** Intentionally omitted — the create form doesn't expose prompt template selection. This is an advanced feature.

### M4: sort_by field omitted
**Not integrating.** The findings endpoint's sort_by is used in split 02's table. Adding it to the type now is fine but the hook won't use it yet. Will add when split 02 is planned.

### M6: Reset defaults hardcoded
**Not integrating.** Duplication is acceptable. The defaults rarely change, and adding a backend "reset" endpoint adds scope. A comment referencing the Rust source is sufficient.

### L3: PaginatedResponse re-export chain
**Not integrating.** Following existing pattern. Refactoring shared types is tech debt for later.

### L4: No test plan
**Not integrating in the plan.** Tests will be covered in the TDD plan (next step).
