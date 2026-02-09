use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::model::ObjectId;
use crate::store::LocalStore;

use super::super::{RemoteClient, with_retries};

pub(super) fn fetch_manifest_tree(
    store: &LocalStore,
    remote: &RemoteClient,
    repo: &str,
    root: &ObjectId,
) -> Result<()> {
    let mut visited = HashSet::new();
    fetch_manifest_tree_inner(store, remote, repo, root, &mut visited)
}

pub(super) fn fetch_manifest_tree_inner(
    store: &LocalStore,
    remote: &RemoteClient,
    repo: &str,
    manifest_id: &ObjectId,
    visited: &mut HashSet<String>,
) -> Result<()> {
    if !visited.insert(manifest_id.as_str().to_string()) {
        return Ok(());
    }

    if !store.has_manifest(manifest_id) {
        let resp = remote
            .client
            .get(remote.url(&format!(
                "/repos/{}/objects/manifests/{}",
                repo,
                manifest_id.as_str()
            )))
            .header(reqwest::header::AUTHORIZATION, remote.auth())
            .send()
            .context("fetch manifest")?;
        let bytes = remote
            .ensure_ok(resp, "fetch manifest")?
            .bytes()
            .context("read manifest bytes")?;

        store.put_manifest_bytes(manifest_id, &bytes)?;
    }

    let manifest = store.get_manifest(manifest_id)?;
    for e in manifest.entries {
        match e.kind {
            crate::model::ManifestEntryKind::Dir { manifest } => {
                fetch_manifest_tree_inner(store, remote, repo, &manifest, visited)?;
            }
            crate::model::ManifestEntryKind::File { blob, .. } => {
                fetch_blob_if_missing(store, remote, repo, &blob)?;
            }
            crate::model::ManifestEntryKind::FileChunks { recipe, .. } => {
                fetch_recipe_and_chunks(store, remote, repo, &recipe)?;
            }
            crate::model::ManifestEntryKind::Symlink { .. } => {}
            crate::model::ManifestEntryKind::Superposition { variants } => {
                for v in variants {
                    match v.kind {
                        crate::model::SuperpositionVariantKind::File { blob, .. } => {
                            fetch_blob_if_missing(store, remote, repo, &blob)?;
                        }
                        crate::model::SuperpositionVariantKind::Dir { manifest } => {
                            fetch_manifest_tree_inner(store, remote, repo, &manifest, visited)?;
                        }
                        crate::model::SuperpositionVariantKind::Symlink { .. } => {}
                        crate::model::SuperpositionVariantKind::Tombstone => {}
                        crate::model::SuperpositionVariantKind::FileChunks { recipe, .. } => {
                            fetch_recipe_and_chunks(store, remote, repo, &recipe)?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn fetch_blob_if_missing(
    store: &LocalStore,
    remote: &RemoteClient,
    repo: &str,
    blob: &ObjectId,
) -> Result<()> {
    if store.has_blob(blob) {
        return Ok(());
    }
    let bytes = with_retries(&format!("fetch blob {}", blob.as_str()), || {
        let resp = remote
            .client
            .get(remote.url(&format!("/repos/{}/objects/blobs/{}", repo, blob.as_str())))
            .header(reqwest::header::AUTHORIZATION, remote.auth())
            .send()
            .context("send")?;
        remote
            .ensure_ok(resp, "fetch blob")?
            .bytes()
            .context("bytes")
    })?;

    let computed = blake3::hash(&bytes).to_hex().to_string();
    if computed != blob.as_str() {
        anyhow::bail!(
            "blob hash mismatch (expected {}, got {})",
            blob.as_str(),
            computed
        );
    }
    let id = store.put_blob(&bytes)?;
    if &id != blob {
        anyhow::bail!("unexpected blob id mismatch");
    }
    Ok(())
}

fn fetch_recipe_and_chunks(
    store: &LocalStore,
    remote: &RemoteClient,
    repo: &str,
    recipe: &ObjectId,
) -> Result<()> {
    if !store.has_recipe(recipe) {
        let bytes = with_retries(&format!("fetch recipe {}", recipe.as_str()), || {
            let resp = remote
                .client
                .get(remote.url(&format!(
                    "/repos/{}/objects/recipes/{}",
                    repo,
                    recipe.as_str()
                )))
                .header(reqwest::header::AUTHORIZATION, remote.auth())
                .send()
                .context("send")?;
            remote
                .ensure_ok(resp, "fetch recipe")?
                .bytes()
                .context("bytes")
        })?;

        store.put_recipe_bytes(recipe, &bytes)?;
    }

    let r = store.get_recipe(recipe)?;
    for c in r.chunks {
        fetch_blob_if_missing(store, remote, repo, &c.blob)?;
    }
    Ok(())
}
