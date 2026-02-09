use super::*;

pub(super) fn register_membership_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route(
            "/repos/:repo_id/members",
            get(list_repo_members).post(add_repo_member),
        )
        .route(
            "/repos/:repo_id/members/:handle",
            axum::routing::delete(remove_repo_member),
        )
        .route(
            "/repos/:repo_id/lanes/:lane_id/members",
            get(list_lane_members).post(add_lane_member),
        )
        .route(
            "/repos/:repo_id/lanes/:lane_id/members/:handle",
            axum::routing::delete(remove_lane_member),
        )
}
