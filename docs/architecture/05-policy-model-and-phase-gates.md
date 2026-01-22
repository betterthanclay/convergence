# Policy Model and Phase Gates

This document defines how gates decide what is "ready for consumption".

## Principles

- Gates decide promotability; bundles can exist even when not promotable.
- Strictness generally increases as you move toward the terminal release gate.
- Policies should be explicit, inspectable, and auditable.

## Policy inputs

A gate policy can depend on:
- bundle contents (diff size, touched paths)
- superposition presence and location
- test/build/lint results
- approvals/reviews
- provenance (trusted publisher lanes, signatures)
- external signals (security scanners, compliance)

## Policy outputs

At minimum:
- `promotable: true|false`
- `reasons[]` (machine-readable codes + human-readable messages)

Optionally:
- required remediations (e.g. "resolve these superpositions", "add approval from owner")

## Execution model

Policies can be implemented as:
- a built-in rules engine (fast, limited)
- external CI workflows (flexible)
- a hybrid (built-in gates for common cases + CI for org-specific rules)

Initial bias:
- implement a small built-in core for determinism and speed
- allow extension via external runners for org-specific checks

## Example gate progression

- Gate: `dev-intake`
  - allow superpositions
  - require secret scanning on publish

- Gate: `team-merge`
  - require 0 unresolved superpositions in `src/`
  - allow superpositions in `docs/`
  - require 1 approval from lane owner

- Gate: `release-candidate`
  - require all checks green
  - require 0 superpositions anywhere
  - require artifacts built and signed

## Release endpoints and channels

Releasing is a policy-governed action.

- Most repos will designate the terminal gate of the primary gate graph as the default release endpoint.
- Some repos may allow releases from earlier gates for specific channels (e.g., compatibility releases, feature-flag distributions).
- Gate policy and permissions determine:
  - which gates can cut which release channels
  - what checks/approvals are required
  - what provenance/attestations must be attached
