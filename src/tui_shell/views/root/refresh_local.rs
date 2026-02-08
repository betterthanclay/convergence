use crate::tui_shell::status::{
    ChangeSummary, collapse_blank_lines, extract_baseline_compact, extract_change_keys,
    extract_change_summary, jaccard_similarity,
};

use super::RootView;

pub(super) fn refresh_local_state(
    view: &mut RootView,
    lines: Vec<String>,
    prev_lines_len: usize,
    prev_baseline: Option<String>,
    prev_keys: Vec<String>,
) {
    let (summary, lines) = extract_change_summary(lines);
    view.change_summary = summary;
    view.baseline_compact = extract_baseline_compact(&lines);

    let new_lines = collapse_blank_lines(lines);
    let new_keys = extract_change_keys(&new_lines);
    view.change_keys = new_keys.clone();

    let significant = significant_change(
        &prev_baseline,
        &view.baseline_compact,
        &prev_keys,
        &new_keys,
    );

    let new_len = new_lines.len();
    let max_scroll = new_len.saturating_sub(1);
    if significant && view.scroll > 0 {
        view.scroll = 0;
    } else if prev_lines_len > 0 && new_len > 0 {
        view.scroll = view.scroll.min(max_scroll);
    } else {
        view.scroll = 0;
    }

    view.lines = new_lines;
}

pub(super) fn clear_local_tracking_for_remote(view: &mut RootView, lines: Vec<String>) {
    view.change_summary = ChangeSummary::default();
    view.baseline_compact = None;
    view.change_keys.clear();
    view.lines = lines;
    view.scroll = 0;
}

fn significant_change(
    prev_baseline: &Option<String>,
    new_baseline: &Option<String>,
    prev_keys: &[String],
    new_keys: &[String],
) -> bool {
    if prev_baseline != new_baseline {
        return true;
    }

    let old_count = prev_keys.len();
    let new_count = new_keys.len();
    if old_count >= 10 && new_count >= 10 {
        let jac = jaccard_similarity(prev_keys, new_keys);
        return jac < 0.40;
    }

    // For small lists, treat size spikes as significant.
    let delta = old_count.abs_diff(new_count);
    delta >= 25 && (delta as f64) / ((old_count.max(new_count)) as f64) > 0.50
}
