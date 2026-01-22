# Security, Identity, and Permissions

This document defines the high-level security model.

## Identity

The server is authoritative for identity.

Minimum:
- user identities
- service identities (CI runners, gate bots)

## Authorization model

Permissions are scoped by:
- repo
- lane
- scope
- gate

Core actions:
- `snap` (local; no server permission)
- `publish`
- `converge`
- `promote`
- `release`

## Audit and provenance

All server-side state transitions must be attributable:
- publish
- converge
- promote
- release

## Secret handling

Because snaps can contain secrets:
- implement secret scanning on publish (and optionally on snap creation locally)
- provide redaction and key rotation guidance

## Trust boundaries

- A publication is not automatically trusted.
- Gates decide what inputs are allowed and what checks are required.
