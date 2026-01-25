# Operations and Semantics

This document specifies the meaning of the key user-facing operations.

## `converge snap`

Creates a new `snap` from the current workspace filesystem state.

Semantics:
- Captures the complete tree state (subject to ignore rules).
- Writes new blobs/manifests to the local store.
- Produces an immutable `snap_id`.
- Never requires the workspace to be buildable.

Notes:
- v1 may be manual-only.
- Later phases can add continuous capture via daemon/IDE integration.

## `converge diff`

Computes and displays differences between:
- workspace filesystem and a base bundle
- two snaps
- two bundles

Diff requirements:
- text: structural hunks
- binary: metadata, hashes, and (optional) specialized viewers

Current implementation:
- `converge diff` compares the working directory to the current HEAD snap.
- `converge diff --from <snap_id> --to <snap_id>` compares two snaps.
- Output is a path-level summary (`A`, `D`, `M`), not a hunked text diff.

## `converge publish`

Creates a `publication` that submits a snap to a specific gate within a scope.

Semantics:
- The snap becomes an input candidate for that gate.
- Publish is a declaration of "complete for this phase".
- Publish does not require the system to converge anything immediately.

## `converge bundle`

Gate operator action that selects a set of inputs (publications and/or upstream bundles) and coalesces them into a new `bundle`.

Semantics:
- A gate always emits a `bundle`.
- The resulting bundle may include unresolved superpositions.
- The gate evaluates its policy to compute promotability.

## `converge promote`

Advances a bundle to the next gate in the graph.

Semantics:
- Promotion is allowed only if the bundle is promotable under the current gate’s rules.
- Promotion records provenance (who promoted, when, policy evaluations).
- Promotion does not rewrite the bundle; it advances a pointer/state in the scope.

## `converge release`

Marks a bundle as a release in a named release channel.

Semantics:
- A release is typically cut from the terminal gate of the primary gate graph, but this is not required.
- A repo can allow release creation from earlier gates (e.g. compatibility releases) if gate policy permits.
- Release creation records provenance (who released, from which bundle, under which policy).

Current implementation (dev server):
- CLI:
  - `converge release create --channel <name> --bundle-id <id> [--notes ...]`
  - `converge release list`
  - `converge release show --channel <name>`
- API:
  - `POST /repos/:repo_id/releases`
  - `GET /repos/:repo_id/releases`
  - `GET /repos/:repo_id/releases/:channel` (latest)
- Enforcement:
  - requires publish permission
  - bundle must be promotable at release time
  - defaults to terminal gate (non-admin releases require `bundle.gate == terminal_gate`)

## `converge resolve`

Creates resolution objects for superpositions.

Semantics:
- Resolution can happen:
  - in a workspace (private)
  - as part of producing a bundle at a gate
- Resolution produces a new manifest that selects/merges variants.

## `converge restore`

Materializes a chosen snap or bundle into the workspace.

Semantics:
- Restore is deterministic: same input yields same tree.
- Restore can optionally include only a subset of paths.

## Failure modes

Core rule:
- capture and publishing should not fail due to conflicts; conflicts become superpositions.

Allowed hard failures are primarily:
- auth/permission denial
- corrupted stores
- policy disallowing promotion
