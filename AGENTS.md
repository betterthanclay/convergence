# AGENTS

Scope: whole `convergence/` repository.

## Hard Rules

- Keep AGENTS content lean: scope, hard rules, validation, links.
- Treat `docs/` as source of truth for vision, architecture, roadmap intent, and rationale history.
- Keep roadmap checklists in sync with completed implementation work.
- Keep terminology consistent (`snap`, `publish`, `bundle`, `promote`, `release`, `superposition`).
- Do not recreate retired `docs/roadmap/` or `docs/decisions/` folders.

## Effigy-First Execution

- Start with `effigy tasks --repo .`.
- Run `effigy doctor --repo .` when environment or task resolution is uncertain.
- Prefer `effigy health --repo .` for the narrow baseline.
- Prefer `effigy validate --repo .` before broader merge-ready checks.
- Prefer `effigy test --repo .` over raw `cargo test`; the repo task intentionally defaults to `cargo nextest` when available.
- Use direct Cargo or Node commands only when the needed operation is not represented in `effigy.toml`.

## Validate

- `effigy health --repo .`
- `effigy validate --repo .`
- `effigy test --repo .` (for test-focused work)

## References

- `docs/README.md`
- `docs/vision/001-convergence-platform-vision.md`
- `docs/architecture/README.md`
- `docs/roadmaps/`
- `docs/logs/`
- `docs/processes/260-agents-operating-guardrails.md`
