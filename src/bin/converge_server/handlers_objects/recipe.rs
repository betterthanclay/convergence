use super::*;

pub(crate) async fn put_recipe(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, recipe_id)): Path<(String, String)>,
    Query(q): Query<PutObjectQuery>,
    body: axum::body::Bytes,
) -> Result<StatusCode, Response> {
    validate_object_id(&recipe_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_publish(repo, &subject) {
            return Err(forbidden());
        }
    }

    let actual = blake3::hash(&body).to_hex().to_string();
    if actual != recipe_id {
        return Err(bad_request(anyhow::anyhow!(
            "recipe hash mismatch (expected {}, got {})",
            recipe_id,
            actual
        )));
    }

    let recipe: converge::model::FileRecipe =
        serde_json::from_slice(&body).map_err(|e| bad_request(anyhow::anyhow!(e)))?;
    if recipe.version != 1 {
        return Err(bad_request(anyhow::anyhow!("unsupported recipe version")));
    }

    for c in &recipe.chunks {
        validate_object_id(c.blob.as_str()).map_err(bad_request)?;
        if !q.allow_missing_blobs {
            let p = repo_data_dir(&state, &repo_id)
                .join("objects/blobs")
                .join(c.blob.as_str());
            if !p.exists() {
                return Err(bad_request(anyhow::anyhow!(
                    "missing referenced blob {}",
                    c.blob.as_str()
                )));
            }
        }
    }

    let path = repo_data_dir(&state, &repo_id)
        .join("objects/recipes")
        .join(format!("{}.json", recipe_id));
    write_if_absent(&path, &body).map_err(internal_error)?;
    Ok(StatusCode::CREATED)
}

pub(crate) async fn get_recipe(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, recipe_id)): Path<(String, String)>,
) -> Result<Response, Response> {
    validate_object_id(&recipe_id).map_err(bad_request)?;

    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_read(repo, &subject) {
            return Err(forbidden());
        }
    }

    let path = repo_data_dir(&state, &repo_id)
        .join("objects/recipes")
        .join(format!("{}.json", recipe_id));
    if !path.exists() {
        return Err(not_found());
    }
    let bytes = std::fs::read(&path)
        .with_context(|| format!("read {}", path.display()))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let actual = blake3::hash(&bytes).to_hex().to_string();
    if actual != recipe_id {
        return Err(internal_error(anyhow::anyhow!(
            "recipe integrity check failed"
        )));
    }

    let _: converge::model::FileRecipe =
        serde_json::from_slice(&bytes).map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    Ok(json_bytes(bytes))
}
