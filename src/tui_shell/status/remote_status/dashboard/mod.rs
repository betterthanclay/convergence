use anyhow::Result;

use super::health::fetch_healthz;
use super::*;

mod actions;
mod bundles;
mod inbox;
mod releases;

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

    inbox::collect(ws, ctx, &client, &remote, &mut out)?;
    bundles::collect(&client, &remote, &mut out)?;
    releases::collect(ctx, &client, &mut out);
    out.next_actions = actions::recommended_actions(&out);

    Ok(out)
}
