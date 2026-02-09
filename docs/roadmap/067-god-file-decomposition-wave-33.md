# Phase 067: God-File Decomposition (Wave 33)

## Goal

Continue remote client decomposition by splitting transport DTOs/requests into focused domain modules.

## Scope

Primary Wave 33 target:
- `src/remote/types.rs` (~255 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Remote Types Decomposition
- [x] Split `src/remote/types.rs` into focused modules for auth/users, repo/lanes, publication flow, gate graph, and request payloads.
- [x] Preserve serde field defaults and visibility for internal request-only types.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- 2026-02-09: `cargo fmt` passed.
- 2026-02-09: `cargo clippy --all-targets -- -D warnings` passed.
- 2026-02-09: `cargo nextest run` compiled and then stalled in this environment after build completion.
- 2026-02-09: fallback `cargo test --lib` passed (15 passed, 0 failed).
