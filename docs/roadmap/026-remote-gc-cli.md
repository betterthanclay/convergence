# Phase 026: Remote GC CLI

## Goal

Make server GC and release-pruning usable without curl by adding a first-class CLI wrapper.

## Scope

In scope:
- `converge remote gc` CLI.
- Remote client method to call `/repos/:repo_id/gc`.
- Doc update to reference CLI usage.
- A small CLI e2e test.

Out of scope:
- TUI UI for GC.
- Additional server-side GC policies.

## Tasks

- [x] Add `RemoteClient::gc_repo(...)`.
- [x] Add `converge remote gc` command.
- [x] Update operator docs to prefer the CLI over curl.
- [x] Add CLI e2e test for `remote gc`.

## Exit Criteria

- Operators can run `converge remote gc --prune-releases-keep-last N`.
