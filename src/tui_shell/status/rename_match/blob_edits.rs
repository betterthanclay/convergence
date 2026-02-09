use std::collections::HashSet;

use super::super::rename_helpers::{
    IdentityKey, blob_prefix_suffix_score, min_blob_rename_matched_bytes, min_blob_rename_score,
};
use super::super::rename_io::load_blob_bytes;
use super::MatchCtx;

pub(super) fn detect_blob_edit_renames(ctx: &mut MatchCtx<'_>) {
    const MAX_BYTES: usize = 1024 * 1024;

    let mut remaining_added_blobs = Vec::new();
    for path in ctx.added {
        if ctx.consumed_added.contains(path) {
            continue;
        }
        let Some(id) = ctx.cur_ids.get(path) else {
            continue;
        };
        let IdentityKey::Blob(blob) = id else {
            continue;
        };
        remaining_added_blobs.push((path.clone(), blob.clone()));
    }

    let mut remaining_deleted_blobs = Vec::new();
    for path in ctx.deleted {
        if ctx.consumed_deleted.contains(path) {
            continue;
        }
        let Some(id) = ctx.base_ids.get(path) else {
            continue;
        };
        let IdentityKey::Blob(blob) = id else {
            continue;
        };
        remaining_deleted_blobs.push((path.clone(), blob.clone()));
    }

    let mut used_added: HashSet<String> = HashSet::new();
    for (from_path, from_blob) in remaining_deleted_blobs {
        let Some(from_bytes) = load_blob_bytes(ctx.store, None, "", &from_blob) else {
            continue;
        };
        if from_bytes.len() > MAX_BYTES {
            continue;
        }

        let mut best: Option<(String, String, f64)> = None;
        for (to_path, to_blob) in &remaining_added_blobs {
            if used_added.contains(to_path) {
                continue;
            }
            let Some(to_bytes) = load_blob_bytes(ctx.store, ctx.workspace_root, to_path, to_blob)
            else {
                continue;
            };
            if to_bytes.len() > MAX_BYTES {
                continue;
            }

            let diff = from_bytes.len().abs_diff(to_bytes.len());
            let max = from_bytes.len().max(to_bytes.len());
            if diff > 8192 && (diff as f64) / (max as f64) > 0.20 {
                continue;
            }

            let (prefix, suffix, max_len, score) = blob_prefix_suffix_score(&from_bytes, &to_bytes);
            let min_score = min_blob_rename_score(max_len);
            let min_matched = min_blob_rename_matched_bytes(max_len);
            if score >= min_score && (prefix + suffix) >= min_matched {
                match &best {
                    None => best = Some((to_path.clone(), to_blob.clone(), score)),
                    Some((_, _, best_score)) if score > *best_score => {
                        best = Some((to_path.clone(), to_blob.clone(), score))
                    }
                    _ => {}
                }
            }
        }

        if let Some((to_path, _to_blob, _score)) = best {
            used_added.insert(to_path.clone());
            ctx.consumed_deleted.insert(from_path.clone());
            ctx.consumed_added.insert(to_path.clone());
            ctx.renames.push((from_path, to_path, true));
        }
    }
}
