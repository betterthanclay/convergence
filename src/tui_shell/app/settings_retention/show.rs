use super::*;

pub(super) fn show_retention(app: &mut App, ws: &Workspace) {
    let cfg = match ws.store.read_config() {
        Ok(c) => c,
        Err(err) => {
            app.push_error(format!("read config: {:#}", err));
            return;
        }
    };
    let r = cfg.retention.unwrap_or_default();
    let mut lines = Vec::new();
    lines.push(format!(
        "keep_last: {}",
        r.keep_last
            .map(|n| n.to_string())
            .unwrap_or_else(|| "(unset)".to_string())
    ));
    lines.push(format!(
        "keep_days: {}",
        r.keep_days
            .map(|n| n.to_string())
            .unwrap_or_else(|| "(unset)".to_string())
    ));
    lines.push(format!("prune_snaps: {}", r.prune_snaps));
    lines.push(format!("pinned: {}", r.pinned.len()));
    for p in r.pinned {
        lines.push(format!("  - {}", p));
    }
    app.open_modal("Retention", lines);
}
