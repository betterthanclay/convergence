# Phase 015: TUI Command Contexts (Interactive Prompts)

## Goal

Support multi-step command flows (like promote gate picking, snap message entry, resolution workflows) without forcing the user to type long flag-heavy commands.

The shell stays the root; commands may temporarily enter a context that changes the prompt and expected input.

## Scope

In scope:
- A prompt/context stack for interactive flows.
- Context-aware suggestions (choices instead of global commands).
- Clear escape semantics (`esc` backs out of context).

Out of scope:
- AI chat mode at the root.

## Tasks

### A) Context model

- [ ] Add a context stack type (e.g. `ShellContext`).
- [ ] Context has:
  - prompt label
  - parser/validator
  - completion choices
  - `enter` handler
  - `esc` handler

### B) First contexts

- [ ] `snap` without `-m`: prompt for message (enter to skip).
- [ ] `promote` without `--to-gate`: prompt with downstream gate choices.
- [ ] `resolve pick` flow: prompt for variant selection when on a conflicted path.

### C) UX

- [ ] Make `Tab` cycle choices within a context.
- [ ] Make `Esc` pop the context and restore previous input.

## Exit Criteria

- The common flows can be driven via short commands + guided prompts.
