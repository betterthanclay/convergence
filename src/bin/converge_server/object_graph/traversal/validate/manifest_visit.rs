use super::*;

pub(super) fn visit_manifest(
    state: &AppState,
    repo_id: &str,
    manifest_id: &str,
    require_blobs: bool,
    visited: &mut HashSet<String>,
) -> Result<(), Response> {
    if !visited.insert(manifest_id.to_string()) {
        return Ok(());
    }

    let manifest = read_manifest(state, repo_id, manifest_id)?;
    for e in manifest.entries {
        match e.kind {
            converge::model::ManifestEntryKind::File { blob, .. } => {
                super::blobs::validate_blob_ref(state, repo_id, blob.as_str(), require_blobs)?;
            }
            converge::model::ManifestEntryKind::FileChunks { recipe, .. } => {
                super::blobs::validate_recipe_refs(state, repo_id, recipe.as_str(), require_blobs)?;
            }
            converge::model::ManifestEntryKind::Dir { manifest } => {
                visit_manifest(state, repo_id, manifest.as_str(), require_blobs, visited)?;
            }
            converge::model::ManifestEntryKind::Symlink { .. } => {}
            converge::model::ManifestEntryKind::Superposition { variants } => {
                for v in variants {
                    match v.kind {
                        converge::model::SuperpositionVariantKind::File { blob, .. } => {
                            super::blobs::validate_blob_ref(
                                state,
                                repo_id,
                                blob.as_str(),
                                require_blobs,
                            )?;
                        }
                        converge::model::SuperpositionVariantKind::FileChunks {
                            recipe, ..
                        } => {
                            super::blobs::validate_recipe_refs(
                                state,
                                repo_id,
                                recipe.as_str(),
                                require_blobs,
                            )?;
                        }
                        converge::model::SuperpositionVariantKind::Dir { manifest } => {
                            visit_manifest(
                                state,
                                repo_id,
                                manifest.as_str(),
                                require_blobs,
                                visited,
                            )?;
                        }
                        converge::model::SuperpositionVariantKind::Symlink { .. } => {}
                        converge::model::SuperpositionVariantKind::Tombstone => {}
                    }
                }
            }
        }
    }

    Ok(())
}
