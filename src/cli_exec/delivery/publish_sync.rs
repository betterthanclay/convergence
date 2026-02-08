use super::*;

pub(in crate::cli_exec) fn handle_publish_command(
    ws: &Workspace,
    snap_id: Option<String>,
    scope: Option<String>,
    gate: Option<String>,
    metadata_only: bool,
    json: bool,
) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote.clone(), token)?;

    let snap = match snap_id {
        Some(id) => ws.show_snap(&id)?,
        None => ws
            .list_snaps()?
            .into_iter()
            .next()
            .context("no snaps found (run `converge snap`)")?,
    };

    let scope = scope.unwrap_or_else(|| remote.scope.clone());
    let gate = gate.unwrap_or_else(|| remote.gate.clone());

    let pubrec = if metadata_only {
        client.publish_snap_metadata_only(&ws.store, &snap, &scope, &gate)?
    } else {
        client.publish_snap(&ws.store, &snap, &scope, &gate)?
    };

    ws.store
        .set_last_published(&remote, &scope, &gate, &snap.id)
        .context("record last published snap")?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&pubrec).context("serialize publish json")?
        );
    } else {
        println!("Published {}", snap.id);
    }

    Ok(())
}

pub(in crate::cli_exec) fn handle_sync_command(
    ws: &Workspace,
    snap_id: Option<String>,
    lane: String,
    client_id: Option<String>,
    json: bool,
) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    let snap = match snap_id {
        Some(id) => ws.show_snap(&id)?,
        None => ws
            .list_snaps()?
            .into_iter()
            .next()
            .context("no snaps to sync")?,
    };

    let head = client.sync_snap(&ws.store, &snap, &lane, client_id)?;

    ws.store
        .set_lane_sync(&lane, &snap.id, &head.updated_at)
        .context("record lane sync")?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&head).context("serialize sync json")?
        );
    } else {
        println!("Synced {} to lane {}", snap.id, lane);
    }

    Ok(())
}

pub(in crate::cli_exec) fn handle_lanes_command(ws: &Workspace, json: bool) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;
    let mut lanes = client.list_lanes()?;
    lanes.sort_by(|a, b| a.id.cmp(&b.id));

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&lanes).context("serialize lanes json")?
        );
    } else {
        for l in lanes {
            println!("lane: {}", l.id);
            let mut members = l.members.into_iter().collect::<Vec<_>>();
            members.sort();
            for m in members {
                if let Some(h) = l.heads.get(&m) {
                    let short = h.snap_id.chars().take(8).collect::<String>();
                    println!("  {} {} {}", m, short, h.updated_at);
                } else {
                    println!("  {} (no head)", m);
                }
            }
        }
    }

    Ok(())
}
