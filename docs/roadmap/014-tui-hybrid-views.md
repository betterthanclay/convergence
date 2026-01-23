# Phase 014: TUI Hybrid Views (Optional Panels)

## Goal

Retain the command shell as the primary interaction surface while providing optional rich panels for high-volume remote browsing (inbox, bundles, superpositions).

Shell remains always available and is not replaced.

## Scope

In scope:
- Keep a single global command input.
- Add optional split views for:
  - Inbox (publications)
  - Bundles
  - Superpositions inspector
- Provide commands that open these views (`inbox`, `bundles`, `superpositions`).

Out of scope:
- Full-screen editor integration.
- Multi-tab panes.

## Tasks

### A) Shell + view composition

- [ ] Make Shell the baseline layout.
- [ ] Add a "panel" area that can be opened/closed.
- [ ] Ensure command execution always writes to scrollback (even if a panel is open).

### B) Inbox panel

- [ ] Remote-backed list with filter and selection.
- [ ] Show publication details (id, snap, publisher, created_at).

### C) Bundles panel

- [ ] Remote-backed list with filter and selection.
- [ ] Actions: approve, promote (with gate picker), view superpositions.

### D) Superpositions panel

- [ ] Navigate conflicted paths.
- [ ] Show variant details + VariantKey JSON.
- [ ] Integrate decision picking and validation status.

## Exit Criteria

- Users can either stay entirely in shell mode or open panels for browsing.
- Panels never prevent running commands.
