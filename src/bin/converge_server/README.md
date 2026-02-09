# converge_server modules

`src/bin/converge-server.rs` is a thin bootstrap entrypoint.

Key module groups:

- Runtime wiring:
  - `runtime/`: startup argument parsing, identity bootstrap/load, and graceful shutdown helpers.
- Route and handlers:
  - `routes.rs`: authenticated route registration.
  - `handlers_system.rs`: auth middleware, health, bootstrap.
  - `handlers_identity/`, `handlers_repo/`, `handlers_gates.rs`, `handlers_objects/`, `handlers_publications/`, `handlers_release/`, `handlers_gc/`.
- Shared server helpers:
  - `persistence/`, `identity_store.rs`, `validators.rs`, `object_graph/`.
  - `access.rs`, `http_error.rs`, `gate_graph_validation/`.

This split keeps request behavior unchanged while making ownership boundaries explicit.
