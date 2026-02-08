# cli_exec

Thin command execution layer used by `src/main.rs`.

- `local.rs`: local workspace/store actions (`init`, `snap`, `snaps`, `show`, `restore`, `diff`, `mv`).
- `identity.rs`: auth and membership operations (`login`, `logout`, `whoami`, `user`, `token`, `members`, `lane`, `lanes`).
- `remote_admin.rs`: remote/admin operations (`remote`, `gates`).
- `delivery.rs`: delivery workflows (`publish`, `sync`, `fetch`, `bundle`, `promote`, `pins`, `pin`, `status`).
- `release_resolve.rs`: release + resolution workflows (`release`, `approve`, `resolve`).

`src/cli_exec.rs` routes top-level CLI commands into these modules.
