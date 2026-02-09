//! Remote fetch/read paths and recursive manifest/object retrieval helpers.

use anyhow::{Context, Result};

use crate::model::{ObjectId, SnapRecord};
use crate::store::LocalStore;

use super::{RemoteClient, with_retries};

mod manifest_tree;
mod object_graph;

pub(super) use self::object_graph::{collect_objects, manifest_postorder};

impl RemoteClient {
    pub fn fetch_publications(
        &self,
        store: &LocalStore,
        only_snap: Option<&str>,
    ) -> Result<Vec<String>> {
        let repo = &self.remote.repo_id;
        let pubs = self.list_publications()?;
        let pubs = pubs
            .into_iter()
            .filter(|p| only_snap.map(|s| p.snap_id == s).unwrap_or(true))
            .collect::<Vec<_>>();

        let mut fetched = Vec::new();
        for p in pubs {
            if let Some(id) = self.fetch_snap_by_id(store, repo, &p.snap_id)? {
                fetched.push(id);
            }
        }

        Ok(fetched)
    }

    pub fn fetch_manifest_tree(&self, store: &LocalStore, root_manifest: &ObjectId) -> Result<()> {
        let repo = &self.remote.repo_id;
        manifest_tree::fetch_manifest_tree(store, self, repo, root_manifest)
    }

    pub fn fetch_lane_heads(
        &self,
        store: &LocalStore,
        lane_id: &str,
        user: Option<&str>,
    ) -> Result<Vec<String>> {
        let repo = &self.remote.repo_id;

        let snap_ids: Vec<String> = if let Some(user) = user {
            vec![self.get_lane_head(lane_id, user)?.snap_id]
        } else {
            let lanes = self.list_lanes()?;
            let lane = lanes
                .into_iter()
                .find(|l| l.id == lane_id)
                .with_context(|| format!("lane not found: {}", lane_id))?;
            lane.heads.values().map(|h| h.snap_id.clone()).collect()
        };

        let mut fetched = Vec::new();
        for sid in snap_ids {
            if let Some(id) = self.fetch_snap_by_id(store, repo, &sid)? {
                fetched.push(id);
            }
        }
        Ok(fetched)
    }

    fn fetch_snap_by_id(
        &self,
        store: &LocalStore,
        repo: &str,
        snap_id: &str,
    ) -> Result<Option<String>> {
        if store.has_snap(snap_id) {
            return Ok(None);
        }

        let snap_bytes = with_retries(&format!("fetch snap {}", snap_id), || {
            let resp = self
                .client
                .get(self.url(&format!("/repos/{}/objects/snaps/{}", repo, snap_id)))
                .header(reqwest::header::AUTHORIZATION, self.auth())
                .send()
                .context("send")?;
            self.ensure_ok(resp, "fetch snap")?.bytes().context("bytes")
        })?;

        let snap: SnapRecord = serde_json::from_slice(&snap_bytes).context("parse snap")?;
        store.put_snap(&snap)?;

        manifest_tree::fetch_manifest_tree(
            store,
            self,
            repo,
            &ObjectId(snap.root_manifest.0.clone()),
        )?;
        Ok(Some(snap.id))
    }
}
