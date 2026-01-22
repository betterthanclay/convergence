# Design Directions: A VCS That "Gets Out of the Way"

This section turns the transcript ideas into concrete primitives and system directions you can prototype.

## 1) Core reframing: two layers

Layer 1: Continuous capture (private, automatic)
- Records every file operation: save/write, rename, delete, chmod, move.
- Optionally records editor-level operations (more semantic than filesystem diffs).
- Local-first; works offline.
- Never blocks.

Layer 2: Publish (shared, curated)
- Creates a "consumable" snapshot: buildable, reviewable, releasable.
- Enforces policy (tests, formatting, secret checks) here, not during private capture.

This matches your stated takeaway: stay in your bubble while iterating; only commit when work is complete.

## 2) First-class "superposition" (conflicts as data, not errors)

Represent a conflict as:
- A set of competing versions for a path, each with provenance (author/device/time/base).
- A selected active version per workspace.
- A resolution object (merge result) that can be created later.

UX goal:
- Synchronization never fails; it produces either a clean update or a conflict object.
- The user can continue working with their active version while seeing alternates.

Scaling mechanism:
- Subscriptions: by default you only subscribe to published channels (or a small set of peers), not every in-progress stream.

## 3) Event log as the primary source of truth

Instead of "a repo is a DAG of commits" as the main abstraction, model the underlying data as:
- An append-only event log of filesystem/editor events.
- Materialized views: snapshots at time T, or "published checkpoints".

Why this helps:
- Restores become trivial (like Time Machine, but repo-aware).
- Forensics can use real signals: edits, saves, builds, test runs, command history.
- "Commit messages" become optional metadata layered on top of a checkpoint.

## 4) Storage: content-addressed + chunked for large binaries

To make "save everything" feasible:
- Content-address files/blobs (like Git) but add chunking/delta for binaries.
- Consider:
  - Rolling hashes (rsync-style) for large binary deltas.
  - Per-file-type strategies (images vs audio vs 3D assets).
  - Server-side GC policies (keep all published, prune old private states after N days).

## 5) Command model: 5-7 intent-driven verbs

One possible CLI surface:
- `vcs init` (create workspace)
- `vcs sync` (capture+upload, fetch others; never fails)
- `vcs status` (what changed, what conflicts exist)
- `vcs publish` (create a consumable checkpoint; runs gates)
- `vcs switch <published|stream>` (move workspace view)
- `vcs resolve <path>` (enter conflict UI)
- `vcs restore <time|checkpoint>` (rewind/repair)

Avoid overloaded verbs (e.g., Git's `checkout`).

## 6) Review and release as first-class (not bolted on)

If "commit" means "ready for consumption", you likely also want:
- A release artifact pipeline tied to published checkpoints.
- Provenance metadata: who published, what gates passed, what environment.
- A default branch/channel concept: "stable", "candidate", "dev".

## 7) Open problems to tackle explicitly

- Workspace determinism: how to guarantee builds/repro are stable when a workspace can contain superpositions.
- Security/privacy: continuous capture can record secrets; need fast secret scanning and redaction.
- Trust model: what does it mean to "subscribe" to someone's in-progress stream?
- UX for conflict sets: presenting alternates without turning the filesystem into a junk drawer.
- Interop/migration: import/export with Git (at least publish checkpoints as Git commits).

## 8) Prototype path (minimal, but real)

If you want to explore quickly:
- Implement a local daemon that watches filesystem events (macOS FSEvents) and records an event log.
- Store blobs in a local content-addressed store.
- Add `sync` that uploads blobs/events to a simple server.
- Implement "superposition" by materializing alternates as `filename~user~timestamp` next to the primary file.
- Implement `publish` as a named snapshot pointer plus optional CI hooks.
