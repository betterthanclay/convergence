use anyhow::Result;

use super::*;

pub(super) fn collect(
    ws: &Workspace,
    ctx: &RenderCtx,
    client: &RemoteClient,
    remote: &crate::model::RemoteConfig,
    out: &mut DashboardData,
) -> Result<()> {
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
    Ok(())
}
