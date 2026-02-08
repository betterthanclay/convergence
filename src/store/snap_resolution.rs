use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};

use crate::model::{Resolution, SnapRecord};

use super::{LocalStore, write_atomic};

impl LocalStore {
    pub fn put_snap(&self, snap: &SnapRecord) -> Result<()> {
        let path = self.root.join("snaps").join(format!("{}.json", snap.id));
        let bytes = serde_json::to_vec_pretty(snap).context("serialize snap")?;
        write_atomic(&path, &bytes).context("write snap")?;
        Ok(())
    }

    pub fn has_snap(&self, snap_id: &str) -> bool {
        self.root
            .join("snaps")
            .join(format!("{}.json", snap_id))
            .exists()
    }

    pub fn get_snap(&self, snap_id: &str) -> Result<SnapRecord> {
        let path = self.root.join("snaps").join(format!("{}.json", snap_id));
        let bytes = fs::read(&path).with_context(|| format!("read snap {}", snap_id))?;
        let s: SnapRecord =
            serde_json::from_slice(&bytes).with_context(|| format!("parse snap {}", snap_id))?;
        Ok(s)
    }

    pub fn list_snaps(&self) -> Result<Vec<SnapRecord>> {
        let mut out = Vec::new();
        let dir = self.root.join("snaps");
        if !dir.is_dir() {
            return Ok(out);
        }

        for entry in fs::read_dir(&dir).context("read snaps dir")? {
            let entry = entry.context("read snaps dir entry")?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }
            let bytes =
                fs::read(&path).with_context(|| format!("read snap file {}", path.display()))?;
            let snap: SnapRecord = serde_json::from_slice(&bytes)
                .with_context(|| format!("parse snap file {}", path.display()))?;
            out.push(snap);
        }
        Ok(out)
    }

    pub fn delete_snap(&self, snap_id: &str) -> Result<()> {
        let path = self.root.join("snaps").join(format!("{}.json", snap_id));
        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("remove snap file {}", path.display()))?;
        }
        Ok(())
    }

    pub fn put_resolution(&self, resolution: &Resolution) -> Result<()> {
        if resolution.version != 1 && resolution.version != 2 {
            return Err(anyhow!("unsupported resolution version"));
        }
        let bytes = serde_json::to_vec_pretty(resolution).context("serialize resolution")?;
        let path = self
            .root
            .join("resolutions")
            .join(format!("{}.json", resolution.bundle_id));
        write_atomic(&path, &bytes).context("write resolution")?;
        Ok(())
    }

    pub fn get_resolution(&self, bundle_id: &str) -> Result<Resolution> {
        let path = self
            .root
            .join("resolutions")
            .join(format!("{}.json", bundle_id));
        let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
        let r: Resolution = serde_json::from_slice(&bytes).context("parse resolution")?;
        if r.version != 1 && r.version != 2 {
            return Err(anyhow!("unsupported resolution version"));
        }
        if r.bundle_id != bundle_id {
            return Err(anyhow!("resolution bundle_id mismatch"));
        }
        Ok(r)
    }

    pub fn has_resolution(&self, bundle_id: &str) -> bool {
        self.root
            .join("resolutions")
            .join(format!("{}.json", bundle_id))
            .exists()
    }

    pub fn update_snap_message(&self, snap_id: &str, message: Option<&str>) -> Result<()> {
        let mut snap = self.get_snap(snap_id)?;
        let msg = message
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        snap.message = msg;
        self.put_snap(&snap)
    }

    fn head_path(&self) -> PathBuf {
        self.root.join("HEAD")
    }

    pub fn get_head(&self) -> Result<Option<String>> {
        let path = self.head_path();
        if !path.exists() {
            return Ok(None);
        }
        let s =
            fs::read_to_string(&path).with_context(|| format!("read head {}", path.display()))?;
        let s = s.trim().to_string();
        if s.is_empty() { Ok(None) } else { Ok(Some(s)) }
    }

    pub fn set_head(&self, snap_id: Option<&str>) -> Result<()> {
        let path = self.head_path();
        match snap_id {
            None => {
                if path.exists() {
                    fs::remove_file(&path).with_context(|| format!("remove {}", path.display()))?;
                }
                Ok(())
            }
            Some(id) => {
                write_atomic(&path, id.as_bytes()).context("write head")?;
                Ok(())
            }
        }
    }
}
