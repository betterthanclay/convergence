# Phase 017: TUI Remote Browsers (Inbox/Bundles/Superpositions)

## Goal

Replace the current split-panel + scrollback-heavy remote UX with modal, dedicated browsers for high-volume remote tasks.

## Scope

In scope:
- `inbox` mode: browse publications.
- `bundles` mode: browse bundles.
- `superpositions` mode: browse conflicted paths for a bundle and drive resolution.
- Lazy HTTP: fetch only when entering a remote mode or running a remote command.

Out of scope:
- Server-side resolution storage.

## Tasks

### A) Inbox mode

- [ ] List publications with selection + details.
- [ ] Mode-local commands:
  - `bundle`: fetch bundle for selected publication
  - `fetch`: fetch selected snap
  - `back`

### B) Bundles mode

- [ ] List bundles with selection + details (promotable + reasons).
- [ ] Mode-local commands:
  - `approve`
  - `promote [--to-gate ...]`
  - `superpositions`: enter superpositions mode for selected bundle
  - `back`

### C) Superpositions mode

- [ ] List conflicted paths with selection.
- [ ] Detail view shows variants + current decision + validation state.
- [ ] Mode-local commands:
  - `pick <n>` / `clear`
  - `next-missing` / `next-invalid`
  - `validate`
  - `apply [--publish]`
  - `back`

## Exit Criteria

- Remote workflows (approve/promote/resolve) are doable without relying on the scrollback as the main UI.
- Errors and results show in the status/notification area and do not destroy the active view.
