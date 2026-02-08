use super::*;

pub(super) fn handle_resolve_init(
    ws: &Workspace,
    client: &RemoteClient,
    bundle_id: String,
    force: bool,
    json: bool,
) -> Result<()> {
    if ws.store.has_resolution(&bundle_id) && !force {
        anyhow::bail!("resolution already exists (use --force to overwrite)");
    }

    let bundle = client.get_bundle(&bundle_id)?;
    let root = converge::model::ObjectId(bundle.root_manifest.clone());
    client.fetch_manifest_tree(&ws.store, &root)?;

    let counts = converge::resolve::superposition_variant_counts(&ws.store, &root)?;

    let created_at = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .context("format time")?;
    let resolution = converge::model::Resolution {
        version: 2,
        bundle_id: bundle_id.clone(),
        root_manifest: root,
        created_at,
        decisions: std::collections::BTreeMap::new(),
    };
    ws.store.put_resolution(&resolution)?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "resolution": resolution,
                "conflicts": counts
            }))
            .context("serialize resolve init json")?
        );
    } else {
        println!("Initialized resolution for bundle {}", bundle_id);
        if counts.is_empty() {
            println!("No superpositions found");
        } else {
            println!("Conflicts:");
            for (p, n) in counts {
                println!("{} (variants: {})", p, n);
            }
        }
    }

    Ok(())
}
