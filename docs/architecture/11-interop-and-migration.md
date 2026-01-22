# Interop and Migration

This document describes how Convergence interacts with Git-centric ecosystems.

## Goals

- Allow incremental adoption in Git-heavy organizations.
- Support importing existing repos for evaluation.
- Support exporting releases (and possibly bundles) into Git commits for downstream tooling.

## Import (conceptual)

- Import Git history as a series of bundles in a scope.
- Preserve:
  - author/time
  - commit messages
  - tags/releases

## Export (conceptual)

- Export a `release` (or `bundle`) as:
  - a Git tree + commit
  - a tarball/zip
  - build artifacts

## Integration points

- CI providers (policy execution)
- code review systems
- artifact registries
