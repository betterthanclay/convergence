use super::*;

pub(super) fn fetch_snaps(
    app: &mut App,
    client: &RemoteClient,
    ws: &Workspace,
    parsed: &FetchSpec,
) {
    let res = if let Some(lane) = parsed.lane.as_deref() {
        client.fetch_lane_heads(&ws.store, lane, parsed.user.as_deref())
    } else {
        client.fetch_publications(&ws.store, parsed.snap_id.as_deref())
    };

    match res {
        Ok(fetched) => {
            app.push_output(vec![format!("fetched {} snaps", fetched.len())]);
            app.refresh_root_view();

            if app.mode() == UiMode::Lanes
                && let Some(v) = app.current_view_mut::<LanesView>()
            {
                for it in &mut v.items {
                    if let Some(h) = &it.head {
                        it.local = ws.store.has_snap(&h.snap_id);
                    }
                }
                v.updated_at = now_ts();
            }
        }
        Err(err) => app.push_error(format!("fetch: {:#}", err)),
    }
}
