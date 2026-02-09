use std::collections::HashSet;

use anyhow::{Context, Result};

use crate::model::{ObjectId, SnapRecord};
use crate::store::LocalStore;

use super::super::fetch::{collect_objects, manifest_postorder};
use super::super::{
    LaneHead, MissingObjectsRequest, MissingObjectsResponse, RemoteClient, with_retries,
};

impl RemoteClient {
    pub fn upload_snap_objects(&self, store: &LocalStore, snap: &SnapRecord) -> Result<()> {
        // Reuse the publish upload path but skip publication creation.
        let (blobs, manifests, recipes) = collect_objects(store, &snap.root_manifest)?;
        let manifest_order = manifest_postorder(store, &snap.root_manifest)?;

        let repo = &self.remote.repo_id;
        let resp = with_retries("missing objects request", || {
            self.client
                .post(self.url(&format!("/repos/{}/objects/missing", repo)))
                .header(reqwest::header::AUTHORIZATION, self.auth())
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
        let resp = self.ensure_ok(resp, "missing objects")?;
        let missing: MissingObjectsResponse = resp.json().context("parse missing objects")?;

        for id in missing.missing_blobs {
            let bytes = store.get_blob(&ObjectId(id.clone()))?;
            with_retries(&format!("upload blob {}", id), || {
                let resp = self
                    .client
                    .put(self.url(&format!("/repos/{}/objects/blobs/{}", repo, id)))
                    .header(reqwest::header::AUTHORIZATION, self.auth())
                    .body(bytes.clone())
                    .send()
                    .context("send")?;
                self.ensure_ok(resp, "upload blob")
            })?;
        }

        for id in missing.missing_recipes {
            let rid = ObjectId(id.clone());
            let bytes = store.get_recipe_bytes(&rid)?;
            with_retries(&format!("upload recipe {}", id), || {
                let resp = self
                    .client
                    .put(self.url(&format!("/repos/{}/objects/recipes/{}", repo, id)))
                    .header(reqwest::header::AUTHORIZATION, self.auth())
                    .body(bytes.clone())
                    .send()
                    .context("send")?;
                self.ensure_ok(resp, "upload recipe")
            })?;
        }

        let mut missing_manifests: HashSet<String> =
            missing.missing_manifests.into_iter().collect();
        for mid in manifest_order {
            let id = mid.as_str();
            if !missing_manifests.remove(id) {
                continue;
            }
            let bytes = store.get_manifest_bytes(&mid)?;
            with_retries(&format!("upload manifest {}", id), || {
                let resp = self
                    .client
                    .put(self.url(&format!("/repos/{}/objects/manifests/{}", repo, id)))
                    .header(reqwest::header::AUTHORIZATION, self.auth())
                    .body(bytes.clone())
                    .send()
                    .context("send")?;
                self.ensure_ok(resp, "upload manifest")
            })?;
        }
        if !missing_manifests.is_empty() {
            anyhow::bail!("missing manifest postorder invariant violated");
        }

        // Upload snap record last.
        if missing.missing_snaps.contains(&snap.id) {
            with_retries("upload snap", || {
                let resp = self
                    .client
                    .put(self.url(&format!("/repos/{}/objects/snaps/{}", repo, snap.id)))
                    .header(reqwest::header::AUTHORIZATION, self.auth())
                    .json(snap)
                    .send()
                    .context("send")?;
                self.ensure_ok(resp, "upload snap")
            })?;
        }

        Ok(())
    }

    pub fn sync_snap(
        &self,
        store: &LocalStore,
        snap: &SnapRecord,
        lane_id: &str,
        client_id: Option<String>,
    ) -> Result<LaneHead> {
        self.upload_snap_objects(store, snap)?;
        self.update_lane_head_me(lane_id, &snap.id, client_id)
    }
}
