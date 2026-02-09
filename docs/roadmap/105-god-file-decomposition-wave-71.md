# Phase 105: God-File Decomposition (Wave 71)

## Goal

Decompose server route registration, manifest-availability validation traversal, and remote publish transfer internals into focused helper modules.

## Scope

Primary Wave 71 targets:
- `src/bin/converge_server/routes/register.rs` (~143 LOC)
- `src/bin/converge_server/object_graph/traversal/validate.rs` (~122 LOC)
- `src/remote/transfer/publish/mod.rs` (~109 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target and decomposition boundaries.

### B) Decomposition
- [x] Split `src/bin/converge_server/routes/register.rs` into grouped route registration helper files.
- [x] Split `src/bin/converge_server/object_graph/traversal/validate.rs` into manifest/blob/recipe validation helpers.
- [x] Split `src/remote/transfer/publish/mod.rs` into publish flow and missing-object request helpers.
- [x] Preserve routing/validation/publish behavior and API semantics.

### C) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- `cargo fmt` passed.
- `cargo clippy --all-targets -- -D warnings` passed.
- `cargo nextest run` stalled after starting build in this environment; fallback `cargo test --lib` passed (`15 passed, 0 failed`).
