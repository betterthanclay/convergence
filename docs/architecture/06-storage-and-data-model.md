# Storage and Data Model

This document describes storage in terms of conceptual entities and invariants. Implementation details can evolve.

## Content-addressed storage

All file contents are stored as blobs addressed by hash (`blob_id`).

Benefits:
- deduplication
- integrity
- efficient sharing between snaps/bundles

## Manifests

A manifest represents a directory tree.

Conceptually:
- `manifest_id`
- `entries: map<path_segment, entry>`

Entry types:
- `file(blob_id, metadata)`
- `dir(manifest_id)`
- `symlink(target)`
- `superposition(superposition_id)`

## Snaps and bundles reference manifests

- A `snap` points to a `root_manifest_id`.
- A `bundle` points to a `root_manifest_id`.

## Provenance and audit

Published objects must be attributable.

Store provenance as immutable records:
- who created/published/converged/promoted
- what inputs were used
- what policy checks ran and their outputs

## Large files and binaries

Requirements:
- handle large binaries efficiently
- avoid naive full-history fetches for new clients

Design directions:
- chunked storage for large blobs
- delta strategies per type (optional)
- server-side retention policies (e.g., keep all bundles/releases; prune unreferenced snaps)

## Garbage collection

GC must preserve:
- all releases
- all bundles referenced by releases or active scopes
- all publications required for audit

Prunable candidates (policy-defined):
- old unreferenced snaps
- intermediate derived blobs
