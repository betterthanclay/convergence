# Phase 095: God-File Decomposition (Wave 61)

## Goal

Decompose remote auth/config command handlers and chunking settings commands into focused helper modules.

## Scope

Primary Wave 61 targets:
- `src/tui_shell/app/cmd_remote/auth_cmds.rs` (~123 LOC)
- `src/tui_shell/app/cmd_remote/config_cmds.rs` (~120 LOC)
- `src/tui_shell/app/settings_chunking.rs` (~122 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/tui_shell/app/cmd_remote/auth_cmds.rs` into focused login and lifecycle handlers.
- [x] Split `src/tui_shell/app/cmd_remote/config_cmds.rs` into focused set/unset handlers.
- [x] Split `src/tui_shell/app/settings_chunking.rs` into focused show/set/reset handlers.
- [x] Preserve current command behavior and argument validation semantics.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo nextest run` compiled tests but stalled in this environment; fallback `cargo test --lib` passed (`15 passed, 0 failed`).
