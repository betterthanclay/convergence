# Phase 049: God-File Decomposition (Wave 15)

## Goal

Continue reducing dense TUI status and input modules while preserving behavior, command flow, and output fidelity.

## Scope

Primary Wave 15 targets:
- `src/tui_shell/status/rename_match.rs` (~266 LOC)
- `src/tui_shell/app/cmd_text_input.rs` (~266 LOC)
- `src/tui_shell/status/remote_status.rs` (~255 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

Progress notes:
- Start with `rename_match.rs` and `cmd_text_input.rs` in this batch.
- Follow with `remote_status.rs` next.

### B) Rename Match Decomposition
- [x] Split `src/tui_shell/status/rename_match.rs` by candidate matching and similarity helpers.
- [x] Preserve rename detection output.

### C) Text Input Command Decomposition
- [x] Split `src/tui_shell/app/cmd_text_input.rs` by modal-action concern.
- [x] Preserve modal submit/cancel behavior and command dispatch side effects.

### D) Remote Status Decomposition
- [x] Split `src/tui_shell/status/remote_status.rs` by data-collection and formatting concerns.
- [x] Preserve remote dashboard/status output and ordering.

### E) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

Progress notes:
- `rename_match` now uses focused submodules: `exact`, `blob_edits`, and `recipe_edits`.
- `cmd_text_input` now routes across focused submodules: `settings_actions`, `direct_commands`, and `wizard_routes`.
- `remote_status` now uses focused submodules: `lines`, `dashboard`, and shared `health` checks.
- Verification:
  - `cargo fmt` passed.
  - `cargo clippy --all-targets -- -D warnings` passed.
  - `cargo nextest run` compiled but stalled in this environment; fallback `cargo test --lib` passed (`15 passed, 0 failed`).
