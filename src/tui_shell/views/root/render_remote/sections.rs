use ratatui::text::Line;

use crate::tui_shell::status::DashboardData;

pub(super) fn action_lines(d: &DashboardData) -> Vec<Line<'static>> {
    let mut action_lines: Vec<Line<'static>> = Vec::new();
    if d.next_actions.is_empty() {
        action_lines.push(Line::from("(none)"));
    } else {
        for a in &d.next_actions {
            action_lines.push(Line::from(format!("- {}", a)));
        }
    }
    action_lines
}

pub(super) fn inbox_lines(d: &DashboardData) -> Vec<Line<'static>> {
    let mut inbox_lines: Vec<Line<'static>> = Vec::new();
    inbox_lines.push(Line::from(format!(
        "{} total  {} pending  {} resolved",
        d.inbox_total, d.inbox_pending, d.inbox_resolved
    )));
    if d.inbox_missing_local > 0 {
        inbox_lines.push(Line::from(format!(
            "{} snaps missing locally",
            d.inbox_missing_local
        )));
    }
    if let Some((sid, ts)) = &d.latest_publication {
        inbox_lines.push(Line::from(format!("latest: {} {}", sid, ts)));
    }
    inbox_lines
}

pub(super) fn bundle_lines(d: &DashboardData) -> Vec<Line<'static>> {
    let mut bundle_lines: Vec<Line<'static>> = Vec::new();
    bundle_lines.push(Line::from(format!(
        "{} total  {} promotable  {} blocked",
        d.bundles_total, d.bundles_promotable, d.bundles_blocked
    )));
    if d.blocked_superpositions > 0 {
        bundle_lines.push(Line::from(format!(
            "blocked by superpositions: {}",
            d.blocked_superpositions
        )));
    }
    if d.blocked_approvals > 0 {
        bundle_lines.push(Line::from(format!(
            "blocked by approvals: {}",
            d.blocked_approvals
        )));
    }
    if d.pinned_bundles > 0 {
        bundle_lines.push(Line::from(format!("pinned: {}", d.pinned_bundles)));
    }
    bundle_lines
}

pub(super) fn gate_lines(d: &DashboardData) -> Vec<Line<'static>> {
    let mut gate_lines: Vec<Line<'static>> = Vec::new();
    if let Some(h) = &d.healthz {
        gate_lines.push(Line::from(format!("healthz: {}", h)));
    }
    if d.gates_total > 0 {
        gate_lines.push(Line::from(format!("gates: {}", d.gates_total)));
    }
    if !d.promotion_state.is_empty() {
        gate_lines.push(Line::from("promotion_state:"));
        for (gate, bid) in d.promotion_state.iter().take(4) {
            gate_lines.push(Line::from(format!("{} {}", gate, bid)));
        }
    }
    gate_lines
}

pub(super) fn release_lines(d: &DashboardData) -> Vec<Line<'static>> {
    let mut rel_lines: Vec<Line<'static>> = Vec::new();
    if d.releases_total == 0 {
        rel_lines.push(Line::from("(none)"));
    } else {
        rel_lines.push(Line::from(format!(
            "{} total ({} channels)",
            d.releases_total, d.releases_channels
        )));
        for (ch, bid, ts) in d.latest_releases.iter() {
            rel_lines.push(Line::from(format!("{} {} {}", ch, bid, ts)));
        }
    }
    rel_lines
}
