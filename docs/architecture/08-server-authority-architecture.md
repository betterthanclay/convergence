# Server Authority Architecture

This document describes the central authority responsibilities.

## Responsibilities

- Identity, auth, and authorization.
- Repo registry.
- Gate graph definition and storage.
- Scope registry and state.
- Intake for publications.
- Gate convergence orchestration.
- Bundle provenance, policy evaluation, and promotability status.
- Promotion tracking.
- Release creation and artifact/provenance attachment.

## API surface (conceptual)

Core endpoints:
- auth/session
- repo/gates/scopes discovery
- publish intake
- bundle fetch (manifests/blobs)
- promotability/status queries
- converge/promote commands (authorized roles)

## Scaling model

- Read-heavy object distribution: manifests/blobs should be cacheable.
- Write paths are controlled:
  - publish intake is high volume
  - converge/promote is lower volume, higher privilege

## Concurrency and consistency

Required consistency points:
- immutable objects (snaps, bundles, releases)
- consistent scope pointers at each gate (what is "current")

Avoid global locks:
- isolate by `(repo, scope, gate)` for promotion updates
