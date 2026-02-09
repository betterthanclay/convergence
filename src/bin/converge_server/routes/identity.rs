use super::*;

pub(super) fn register_identity_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route("/whoami", get(whoami))
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:user_id/tokens",
            axum::routing::post(create_token_for_user),
        )
        .route("/tokens", get(list_tokens).post(create_token))
        .route(
            "/tokens/:token_id/revoke",
            axum::routing::post(revoke_token),
        )
}
