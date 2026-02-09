# Phase 077: God-File Decomposition (Wave 43)

## Goal

Decompose remote identity lane/member operations and TUI key dispatch handling into focused modules.

## Scope

Primary Wave 43 targets:
- `src/remote/identity/members_lanes.rs` (~169 LOC)
- `src/tui_shell/app/event_loop/key_dispatch.rs` (~168 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/remote/identity/members_lanes.rs` into focused repo-member, lane-member, and lane-head operation helpers.
- [x] Split `src/tui_shell/app/event_loop/key_dispatch.rs` into focused text-edit/nav/mode-shortcut handlers.
- [x] Preserve request URLs, auth/error behavior, and keybinding behavior.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo clippy --all-targets -- -D warnings` completed successfully.
- `cargo nextest run -E 'kind(lib)'` completed successfully (`15 passed, 0 failed`).
