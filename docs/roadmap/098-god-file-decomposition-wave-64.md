# Phase 098: God-File Decomposition (Wave 64)

## Goal

Decompose remote dashboard aggregation, login/bootstrap effects, and lane-member command handling into focused helper modules.

## Scope

Primary Wave 64 targets:
- `src/tui_shell/status/remote_status/dashboard.rs` (~129 LOC)
- `src/tui_shell/wizard/login_bootstrap_effects.rs` (~125 LOC)
- `src/tui_shell/app/remote_members/lane_member.rs` (~127 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/tui_shell/status/remote_status/dashboard.rs` into focused inbox/bundle/release/action helpers.
- [x] Split `src/tui_shell/wizard/login_bootstrap_effects.rs` into focused login/bootstrap/repo-ensure helpers.
- [x] Split `src/tui_shell/app/remote_members/lane_member.rs` into focused prompt-first and legacy-flag handlers.
- [x] Preserve current remote/wizard/member command behavior and output semantics.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo nextest run` built tests but stalled in this environment; fallback `cargo test --lib` passed (`15 passed, 0 failed`).
