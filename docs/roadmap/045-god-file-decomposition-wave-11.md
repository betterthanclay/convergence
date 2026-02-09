# Phase 045: God-File Decomposition (Wave 11)

## Goal

Continue reducing high-LOC hotspots in core resolution and TUI interaction code while preserving behavior and command/output compatibility.

## Scope

Primary Wave 11 targets:
- `src/resolve.rs` (~316 LOC)
- `src/tui_shell/app/remote_members.rs` (~313 LOC)
- `src/tui_shell/status.rs` (~293 LOC)

## Tasks

### A) Baseline and Boundaries
- [x] Capture target order and split boundaries.

Progress notes:
- Start with `resolve.rs` (core data/validation/apply concerns are separable).
- Follow with TUI slices (`remote_members.rs`, then `status.rs`).

### B) Resolve Module Decomposition
- [x] Split `src/resolve.rs` into focused submodules for model/validation/apply behavior.
- [x] Preserve public APIs and resolution semantics.

Progress notes:
- Replaced `src/resolve.rs` with module directory:
  - `src/resolve/mod.rs`
  - `src/resolve/types.rs`
  - `src/resolve/variants.rs`
  - `src/resolve/validate.rs`
  - `src/resolve/apply.rs`
- Preserved public API surface used by CLI and TUI (`apply_resolution`, `validate_resolution`, `superposition_variants`, `superposition_variant_counts`, and validation structs).

### C) Remote Members TUI Decomposition
- [x] Split `src/tui_shell/app/remote_members.rs` by parse/state/effect concerns.
- [x] Preserve prompt flow and command behavior.

Progress notes:
- Replaced `src/tui_shell/app/remote_members.rs` with module directory:
  - `src/tui_shell/app/remote_members/mod.rs`
  - `src/tui_shell/app/remote_members/list.rs`
  - `src/tui_shell/app/remote_members/member.rs`
  - `src/tui_shell/app/remote_members/lane_member.rs`
- Preserved command behaviors for `members`, `member`, and `lane-member` flows, including prompt-first behavior and legacy flag forms.

### D) Status TUI Decomposition
- [x] Split `src/tui_shell/status.rs` by transform/render helper concerns.
- [x] Preserve status output fidelity.

Progress notes:
- Replaced `src/tui_shell/status.rs` with module root plus dedicated rename test file:
  - `src/tui_shell/status/mod.rs`
  - `src/tui_shell/status/rename_tests.rs`
- Preserved status helper exports used by app/root views and kept rename detection tests intact.

### E) Verification and Hygiene
- [x] Run `cargo fmt`.
- [x] Run `cargo clippy --all-targets -- -D warnings`.
- [x] Run `cargo nextest run` (or document fallback if environment stalls persist).
- [x] Keep this phase doc updated as slices land.

Progress notes:
- Validation for `resolve` decomposition:
  - `cargo fmt` passed
  - `cargo clippy --all-targets -- -D warnings` passed
  - Targeted `nextest` passed:
    - `cargo nextest run resolve_validate phase6_e2e_resolve_superpositions`
  - Additional targeted test sweep passed:
    - `cargo test resolve_validate -- --nocapture`
- Validation for `remote_members` decomposition:
  - `cargo fmt` passed
  - `cargo clippy --all-targets -- -D warnings` passed
  - `cargo test --lib` passed (`15 passed`, `0 failed`)
- Validation for `status` decomposition:
  - `cargo fmt` passed
  - `cargo clippy --all-targets -- -D warnings` passed
  - `cargo test --lib` passed (`15 passed`, `0 failed`)
