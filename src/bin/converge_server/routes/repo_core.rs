use super::*;

pub(super) fn register_repo_core_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route("/repos", get(list_repos).post(create_repo))
        .route("/repos/:repo_id", get(get_repo))
        .route("/repos/:repo_id/permissions", get(get_repo_permissions))
        .route("/repos/:repo_id/lanes", get(list_lanes))
        .route(
            "/repos/:repo_id/lanes/:lane_id/heads/me",
            axum::routing::post(update_lane_head_me),
        )
        .route(
            "/repos/:repo_id/lanes/:lane_id/heads/:user",
            get(get_lane_head),
        )
        .route("/repos/:repo_id/gates", get(list_gates))
        .route(
            "/repos/:repo_id/gate-graph",
            get(get_gate_graph).put(put_gate_graph),
        )
        .route(
            "/repos/:repo_id/scopes",
            get(list_scopes).post(create_scope),
        )
        .route(
            "/repos/:repo_id/publications",
            get(list_publications).post(create_publication),
        )
        .route(
            "/repos/:repo_id/bundles",
            get(list_bundles).post(create_bundle),
        )
        .route("/repos/:repo_id/bundles/:bundle_id", get(get_bundle))
        .route(
            "/repos/:repo_id/bundles/:bundle_id/pin",
            axum::routing::post(pin_bundle),
        )
        .route(
            "/repos/:repo_id/bundles/:bundle_id/unpin",
            axum::routing::post(unpin_bundle),
        )
        .route("/repos/:repo_id/pins", get(list_pins))
        .route(
            "/repos/:repo_id/bundles/:bundle_id/approve",
            axum::routing::post(approve_bundle),
        )
}
