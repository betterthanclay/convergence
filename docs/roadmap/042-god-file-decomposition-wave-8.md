# Phase 042: God-File Decomposition (Wave 8)

## Goal

Continue reducing remaining high-LOC files by decomposing handler and wizard hotspots into focused modules while preserving behavior.

## Scope

Primary Wave 8 targets (current snapshot):
- `src/bin/converge_server/handlers_objects.rs` (~322 LOC)
- `src/tui_shell/wizard/member_flow.rs` (~338 LOC)
- `src/tui_shell/wizard/publish_sync_flow.rs` (~322 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target ordering and decomposition boundaries.

Progress notes:
- Start with `handlers_objects.rs` (object-type handler split with low behavior risk).
- Then wizard flows (`member_flow`, `publish_sync_flow`) where transition/effect split boundaries mirror prior waves.

### B) Server Object Handler Decomposition
- [x] Split `src/bin/converge_server/handlers_objects.rs` into object-family modules.
- [x] Preserve route signatures and response behavior.
- [x] Keep shared query/types in a thin module root.

Progress notes:
- Replaced monolithic file with module directory:
  - `src/bin/converge_server/handlers_objects/mod.rs`
  - `src/bin/converge_server/handlers_objects/blob.rs`
  - `src/bin/converge_server/handlers_objects/manifest.rs`
  - `src/bin/converge_server/handlers_objects/recipe.rs`
  - `src/bin/converge_server/handlers_objects/snap.rs`
- Updated server entry composition to load handlers from `handlers_objects/mod.rs`.

### C) Wizard Flow Decomposition
- [ ] Split `src/tui_shell/wizard/member_flow.rs` into state transitions, validation, and side-effect helpers.
- [ ] Split `src/tui_shell/wizard/publish_sync_flow.rs` into parse/transition/effect helpers.

### D) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [ ] Run `cargo nextest run` (or document fallback).
- [x] Update roadmap notes/checkboxes as slices land.

Progress notes:
- Validation for this slice:
  - `cargo fmt` passed
  - `cargo clippy --all-targets -- -D warnings` passed
  - Targeted `nextest`/`cargo test` invocations for server integration tests were intermittently hanging in this environment after compile; full integration verification remains pending.
