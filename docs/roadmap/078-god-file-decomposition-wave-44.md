# Phase 078: God-File Decomposition (Wave 44)

## Goal

Decompose hint selection and move wizard flow logic into focused modules while preserving UI behavior.

## Scope

Primary Wave 44 targets:
- `src/tui_shell/app/default_actions/hints.rs` (~168 LOC)
- `src/tui_shell/wizard/move_flow.rs` (~166 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/tui_shell/app/default_actions/hints.rs` into focused hint-key/rotation and mode-specific command selection helpers.
- [x] Split `src/tui_shell/wizard/move_flow.rs` into focused source resolution, destination apply, and prompt builders.
- [x] Preserve default action hint ordering/rotation and move wizard prompt behavior.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo clippy --all-targets -- -D warnings` completed successfully.
- `cargo nextest run -E 'kind(lib)'` completed successfully (`15 passed, 0 failed`).
