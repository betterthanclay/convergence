use super::super::super::*;

pub(super) fn load_snap_ids_from_disk(state: &AppState, repo_id: &str) -> Result<HashSet<String>> {
    let dir = repo_data_dir(state, repo_id).join("objects/snaps");
    if !dir.is_dir() {
        return Ok(HashSet::new());
    }

    let mut out = HashSet::new();
    for entry in std::fs::read_dir(&dir).context("read snaps dir")? {
        let entry = entry.context("read snaps dir entry")?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if stem.len() == 64 {
            out.insert(stem.to_string());
        }
    }
    Ok(out)
}

pub(super) fn load_bundles_from_disk(state: &AppState, repo_id: &str) -> Result<Vec<Bundle>> {
    let dir = repo_data_dir(state, repo_id).join("bundles");
    if !dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).context("read bundles dir")? {
        let entry = entry.context("read bundles dir entry")?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let bytes = std::fs::read(&path).with_context(|| format!("read {}", path.display()))?;
        let bundle: Bundle =
            serde_json::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))?;
        out.push(bundle);
    }
    out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(out)
}

pub(super) fn load_promotions_from_disk(state: &AppState, repo_id: &str) -> Result<Vec<Promotion>> {
    let dir = repo_data_dir(state, repo_id).join("promotions");
    if !dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).context("read promotions dir")? {
        let entry = entry.context("read promotions dir entry")?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let bytes = std::fs::read(&path).with_context(|| format!("read {}", path.display()))?;
        let p: Promotion =
            serde_json::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))?;
        out.push(p);
    }
    out.sort_by(|a, b| b.promoted_at.cmp(&a.promoted_at));
    Ok(out)
}

pub(super) fn load_releases_from_disk(state: &AppState, repo_id: &str) -> Result<Vec<Release>> {
    let dir = repo_data_dir(state, repo_id).join("releases");
    if !dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).context("read releases dir")? {
        let entry = entry.context("read releases dir entry")?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let bytes = std::fs::read(&path).with_context(|| format!("read {}", path.display()))?;
        let r: Release =
            serde_json::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))?;
        out.push(r);
    }
    out.sort_by(|a, b| b.released_at.cmp(&a.released_at));
    Ok(out)
}
