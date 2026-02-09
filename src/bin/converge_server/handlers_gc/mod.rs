//! Garbage-collection endpoint logic for retained and prunable repo objects.

use super::*;

mod prune;
mod query;
mod roots;
mod sweep;
mod workflow;

use self::prune::prune_release_history;
use self::query::GcQuery;
use self::roots::collect_retained_roots;

pub(super) async fn gc_repo(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Query(q): Query<GcQuery>,
) -> Result<Json<serde_json::Value>, Response> {
    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }

    if !q.prune_metadata && !q.dry_run {
        return Err(bad_request(anyhow::anyhow!(
            "refusing destructive GC with prune_metadata=false (would create dangling references); use dry_run=true or prune_metadata=true"
        )));
    }

    workflow::run_gc(state.as_ref(), &repo_id, repo, q).map(Json)
}
