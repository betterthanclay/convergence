use ratatui::text::Line;

use super::*;

pub(super) fn details_lines(view: &GateGraphView) -> Vec<Line<'static>> {
    if view.graph.gates.is_empty() {
        return vec![Line::from("(no selection)")];
    }

    let idx = view.selected.min(view.graph.gates.len().saturating_sub(1));
    let g = &view.graph.gates[idx];
    let mut out = Vec::new();
    out.push(Line::from(format!("id: {}", g.id)));
    out.push(Line::from(format!("name: {}", g.name)));
    if g.upstream.is_empty() {
        out.push(Line::from("upstream: (none)"));
    } else {
        out.push(Line::from(format!("upstream: {}", g.upstream.join(", "))));
    }
    out.push(Line::from(""));
    out.push(Line::from("policy:"));
    out.push(Line::from(format!("allow_releases: {}", g.allow_releases)));
    out.push(Line::from(format!(
        "allow_superpositions: {}",
        g.allow_superpositions
    )));
    out.push(Line::from(format!(
        "allow_metadata_only_publications: {}",
        g.allow_metadata_only_publications
    )));
    out.push(Line::from(format!(
        "required_approvals: {}",
        g.required_approvals
    )));
    out
}
