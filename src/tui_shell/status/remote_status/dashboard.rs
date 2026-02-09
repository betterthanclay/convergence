use anyhow::Result;

use super::health::fetch_healthz;
use super::*;

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

    let mut out = new_dashboard_data();
    out.healthz = Some(fetch_healthz(&remote.base_url));

    let client = RemoteClient::new(remote.clone(), token)?;

    if let Ok(graph) = client.get_gate_graph() {
        out.gates_total = graph.gates.len();
    }

    let mut publications = client.list_publications()?;
    publications.retain(|p| p.scope == remote.scope && p.gate == remote.gate);
    out.inbox_total = publications.len();
    out.inbox_resolved = publications
        .iter()
        .filter(|p| p.resolution.is_some())
        .count();
    out.inbox_pending = out.inbox_total.saturating_sub(out.inbox_resolved);
    out.inbox_missing_local = publications
        .iter()
        .filter(|p| !ws.store.has_snap(&p.snap_id))
        .count();
    publications.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    if let Some(publication) = publications.first() {
        out.latest_publication = Some((
            publication.snap_id.chars().take(8).collect::<String>(),
            fmt_ts_list(&publication.created_at, ctx),
        ));
    }

    let mut bundles = client.list_bundles()?;
    bundles.retain(|b| b.scope == remote.scope && b.gate == remote.gate);
    out.bundles_total = bundles.len();
    out.bundles_promotable = bundles.iter().filter(|b| b.promotable).count();
    out.bundles_blocked = out.bundles_total.saturating_sub(out.bundles_promotable);
    for bundle in &bundles {
        if bundle.promotable {
            continue;
        }
        if bundle
            .reasons
            .iter()
            .any(|reason| reason == "superpositions_present")
        {
            out.blocked_superpositions += 1;
        }
        if bundle
            .reasons
            .iter()
            .any(|reason| reason == "approvals_missing")
        {
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
        for release in latest.into_iter().take(3) {
            out.latest_releases.push((
                release.channel,
                release.bundle_id.chars().take(8).collect::<String>(),
                fmt_ts_list(&release.released_at, ctx),
            ));
        }
    }

    out.next_actions = recommended_actions(&out);

    Ok(out)
}

fn recommended_actions(data: &DashboardData) -> Vec<String> {
    let mut actions = Vec::new();
    if data.inbox_pending > 0 {
        actions.push(format!("open inbox ({} pending)", data.inbox_pending));
    }
    if data.inbox_missing_local > 0 {
        actions.push(format!(
            "fetch missing snaps ({})",
            data.inbox_missing_local
        ));
    }
    if data.bundles_promotable > 0 {
        actions.push(format!("promote bundles ({})", data.bundles_promotable));
    }
    if data.blocked_superpositions > 0 {
        actions.push(format!(
            "resolve superpositions ({})",
            data.blocked_superpositions
        ));
    }
    if data.blocked_approvals > 0 {
        actions.push(format!("collect approvals ({})", data.blocked_approvals));
    }
    actions.into_iter().take(4).collect()
}
