# Opus Review Summary

## Critical: 3 issues
- CRITICAL-1: docx-rs not in Cargo.toml (split 02 used zip+quick-xml for parsing, not generation)
- CRITICAL-2: Plan types diverge from existing models.rs types (AnalysisListQuery, FindingsQuery, AnalysisSummary)
- CRITICAL-3: DocumentParser blocking I/O called in async handler — needs spawn_blocking

## High: 5 issues
- HIGH-1: Prompt template file writes — race condition, non-writable in containers
- HIGH-2: File upload path traversal — use UUID filename instead
- HIGH-3: No max length on input_text
- HIGH-4: No timeout on synchronous processing
- HIGH-5: No file cleanup for failed/deleted analyses

## Medium: 6 issues
- MED-1: Chart dependency bloat
- MED-2: Font files not in repo, no fallback
- MED-3: genpdf 0.2 + image version compat
- MED-4: SQL injection in ORDER BY columns
- MED-5: Topic loading strategy ambiguous
- MED-6: sort_order field missing from NewFinding

## Low: 5 issues, 3 spec gaps
