use super::super::super::super::*;

use super::types::ListBundlesQuery;

pub(super) async fn list_bundles(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Query(q): Query<ListBundlesQuery>,
) -> Result<Json<Vec<Bundle>>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }

    let mut out = Vec::new();
    for b in &repo.bundles {
        if let Some(scope) = &q.scope
            && &b.scope != scope
        {
            continue;
        }
        if let Some(gate) = &q.gate
            && &b.gate != gate
        {
            continue;
        }
        out.push(b.clone());
    }
    Ok(Json(out))
}

pub(super) async fn get_bundle(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, bundle_id)): Path<(String, String)>,
) -> Result<Json<Bundle>, Response> {
    validate_object_id(&bundle_id).map_err(bad_request)?;

    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }

    if let Some(b) = repo.bundles.iter().find(|b| b.id == bundle_id) {
        return Ok(Json(b.clone()));
    }

    // Best-effort disk fallback.
    let path = repo_data_dir(&state, &repo_id)
        .join("bundles")
        .join(format!("{}.json", bundle_id));
    if !path.exists() {
        return Err(not_found());
    }
    let bytes = std::fs::read(&path)
        .with_context(|| format!("read {}", path.display()))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let bundle: Bundle =
        serde_json::from_slice(&bytes).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    Ok(Json(bundle))
}
