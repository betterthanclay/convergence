# Convergence (Project Guide for Agents)

Convergence is an experimental next-generation version control and collaboration system.

Core idea: capture work continuously (or via explicit snapshots), then converge it through configurable, policy-driven phase gates into increasingly "consumable" bundles, culminating in a release.

Key concepts you will see in the docs:
- `snap`: a snapshot of a workspace state (not necessarily buildable)
- `publish`: submit a snap to a gate/scope as an input
- `bundle`: output produced by a gate after coalescing inputs
- `promote`: move a bundle to the next gate
- `release`: final public output (typically from the terminal gate, but not strictly required)
- "superpositions": conflicts preserved as data, resolved when appropriate per gate policy

## Implementation

- Language: Rust.
- Primary UX: a deterministic CLI (`cnv <action>`) plus an interactive TUI (`cnv` with no args).
- Architecture: centralized authority (GitHub-like) with identity, access control, gates, scopes, provenance.

## Documentation Is The Source Of Truth

Use `docs/` as the canonical reference for intent, terminology, and decisions. Keep documentation up to date as code changes.

Notable locations:
- `docs/decisions/`: timestamped architectural/product decisions.
- `docs/git-podcast/`: transcript-derived notes and early design directions.

## Roadmap And Phases (Entry Point For Work)

We organize significant work via phase documents:
- Folder: `docs/roadmap/`
- Files: numbered "Phase" documents (for example: `Phase-001-<title>.md`).
- Each Phase document defines:
  - the goal
  - scope and non-goals
  - a grouped list of actionable tasks
  - acceptance/exit criteria

Process rule:
- For any significant work in this repository, start by creating or updating a Phase document in `docs/roadmap/`.
- When implementing, keep the Phase doc (and any impacted decision docs) updated to reflect what is actually being built.
