# Roadmap Backlog Retirement — Disposition Manifest

Date: 2026-09-09. The `docs/roadmaps/backlog/` surface is retired: Northstar
retired the roadmap backlog because it duplicated triage and blurred the
boundary between a candidate and approved execution. Every item the backlog
carried is dispositioned below; its current meaning is reachable at exactly
one canonical destination or deliberately removed. Migration is not approval:
nothing here authorizes implementation.

## Manifest

| # | Backlog item | Disposition | Canonical destination |
| --- | --- | --- | --- |
| 1 | Real identity | Implemented; pointer removed | `docs/roadmaps/g02/021-real-identity.md` (complete) |
| 2 | Workflow profiles | Already owned by an executable task; pointer removed | `docs/roadmaps/g02/024-workflow-profiles.md` (parked on a design partner) |
| 3 | Edge nodes | Already owned by an executable task; pointer removed | `docs/roadmaps/g02/025-edge-and-scale.md` (parked on measured demand) |
| 4 | Gate graph administration | Implemented; pointer removed | `docs/roadmaps/g02/026-gate-administration.md` (complete) |
| 5 | Manifest paging | Deferred candidate → triage (§ Candidates) | This note; source: architecture doc 16 §1b |
| 6 | Encrypted secret names | Deferred candidate → triage (§ Candidates) | This note; source: architecture doc 19 §9 |
| 7 | Hardware-backed keys | Deferred candidate → triage (§ Candidates) | This note; source: architecture doc 19 §9 |
| 8 | g03 distributed control plane sketch | Already owned by the strategy surface; pointer removed | `docs/roadmaps/generation-index.md` strategic horizons (H4) |

No new executable task was created: items 2–4 and 8 already have owning
surfaces, items 1 and 4 are implemented, and items 5–7 wait on their stated
triggers under triage's non-authoritative rule.

## Candidates

### Manifest paging (efficiency deferral, not a correctness risk)

- State: directories over 4096 entries work today; the 4096 cap is on wire
  batch frames, which clients already split — a large directory is merely a
  large manifest.
- Constraint: sub-manifest pages touch every manifest walker; only worth it
  against measured cost.
- Source: architecture doc 16 §1b; prior backlog note 2026-07-25.
- Promotion condition: measured manifest cost against real trees, via an
  operator-approved `gNN.NNN` task. Owner: repo maintainers; next check: on
  a real workload showing the cost.

### Encrypted secret names

- State: the server sees secret names today.
- Constraint: listing requires decrypting every entry.
- Source: architecture doc 19 §9.
- Promotion condition: a deployment where the *existence* of a credential is
  sensitive, via an operator-approved `gNN.NNN` task. Owner: repo
  maintainers; next check: on such a deployment.

### Hardware-backed keys (OS keychain, Secure Enclave, YubiKey)

- State: machine-local key on disk; passphrase path stays the portable
  default.
- Constraint: keep the private key off disk only when asked; do not prompt
  per command.
- Source: architecture doc 19 §9.
- Promotion condition: a user asking to keep the private key off disk, via
  an operator-approved `gNN.NNN` task. Owner: repo maintainers; next check:
  on such a request.

## Inbound live references rewritten to triage

- `docs/README.md` — roadmaps line no longer advertises a backlog.
- `docs/roadmaps/README.md` — backlog rule, index link, and rollover
  guardrail now name triage.
- `docs/roadmaps/generation-index.md` — H3 deferred items now name triage.
- `docs/contracts/001-working-rules.md` — rollover rule now carries
  triage candidates instead of a backlog move.
- `docs/architecture/14-server-authority-and-distribution.md` §7 — deferred
  target rows now name triage.
- `docs/architecture/16-sync-protocol-and-chunking.md` §1b — paging deferral
  now names triage.
- `docs/architecture/19-secrets-and-key-management.md` §9 — retired term
  replaced; deferral list already matches triage doctrine.

## Retained historical exceptions (unchanged)

These record decisions taken while the backlog existed and are evidence, not
live queues: `docs/logs/2026-07/*`, `docs/logs/2026-08/*`,
`docs/roadmaps/g02/014-architecture-honesty.md`,
`docs/roadmaps/g02/022-ship-readiness.md`,
`docs/specs/archive/005-convergence-semantics-revision.md`. The committed
handoff `docs/handoffs/20260909-151020-retire-roadmap-backlog.md` is a closed
record and keeps its quoted wording.
