# Prompt Contract: claude-plan.md

## GOAL
Deliver a self-contained prose blueprint for adding guidance data tables, FTS5 extension, and import pipeline to the existing Rust/SQLx backend.

## CONTEXT
This plan drives all downstream section files and implementation. An engineer with no prior context must understand what to build, why, and how.

## CONSTRAINTS
- Plans are prose documents with minimal code (type definitions, SQL DDL, directory structure only)
- Zero full function implementations
- Must follow existing codebase patterns (SQLx query macros, UUID generation, migration numbering)
- Additive changes only — no modifications to existing tables
- Rebuild-based FTS5 approach (not per-row triggers)
- Dynamic *-guidance.json scanning in import pipeline
- Transaction per guidance entry for atomicity
- ON CONFLICT DO UPDATE for concept_guidance, delete-and-reinsert for child rows

## FORMAT
Single file `claude-plan.md` with sections mapping to implementable units.

## FAILURE CONDITIONS
- SHALL NOT contain full function bodies
- SHALL NOT assume reader has prior context
- SHALL NOT omit testing strategy
- SHALL NOT add features beyond the spec
- SHALL NOT modify existing tables or import behavior

## STIG Constraints (auto-detected: input-validation, injection)

Minimal applicability — this is backend database code with no direct user input:

- **V-222607 (CAT I)**: All database queries must use parameterized statements (SQLx query macros with bind). No string concatenation for SQL.
- **V-222606 (CAT I)**: Validate concept_id foreign key references before inserting child rows. Reject invalid guidance entries gracefully.
