use super::util::default_temp_destination;
use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_snap_or_lane_fetch(
    ws: &Workspace,
    client: &RemoteClient,
    snap_id: Option<&str>,
    lane: Option<&str>,
    user: Option<&str>,
    restore: bool,
    into: Option<&str>,
    force: bool,
    json: bool,
) -> Result<()> {
    let fetched = if let Some(lane) = lane {
        client.fetch_lane_heads(&ws.store, lane, user)?
    } else {
        client.fetch_publications(&ws.store, snap_id)?
    };

    if restore {
        let snap_to_restore = if let Some(id) = snap_id {
            id.to_string()
        } else if fetched.len() == 1 {
            fetched[0].clone()
        } else {
            anyhow::bail!(
                "--restore requires a specific snap (use --snap-id, or use --user so only one lane head is fetched)"
            );
        };

        let dest = if let Some(p) = into {
            std::path::PathBuf::from(p)
        } else {
            default_temp_destination("converge-grab", &snap_to_restore)
        };

        ws.materialize_snap_to(&snap_to_restore, &dest, force)
            .with_context(|| format!("materialize snap to {}", dest.display()))?;
        if !json {
            println!("Materialized {} into {}", snap_to_restore, dest.display());
        }
    }

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&fetched).context("serialize fetch json")?
        );
    } else {
        for id in fetched {
            println!("Fetched {}", id);
        }
    }

    Ok(())
}
