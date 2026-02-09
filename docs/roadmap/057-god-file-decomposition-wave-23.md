# Phase 057: God-File Decomposition (Wave 23)

## Goal

Continue reducing dense mode-action and server validation modules by splitting command handlers and graph-validation checks into focused submodules.

## Scope

Primary Wave 23 targets:
- `src/tui_shell/app/cmd_mode_actions/superpositions.rs` (~203 LOC)
- `src/bin/converge_server/gate_graph_validation.rs` (~202 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

### B) Superpositions Mode Actions Decomposition
- [x] Split `src/tui_shell/app/cmd_mode_actions/superpositions.rs` by navigation/validation/apply concerns.
- [x] Preserve usage checks, resolution application, and optional publish behavior.

### C) Gate Graph Validation Decomposition
- [x] Split `src/bin/converge_server/gate_graph_validation.rs` by structural checks and DFS/reachability checks.
- [x] Preserve issue codes/messages and validation ordering semantics.

### D) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- 2026-02-09: `cargo fmt` passed.
- 2026-02-09: `cargo clippy --all-targets -- -D warnings` passed.
- 2026-02-09: `cargo nextest run` compiled and then stalled in this environment after build completion; fallback `cargo test --lib` passed (15 passed, 0 failed).
