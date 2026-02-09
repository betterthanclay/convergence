use super::util::default_temp_destination;
use super::*;

pub(super) fn handle_bundle_fetch(
    ws: &Workspace,
    client: &RemoteClient,
    bundle_id: &str,
    restore: bool,
    into: Option<&str>,
    force: bool,
    json: bool,
) -> Result<()> {
    let bundle = client.get_bundle(bundle_id)?;
    let root = converge::model::ObjectId(bundle.root_manifest.clone());
    client.fetch_manifest_tree(&ws.store, &root)?;

    let mut restored_to: Option<String> = None;
    if restore {
        let dest = if let Some(p) = into {
            std::path::PathBuf::from(p)
        } else {
            default_temp_destination("converge-grab-bundle", bundle_id)
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
    Ok(())
}

pub(super) fn handle_release_fetch(
    ws: &Workspace,
    client: &RemoteClient,
    channel: &str,
    restore: bool,
    into: Option<&str>,
    force: bool,
    json: bool,
) -> Result<()> {
    let rel = client.get_release(channel)?;
    let bundle = client.get_bundle(&rel.bundle_id)?;
    let root = converge::model::ObjectId(bundle.root_manifest.clone());
    client.fetch_manifest_tree(&ws.store, &root)?;

    let mut restored_to: Option<String> = None;
    if restore {
        let dest = if let Some(p) = into {
            std::path::PathBuf::from(p)
        } else {
            default_temp_destination("converge-grab-release", &rel.bundle_id)
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
    Ok(())
}
