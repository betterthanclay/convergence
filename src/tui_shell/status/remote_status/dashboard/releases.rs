use super::*;

pub(super) fn collect(ctx: &RenderCtx, client: &RemoteClient, out: &mut DashboardData) {
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
}
