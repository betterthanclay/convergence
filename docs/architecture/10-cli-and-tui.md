# CLI and TUI

This document describes UX surfaces and determinism rules.

## CLI principles

- Deterministic by default.
- Stable, scriptable output.
- Prefer a small set of orthogonal verbs.
- Provide `--json` for automation.

Implemented verbs (current):
- `init`, `snap`, `snaps`, `show`, `restore`
- `remote` (configure + `create-repo` dev convenience)
- `publish`, `fetch`
- `bundle`, `approve`, `promote`
- `status`

Planned verbs (not yet implemented):
- `diff`, `resolve`, `release`

## TUI principles

- `converge` (no args) opens an interactive TUI.
- TUI is a client of the same underlying commands/APIs.

TUI capabilities (current):
- Overview: remote config, gate graph, promotion state
- Inbox: publications for configured scope+gate; quick filter; create bundle
- Bundles: list bundles; show promotability + reasons; approve; promote
- Superpositions: browse conflicted paths in a bundle root manifest and inspect variants

TUI key bindings (current):
- global: `q`/`esc` quit
- overview: `i` inbox, `b` bundles, `r` reload
- inbox: `space` select, `c` create bundle, `/` filter, `r` refresh
- bundles: `a` approve, `p` promote (with gate chooser if needed), `s` superpositions
