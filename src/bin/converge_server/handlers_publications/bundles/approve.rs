use super::*;

pub(crate) async fn approve_bundle(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, bundle_id)): Path<(String, String)>,
) -> Result<Json<Bundle>, Response> {
    validate_object_id(&bundle_id).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }

    // Load bundle.
    let mut bundle = if let Some(b) = repo.bundles.iter().find(|b| b.id == bundle_id) {
        b.clone()
    } else {
        load_bundle_from_disk(state.as_ref(), &repo_id, &bundle_id)?
    };

    if !bundle.approvals.contains(&subject.user) {
        bundle.approvals.push(subject.user.clone());
        bundle.approvals.sort();
        bundle.approvals.dedup();
    }

    if !bundle
        .approval_user_ids
        .iter()
        .any(|u| u == &subject.user_id)
    {
        bundle.approval_user_ids.push(subject.user_id.clone());
        bundle.approval_user_ids.sort();
        bundle.approval_user_ids.dedup();
    }

    let gate_def = repo
        .gate_graph
        .gates
        .iter()
        .find(|g| g.id == bundle.gate)
        .ok_or_else(|| internal_error(anyhow::anyhow!("bundle gate not found")))?;

    let has_superpositions =
        manifest_has_superpositions(state.as_ref(), &repo_id, &bundle.root_manifest)?;
    let (promotable, reasons) =
        compute_promotability(gate_def, has_superpositions, bundle.approvals.len());
    bundle.promotable = promotable;
    bundle.reasons = reasons;

    // Persist updated bundle.
    let bytes =
        serde_json::to_vec_pretty(&bundle).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let path = repo_data_dir(state.as_ref(), &repo_id)
        .join("bundles")
        .join(format!("{}.json", bundle.id));
    write_atomic_overwrite(&path, &bytes).map_err(internal_error)?;

    // Update in-memory copy if present.
    if let Some(existing) = repo.bundles.iter_mut().find(|b| b.id == bundle.id) {
        *existing = bundle.clone();
    } else {
        repo.bundles.push(bundle.clone());
    }

    persist_repo(state.as_ref(), repo).map_err(internal_error)?;

    Ok(Json(bundle))
}
