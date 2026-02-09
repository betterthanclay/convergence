use super::*;

pub(crate) async fn put_blob(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, blob_id)): Path<(String, String)>,
    body: axum::body::Bytes,
) -> Result<StatusCode, Response> {
    validate_object_id(&blob_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_publish(repo, &subject) {
            return Err(forbidden());
        }
    }

    let actual = blake3::hash(&body).to_hex().to_string();
    if actual != blob_id {
        return Err(bad_request(anyhow::anyhow!(
            "blob hash mismatch (expected {}, got {})",
            blob_id,
            actual
        )));
    }

    let path = repo_data_dir(&state, &repo_id)
        .join("objects/blobs")
        .join(&blob_id);
    write_if_absent(&path, &body).map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

pub(crate) async fn get_blob(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, blob_id)): Path<(String, String)>,
) -> Result<axum::body::Bytes, Response> {
    validate_object_id(&blob_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_read(repo, &subject) {
            return Err(forbidden());
        }
    }

    let path = repo_data_dir(&state, &repo_id)
        .join("objects/blobs")
        .join(&blob_id);
    if !path.exists() {
        return Err(not_found());
    }
    let bytes = std::fs::read(&path)
        .with_context(|| format!("read {}", path.display()))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let actual = blake3::hash(&bytes).to_hex().to_string();
    if actual != blob_id {
        return Err(internal_error(anyhow::anyhow!(
            "blob integrity check failed"
        )));
    }
    Ok(axum::body::Bytes::from(bytes))
}
