use super::*;

pub(super) fn register_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    identity::register_identity_routes(router)
        .pipe(membership::register_membership_routes)
        .pipe(repo_core::register_repo_core_routes)
        .pipe(release_promotion::register_release_promotion_routes)
        .pipe(objects::register_object_routes)
        .route("/repos/:repo_id/gc", axum::routing::post(gc_repo))
}

mod identity;
mod membership;
mod objects;
mod release_promotion;
mod repo_core;

trait RouterPipe {
    fn pipe(self, f: fn(Router<Arc<AppState>>) -> Router<Arc<AppState>>) -> Router<Arc<AppState>>;
}

impl RouterPipe for Router<Arc<AppState>> {
    fn pipe(self, f: fn(Router<Arc<AppState>>) -> Router<Arc<AppState>>) -> Router<Arc<AppState>> {
        f(self)
    }
}
