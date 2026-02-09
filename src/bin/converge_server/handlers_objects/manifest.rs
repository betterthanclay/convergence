use super::*;

pub(crate) async fn put_manifest(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, manifest_id)): Path<(String, String)>,
    Query(q): Query<PutObjectQuery>,
    body: axum::body::Bytes,
) -> Result<StatusCode, Response> {
    validate_object_id(&manifest_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_publish(repo, &subject) {
            return Err(forbidden());
        }
    }

    let actual = blake3::hash(&body).to_hex().to_string();
    if actual != manifest_id {
        return Err(bad_request(anyhow::anyhow!(
            "manifest hash mismatch (expected {}, got {})",
            manifest_id,
            actual
        )));
    }

    let manifest: converge::model::Manifest =
        serde_json::from_slice(&body).map_err(|e| bad_request(anyhow::anyhow!(e)))?;
    if manifest.version != 1 {
        return Err(bad_request(anyhow::anyhow!("unsupported manifest version")));
    }

    for entry in &manifest.entries {
        validate_manifest_entry_refs(&state, &repo_id, &entry.kind, q.allow_missing_blobs)?;
    }

    let path = repo_data_dir(&state, &repo_id)
        .join("objects/manifests")
        .join(format!("{}.json", manifest_id));
    write_if_absent(&path, &body).map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

pub(crate) async fn get_manifest(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, manifest_id)): Path<(String, String)>,
) -> Result<Response, Response> {
    validate_object_id(&manifest_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_read(repo, &subject) {
            return Err(forbidden());
        }
    }

    let path = repo_data_dir(&state, &repo_id)
        .join("objects/manifests")
        .join(format!("{}.json", manifest_id));
    if !path.exists() {
        return Err(not_found());
    }
    let bytes = std::fs::read(&path)
        .with_context(|| format!("read {}", path.display()))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let actual = blake3::hash(&bytes).to_hex().to_string();
    if actual != manifest_id {
        return Err(internal_error(anyhow::anyhow!(
            "manifest integrity check failed"
        )));
    }
    let _: converge::model::Manifest =
        serde_json::from_slice(&bytes).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    Ok(json_bytes(bytes))
}
