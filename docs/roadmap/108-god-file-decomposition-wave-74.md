# Phase 108: God-File Decomposition (Wave 74)

## Goal

Decompose remote gate-graph CLI handling, identity token/user CLI handling, and release-view rendering helpers into focused modules.

## Scope

Primary Wave 74 targets:
- `src/cli_exec/remote_admin/gate_graph.rs` (~106 LOC)
- `src/cli_exec/identity/token_user.rs` (~106 LOC)
- `src/tui_shell/views/releases.rs` (~109 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/cli_exec/remote_admin/gate_graph.rs` into show/set/init modules.
- [x] Split `src/cli_exec/identity/token_user.rs` into token and user handlers.
- [x] Extract release view row/detail rendering helpers into a dedicated module.
- [x] Preserve CLI output and TUI behavior semantics.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo test --lib` currently stalls in this environment after starting the lib test binary.
- Fallback targeted lib test passed:
  `cargo test --lib remote::operations::repo_gate::validation::tests::format_validation_error_without_issues_returns_top_level_error`.
- `cargo nextest run` still stalls in this environment after build/start.
