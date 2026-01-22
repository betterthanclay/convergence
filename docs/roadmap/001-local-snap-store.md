# Phase 001: Local Snap Store (MVP Foundation)

## Goal

Ship a working local-first Convergence CLI that can:
- initialize a workspace
- create immutable `snap`s from the filesystem
- list and inspect snaps
- restore a snap back into the working directory

This phase intentionally does not require a server or background file watcher; users explicitly run `converge snap`.

## Why This Phase Exists

- Establish the core data model (blobs/manifests/snaps) with strong determinism.
- Provide an end-to-end vertical slice that future phases (publish/gates/server/TUI) can build on.
- Validate the ergonomics of "snap without needing a working build".

## Scope

In scope:
- Rust project skeleton with a `converge` binary.
- Local content-addressed blob store.
- Tree manifests representing directory state.
- Snap creation from a working directory.
- Snap listing and inspection.
- Snap restore (materialize to working directory).
- Ignore rules (initially minimal; can mirror `.gitignore` semantics later).
- `--json` output for machine readability (at least for `status`/`list`/`show`).

Explicitly out of scope:
- Central authority server.
- `publish`, gates, bundles, promotion, release channels.
- Background file watching / IDE integration.
- TUI.
- Rich merge/superposition UX (conflict objects may be represented later).

## Architecture Notes

- The store should be content-addressed for blobs.
- Snaps should reference a root manifest.
- All IDs should be stable and deterministic.
- Restoring the same snap into an empty directory should produce byte-identical results.

## Tasks

### A) Repository + CLI skeleton

- [ ] Create a Rust workspace (Cargo) and a `converge` binary.
- [ ] Implement top-level command parsing and help output.
- [ ] Implement `--json` output plumbing (even if only a subset of commands supports it initially).

### B) Local workspace metadata

- [ ] Define the workspace config format (repo-independent for this phase).
- [ ] Decide and document on-disk layout (e.g. `.converge/`).
- [ ] Implement `converge init` to set up metadata and directories.

### C) Content-addressed blob store

- [ ] Define blob hashing algorithm (e.g. BLAKE3, SHA-256) and ID representation.
- [ ] Store blobs by hash and prevent duplication.
- [ ] Implement integrity checks when reading blobs.

### D) Manifests (directory trees)

- [ ] Define manifest encoding (e.g. CBOR/JSON) and hashing.
- [ ] Support entry types:
  - [ ] file (blob + metadata)
  - [ ] dir (child manifest)
  - [ ] symlink (optional; can be deferred)
- [ ] Implement deterministic ordering and hashing.

### E) Snap creation

- [ ] Walk the filesystem and build a manifest tree.
- [ ] Store newly discovered blobs/manifests.
- [ ] Create a snap record that points to the root manifest.
- [ ] Implement `converge snap`.

### F) Listing / inspection

- [ ] Implement `converge snaps` (list snaps, newest first).
- [ ] Implement `converge show <snap-id>` (metadata + summary).

### G) Restore

- [ ] Implement `converge restore <snap-id>`.
- [ ] Define behavior for existing files (default: refuse unless `--force`, or restore into empty dir).

### H) Tests

- [ ] Unit tests for hashing and manifest determinism.
- [ ] Golden tests for restore determinism.

## Exit Criteria

- `converge init` creates `.converge/` and a workspace config.
- `converge snap` creates a new snap that can be listed.
- `converge restore <snap-id>` recreates the snap’s tree deterministically.
- At least minimal `--json` support exists for listing/inspection.

## Follow-on Phases

- Phase 002: Central Authority MVP (auth, publish intake, fetch) + object distribution.
- Phase 003: Gates + Bundles + Promotion semantics wired to the server.
- Phase 004: TUI for inbox/superpositions/bundle promotion.
- Phase 005: Background capture (daemon/IDE).
