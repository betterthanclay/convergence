# Roadmaps

Convergence roadmaps hold executable Northstar task work, one task per
`gNN.NNN`.

## Rules

- Active task files live in generation folders such as `g02/`.
- File names use `NNN-slug.md` with numbering local to the generation.
- References should use task IDs such as `g02.031`.
- Generation rollover is manual only.
- Treat generations as substantial sequencing eras, not one-or-two-file buckets. As a healthy default, expect roughly 20 to 40 task files in one generation before rollover is even worth discussing.
- Treat rollover as full generation closeout, not a convenience reset: close, supersede, or rehome every task in the current generation first, then purge stale generation-specific specs from `docs/specs/` before opening the next generation.
- Backlog items belong in `backlog/`.
- Metadata files stay at the `roadmaps/` root if later needed.

## Current generation

- Active generation: `g02`
- Next task ID: `g02.032`

## Index

- [generation-index.md](./generation-index.md)
- [g02/README.md](./g02/README.md)
- [backlog/README.md](./backlog/README.md)

`g01` roadmap files are archived on branch `archive/g01`; the generation index
records the closed state.

## Active strict lane

`g02` is in closing posture. Programs complete through `g02.029` (candidate
rename). Open tasks:

- `g02.027` TUI usability — decision and guidance work complete (batch
  records 096, 097, 100); operator cold-drive verdict pending before
  closeout
- `g02.022` ship readiness — 22.1–22.4 complete; release step pipeline
  built, **release not cut** (operator-gated)

Parked on triggers: `g02.024` workflow profiles, `g02.025` edge/scale.

Independent maintenance tasks:

- `g02.030` Northstar instruction and Rust quality audit — complete
- `g02.031` installed Rust package consumer canary — complete (PR #4)

Product execution still waits on operator direction.

## Rollover guardrail

Do not open `gNN+1` while the current generation still has live task files or stale strict-lane debris in the active specs tree.

Before rollover:

- every task in the closing generation must be explicitly closed, paused, superseded, or moved to backlog
- the roadmap front doors must agree that the old generation is no longer the live queue
- `docs/specs/` must be purged so only live or near-live planning artifacts remain in the active tree

## Next Task

Card 102 canary merged (PR #4). Product direction remains TUI usability
closeout (`g02.027`) or the operator-gated first release (`g02.022` release
step). See `g02/README.md`.
