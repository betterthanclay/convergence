# Repo, Gate Graph, Lanes (Breadth), and Scopes (Depth)

This document defines how Convergence models a large organization.

## Gate graph

Each repo defines a directed acyclic graph of gates:
- multiple upstream gates can feed a downstream gate
- the graph typically converges to a single terminal gate for the primary release flow

Gate graph responsibilities:
- defines the phases of "ready for consumption"
- defines where policy changes (strictness typically increases as you approach release)
- defines organizational responsibility boundaries (who converges what)

## Releases are not strictly tied to the terminal gate

While most repos will designate the terminal gate as the default place to cut public releases, Convergence may support creating releases from earlier gates when desired, such as:
- maintaining compatibility releases on older versions
- producing feature-flagged distributions
- cutting emergency patches that intentionally bypass later-phase policies

This is always controlled by gate policy and permissions.

## Lanes (breadth)

Lanes are organizational partitions (teams/areas) with three roles:

1) Ownership
- a lane owns a subgraph of gates (or a set of responsibilities within gates)

2) Visibility default
- users see the lane’s publications and bundles by default
- cross-lane visibility is explicit (subscribe/override), not ambient

3) Superposition breadth scoping
- lanes bound the default set of superpositions you can observe and be affected by

### Cross-reach

Cross-reach is explicitly requested access to observe or consume artifacts outside the default lane.

Examples:
- temporarily subscribe to another lane’s scope
- pull a specific publication/bundle by id

Cross-reach must be controlled by:
- authorization
- audit logging

## Scopes (depth)

Scopes are the branch-like dimension.

A scope:
- is named (`feature/x`, `milestone-2026q1`, `release/1.0`)
- threads through the gate graph
- has its own lineage of publications/bundles

Semantics:
- publishing always targets a `(scope, gate)`
- bundles are produced within a scope
- promotion advances bundles within the same scope

## Defaults for large orgs

- A developer works in a single scope most of the time.
- The developer’s default visibility is their lane.
- Integration responsibility is concentrated in designated roles at particular gates.

## Convergence ordering

Git often implicitly orders integration by "who pushed first".

In Convergence:
- ordering is an explicit decision at each gate (human and/or automation)
- a gate can choose a merge/coalesce order that minimizes conflicts and risk
