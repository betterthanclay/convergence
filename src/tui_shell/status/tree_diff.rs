use anyhow::Result;

use crate::model::{Manifest, ObjectId};
use crate::store::LocalStore;

use super::identity_collect::{collect_identities_base, collect_identities_current};
use super::rename_helpers::StatusChange;
use super::rename_match::detect_renames;
use super::tree_walk::{StatusDelta, diff_trees};
pub(super) fn diff_trees_with_renames(
    store: &LocalStore,
    base_root: Option<&ObjectId>,
    cur_root: &ObjectId,
    cur_manifests: &std::collections::HashMap<ObjectId, Manifest>,
    workspace_root: Option<&std::path::Path>,
    chunk_size_bytes: usize,
) -> Result<Vec<StatusChange>> {
    let raw = diff_trees(store, base_root, cur_root, cur_manifests)?;
    let Some(base_root) = base_root else {
        return Ok(raw
            .into_iter()
            .map(|(k, p)| match k {
                StatusDelta::Added => StatusChange::Added(p),
                StatusDelta::Modified => StatusChange::Modified(p),
                StatusDelta::Deleted => StatusChange::Deleted(p),
            })
            .collect());
    };

    let mut base_ids = std::collections::HashMap::new();
    collect_identities_base("", store, base_root, &mut base_ids)?;

    let mut cur_ids = std::collections::HashMap::new();
    collect_identities_current("", cur_root, cur_manifests, &mut cur_ids)?;

    let mut added = Vec::new();
    let mut modified = Vec::new();
    let mut deleted = Vec::new();
    for (k, p) in raw {
        match k {
            StatusDelta::Added => added.push(p),
            StatusDelta::Modified => modified.push(p),
            StatusDelta::Deleted => deleted.push(p),
        }
    }

    let rename_detection = detect_renames(
        store,
        workspace_root,
        chunk_size_bytes,
        &added,
        &deleted,
        &base_ids,
        &cur_ids,
    );
    let renames = rename_detection.renames;
    let consumed_added = rename_detection.consumed_added;
    let consumed_deleted = rename_detection.consumed_deleted;

    let mut out = Vec::new();
    for p in modified {
        out.push(StatusChange::Modified(p));
    }
    for (from, to, modified) in renames {
        out.push(StatusChange::Renamed { from, to, modified });
    }
    for p in added {
        if !consumed_added.contains(&p) {
            out.push(StatusChange::Added(p));
        }
    }
    for p in deleted {
        if !consumed_deleted.contains(&p) {
            out.push(StatusChange::Deleted(p));
        }
    }

    out.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
    Ok(out)
}
