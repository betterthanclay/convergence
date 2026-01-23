# Phase 012: TUI Command Shell (Local-First)

## Goal

Make the TUI useful without any remote configuration or connectivity.

The TUI becomes a local-first command shell (Opencode-style) with a global input line, scrollback, command history, and lightweight autocomplete.

## Scope

In scope:
- No remote HTTP calls on startup.
- A "Shell" UI that accepts commands (leading `/` optional).
- Local workflow commands: status, snap, snaps, show, restore.
- A mode indicator with `Tab` toggling Local/Remote only when the input is empty.

Out of scope:
- Remote commands (handled in Phase 013).
- Rich list/detail views (handled in Phase 014).
- Interactive per-command prompting contexts (handled in Phase 015).

## Tasks

### A) App lifecycle + modes

- [ ] Refactor `src/tui.rs` startup to only discover workspace + config (no HTTP).
- [ ] Add `Mode::{Local, Remote}` with a visible indicator.
- [ ] Bind `Tab` to toggle mode only when input is empty.
- [ ] Bind `q`/`esc` to quit (with `esc` first clearing input/palette).

### B) Shell UI

- [ ] Add a scrollback model: timestamp + kind (command/output/error).
- [ ] Add a single-line input editor:
  - cursor left/right
  - backspace/delete
  - ctrl-u clear line
  - history up/down
- [ ] Add a suggestion/palette box below input:
  - fuzzy match command names
  - `Tab` cycles suggestions when input non-empty
  - `Enter` runs selected / best match

### C) Command parsing

- [ ] Parse input as a command line; accept optional leading `/`.
- [ ] Tokenize with quotes (e.g. `snap -m "msg"`).
- [ ] Provide `help` with short usage lines.

### D) Local commands

- [ ] `status`: workspace root, remote configured yes/no, snap count, latest snap id/time.
- [ ] `snap [-m "..."] [--json]`: create a snap.
- [ ] `snaps [--limit N] [--json]`: list snaps.
- [ ] `show <snap_id> [--json]`: show snap details.
- [ ] `restore <snap_id> [--force]`: restore snap.
- [ ] `clear`: clear scrollback.
- [ ] `quit`: exit.

## Exit Criteria

- Running `converge` (no args) inside a workspace provides a usable local-first shell.
- No remote configuration is required to use the TUI.
- `Tab` toggles Local/Remote only when input is empty; otherwise it behaves as completion.
