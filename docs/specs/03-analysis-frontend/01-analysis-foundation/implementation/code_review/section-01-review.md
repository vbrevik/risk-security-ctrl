# Code Review: Section 01 - Types and Hooks

The implementation is largely faithful to the plan, with types, hooks, query keys, and barrel export all matching the spec. However, there are several issues ranging from missing test coverage to a subtle runtime bug.

## High Severity

1. **useAnalyses page=0 silently dropped (api/index.ts):** The condition `if (params?.page)` is falsy when `page === 0`. While page 0 may be unlikely in a 1-indexed API, this is a latent bug if the API ever accepts 0-indexed pages, or if `limit` is set to 0 for some reason. The same applies to `params?.limit` and `params?.priority` in useFindings. The plan says 'skip undefined values', but these checks skip all falsy values (0 included). Use `params?.page != null` instead.

2. **Upload progress test does not actually verify progress was set to 50:** The test titled 'returns progress percentage during upload' never observes the progress at 50. It only checks `progress === 0` after settled. The onUploadProgress callback fires synchronously inside the mock before the promise resolves, and by the time the test asserts, onSettled has already reset it. This test proves nothing about progress tracking during upload -- it only proves the reset works. The plan explicitly asks to 'verify the returned progress value is 50'.

## Medium Severity

3. **Missing test: 'invalidates analysis cache on success' for useCreateAnalysis, useDeleteAnalysis, and useUploadAnalysis.**

4. **Missing test: 'invalidates prompt-template cache on success' for useUpdatePromptTemplate.**

5. **Missing test: 'resets progress on settled (both success and error)' for useUploadAnalysis.**

6. **Missing tests for refetchInterval behavior in useAnalyses.**

7. **Missing test for useFindings hook.**

8. **Missing test for exportAnalysis utility function.**

## Low Severity

9. **usePromptTemplate test combines endpoint and shape validation into single test.**

10. **analysisKeys test uses `toContain` instead of `toEqual` for full key verification.**

## Summary

The production code (types and hooks) is well-implemented and matches the plan closely. The main deficiency is in test coverage: roughly 8-10 test cases specified in the plan are either missing entirely or testing the wrong thing.
