use super::*;

pub(super) fn validate_blob_ref(
    state: &AppState,
    repo_id: &str,
    blob_id: &str,
    require_blobs: bool,
) -> Result<(), Response> {
    validate_object_id(blob_id).map_err(bad_request)?;
    if require_blobs {
        let p = repo_data_dir(state, repo_id)
            .join("objects/blobs")
            .join(blob_id);
        if !p.exists() {
            return Err(bad_request(anyhow::anyhow!(
                "missing referenced blob {}",
                blob_id
            )));
        }
    }
    Ok(())
}

pub(super) fn validate_recipe_refs(
    state: &AppState,
    repo_id: &str,
    recipe_id: &str,
    require_blobs: bool,
) -> Result<(), Response> {
    let recipe = read_recipe(state, repo_id, recipe_id)?;
    for c in recipe.chunks {
        validate_blob_ref(state, repo_id, c.blob.as_str(), require_blobs)?;
    }
    Ok(())
}
