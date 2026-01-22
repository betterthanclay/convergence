# Client Workspace Architecture

This document describes the client responsibilities and local state.

## Responsibilities

- Maintain a working directory for a chosen `(repo, scope)`.
- Maintain a local object store (blobs + manifests) as a cache.
- Create snaps from filesystem state.
- Compute diffs.
- Publish snaps.
- Materialize bundles into the working directory.

## Local state

Suggested local structure:
- workspace config (repo, scope, lane, auth)
- local object store
- local snap timeline
- workspace view pointer (what bundle is currently checked out)

## Offline-first behavior

- `snap` works offline.
- `publish` queues when offline and retries.
- `fetch` updates local cache when online.

## Determinism

Restores and diffs must be deterministic:
- given a bundle/snap id, materialization yields the same bytes
- superpositions must be represented explicitly, not via implicit local heuristics
