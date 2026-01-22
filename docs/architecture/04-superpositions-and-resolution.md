# Superpositions and Resolution

This document defines conflict representation as data.

## What is a superposition?

A superposition exists when multiple versions compete for the same logical path in the same view.

Examples:
- two publications modify `src/lib.rs` in incompatible ways
- two bundles both claim different contents for `assets/logo.png`

## Representation (conceptual)

At the manifest level, a path can map to either:
- a single entry (normal)
- a superposition entry (conflict)

Superposition entry includes:
- `path`
- `variants[]`, each with:
  - `variant_id`
  - `blob_id` or subtree `manifest_id`
  - `provenance` (author, publication/bundle origin, timestamps)
- optional `default_selected_variant` (workspace-local only)

## Where superpositions can exist

- Workspace view:
  - user can choose a default variant without resolving globally
- Bundle output:
  - a gate can emit a bundle containing superpositions
  - promotability can require resolving before promotion

## Resolution

Resolution is the act of collapsing a superposition to a single result.

Resolution types:
- Choose: select one variant.
- Merge: produce a new blob (for text) or a new derived artifact (for binaries where possible).

Resolution outputs:
- a new manifest (or manifest patch) where the path maps to the resolved entry
- provenance linking back to all variants

## UX constraints (large-org safe)

- Superpositions must be discoverable and inspectable.
- Superpositions must not explode the filesystem into unbounded alternate files.
- Resolution must be attributable (who resolved, what inputs were considered).

Suggested UX strategy:
- keep alternates in the object model
- materialize alternates into the filesystem only on demand (e.g. via TUI export or `converge resolve --export`)
