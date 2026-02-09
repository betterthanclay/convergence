//! Garbage-collection endpoint logic for retained and prunable repo objects.

use super::*;

mod prune;
mod roots;
mod sweep;

use self::prune::prune_release_history;
use self::roots::collect_retained_roots;

#[derive(Debug, serde::Deserialize)]
pub(super) struct GcQuery {
    #[serde(default = "default_true")]
    dry_run: bool,
    #[serde(default = "default_true")]
    prune_metadata: bool,

    /// If set, prune release history by keeping only the latest N releases per channel.
    ///
    /// This affects GC roots: pruned releases stop retaining their referenced bundles/objects.
    #[serde(default)]
    prune_releases_keep_last: Option<usize>,
}

fn default_true() -> bool {
    true
}

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

    let pruned_releases_keep_last = prune_release_history(repo, q.prune_releases_keep_last)?;
    let retained = collect_retained_roots(state.as_ref(), &repo_id, repo)?;

    let objects_root = repo_data_dir(state.as_ref(), &repo_id).join("objects");
    let (deleted_blobs, kept_blobs_count) = sweep::sweep_ids(
        &objects_root.join("blobs"),
        None,
        &retained.keep_blobs,
        q.dry_run,
    )?;
    let (deleted_manifests, kept_manifests_count) = sweep::sweep_ids(
        &objects_root.join("manifests"),
        Some("json"),
        &retained.keep_manifests,
        q.dry_run,
    )?;
    let (deleted_recipes, kept_recipes_count) = sweep::sweep_ids(
        &objects_root.join("recipes"),
        Some("json"),
        &retained.keep_recipes,
        q.dry_run,
    )?;

    let (deleted_snaps, _kept_snaps_count) = if q.prune_metadata {
        sweep::sweep_ids(
            &objects_root.join("snaps"),
            Some("json"),
            &retained.keep_snaps,
            q.dry_run,
        )?
    } else {
        (0, 0)
    };

    let (deleted_bundles, _kept_bundles_count) = if q.prune_metadata {
        sweep::sweep_ids(
            &repo_data_dir(state.as_ref(), &repo_id).join("bundles"),
            Some("json"),
            &retained.keep_bundles,
            q.dry_run,
        )?
    } else {
        (0, 0)
    };

    let keep_release_ids: HashSet<String> = repo
        .releases
        .iter()
        .filter(|r| retained.keep_bundles.contains(&r.bundle_id))
        .map(|r| r.id.clone())
        .collect();

    let (deleted_releases, kept_releases_count) = if q.prune_metadata {
        sweep::sweep_ids(
            &repo_data_dir(state.as_ref(), &repo_id).join("releases"),
            Some("json"),
            &keep_release_ids,
            q.dry_run,
        )?
    } else {
        (0, 0)
    };

    if q.prune_metadata && !q.dry_run {
        repo.bundles
            .retain(|b| retained.keep_bundles.contains(&b.id));
        repo.pinned_bundles
            .retain(|bundle_id| retained.keep_bundles.contains(bundle_id));
        repo.releases
            .retain(|r| retained.keep_bundles.contains(&r.bundle_id));
        repo.publications
            .retain(|p| retained.keep_publications.contains(&p.id));
        repo.snaps = retained.keep_snaps.clone();
        persist_repo(state.as_ref(), repo).map_err(internal_error)?;
    }

    Ok(Json(serde_json::json!({
        "dry_run": q.dry_run,
        "prune_metadata": q.prune_metadata,
        "pruned": {
            "releases_keep_last": pruned_releases_keep_last
        },
        "kept": {
            "bundles": retained.keep_bundles.len(),
            "releases": kept_releases_count,
            "publications": retained.keep_publications.len(),
            "snaps": retained.keep_snaps.len(),
            "blobs": kept_blobs_count,
            "manifests": kept_manifests_count,
            "recipes": kept_recipes_count
        },
        "deleted": {
            "bundles": deleted_bundles,
            "releases": deleted_releases,
            "snaps": deleted_snaps,
            "blobs": deleted_blobs,
            "manifests": deleted_manifests,
            "recipes": deleted_recipes
        }
    })))
}
