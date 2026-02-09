use super::super::super::*;

mod bundle_release;
mod snaps;
mod util;

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_fetch_command(
    ws: &Workspace,
    snap_id: Option<String>,
    bundle_id: Option<String>,
    release: Option<String>,
    lane: Option<String>,
    user: Option<String>,
    restore: bool,
    into: Option<String>,
    force: bool,
    json: bool,
) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    if let Some(bundle_id) = bundle_id.as_deref() {
        return bundle_release::handle_bundle_fetch(
            ws,
            &client,
            bundle_id,
            restore,
            into.as_deref(),
            force,
            json,
        );
    }

    if let Some(channel) = release.as_deref() {
        return bundle_release::handle_release_fetch(
            ws,
            &client,
            channel,
            restore,
            into.as_deref(),
            force,
            json,
        );
    }

    snaps::handle_snap_or_lane_fetch(
        ws,
        &client,
        snap_id.as_deref(),
        lane.as_deref(),
        user.as_deref(),
        restore,
        into.as_deref(),
        force,
        json,
    )
}
