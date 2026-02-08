use super::*;

pub(super) fn handle_release_command(ws: &Workspace, command: ReleaseCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote, token)?;

    match command {
        ReleaseCommands::Create {
            channel,
            bundle_id,
            notes,
            json,
        } => {
            let r = client.create_release(&channel, &bundle_id, notes)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&r).context("serialize release create json")?
                );
            } else {
                println!("{} {}", r.channel, r.bundle_id);
            }
        }
        ReleaseCommands::List { json } => {
            let mut rs = client.list_releases()?;
            rs.sort_by(|a, b| b.released_at.cmp(&a.released_at));
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&rs).context("serialize release list json")?
                );
            } else {
                for r in rs {
                    let short = r.bundle_id.chars().take(8).collect::<String>();
                    println!(
                        "{} {} {} {}",
                        r.channel, short, r.released_at, r.released_by
                    );
                }
            }
        }
        ReleaseCommands::Show { channel, json } => {
            let r = client.get_release(&channel)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&r).context("serialize release show json")?
                );
            } else {
                println!("channel: {}", r.channel);
                println!("bundle: {}", r.bundle_id);
                println!("scope: {}", r.scope);
                println!("gate: {}", r.gate);
                println!("released_at: {}", r.released_at);
                println!("released_by: {}", r.released_by);
                if let Some(n) = r.notes {
                    println!("notes: {}", n);
                }
            }
        }
    }

    Ok(())
}

pub(super) fn handle_resolve_command(ws: &Workspace, command: ResolveCommands) -> Result<()> {
    let (remote, token) = require_remote_and_token(&ws.store)?;
    let client = RemoteClient::new(remote.clone(), token)?;

    match command {
        ResolveCommands::Init {
            bundle_id,
            force,
            json,
        } => {
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
        }

        ResolveCommands::Pick {
            bundle_id,
            path,
            variant,
            key,
            json,
        } => {
            let bundle = client.get_bundle(&bundle_id)?;
            let root = converge::model::ObjectId(bundle.root_manifest.clone());
            client.fetch_manifest_tree(&ws.store, &root)?;

            let variants = converge::resolve::superposition_variants(&ws.store, &root)?;
            let Some(vs) = variants.get(&path) else {
                anyhow::bail!("no superposition at path {}", path);
            };
            let vlen = vs.len();

            let decision = match (variant, key) {
                (Some(_), Some(_)) => {
                    anyhow::bail!("use either --variant or --key (not both)");
                }
                (None, None) => {
                    anyhow::bail!("missing required flag: --variant or --key");
                }
                (Some(variant), None) => {
                    if variant == 0 {
                        anyhow::bail!("variant is 1-based (use --variant 1..{})", vlen);
                    }
                    let idx = (variant - 1) as usize;
                    if idx >= vlen {
                        anyhow::bail!("variant out of range (variants: {})", vlen);
                    }
                    converge::model::ResolutionDecision::Key(vs[idx].key())
                }
                (None, Some(key_json)) => {
                    let key: converge::model::VariantKey =
                        serde_json::from_str(&key_json).context("parse --key")?;
                    if !vs.iter().any(|v| v.key() == key) {
                        anyhow::bail!("key not present at path {}", path);
                    }
                    converge::model::ResolutionDecision::Key(key)
                }
            };

            let mut r = ws.store.get_resolution(&bundle_id)?;
            if r.root_manifest != root {
                anyhow::bail!(
                    "resolution root_manifest mismatch (resolution {}, bundle {})",
                    r.root_manifest.as_str(),
                    root.as_str()
                );
            }

            // Best-effort upgrade: convert index decisions to keys using current variants.
            if r.version == 1 {
                r.version = 2;
            }
            let existing = r.decisions.clone();
            for (p, d) in existing {
                if let converge::model::ResolutionDecision::Index(i) = d {
                    let i = i as usize;
                    if let Some(vs) = variants.get(&p)
                        && i < vs.len()
                    {
                        r.decisions
                            .insert(p, converge::model::ResolutionDecision::Key(vs[i].key()));
                    }
                }
            }

            r.decisions.insert(path.clone(), decision);
            ws.store.put_resolution(&r)?;

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&r).context("serialize resolution")?
                );
            } else if let Some(v) = variant {
                println!("Picked variant #{} for {}", v, path);
            } else {
                println!("Picked key for {}", path);
            }
        }

        ResolveCommands::Clear {
            bundle_id,
            path,
            json,
        } => {
            let mut r = ws.store.get_resolution(&bundle_id)?;
            r.decisions.remove(&path);
            if r.version == 1 {
                r.version = 2;
            }
            ws.store.put_resolution(&r)?;

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&r).context("serialize resolution")?
                );
            } else {
                println!("Cleared decision for {}", path);
            }
        }

        ResolveCommands::Show { bundle_id, json } => {
            let r = ws.store.get_resolution(&bundle_id)?;

            // Best-effort fetch so we can enumerate current conflicts.
            let _ = client.fetch_manifest_tree(&ws.store, &r.root_manifest);

            let variants = converge::resolve::superposition_variants(&ws.store, &r.root_manifest)
                .unwrap_or_default();
            let decided = variants
                .keys()
                .filter(|p| r.decisions.contains_key(*p))
                .count();

            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::json!({
                        "resolution": r,
                        "conflicts": variants,
                        "decided": decided
                    }))
                    .context("serialize resolve show json")?
                );
            } else {
                println!("bundle: {}", r.bundle_id);
                println!("root_manifest: {}", r.root_manifest.as_str());
                println!("created_at: {}", r.created_at);
                println!("decisions: {}", r.decisions.len());

                if !variants.is_empty() {
                    println!("decided: {}/{}", decided, variants.len());
                    println!("conflicts:");
                    for (p, vs) in variants {
                        println!("{} (variants: {})", p, vs.len());
                        for (idx, v) in vs.iter().enumerate() {
                            let n = idx + 1;
                            let key_json =
                                serde_json::to_string(&v.key()).context("serialize variant key")?;
                            println!("  #{} source={}", n, v.source);
                            println!("    key={}", key_json);
                        }
                    }
                }
            }
        }

        ResolveCommands::Apply {
            bundle_id,
            message,
            publish,
            json,
        } => {
            let bundle = client.get_bundle(&bundle_id)?;

            // Ensure we can read manifests/blobs needed for applying resolution.
            let root = converge::model::ObjectId(bundle.root_manifest.clone());
            client.fetch_manifest_tree(&ws.store, &root)?;

            let resolution = ws.store.get_resolution(&bundle_id)?;
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
                message,
                stats: converge::model::SnapStats::default(),
            };

            ws.store.put_snap(&snap)?;

            let mut pub_id = None;
            if publish {
                let pubrec = client.publish_snap_with_resolution(
                    &ws.store,
                    &snap,
                    &remote.scope,
                    &remote.gate,
                    Some(converge::remote::PublicationResolution {
                        bundle_id: bundle_id.clone(),
                        root_manifest: root.as_str().to_string(),
                        resolved_root_manifest: snap.root_manifest.as_str().to_string(),
                        created_at: snap.created_at.clone(),
                    }),
                )?;
                pub_id = Some(pubrec.id);
            }

            if json {
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
        }

        ResolveCommands::Validate { bundle_id, json } => {
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
        }
    }

    Ok(())
}
