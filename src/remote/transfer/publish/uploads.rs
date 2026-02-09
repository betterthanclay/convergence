use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::model::{ObjectId, SnapRecord};
use crate::store::LocalStore;

use super::{MissingObjectsResponse, RemoteClient, with_retries};

pub(super) fn upload_missing_objects(
    client: &RemoteClient,
    store: &LocalStore,
    repo: &str,
    snap: &SnapRecord,
    manifest_order: &[ObjectId],
    missing: MissingObjectsResponse,
    metadata_only: bool,
) -> Result<()> {
    if !metadata_only {
        upload_blobs(client, store, repo, missing.missing_blobs)?;
    }

    upload_recipes(client, store, repo, missing.missing_recipes, metadata_only)?;

    upload_manifests(
        client,
        store,
        repo,
        manifest_order,
        missing.missing_manifests,
        metadata_only,
    )?;

    if !missing.missing_snaps.is_empty() {
        with_retries("upload snap", || {
            let resp = client
                .client
                .put(client.url(&format!("/repos/{}/objects/snaps/{}", repo, snap.id)))
                .header(reqwest::header::AUTHORIZATION, client.auth())
                .json(snap)
                .send()
                .context("send")?;
            client.ensure_ok(resp, "upload snap")
        })?;
    }

    Ok(())
}

fn upload_blobs(
    client: &RemoteClient,
    store: &LocalStore,
    repo: &str,
    blob_ids: Vec<String>,
) -> Result<()> {
    for id in blob_ids {
        let bytes = store.get_blob(&ObjectId(id.clone()))?;
        with_retries(&format!("upload blob {}", id), || {
            let resp = client
                .client
                .put(client.url(&format!("/repos/{}/objects/blobs/{}", repo, id)))
                .header(reqwest::header::AUTHORIZATION, client.auth())
                .body(bytes.clone())
                .send()
                .context("send")?;
            client.ensure_ok(resp, "upload blob")
        })?;
    }
    Ok(())
}

fn upload_recipes(
    client: &RemoteClient,
    store: &LocalStore,
    repo: &str,
    recipe_ids: Vec<String>,
    metadata_only: bool,
) -> Result<()> {
    for id in recipe_ids {
        let rid = ObjectId(id.clone());
        let bytes = store.get_recipe_bytes(&rid)?;

        let path = if metadata_only {
            format!(
                "/repos/{}/objects/recipes/{}?allow_missing_blobs=true",
                repo, id
            )
        } else {
            format!("/repos/{}/objects/recipes/{}", repo, id)
        };
        with_retries(&format!("upload recipe {}", id), || {
            let resp = client
                .client
                .put(client.url(&path))
                .header(reqwest::header::AUTHORIZATION, client.auth())
                .body(bytes.clone())
                .send()
                .context("send")?;
            client.ensure_ok(resp, "upload recipe")
        })?;
    }
    Ok(())
}

fn upload_manifests(
    client: &RemoteClient,
    store: &LocalStore,
    repo: &str,
    manifest_order: &[ObjectId],
    missing_manifests: Vec<String>,
    metadata_only: bool,
) -> Result<()> {
    let mut missing_manifests: HashSet<String> = missing_manifests.into_iter().collect();
    for mid in manifest_order {
        let id = mid.as_str();
        if !missing_manifests.remove(id) {
            continue;
        }

        let bytes = store.get_manifest_bytes(mid)?;

        let path = if metadata_only {
            format!(
                "/repos/{}/objects/manifests/{}?allow_missing_blobs=true",
                repo, id
            )
        } else {
            format!("/repos/{}/objects/manifests/{}", repo, id)
        };
        with_retries(&format!("upload manifest {}", id), || {
            let resp = client
                .client
                .put(client.url(&path))
                .header(reqwest::header::AUTHORIZATION, client.auth())
                .body(bytes.clone())
                .send()
                .context("send")?;
            client.ensure_ok(resp, "upload manifest")
        })?;
    }

    if !missing_manifests.is_empty() {
        anyhow::bail!(
            "missing manifest upload ordering bug (still missing: {})",
            missing_manifests.len()
        );
    }

    Ok(())
}
