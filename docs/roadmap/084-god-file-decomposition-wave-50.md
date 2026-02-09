# Phase 084: God-File Decomposition (Wave 50)

## Goal

Decompose snap view rendering, repo/gate remote operations, and delivery CLI arg definitions into focused helper modules.

## Scope

Primary Wave 50 targets:
- `src/tui_shell/views/snaps/mod.rs` (~152 LOC)
- `src/remote/operations/repo_gate.rs` (~152 LOC)
- `src/cli_commands/delivery.rs` (~151 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/tui_shell/views/snaps/mod.rs` into focused state/selection/render helpers.
- [x] Split `src/remote/operations/repo_gate.rs` into focused repo/publication/gate-graph helpers.
- [x] Split `src/cli_commands/delivery.rs` into focused argument-group modules.
- [x] Preserve existing CLI, remote operation, and TUI behavior.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo clippy --all-targets -- -D warnings` completed successfully.
- `cargo nextest run` stalled in this environment after build; fallback `cargo nextest run -E 'kind(lib)'` completed successfully (`15 passed, 0 failed`).
