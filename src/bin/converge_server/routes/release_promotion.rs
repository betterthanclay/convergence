use super::*;

pub(super) fn register_release_promotion_routes(
    router: Router<Arc<AppState>>,
) -> Router<Arc<AppState>> {
    router
        .route(
            "/repos/:repo_id/releases",
            get(list_releases).post(create_release),
        )
        .route(
            "/repos/:repo_id/releases/:channel",
            get(get_release_channel),
        )
        .route(
            "/repos/:repo_id/promotions",
            get(list_promotions).post(create_promotion),
        )
        .route("/repos/:repo_id/promotion-state", get(get_promotion_state))
}
