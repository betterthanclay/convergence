use crate::workspace::Workspace;

mod summary_utils;
pub(super) use self::summary_utils::{
    ChangeSummary, collapse_blank_lines, extract_baseline_compact, extract_change_keys,
    extract_change_summary, jaccard_similarity,
};
mod identity_collect;
mod local_status;
mod remote_status;
mod rename_helpers;
mod rename_io;
mod rename_match;
mod text_delta;
mod tree_diff;
mod tree_walk;
pub(in crate::tui_shell) use self::local_status::local_status_lines;
pub(in crate::tui_shell) use self::remote_status::{
    DashboardData, dashboard_data, remote_status_lines,
};
use self::rename_helpers::default_chunk_size_bytes;

#[cfg(test)]
use self::rename_helpers::StatusChange;
#[cfg(test)]
use self::tree_diff::diff_trees_with_renames;

#[cfg(test)]
mod rename_tests;

fn chunk_size_bytes_from_workspace(ws: &Workspace) -> usize {
    let cfg = ws.store.read_config().ok();
    let chunk_size = cfg
        .as_ref()
        .and_then(|c| c.chunking.as_ref().map(|x| x.chunk_size))
        .unwrap_or(default_chunk_size_bytes() as u64);
    let chunk_size = chunk_size.max(64 * 1024);
    usize::try_from(chunk_size).unwrap_or(default_chunk_size_bytes())
}
