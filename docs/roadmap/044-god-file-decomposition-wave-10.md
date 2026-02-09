# Phase 044: God-File Decomposition (Wave 10)

## Goal

Continue reducing remote and server hotspots by decomposing transport/fetch and gate handler modules into focused files with unchanged behavior.

## Scope

Primary Wave 10 targets:
- `src/remote/fetch.rs` (~317 LOC)
- `src/remote/transfer.rs` (~302 LOC)
- `src/bin/converge_server/handlers_release.rs` (~311 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

Progress notes:
- Start with `remote/fetch.rs` (request/restore/list concerns are separable).
- Follow with `remote/transfer.rs` then `handlers_release.rs`.

### B) Remote Fetch Decomposition
- [x] Split `src/remote/fetch.rs` by operation concern (fetch, restore, release/list helpers).
- [x] Preserve API signatures and fetch/restore behavior.

Progress notes:
- Replaced `src/remote/fetch.rs` with module directory:
  - `src/remote/fetch/mod.rs`
  - `src/remote/fetch/manifest_tree.rs`
  - `src/remote/fetch/object_graph.rs`
- Kept `RemoteClient` fetch entry points and `transfer.rs` helper imports stable via `pub(super)` re-exports from `fetch/mod.rs`.

### C) Remote Transfer Decomposition
- [ ] Split `src/remote/transfer.rs` into upload/download concerns.
- [ ] Preserve integrity checks and progress reporting behavior.

### D) Server Release Handler Decomposition
- [ ] Split `src/bin/converge_server/handlers_release.rs` by endpoint concern.
- [ ] Preserve route signatures and response payloads.

### E) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run`.
- [x] Keep this phase doc updated as slices land.

Progress notes:
- Validation for remote/fetch slice:
  - `cargo fmt` passed
  - `cargo clippy --all-targets -- -D warnings` passed
  - `cargo nextest run remote::operations::repo_gate::tests::format_validation_error_limits_issue_lines` passed
  - `cargo test --lib` passed (`15 passed`, `0 failed`)
