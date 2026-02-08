use super::*;

#[allow(clippy::too_many_arguments)]
pub(in crate::cli_exec) fn handle_fetch_command(
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
        let bundle = client.get_bundle(bundle_id)?;
        let root = converge::model::ObjectId(bundle.root_manifest.clone());
        client.fetch_manifest_tree(&ws.store, &root)?;

        let mut restored_to: Option<String> = None;
        if restore {
            let dest = if let Some(p) = into.as_deref() {
                std::path::PathBuf::from(p)
            } else {
                let short = bundle_id.chars().take(8).collect::<String>();
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                std::env::temp_dir().join(format!("converge-grab-bundle-{}-{}", short, nanos))
            };

            ws.materialize_manifest_to(&root, &dest, force)
                .with_context(|| format!("materialize bundle to {}", dest.display()))?;
            restored_to = Some(dest.display().to_string());
            if !json {
                println!("Materialized bundle {} into {}", bundle_id, dest.display());
            }
        }

        if json {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "kind": "bundle",
                    "bundle_id": bundle.id,
                    "root_manifest": bundle.root_manifest,
                    "restored_to": restored_to,
                }))
                .context("serialize fetch bundle json")?
            );
        } else {
            println!("Fetched bundle {}", bundle.id);
        }
        return Ok(());
    }

    if let Some(channel) = release.as_deref() {
        let rel = client.get_release(channel)?;
        let bundle = client.get_bundle(&rel.bundle_id)?;
        let root = converge::model::ObjectId(bundle.root_manifest.clone());
        client.fetch_manifest_tree(&ws.store, &root)?;

        let mut restored_to: Option<String> = None;
        if restore {
            let dest = if let Some(p) = into.as_deref() {
                std::path::PathBuf::from(p)
            } else {
                let short = rel.bundle_id.chars().take(8).collect::<String>();
                let nanos = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos();
                std::env::temp_dir().join(format!("converge-grab-release-{}-{}", short, nanos))
            };

            ws.materialize_manifest_to(&root, &dest, force)
                .with_context(|| format!("materialize release to {}", dest.display()))?;
            restored_to = Some(dest.display().to_string());
            if !json {
                println!(
                    "Materialized release {} (bundle {}) into {}",
                    rel.channel,
                    rel.bundle_id,
                    dest.display()
                );
            }
        }

        if json {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "kind": "release",
                    "channel": rel.channel,
                    "bundle_id": rel.bundle_id,
                    "root_manifest": bundle.root_manifest,
                    "restored_to": restored_to,
                }))
                .context("serialize fetch release json")?
            );
        } else {
            println!("Fetched release {} ({})", rel.channel, rel.bundle_id);
        }
        return Ok(());
    }

    let fetched = if let Some(lane) = lane.as_deref() {
        client.fetch_lane_heads(&ws.store, lane, user.as_deref())?
    } else {
        client.fetch_publications(&ws.store, snap_id.as_deref())?
    };

    if restore {
        let snap_to_restore = if let Some(id) = snap_id.as_deref() {
            id.to_string()
        } else if fetched.len() == 1 {
            fetched[0].clone()
        } else {
            anyhow::bail!(
                "--restore requires a specific snap (use --snap-id, or use --user so only one lane head is fetched)"
            );
        };

        let dest = if let Some(p) = into.as_deref() {
            std::path::PathBuf::from(p)
        } else {
            let short = snap_to_restore.chars().take(8).collect::<String>();
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            std::env::temp_dir().join(format!("converge-grab-{}-{}", short, nanos))
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

pub(in crate::cli_exec) fn handle_bundle_command(
    ws: &Workspace,
    scope: Option<String>,
    gate: Option<String>,
    publications: Vec<String>,
    json: bool,
) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote.clone(), token)?;
    let scope = scope.unwrap_or_else(|| remote.scope.clone());
    let gate = gate.unwrap_or_else(|| remote.gate.clone());

    let pubs = if publications.is_empty() {
        let all = client.list_publications()?;
        all.into_iter()
            .filter(|p| p.scope == scope && p.gate == gate)
            .map(|p| p.id)
            .collect::<Vec<_>>()
    } else {
        publications
    };

    if pubs.is_empty() {
        anyhow::bail!(
            "no publications found for scope={} gate={} (publish first)",
            scope,
            gate
        );
    }

    let bundle = client.create_bundle(&scope, &gate, &pubs)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).context("serialize bundle json")?
        );
    } else {
        println!("{}", bundle.id);
    }

    Ok(())
}

pub(in crate::cli_exec) fn handle_promote_command(
    ws: &Workspace,
    bundle_id: String,
    to_gate: String,
    json: bool,
) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;
    let promotion = client.promote_bundle(&bundle_id, &to_gate)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&promotion).context("serialize promotion json")?
        );
    } else {
        println!("Promoted {} -> {}", promotion.from_gate, promotion.to_gate);
    }

    Ok(())
}
