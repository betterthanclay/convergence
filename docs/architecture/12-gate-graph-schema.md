# Gate Graph Schema (Draft)

This document specifies the first server-side schema for gates and gate graphs.

## Goal

Represent an org-defined convergence pipeline as a validated DAG of gates.

## Gate

Fields (v1):
- `id`: stable identifier (`lowercase`, `0-9`, `-`)
- `name`: display name
- `upstream`: list of gate ids this gate consumes from
- `lane`: optional lane id that owns/operates this gate
- `policy`: promotability rules (Phase 3 minimal):
  - `allow_releases`: whether bundles at this gate can be released (default: true)
  - `allow_superpositions`: whether superpositions are allowed to pass this gate
  - `allow_metadata_only_publications`: whether metadata-only publications are allowed at this gate
  - `required_approvals`: number of manual approvals required to be promotable

## Gate Graph

Fields (v1):
- `version`: schema version
- `gates`: list of gate definitions

Validation rules:
- unique gate ids
- all `upstream` references exist
- graph is acyclic
- all gates are reachable from at least one "root" gate (a gate with no upstream)

Notes:
- Releases are controlled per gate via `allow_releases`.
