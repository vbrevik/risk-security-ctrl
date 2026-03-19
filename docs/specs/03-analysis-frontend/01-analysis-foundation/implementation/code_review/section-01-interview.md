# Code Review Interview: Section 01 - Types and Hooks

**Date:** 2026-03-19

## Auto-fixes

### Fix 1: params falsy check bug (High severity)
`if (params?.page)` skips page=0. Change to `params?.page != null` in useAnalyses, useFindings.

### Fix 2: analysisKeys test tightened (Low severity)
Replace `toContain` with `toEqual` for full hierarchical key verification.

## User Decision

### Missing test coverage
**Decision:** Add all ~8 missing test cases from the plan specification.

Tests to add:
- Cache invalidation tests for useCreateAnalysis, useDeleteAnalysis, useUploadAnalysis, useUpdatePromptTemplate
- refetchInterval behavior tests for useAnalyses (processing/non-processing)
- Upload progress mid-flight observation test
- Upload progress reset on error test
- useFindings basic test
- exportAnalysis utility test
