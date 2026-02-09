# Phase 073: God-File Decomposition (Wave 39)

## Goal

Decompose member and publish wizard flows into focused transition/prompt/finish modules while preserving existing modal behavior.

## Scope

Primary Wave 39 targets:
- `src/tui_shell/wizard/member_flow/repo_member.rs` (~177 LOC)
- `src/tui_shell/wizard/publish_sync_flow/publish.rs` (~175 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Wizard Decomposition
- [x] Split `src/tui_shell/wizard/member_flow/repo_member.rs` into focused helpers for transitions/prompts and finish execution.
- [x] Split `src/tui_shell/wizard/publish_sync_flow/publish.rs` into focused helpers for prompts/transitions and command arg assembly.
- [x] Preserve modal text, defaults, and fallback/validation behavior across both flows.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [ ] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo check --lib` completed successfully.
- `cargo test -q --lib` passed (`15 passed, 0 failed`).
- `cargo clippy --all-targets -- -D warnings` was attempted but stalled with long-running `clippy-driver` processes in this environment.
- `cargo nextest run -E 'kind(lib)'` was attempted and stalled after compile in this environment; fallback validation used targeted library compile/tests listed above.
