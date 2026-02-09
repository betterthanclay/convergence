# Phase 076: God-File Decomposition (Wave 42)

## Goal

Decompose command dispatch and tree diff modules into focused helpers while preserving command and diff behavior.

## Scope

Primary Wave 42 targets:
- `src/tui_shell/app/cmd_dispatch/mod.rs` (~173 LOC)
- `src/diff.rs` (~171 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/tui_shell/app/cmd_dispatch/mod.rs` into suggestion, input-run, and global-dispatch helpers.
- [x] Split `src/diff.rs` into focused signatures/tree/diff helpers.
- [x] Preserve command alias/prefix dispatch behavior and diff output ordering.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo check --lib` completed successfully.
- `cargo clippy --all-targets -- -D warnings` completed successfully.
- `cargo nextest run -E 'kind(lib)'` completed successfully (`15 passed, 0 failed`).
