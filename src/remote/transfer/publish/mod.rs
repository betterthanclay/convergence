use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::model::SnapRecord;
use crate::store::LocalStore;

use super::super::fetch::{collect_objects, manifest_postorder};
use super::super::{
    MissingObjectsRequest, MissingObjectsResponse, Publication, PublicationResolution,
    RemoteClient, with_retries,
};

mod publication;
mod uploads;

impl RemoteClient {
    pub fn publish_snap(
        &self,
        store: &LocalStore,
        snap: &SnapRecord,
        scope: &str,
        gate: &str,
    ) -> Result<Publication> {
        self.publish_snap_with_resolution(store, snap, scope, gate, None)
    }

    pub fn publish_snap_metadata_only(
        &self,
        store: &LocalStore,
        snap: &SnapRecord,
        scope: &str,
        gate: &str,
    ) -> Result<Publication> {
        self.publish_snap_inner(store, snap, scope, gate, None, true)
    }

    pub fn publish_snap_with_resolution(
        &self,
        store: &LocalStore,
        snap: &SnapRecord,
        scope: &str,
        gate: &str,
        resolution: Option<PublicationResolution>,
    ) -> Result<Publication> {
        self.publish_snap_inner(store, snap, scope, gate, resolution, false)
    }

    fn publish_snap_inner(
        &self,
        store: &LocalStore,
        snap: &SnapRecord,
        scope: &str,
        gate: &str,
        resolution: Option<PublicationResolution>,
        metadata_only: bool,
    ) -> Result<Publication> {
        let (blobs, manifests, recipes) = collect_objects(store, &snap.root_manifest)?;
        let manifest_order = manifest_postorder(store, &snap.root_manifest)?;

        let repo = &self.remote.repo_id;
        let missing = request_missing_objects(self, repo, &blobs, &manifests, &recipes, snap)?;

        uploads::upload_missing_objects(
            self,
            store,
            repo,
            snap,
            &manifest_order,
            missing,
            metadata_only,
        )?;

        publication::create_publication(self, repo, snap, scope, gate, metadata_only, resolution)
    }
}

fn request_missing_objects(
    client: &RemoteClient,
    repo: &str,
    blobs: &HashSet<String>,
    manifests: &HashSet<String>,
    recipes: &HashSet<String>,
    snap: &SnapRecord,
) -> Result<MissingObjectsResponse> {
    let resp = with_retries("missing objects request", || {
        client
            .client
            .post(client.url(&format!("/repos/{}/objects/missing", repo)))
            .header(reqwest::header::AUTHORIZATION, client.auth())
            .json(&MissingObjectsRequest {
                blobs: blobs.iter().cloned().collect(),
                manifests: manifests.iter().cloned().collect(),
                recipes: recipes.iter().cloned().collect(),
                snaps: vec![snap.id.clone()],
            })
            .send()
            .context("send")
    })?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!(
            "remote repo not found (create it with `converge remote create-repo` or POST /repos)"
        );
    }

    let resp = client.ensure_ok(resp, "missing objects")?;
    resp.json().context("parse missing objects")
}
