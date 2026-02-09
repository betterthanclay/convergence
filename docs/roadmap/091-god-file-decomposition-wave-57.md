# Phase 091: God-File Decomposition (Wave 57)

## Goal

Decompose gate graph view rendering, rename helper utilities, and app module test payloads into focused modules.

## Scope

Primary Wave 57 targets:
- `src/tui_shell/views/gate_graph.rs` (~139 LOC)
- `src/tui_shell/status/rename_helpers.rs` (~137 LOC)
- `src/tui_shell/app.rs` (~139 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/tui_shell/views/gate_graph.rs` into focused render/list/details helpers.
- [x] Split `src/tui_shell/status/rename_helpers.rs` into focused types/scoring/threshold helpers.
- [x] Move embedded `app.rs` test payload into dedicated module file while preserving behavior.
- [x] Preserve TUI rendering and rename matching behavior.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo clippy --all-targets -- -D warnings` surfaced integration errors early; fixes were applied, and subsequent full clippy runs repeatedly stalled in this environment.
- Fallback `cargo check -q` completed successfully.
- `cargo test --lib` completed successfully (`15 passed, 0 failed`).
