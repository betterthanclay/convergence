use ratatui::widgets::ListItem;

use super::*;

pub(super) fn list_rows(view: &GateGraphView) -> Vec<ListItem<'static>> {
    let mut rows = Vec::new();
    for g in &view.graph.gates {
        let tag = if g.allow_releases { "" } else { "no-releases" };
        if tag.is_empty() {
            rows.push(ListItem::new(g.id.to_string()));
        } else {
            rows.push(ListItem::new(format!("{} {}", g.id, tag)));
        }
    }
    if rows.is_empty() {
        rows.push(ListItem::new("(empty)"));
    }
    rows
}

pub(super) fn list_title(view: &GateGraphView) -> String {
    let releases_enabled = view.graph.gates.iter().filter(|g| g.allow_releases).count();
    format!(
        "gates={} releases_enabled={} (/ for commands)",
        view.graph.gates.len(),
        releases_enabled
    )
}
