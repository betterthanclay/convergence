use std::fs;

use anyhow::{Context, Result};

use crate::model::{LaneSyncRecord, RemoteConfig, WorkspaceState};

use super::{LocalStore, write_atomic};

impl LocalStore {
    pub fn read_state(&self) -> Result<WorkspaceState> {
        let path = self.root.join("state.json");
        if !path.exists() {
            return Ok(WorkspaceState {
                version: 1,
                lane_sync: std::collections::HashMap::new(),
                remote_tokens: std::collections::HashMap::new(),
                last_published: std::collections::HashMap::new(),
            });
        }
        let bytes = fs::read(&path).context("read state.json")?;
        let st: WorkspaceState = serde_json::from_slice(&bytes).context("parse state.json")?;
        Ok(st)
    }

    pub fn write_state(&self, st: &WorkspaceState) -> Result<()> {
        let bytes = serde_json::to_vec_pretty(st).context("serialize state")?;
        write_atomic(&self.root.join("state.json"), &bytes).context("write state.json")?;
        Ok(())
    }

    pub fn set_lane_sync(&self, lane_id: &str, snap_id: &str, synced_at: &str) -> Result<()> {
        let mut st = self.read_state()?;
        if st.version != 1 {
            anyhow::bail!("unsupported workspace state version {}", st.version);
        }
        st.lane_sync.insert(
            lane_id.to_string(),
            LaneSyncRecord {
                snap_id: snap_id.to_string(),
                synced_at: synced_at.to_string(),
            },
        );
        self.write_state(&st)
    }

    pub fn remote_token_key(&self, remote: &RemoteConfig) -> String {
        format!("{}#{}", remote.base_url, remote.repo_id)
    }

    fn publish_key(&self, remote: &RemoteConfig, scope: &str, gate: &str) -> String {
        format!("{}#{}#{}#{}", remote.base_url, remote.repo_id, scope, gate)
    }

    pub fn get_last_published(
        &self,
        remote: &RemoteConfig,
        scope: &str,
        gate: &str,
    ) -> Result<Option<String>> {
        let st = self.read_state()?;
        if st.version != 1 {
            anyhow::bail!("unsupported workspace state version {}", st.version);
        }
        Ok(st
            .last_published
            .get(&self.publish_key(remote, scope, gate))
            .cloned())
    }

    pub fn set_last_published(
        &self,
        remote: &RemoteConfig,
        scope: &str,
        gate: &str,
        snap_id: &str,
    ) -> Result<()> {
        let mut st = self.read_state()?;
        if st.version != 1 {
            anyhow::bail!("unsupported workspace state version {}", st.version);
        }
        st.last_published
            .insert(self.publish_key(remote, scope, gate), snap_id.to_string());
        self.write_state(&st)
    }

    pub fn get_remote_token(&self, remote: &RemoteConfig) -> Result<Option<String>> {
        let st = self.read_state()?;
        if st.version != 1 {
            anyhow::bail!("unsupported workspace state version {}", st.version);
        }
        Ok(st
            .remote_tokens
            .get(&self.remote_token_key(remote))
            .cloned())
    }

    pub fn set_remote_token(&self, remote: &RemoteConfig, token: &str) -> Result<()> {
        let mut st = self.read_state()?;
        if st.version != 1 {
            anyhow::bail!("unsupported workspace state version {}", st.version);
        }
        st.remote_tokens
            .insert(self.remote_token_key(remote), token.to_string());
        self.write_state(&st)
    }

    pub fn clear_remote_token(&self, remote: &RemoteConfig) -> Result<()> {
        let mut st = self.read_state()?;
        if st.version != 1 {
            anyhow::bail!("unsupported workspace state version {}", st.version);
        }
        st.remote_tokens.remove(&self.remote_token_key(remote));
        self.write_state(&st)
    }
}
