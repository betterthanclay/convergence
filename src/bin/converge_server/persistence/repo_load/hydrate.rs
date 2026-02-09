use super::super::super::*;

use super::collection_loaders::{
    load_bundles_from_disk, load_promotions_from_disk, load_releases_from_disk,
    load_snap_ids_from_disk,
};
use super::promotion_state::rebuild_promotion_state;

pub(super) fn load_repos_from_disk(
    state: &AppState,
    handle_to_id: &HashMap<String, String>,
) -> Result<HashMap<String, Repo>> {
    let mut out = HashMap::new();
    if !state.data_dir.is_dir() {
        return Ok(out);
    }

    for entry in std::fs::read_dir(&state.data_dir).context("read data dir")? {
        let entry = entry.context("read data dir entry")?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let repo_id = entry
            .file_name()
            .into_string()
            .map_err(|_| anyhow::anyhow!("non-utf8 repo dir name"))?;

        let repo = load_repo_from_disk(state, &repo_id, handle_to_id)
            .with_context(|| format!("load repo {}", repo_id))?;
        out.insert(repo_id, repo);
    }

    Ok(out)
}

fn load_repo_from_disk(
    state: &AppState,
    repo_id: &str,
    handle_to_id: &HashMap<String, String>,
) -> Result<Repo> {
    let mut repo = if repo_state_path(state, repo_id).exists() {
        let bytes = std::fs::read(repo_state_path(state, repo_id)).context("read repo.json")?;
        serde_json::from_slice::<Repo>(&bytes).context("parse repo.json")?
    } else {
        default_repo_state(state, repo_id)
    };

    // Ensure id matches directory (best-effort).
    repo.id = repo_id.to_string();

    // Hydrate lists from existing on-disk records (needed for older data dirs).
    let snaps = load_snap_ids_from_disk(state, repo_id).unwrap_or_default();
    if !snaps.is_empty() {
        repo.snaps = snaps;
    }

    let bundles = load_bundles_from_disk(state, repo_id).unwrap_or_default();
    if !bundles.is_empty() {
        repo.bundles = bundles;
    }

    let promotions = load_promotions_from_disk(state, repo_id).unwrap_or_default();
    if !promotions.is_empty() {
        repo.promotions = promotions;
        repo.promotion_state = rebuild_promotion_state(&repo.promotions);
    }

    let releases = load_releases_from_disk(state, repo_id).unwrap_or_default();
    if !releases.is_empty() {
        repo.releases = releases;
    }

    // Backfill user_id fields for older on-disk records (best-effort).
    backfill_provenance_user_ids(&mut repo, handle_to_id);
    backfill_acl_user_ids(&mut repo, handle_to_id);

    Ok(repo)
}
