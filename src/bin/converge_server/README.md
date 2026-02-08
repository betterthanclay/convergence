# converge_server modules

`src/bin/converge-server.rs` is a thin bootstrap entrypoint.

Key module groups:

- Route and handlers:
  - `routes.rs`: authenticated route registration.
  - `handlers_system.rs`: auth middleware, health, bootstrap.
  - `handlers_identity.rs`, `handlers_repo.rs`, `handlers_gates.rs`, `handlers_objects.rs`, `handlers_publications.rs`, `handlers_release.rs`, `handlers_gc.rs`.
- Shared server helpers:
  - `persistence.rs`, `identity_store.rs`, `validators.rs`, `object_graph.rs`.
  - `access.rs`, `http_error.rs`, `gate_graph_validation.rs`.

This split keeps request behavior unchanged while making ownership boundaries explicit.
