use anyhow::Result;

use crate::remote::RemoteClient;
use crate::tui_shell::{RenderCtx, fmt_ts_list, latest_releases_by_channel};
use crate::workspace::Workspace;

#[derive(Debug, Clone)]
pub(in crate::tui_shell) struct DashboardData {
    pub(in crate::tui_shell) healthz: Option<String>,
    pub(in crate::tui_shell) gates_total: usize,

    pub(in crate::tui_shell) inbox_total: usize,
    pub(in crate::tui_shell) inbox_pending: usize,
    pub(in crate::tui_shell) inbox_resolved: usize,
    pub(in crate::tui_shell) inbox_missing_local: usize,
    pub(in crate::tui_shell) latest_publication: Option<(String, String)>,

    pub(in crate::tui_shell) bundles_total: usize,
    pub(in crate::tui_shell) bundles_promotable: usize,
    pub(in crate::tui_shell) bundles_blocked: usize,
    pub(in crate::tui_shell) blocked_superpositions: usize,
    pub(in crate::tui_shell) blocked_approvals: usize,
    pub(in crate::tui_shell) pinned_bundles: usize,

    pub(in crate::tui_shell) promotion_state: Vec<(String, String)>,

    pub(in crate::tui_shell) releases_total: usize,
    pub(in crate::tui_shell) releases_channels: usize,
    pub(in crate::tui_shell) latest_releases: Vec<(String, String, String)>,

    pub(in crate::tui_shell) next_actions: Vec<String>,
}

pub(in crate::tui_shell) fn remote_status_lines(
    ws: &Workspace,
    ctx: &RenderCtx,
) -> Result<Vec<String>> {
    let cfg = ws.store.read_config()?;
    let Some(remote) = cfg.remote else {
        return Ok(vec!["No remote configured".to_string()]);
    };

    let mut lines = Vec::new();
    lines.push(format!("remote: {}", remote.base_url));
    lines.push(format!("repo: {}", remote.repo_id));
    lines.push(format!("scope: {}", remote.scope));
    lines.push(format!("gate: {}", remote.gate));

    let token = ws.store.get_remote_token(&remote)?;
    if token.is_some() {
        lines.push("token: (configured)".to_string());
    } else {
        lines.push("token: (missing; run `login --url ... --token ... --repo ...`)".to_string());
        return Ok(lines);
    }

    let url = format!("{}/healthz", remote.base_url.trim_end_matches('/'));
    let start = std::time::Instant::now();
    match reqwest::blocking::get(&url) {
        Ok(r) => {
            let ms = start.elapsed().as_millis();
            lines.push(format!("healthz: {} {}ms", r.status(), ms));
        }
        Err(err) => {
            lines.push(format!("healthz: error {:#}", err));
        }
    }

    let client = RemoteClient::new(remote.clone(), token.expect("checked is_some above"))?;
    let promotion_state = client.promotion_state(&remote.scope)?;
    lines.push("".to_string());
    lines.push("promotion_state:".to_string());
    if promotion_state.is_empty() {
        lines.push("(none)".to_string());
    } else {
        let mut keys = promotion_state.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        for gate in keys {
            let bid = promotion_state.get(&gate).cloned().unwrap_or_default();
            let short = bid.chars().take(8).collect::<String>();
            lines.push(format!("{} {}", gate, short));
        }
    }

    let mut pubs = client.list_publications()?;
    pubs.retain(|p| p.scope == remote.scope && p.gate == remote.gate);
    pubs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    pubs.truncate(10);
    lines.push("".to_string());
    lines.push("publications:".to_string());
    if pubs.is_empty() {
        lines.push("(none)".to_string());
    } else {
        for p in pubs {
            let short = p.snap_id.chars().take(8).collect::<String>();
            let present = if ws.store.has_snap(&p.snap_id) {
                "local"
            } else {
                "missing"
            };
            lines.push(format!(
                "{} {} {} {} {}",
                short,
                fmt_ts_list(&p.created_at, ctx),
                p.publisher,
                p.gate,
                present
            ));
        }
    }

    Ok(lines)
}

pub(in crate::tui_shell) fn dashboard_data(
    ws: &Workspace,
    ctx: &RenderCtx,
) -> Result<DashboardData> {
    let cfg = ws.store.read_config()?;
    let Some(remote) = cfg.remote else {
        anyhow::bail!("remote: not configured");
    };

    let token = ws.store.get_remote_token(&remote)?;
    let Some(token) = token else {
        anyhow::bail!("token missing");
    };

    let mut out = DashboardData {
        healthz: None,
        gates_total: 0,

        inbox_total: 0,
        inbox_pending: 0,
        inbox_resolved: 0,
        inbox_missing_local: 0,
        latest_publication: None,

        bundles_total: 0,
        bundles_promotable: 0,
        bundles_blocked: 0,
        blocked_superpositions: 0,
        blocked_approvals: 0,
        pinned_bundles: 0,

        promotion_state: Vec::new(),

        releases_total: 0,
        releases_channels: 0,
        latest_releases: Vec::new(),

        next_actions: Vec::new(),
    };

    let url = format!("{}/healthz", remote.base_url.trim_end_matches('/'));
    let start = std::time::Instant::now();
    match reqwest::blocking::get(&url) {
        Ok(r) => {
            let ms = start.elapsed().as_millis();
            out.healthz = Some(format!("{} {}ms", r.status(), ms));
        }
        Err(err) => {
            out.healthz = Some(format!("error {:#}", err));
        }
    }

    let client = RemoteClient::new(remote.clone(), token)?;

    if let Ok(graph) = client.get_gate_graph() {
        out.gates_total = graph.gates.len();
    }

    let mut pubs = client.list_publications()?;
    pubs.retain(|p| p.scope == remote.scope && p.gate == remote.gate);
    out.inbox_total = pubs.len();
    out.inbox_resolved = pubs.iter().filter(|p| p.resolution.is_some()).count();
    out.inbox_pending = out.inbox_total.saturating_sub(out.inbox_resolved);
    out.inbox_missing_local = pubs
        .iter()
        .filter(|p| !ws.store.has_snap(&p.snap_id))
        .count();
    pubs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    if let Some(p) = pubs.first() {
        out.latest_publication = Some((
            p.snap_id.chars().take(8).collect::<String>(),
            fmt_ts_list(&p.created_at, ctx),
        ));
    }

    let mut bundles = client.list_bundles()?;
    bundles.retain(|b| b.scope == remote.scope && b.gate == remote.gate);
    out.bundles_total = bundles.len();
    out.bundles_promotable = bundles.iter().filter(|b| b.promotable).count();
    out.bundles_blocked = out.bundles_total.saturating_sub(out.bundles_promotable);
    for b in &bundles {
        if b.promotable {
            continue;
        }
        if b.reasons.iter().any(|r| r == "superpositions_present") {
            out.blocked_superpositions += 1;
        }
        if b.reasons.iter().any(|r| r == "approvals_missing") {
            out.blocked_approvals += 1;
        }
    }
    if let Ok(pins) = client.list_pins() {
        out.pinned_bundles = pins.bundles.len();
    }

    if let Ok(state) = client.promotion_state(&remote.scope) {
        let mut keys = state.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        for gate in keys {
            let bid = state.get(&gate).cloned().unwrap_or_default();
            let short = bid.chars().take(8).collect::<String>();
            out.promotion_state.push((gate, short));
        }
    }

    if let Ok(releases) = client.list_releases() {
        out.releases_total = releases.len();
        let latest = latest_releases_by_channel(releases);
        out.releases_channels = latest.len();
        for r in latest.into_iter().take(3) {
            out.latest_releases.push((
                r.channel,
                r.bundle_id.chars().take(8).collect::<String>(),
                fmt_ts_list(&r.released_at, ctx),
            ));
        }
    }

    let mut actions = Vec::new();
    if out.inbox_pending > 0 {
        actions.push(format!("open inbox ({} pending)", out.inbox_pending));
    }
    if out.inbox_missing_local > 0 {
        actions.push(format!("fetch missing snaps ({})", out.inbox_missing_local));
    }
    if out.bundles_promotable > 0 {
        actions.push(format!("promote bundles ({})", out.bundles_promotable));
    }
    if out.blocked_superpositions > 0 {
        actions.push(format!(
            "resolve superpositions ({})",
            out.blocked_superpositions
        ));
    }
    if out.blocked_approvals > 0 {
        actions.push(format!("collect approvals ({})", out.blocked_approvals));
    }
    out.next_actions = actions.into_iter().take(4).collect();

    Ok(out)
}
