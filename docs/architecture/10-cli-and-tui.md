# CLI and TUI

This document describes UX surfaces and determinism rules.

## CLI principles

- Deterministic by default.
- Stable, scriptable output.
- Prefer a small set of orthogonal verbs.
- Provide `--json` for automation.

Core verbs (initial set):
- `snap`
- `publish`
- `status`
- `diff`
- `fetch`
- `converge`
- `promote`
- `release`
- `resolve`
- `restore`

## TUI principles

- `cnv` (no args) opens an interactive TUI.
- TUI is a client of the same underlying commands/APIs.

TUI capabilities:
- show the gate graph and current scope position
- browse local snaps
- inbox of publications/bundles relevant to the user’s responsibilities
- inspect superpositions and drive resolution
- promote bundles and understand policy blockers
