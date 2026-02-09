# Phase 074: God-File Decomposition (Wave 40)

## Goal

Decompose remote fetch parser/execution modules into focused components while preserving command behavior and output.

## Scope

Primary Wave 40 targets:
- `src/tui_shell/app/remote_fetch_parse.rs` (~174 LOC)
- `src/tui_shell/app/remote_fetch_exec.rs` (~172 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Fetch Decomposition
- [x] Split `src/tui_shell/app/remote_fetch_parse.rs` into focused parse/validation helpers.
- [x] Split `src/tui_shell/app/remote_fetch_exec.rs` into focused bundle/release/snap fetch handlers.
- [x] Preserve fetch CLI parsing compatibility and fetch output/error behavior.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [ ] Run `cargo clippy --all-targets -- -D warnings`.
- [ ] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo check --lib`, `cargo test -q --lib`, and targeted fetch parser test runs were attempted but blocked by repeated cargo lock/stall behavior in this environment (long-running idle cargo process with lockfiles and no active rustc child).
- `cargo clippy --all-targets -- -D warnings` and `cargo nextest run` remain pending due the same environment stall pattern.
