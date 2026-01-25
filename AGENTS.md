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
- Primary UX: a deterministic CLI (`converge <action>`) plus an interactive TUI (`converge` with no args).
- Architecture: centralized authority (GitHub-like) with identity, access control, gates, scopes, provenance.

## Documentation Is The Source Of Truth

Use `docs/` as the canonical reference for intent, terminology, and decisions. Keep documentation up to date as code changes.

Notable locations:
- `docs/decisions/`: timestamped architectural/product decisions.
- `docs/git-podcast/`: transcript-derived notes and early design directions.
- `docs/architecture/`: architecture and semantics.
- `docs/roadmap/`: Phase documents (entry point for significant work).

## Roadmap And Phases (Entry Point For Work)

We organize significant work via phase documents:
- Folder: `docs/roadmap/`
- Files: numbered, lowercase phase documents (for example: `001-<title>.md`).
- Each Phase document defines:
  - the goal
  - scope and non-goals
  - a grouped list of actionable tasks as Markdown checkboxes (`- [ ] ...`)
  - acceptance/exit criteria

Process rule:
- For any significant work in this repository, start by creating or updating a Phase document in `docs/roadmap/`.
- When implementing, keep the Phase doc (and any impacted decision docs) updated to reflect what is actually being built.

## Continue Policy

At each point in the conversation, the agent should propose the next actionable todo, either:
- by selecting the next checkbox task from the current Phase document in `docs/roadmap/`, or
- by suggesting the next logical implementation step if the roadmap does not cover it.

When the user replies with "Continue":
- Proceed with the proposed todo(s).
- If multiple options were proposed and the user replies only "Continue", execute them in the order presented.
- If the options should be sequenced across multiple messages, complete the first option, then propose the next.

Roadmap hygiene:
- As tasks are completed, tick the corresponding checkboxes in the relevant `docs/roadmap/*.md` file.

## btca

When you need up-to-date information about technologies used in this project, use btca to query source repositories directly.

**Available resources**: axum, tokio, reqwest, ratatui, crossterm, clap, serde, serdeJson, time, anyhow, blake3, getrandom, tempfile

### Usage

```bash
btca ask -r <resource> -q "<question>"
```

Use multiple `-r` flags to query multiple resources at once:

```bash
btca ask -r axum -r tokio -q "How should I structure graceful shutdown in an Axum server using Tokio?"
```
