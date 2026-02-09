# Phase 050: God-File Decomposition (Wave 16)

## Goal

Continue reducing dense CLI/TUI and server modules by extracting parsing, rendering, and retention concerns into focused submodules while preserving behavior.

## Scope

Primary Wave 16 targets:
- `src/tui_shell/app/remote_action_parse.rs` (~239 LOC)
- `src/tui_shell/views/settings.rs` (~249 LOC)
- `src/bin/converge_server/handlers_gc/mod.rs` (~247 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

Progress notes:
- Decompose parser surfaces first, then settings rendering concerns, then GC retention/root concern splits.

### B) Remote Action Parse Decomposition
- [x] Split `src/tui_shell/app/remote_action_parse.rs` into focused parser submodules.
- [x] Preserve accepted arg forms and validation errors for bundle/pin/approve/promote/release/superpositions commands.

### C) Settings View Decomposition
- [x] Split `src/tui_shell/views/settings.rs` by list-row rendering and detail rendering concerns.
- [x] Preserve settings UI row text, details text, navigation behavior, and highlight behavior.

### D) GC Handler Decomposition
- [x] Split `src/bin/converge_server/handlers_gc/mod.rs` by release-pruning and retention-root collection concerns.
- [x] Preserve GC dry-run/metadata pruning behavior and response summary fields.

### E) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

Verification notes:
- `cargo fmt` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo nextest run` stalled in this environment; fallback `cargo test --lib` passed (`15 passed, 0 failed`).
