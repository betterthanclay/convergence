use anyhow::Result;

use crate::model::ObjectId;
use crate::tui_shell::{RenderCtx, fmt_ts_list};
use crate::workspace::Workspace;

use super::chunk_size_bytes_from_workspace;
use super::identity_collect::{collect_identities_base, collect_identities_current};
use super::rename_helpers::{IdentityKey, StatusChange};
use super::text_delta::{count_lines_utf8, fmt_line_delta, line_delta_utf8};
use super::tree_diff::diff_trees_with_renames;

pub(in crate::tui_shell) fn local_status_lines(
    ws: &Workspace,
    ctx: &RenderCtx,
) -> Result<Vec<String>> {
    let snaps = ws.list_snaps()?;

    let mut baseline: Option<crate::model::SnapRecord> = None;
    if let Ok(Some(head_id)) = ws.store.get_head()
        && let Ok(s) = ws.show_snap(&head_id)
    {
        baseline = Some(s);
    }
    if baseline.is_none() {
        baseline = snaps.first().cloned();
    }

    let (cur_root, cur_manifests, _stats) = ws.current_manifest_tree()?;

    let mut lines = Vec::new();
    if let Some(s) = &baseline {
        let short = s.id.chars().take(8).collect::<String>();
        lines.push(format!(
            "baseline: {} {}",
            short,
            fmt_ts_list(&s.created_at, ctx)
        ));
    } else {
        lines.push("baseline: (none; no snaps yet)".to_string());
    }

    let changes = diff_trees_with_renames(
        &ws.store,
        baseline.as_ref().map(|s| &s.root_manifest),
        &cur_root,
        &cur_manifests,
        Some(ws.root.as_path()),
        chunk_size_bytes_from_workspace(ws),
    )?;

    if changes.is_empty() {
        lines.push("".to_string());
        lines.push("Clean".to_string());
        return Ok(lines);
    }

    let mut added = 0;
    let mut modified = 0;
    let mut deleted = 0;
    let mut renamed = 0;
    for c in &changes {
        match c {
            StatusChange::Added(_) => added += 1,
            StatusChange::Modified(_) => modified += 1,
            StatusChange::Deleted(_) => deleted += 1,
            StatusChange::Renamed { .. } => renamed += 1,
        }
    }
    lines.push("".to_string());
    if renamed > 0 {
        lines.push(format!(
            "changes: {} added, {} modified, {} deleted, {} renamed",
            added, modified, deleted, renamed
        ));
    } else {
        lines.push(format!(
            "changes: {} added, {} modified, {} deleted",
            added, modified, deleted
        ));
    }
    lines.push("".to_string());

    let base_ids = if let Some(s) = &baseline {
        let mut m = std::collections::HashMap::new();
        collect_identities_base("", &ws.store, &s.root_manifest, &mut m)?;
        Some(m)
    } else {
        None
    };
    let mut cur_ids = std::collections::HashMap::new();
    collect_identities_current("", &cur_root, &cur_manifests, &mut cur_ids)?;

    const MAX: usize = 200;
    let more = changes.len().saturating_sub(MAX);
    for (i, c) in changes.into_iter().enumerate() {
        if i >= MAX {
            break;
        }

        let delta = match &c {
            StatusChange::Added(p) => {
                let id = cur_ids.get(p);
                if let Some(IdentityKey::Blob(_)) = id {
                    let bytes = std::fs::read(ws.root.join(std::path::Path::new(p))).ok();
                    bytes.and_then(|b| count_lines_utf8(&b)).map(|n| (n, 0))
                } else {
                    None
                }
            }
            StatusChange::Deleted(p) => {
                let id = base_ids.as_ref().and_then(|m| m.get(p));
                if let Some(IdentityKey::Blob(bid)) = id {
                    let bytes = ws.store.get_blob(&ObjectId(bid.clone())).ok();
                    bytes.and_then(|b| count_lines_utf8(&b)).map(|n| (0, n))
                } else {
                    None
                }
            }
            StatusChange::Modified(p) => {
                let base = base_ids.as_ref().and_then(|m| m.get(p));
                let cur = cur_ids.get(p);
                if let (Some(IdentityKey::Blob(bid)), Some(IdentityKey::Blob(_))) = (base, cur) {
                    let old_bytes = ws.store.get_blob(&ObjectId(bid.clone())).ok();
                    let new_bytes = std::fs::read(ws.root.join(std::path::Path::new(p))).ok();
                    if let (Some(a), Some(b)) = (old_bytes, new_bytes) {
                        line_delta_utf8(&a, &b)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            StatusChange::Renamed { from, to, modified } => {
                if !*modified {
                    None
                } else {
                    let base = base_ids.as_ref().and_then(|m| m.get(from));
                    let cur = cur_ids.get(to);
                    if let (Some(IdentityKey::Blob(bid)), Some(IdentityKey::Blob(_))) = (base, cur)
                    {
                        let old_bytes = ws.store.get_blob(&ObjectId(bid.clone())).ok();
                        let new_bytes = std::fs::read(ws.root.join(std::path::Path::new(to))).ok();
                        if let (Some(a), Some(b)) = (old_bytes, new_bytes) {
                            line_delta_utf8(&a, &b)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            }
        };
        let delta_s = delta.map(|(a, d)| fmt_line_delta(a, d)).unwrap_or_default();

        match c {
            StatusChange::Added(p) => lines.push(format!("A {}{}", p, delta_s)),
            StatusChange::Modified(p) => lines.push(format!("M {}{}", p, delta_s)),
            StatusChange::Deleted(p) => lines.push(format!("D {}{}", p, delta_s)),
            StatusChange::Renamed { from, to, modified } => {
                if modified {
                    lines.push(format!("R* {} -> {}{}", from, to, delta_s))
                } else {
                    lines.push(format!("R {} -> {}{}", from, to, delta_s))
                }
            }
        }
    }
    if more > 0 {
        lines.push(format!("... and {} more", more));
    }

    Ok(lines)
}
