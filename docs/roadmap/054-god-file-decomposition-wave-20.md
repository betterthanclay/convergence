# Phase 054: God-File Decomposition (Wave 20)

## Goal

Continue reducing high-LOC UI orchestration files by splitting root-view and frame-draw concerns into focused modules while preserving behavior.

## Scope

Primary Wave 20 targets:
- `src/tui_shell/views/root.rs` (~218 LOC)
- `src/tui_shell/app/render.rs` (~216 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

### B) Root View Decomposition
- [x] Split `src/tui_shell/views/root.rs` by state refresh, header/render, and line styling concerns.
- [x] Preserve local/remote rendering behavior and fallback error output.

### C) App Render Decomposition
- [x] Split `src/tui_shell/app/render.rs` by header/status/suggestions/input rendering concerns.
- [x] Preserve modal behavior, cursor placement, and hint collision logic.

### D) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

Verification notes:
- `cargo fmt` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo nextest run` stalled in this environment; fallback `cargo test --lib` passed (`15 passed, 0 failed`).
