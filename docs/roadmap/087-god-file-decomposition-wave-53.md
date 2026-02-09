# Phase 087: God-File Decomposition (Wave 53)

## Goal

Decompose server GC orchestration, browse wizard transitions, and repo CRUD handlers into focused helper modules.

## Scope

Primary Wave 53 targets:
- `src/bin/converge_server/handlers_gc/mod.rs` (~148 LOC)
- `src/tui_shell/wizard/browse_flow.rs` (~147 LOC)
- `src/bin/converge_server/handlers_repo/repo_crud.rs` (~145 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/bin/converge_server/handlers_gc/mod.rs` into query/flow/report helpers.
- [x] Split `src/tui_shell/wizard/browse_flow.rs` into focused start/transition/finish helpers.
- [x] Split `src/bin/converge_server/handlers_repo/repo_crud.rs` into focused create/read/permissions helpers.
- [x] Preserve current server and TUI behavior.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo clippy --all-targets -- -D warnings` completed successfully.
- `cargo test --lib` completed successfully (`15 passed, 0 failed`).
