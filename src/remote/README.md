# remote

Remote client implementation split by concern.

- `types.rs`: request/response DTOs and payload structs.
- `http_client.rs`: shared HTTP helpers (`with_retries`, auth header, URL building, status handling).
- `identity.rs`: identity, user/token, and membership/lane operations.
- `operations.rs`: repo/gate/bundle/release/promotion/pin/gc operations.
- `transfer.rs`: upload/publish/sync flows.
- `fetch.rs`: fetch/publication sync and manifest/blob/recipe traversal.

`src/remote.rs` defines `RemoteClient` storage and constructor, and composes the split modules.
