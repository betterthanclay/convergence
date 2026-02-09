use super::*;

pub(crate) async fn put_snap(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, snap_id)): Path<(String, String)>,
    Json(snap): Json<converge::model::SnapRecord>,
) -> Result<StatusCode, Response> {
    validate_object_id(&snap_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_publish(repo, &subject) {
            return Err(forbidden());
        }
    }

    if snap.id != snap_id {
        return Err(bad_request(anyhow::anyhow!(
            "snap id mismatch (path {}, body {})",
            snap_id,
            snap.id
        )));
    }

    if snap.version != 1 {
        return Err(bad_request(anyhow::anyhow!("unsupported snap version")));
    }

    let bytes = serde_json::to_vec_pretty(&snap).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let path = repo_data_dir(&state, &repo_id)
        .join("objects/snaps")
        .join(format!("{}.json", snap_id));
    write_if_absent(&path, &bytes).map_err(internal_error)?;

    {
        let mut repos = state.repos.write().await;
        if let Some(repo) = repos.get_mut(&repo_id) {
            repo.snaps.insert(snap_id);
            persist_repo(state.as_ref(), repo).map_err(internal_error)?;
        }
    }

    Ok(StatusCode::CREATED)
}

pub(crate) async fn get_snap(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, snap_id)): Path<(String, String)>,
) -> Result<Response, Response> {
    validate_object_id(&snap_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_read(repo, &subject) {
            return Err(forbidden());
        }
    }

    let path = repo_data_dir(&state, &repo_id)
        .join("objects/snaps")
        .join(format!("{}.json", snap_id));
    if !path.exists() {
        return Err(not_found());
    }
    let bytes = std::fs::read(&path)
        .with_context(|| format!("read {}", path.display()))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let _snap: converge::model::SnapRecord =
        serde_json::from_slice(&bytes).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    Ok(json_bytes(bytes))
}
