# Problem Analysis: What "Git Gets Wrong" (and What It's Actually Good At)

## 1) Two different jobs get conflated

Git is used for at least two distinct purposes:
- Continuous capture/sync/backup of work-in-progress.
- Publishing curated, reviewable, releasable units of change.

Git is strong at the second (immutable, content-addressed history; DAG; cheap branching), but its everyday UX forces users to treat the first job like the second.

## 2) Mismatch between user mental model and Git's model

Common mental model:
- "I have a working directory; I change files; I want my state preserved and shareable."

Git's operational model:
- "You manage a DAG of commits; the working directory is a checkout of a commit plus staged/unstaged deltas; integration is a history rewrite/merge operation."

When the tool surfaces its internals (index, detached HEAD, conflict states, refspecs), users must learn the storage model to accomplish simple intent.

## 3) Conflicts are treated as workflow gates

Git often forces a merge/rebase at the moment of synchronization/integration.
- This is sometimes correct (you need the integrated result now).
- But in many cases, a developer wants to (a) preserve work, and (b) continue working, without deciding the final integrated state yet.

Casey's critique is essentially: "don't make synchronization equivalent to integration".

## 4) Push-order is an arbitrary global ordering mechanism

In many Git workflows:
- Whoever pushes first establishes the new base.
- Later pushers must reconcile.

This is a convenience mechanism, but it bakes in an implicit org-wide ordering that may not match the easiest integration order.

## 5) Large assets expose Git's weakest path

For binary assets and huge repos:
- Storing and transferring entire histories becomes expensive.
- Diffs/merges are not meaningful for many file types.
- Hosting products add additional constraints (size limits, diff limits, LFS ergonomics).

These workflows make a "versioned filesystem" approach more intuitive: preserve full file versions, allow easy selection, avoid pretending text-merge is universal.

## 6) What seems fundamental (and won't go away)

Even with perfect tooling:
- People disagree on what a "unit" of change is.
- People will create incompatible designs.
- Organizations need policy (access, review, compliance, release discipline).

The opportunity isn't to erase coordination, but to:
- Reduce how often coordination interrupts flow.
- Make coordination intent-driven and correctly scoped.
- Capture state so coordination can happen without data loss.

## 7) What seems solvable (and high leverage)

- Separate capture from publish.
- Make "my work is safe" a one-command operation.
- Make conflicts preservable, navigable objects (not immediate failures).
- Provide first-class support for binaries and huge repos.
- Provide consistent, small command surface with good defaults.
- Integrate richer forensic signals (edit timeline, test runs) without forcing users to curate them manually.
