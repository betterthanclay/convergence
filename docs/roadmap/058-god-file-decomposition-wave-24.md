# Phase 058: God-File Decomposition (Wave 24)

## Goal

Continue reducing dense runtime and TUI event-loop modules by splitting startup/bootstrap logic and key-handling dispatch into focused submodules.

## Scope

Primary Wave 24 targets:
- `src/bin/converge_server/runtime.rs` (~201 LOC)
- `src/tui_shell/app/event_loop.rs` (~198 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

### B) Server Runtime Decomposition
- [x] Split `src/bin/converge_server/runtime.rs` by startup orchestration and identity bootstrap/shutdown helpers.
- [x] Preserve CLI argument behavior, bootstrap token semantics, and persisted identity behavior.

### C) TUI Event Loop Decomposition
- [x] Split `src/tui_shell/app/event_loop.rs` by loop/render flow and key-handling dispatch.
- [x] Preserve modal handling, navigation shortcuts, command input behavior, and default actions.

### D) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- 2026-02-09: `cargo fmt` passed.
- 2026-02-09: `cargo clippy --all-targets -- -D warnings` passed.
- 2026-02-09: `cargo nextest run` compiled and then stalled in this environment after build completion; fallback `cargo test --lib` passed (15 passed, 0 failed).
