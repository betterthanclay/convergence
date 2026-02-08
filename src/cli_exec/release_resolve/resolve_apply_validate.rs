use super::*;

pub(super) struct ResolveApplyInput {
    pub(super) bundle_id: String,
    pub(super) message: Option<String>,
    pub(super) publish: bool,
    pub(super) json: bool,
    pub(super) scope: String,
    pub(super) gate: String,
}

pub(super) fn handle_resolve_apply(
    ws: &Workspace,
    client: &RemoteClient,
    input: ResolveApplyInput,
) -> Result<()> {
    let bundle = client.get_bundle(&input.bundle_id)?;

    // Ensure we can read manifests/blobs needed for applying resolution.
    let root = converge::model::ObjectId(bundle.root_manifest.clone());
    client.fetch_manifest_tree(&ws.store, &root)?;

    let resolution = ws.store.get_resolution(&input.bundle_id)?;
    if resolution.root_manifest != root {
        anyhow::bail!(
            "resolution root_manifest mismatch (resolution {}, bundle {})",
            resolution.root_manifest.as_str(),
            root.as_str()
        );
    }

    let resolved_root =
        converge::resolve::apply_resolution(&ws.store, &root, &resolution.decisions)?;

    let created_at = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .context("format time")?;
    let snap_id = converge::model::compute_snap_id(&created_at, &resolved_root);

    let snap = converge::model::SnapRecord {
        version: 1,
        id: snap_id,
        created_at,
        root_manifest: resolved_root,
        message: input.message,
        stats: converge::model::SnapStats::default(),
    };

    ws.store.put_snap(&snap)?;

    let mut pub_id = None;
    if input.publish {
        let pubrec = client.publish_snap_with_resolution(
            &ws.store,
            &snap,
            &input.scope,
            &input.gate,
            Some(converge::remote::PublicationResolution {
                bundle_id: input.bundle_id.clone(),
                root_manifest: root.as_str().to_string(),
                resolved_root_manifest: snap.root_manifest.as_str().to_string(),
                created_at: snap.created_at.clone(),
            }),
        )?;
        pub_id = Some(pubrec.id);
    }

    if input.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "snap": snap,
                "published_publication_id": pub_id
            }))
            .context("serialize resolve json")?
        );
    } else {
        println!("Resolved snap {}", snap.id);
        if let Some(pid) = pub_id {
            println!("Published {}", pid);
        }
    }

    Ok(())
}

pub(super) fn handle_resolve_validate(
    ws: &Workspace,
    client: &RemoteClient,
    bundle_id: String,
    json: bool,
) -> Result<()> {
    let bundle = client.get_bundle(&bundle_id)?;
    let root = converge::model::ObjectId(bundle.root_manifest.clone());
    client.fetch_manifest_tree(&ws.store, &root)?;

    let r = ws.store.get_resolution(&bundle_id)?;
    if r.root_manifest != root {
        anyhow::bail!(
            "resolution root_manifest mismatch (resolution {}, bundle {})",
            r.root_manifest.as_str(),
            root.as_str()
        );
    }

    let report = converge::resolve::validate_resolution(&ws.store, &root, &r.decisions)?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "bundle_id": bundle_id,
                "root_manifest": root,
                "report": report,
            }))
            .context("serialize resolve validate json")?
        );
    } else {
        if report.ok {
            println!("OK");
        } else {
            println!("Invalid");
        }
        if !report.missing.is_empty() {
            println!("missing:");
            for p in &report.missing {
                println!("{}", p);
            }
        }
        if !report.out_of_range.is_empty() {
            println!("out_of_range:");
            for d in &report.out_of_range {
                println!("{} index={} variants={}", d.path, d.index, d.variants);
            }
        }
        if !report.invalid_keys.is_empty() {
            println!("invalid_keys:");
            for d in &report.invalid_keys {
                println!("{} source={}", d.path, d.wanted.source);
            }
        }
        if !report.extraneous.is_empty() {
            println!("extraneous:");
            for p in &report.extraneous {
                println!("{}", p);
            }
        }
    }

    Ok(())
}
