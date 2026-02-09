use super::roots::RetainedRoots;
use super::*;

#[derive(Debug)]
struct SweepCounts {
    deleted_blobs: usize,
    kept_blobs_count: usize,
    deleted_manifests: usize,
    kept_manifests_count: usize,
    deleted_recipes: usize,
    kept_recipes_count: usize,
    deleted_snaps: usize,
    deleted_bundles: usize,
    deleted_releases: usize,
    kept_releases_count: usize,
}

pub(super) fn run_gc(
    state: &AppState,
    repo_id: &str,
    repo: &mut Repo,
    q: GcQuery,
) -> Result<serde_json::Value, Response> {
    let pruned_releases_keep_last = prune_release_history(repo, q.prune_releases_keep_last)?;
    let retained = collect_retained_roots(state, repo_id, repo)?;
    let counts = sweep_repo_objects(state, repo_id, repo, &retained, &q)?;

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
        persist_repo(state, repo).map_err(internal_error)?;
    }

    Ok(serde_json::json!({
        "dry_run": q.dry_run,
        "prune_metadata": q.prune_metadata,
        "pruned": {
            "releases_keep_last": pruned_releases_keep_last
        },
        "kept": {
            "bundles": retained.keep_bundles.len(),
            "releases": counts.kept_releases_count,
            "publications": retained.keep_publications.len(),
            "snaps": retained.keep_snaps.len(),
            "blobs": counts.kept_blobs_count,
            "manifests": counts.kept_manifests_count,
            "recipes": counts.kept_recipes_count
        },
        "deleted": {
            "bundles": counts.deleted_bundles,
            "releases": counts.deleted_releases,
            "snaps": counts.deleted_snaps,
            "blobs": counts.deleted_blobs,
            "manifests": counts.deleted_manifests,
            "recipes": counts.deleted_recipes
        }
    }))
}

fn sweep_repo_objects(
    state: &AppState,
    repo_id: &str,
    repo: &Repo,
    retained: &RetainedRoots,
    q: &GcQuery,
) -> std::result::Result<SweepCounts, Response> {
    let objects_root = repo_data_dir(state, repo_id).join("objects");
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

    let (deleted_snaps, _) = if q.prune_metadata {
        sweep::sweep_ids(
            &objects_root.join("snaps"),
            Some("json"),
            &retained.keep_snaps,
            q.dry_run,
        )?
    } else {
        (0, 0)
    };

    let (deleted_bundles, _) = if q.prune_metadata {
        sweep::sweep_ids(
            &repo_data_dir(state, repo_id).join("bundles"),
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
            &repo_data_dir(state, repo_id).join("releases"),
            Some("json"),
            &keep_release_ids,
            q.dry_run,
        )?
    } else {
        (0, 0)
    };

    Ok(SweepCounts {
        deleted_blobs,
        kept_blobs_count,
        deleted_manifests,
        kept_manifests_count,
        deleted_recipes,
        kept_recipes_count,
        deleted_snaps,
        deleted_bundles,
        deleted_releases,
        kept_releases_count,
    })
}
