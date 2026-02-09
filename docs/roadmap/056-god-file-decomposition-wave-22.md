# Phase 056: God-File Decomposition (Wave 22)

## Goal

Continue decomposing dense settings and wizard orchestration code into focused modules while preserving command semantics and wizard flows.

## Scope

Primary Wave 22 targets:
- `src/tui_shell/app/settings_retention.rs` (~187 LOC)
- `src/tui_shell/wizard/release_ops_flow.rs` (~206 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

### B) Retention Settings Decomposition
- [x] Split `src/tui_shell/app/settings_retention.rs` by show/set/reset and pin/unpin concerns.
- [x] Preserve usage strings, parse behavior, and config persistence semantics.

### C) Release Ops Wizard Decomposition
- [x] Split `src/tui_shell/wizard/release_ops_flow.rs` by release/pin/promote flow concerns.
- [x] Preserve modal prompts, validation behavior, and side effects.

### D) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

Verification notes:
- `cargo fmt` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo nextest run` stalled in this environment; fallback `cargo test --lib` passed (`15 passed, 0 failed`).
