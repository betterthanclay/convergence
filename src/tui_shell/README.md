# TUI Shell Layout

This folder contains the interactive TUI implementation.

Where to look:

- `src/tui_shell/mod.rs`: module root + entry point (`tui_shell::run()`).
- `src/tui_shell/app.rs`: `App` + event loop/state machine + command dispatch (still the largest file during the split).
- `src/tui_shell/commands.rs`: command palette definitions (what `/` can show).
- `src/tui_shell/input.rs`: input editing + history.
- `src/tui_shell/suggest.rs`: palette matching + sorting.
- `src/tui_shell/view.rs`: view trait + shared chrome.
- `src/tui_shell/views/`: one file per view.
- `src/tui_shell/modal.rs`: modal rendering + key handling.
- `src/tui_shell/status.rs`: local/remote status + diff helpers.
- `src/tui_shell/wizard.rs`: multi-step wizards.

Planned next (Phase 030):

- Further split `src/tui_shell/app.rs` into smaller modules (modal handling, command handlers).
