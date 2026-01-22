# Key Points and Arguments (Pros/Cons)

## A) Casey's primary claims

- VCS is not solved; Git's complexity is evidence.
- VCS should be "invisible" most of the time.
- The user intent is usually: back up my work, share my work, get other people's work.
- Conflicts should not block progress; preserve both versions and let humans resolve later.
- Git exposes internals (branches/index/conflict states) instead of presenting a small set of intent-driven actions.

## B) "Superposition" model (preserve concurrent edits)

What it is:
- Concurrent edits create multiple co-existing versions of the same file.
- No global "repo must be pristine" gate.
- Resolution is a later, explicit action.

Pros:
- Removes forced context switching (no mandatory conflict resolution at push/pull time).
- Prevents losing work; never requires recloning/"start over".
- Mirrors how people already work informally (copy files, send patches, Dropbox-like sharing).
- For art/binary-heavy workflows, "pick the right asset" is often the real need, not text-merge.

Cons / open risks:
- Drift can accumulate; later reconciliation may be harder.
- For codebases with tight coupling, "alternate versions" can prevent building/running without a decision.
- "Which reality am I in?" becomes a first-class UX problem (tooling must clearly show overlays).
- Scaling requires scoping (you cannot subscribe to thousands of people's in-progress alternates).

## C) "Commits should be working" vs "capture everything"

Working-commit view (Prime's stance):
- In larger orgs, commits should generally be coherent and buildable.
- Squashing can keep history cleaner; smaller/atomic commits help rollback and bisect.

Capture-everything view (Casey's stance):
- The system should capture work continuously; "commit" is an optional publish/release milestone.
- Forensics should use richer signals than commit messages (edit timeline, test runs, etc.).

Tradeoffs:
- Rich capture improves safety and debugging but increases storage/privacy concerns.
- Strict working commits reduce breakage but can force premature integration work.

## D) Hooks and enforcement

Pro-hook arguments:
- Prevent secret leakage.
- Enforce formatting/lint/test gates for shared code.

Anti-hook arguments (Casey):
- Blocking local progress feels hostile; indicates deeper org/process issues.
- Hooks often fail in surprising ways and increase tool friction.

Middle ground:
- Allow unconstrained private capture; apply enforcement only when publishing to a shared channel.

## E) "Git is hard": fundamental vs accidental complexity

Accidental complexity (fixable):
- Inconsistent UX/commands; too many sharp edges.
- Conflicts treated as hard errors/gates.
- Poor first-class support for large binaries.
- High reliance on institutional knowledge and custom scripts.

Fundamental complexity (hard to remove):
- Coordinating humans on shared artifacts.
- Representing intent and responsibility (ownership, review, release).
- Integrating changes that are logically incompatible.
