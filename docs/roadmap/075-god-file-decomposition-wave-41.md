# Phase 075: God-File Decomposition (Wave 41)

## Goal

Decompose workspace materialization and bootstrap transition logic into focused modules while preserving behavior.

## Scope

Primary Wave 41 targets:
- `src/workspace/materialize_fs.rs` (~174 LOC)
- `src/tui_shell/wizard/login_bootstrap_transitions/bootstrap.rs` (~174 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/workspace/materialize_fs.rs` into focused cleanup/materialization/platform helpers.
- [x] Split `src/tui_shell/wizard/login_bootstrap_transitions/bootstrap.rs` into focused prompt/transition helpers.
- [x] Preserve restore/materialize behavior and bootstrap modal flow text/defaults.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo check --lib` completed successfully.
- `cargo test -q --lib` passed (`15 passed, 0 failed`).
- `cargo clippy --all-targets -- -D warnings` completed successfully.
- `cargo nextest run` was attempted and stalled after build in this environment; fallback validation used the compile/library test checks above.
