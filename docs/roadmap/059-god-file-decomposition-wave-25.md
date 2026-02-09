# Phase 059: God-File Decomposition (Wave 25)

## Goal

Continue reducing mixed-responsibility command/server modules by splitting system handler auth/bootstrap flows and remote admin command surfaces into focused submodules.

## Scope

Primary Wave 25 targets:
- `src/bin/converge_server/handlers_system.rs` (~191 LOC)
- `src/cli_exec/remote_admin.rs` (~212 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and decomposition boundaries.

### B) Server System Handler Decomposition
- [x] Split `src/bin/converge_server/handlers_system.rs` by bearer auth and bootstrap concerns.
- [x] Preserve unauthorized/conflict behaviors, token indexing semantics, and bootstrap persistence flow.

### C) CLI Remote Admin Decomposition
- [x] Split `src/cli_exec/remote_admin.rs` by remote config/repo/gc commands and gate-graph commands.
- [x] Preserve text/json output behavior and starter gate graph init/apply semantics.

### D) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

## Verification Notes

- 2026-02-09: `cargo fmt` passed.
- 2026-02-09: `cargo clippy --all-targets -- -D warnings` passed.
- 2026-02-09: `cargo nextest run` compiled and then stalled in this environment after build completion; fallback `cargo test --lib` passed (15 passed, 0 failed).
