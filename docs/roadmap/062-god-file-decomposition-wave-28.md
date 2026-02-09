# Phase 062: God-File Decomposition (Wave 28)

## Goal

Continue reducing server-side data/handler density by splitting repository-load persistence helpers and bundle create/list/get handlers into focused submodules.

## Scope

Primary Wave 28 targets:
- `src/bin/converge_server/persistence/repo_load.rs` (~198 LOC)
- `src/bin/converge_server/handlers_publications/bundles/create_list_get.rs` (~197 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

### B) Persistence Repo-Load Decomposition
- [x] Split `src/bin/converge_server/persistence/repo_load.rs` by repo hydration, collection loaders, and promotion-state rebuild helpers.
- [x] Preserve on-disk fallback behavior and best-effort backfill semantics.

### C) Bundle Create/List/Get Decomposition
- [x] Split `src/bin/converge_server/handlers_publications/bundles/create_list_get.rs` by create/list/get and query/request DTO concerns.
- [x] Preserve validation, permission checks, and response body semantics.

### D) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- 2026-02-09: `cargo fmt` passed.
- 2026-02-09: `cargo clippy --all-targets -- -D warnings` passed.
- 2026-02-09: `cargo nextest run` passed (64 passed, 0 failed).
