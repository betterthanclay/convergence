use super::*;

pub(super) struct RetainedRoots {
    pub(super) keep_bundles: HashSet<String>,
    pub(super) keep_publications: HashSet<String>,
    pub(super) keep_snaps: HashSet<String>,
    pub(super) keep_blobs: HashSet<String>,
    pub(super) keep_manifests: HashSet<String>,
    pub(super) keep_recipes: HashSet<String>,
}

pub(super) fn collect_retained_roots(
    state: &AppState,
    repo_id: &str,
    repo: &Repo,
) -> Result<RetainedRoots, Response> {
    let mut keep_bundles: HashSet<String> = repo.pinned_bundles.iter().cloned().collect();
    for release in &repo.releases {
        keep_bundles.insert(release.bundle_id.clone());
    }
    for per_scope in repo.promotion_state.values() {
        for bundle_id in per_scope.values() {
            keep_bundles.insert(bundle_id.clone());
        }
    }

    let mut keep_publications: HashSet<String> = HashSet::new();
    let mut keep_snaps: HashSet<String> = HashSet::new();
    let mut keep_blobs: HashSet<String> = HashSet::new();
    let mut keep_manifests: HashSet<String> = HashSet::new();
    let mut keep_recipes: HashSet<String> = HashSet::new();

    let mut bundle_roots: Vec<String> = Vec::new();
    for bundle_id in &keep_bundles {
        let bundle = if let Some(existing) = repo.bundles.iter().find(|b| b.id == *bundle_id) {
            existing.clone()
        } else {
            load_bundle_from_disk(state, repo_id, bundle_id)?
        };

        bundle_roots.push(bundle.root_manifest.clone());
        for publication_id in bundle.input_publications {
            keep_publications.insert(publication_id);
        }
    }

    for publication in &repo.publications {
        if keep_publications.contains(&publication.id) {
            keep_snaps.insert(publication.snap_id.clone());
        }
    }

    for lane in repo.lanes.values() {
        for head in lane.heads.values() {
            keep_snaps.insert(head.snap_id.clone());
        }
        for history in lane.head_history.values() {
            for head in history {
                keep_snaps.insert(head.snap_id.clone());
            }
        }
    }

    for root_manifest in &bundle_roots {
        collect_objects_from_manifest_tree(
            state,
            repo_id,
            root_manifest,
            &mut keep_blobs,
            &mut keep_manifests,
            &mut keep_recipes,
        )?;
    }

    for snap_id in keep_snaps.clone() {
        let path = repo_data_dir(state, repo_id)
            .join("objects/snaps")
            .join(format!("{}.json", snap_id));
        if !path.exists() {
            continue;
        }
        let bytes = std::fs::read(&path)
            .with_context(|| format!("read {}", path.display()))
            .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
        let snap: converge::model::SnapRecord =
            serde_json::from_slice(&bytes).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
        collect_objects_from_manifest_tree(
            state,
            repo_id,
            snap.root_manifest.as_str(),
            &mut keep_blobs,
            &mut keep_manifests,
            &mut keep_recipes,
        )?;
    }

    Ok(RetainedRoots {
        keep_bundles,
        keep_publications,
        keep_snaps,
        keep_blobs,
        keep_manifests,
        keep_recipes,
    })
}
