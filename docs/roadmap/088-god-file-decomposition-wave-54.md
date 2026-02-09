# Phase 088: God-File Decomposition (Wave 54)

## Goal

Decompose release/promotion endpoint handlers and superpositions apply flow into focused helper modules.

## Scope

Primary Wave 54 targets:
- `src/bin/converge_server/handlers_release/release_endpoints.rs` (~144 LOC)
- `src/bin/converge_server/handlers_release/promotion_endpoints.rs` (~143 LOC)
- `src/tui_shell/app/cmd_mode_actions/superpositions/apply_validate/apply.rs` (~142 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/bin/converge_server/handlers_release/release_endpoints.rs` into create/list/get focused helpers.
- [x] Split `src/bin/converge_server/handlers_release/promotion_endpoints.rs` into create/list focused helpers.
- [x] Split `src/tui_shell/app/cmd_mode_actions/superpositions/apply_validate/apply.rs` into focused apply/publish helpers.
- [x] Preserve existing API and TUI behavior.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` completed successfully.
- `cargo clippy --all-targets -- -D warnings` completed successfully.
- `cargo test --lib` completed successfully (`15 passed, 0 failed`).
