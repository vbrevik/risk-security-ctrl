# Prompt Contract: claude-spec.md

## GOAL
Synthesize all requirements from the initial spec, research findings, and interview transcript into a complete specification for the Playbook data extraction feature.

## CONSTRAINTS
- Must incorporate requirements from all three input sources (spec.md, claude-research.md, claude-interview.md)
- Must not add implementation decisions beyond what was decided in the interview
- Must preserve the output JSON schema from the original spec
- Must reflect the Rust + plugin architecture decision from the interview

## FAILURE CONDITIONS
- SHALL NOT omit requirements from any input source
- SHALL NOT include architecture or implementation choices not supported by interview answers
- SHALL NOT contradict the original spec's quality criteria
