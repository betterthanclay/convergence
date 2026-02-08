use super::*;

#[derive(Debug)]
enum PickSpecifier {
    VariantIndex(usize),
    KeyJson(String),
}

fn parse_pick_specifier(
    variant: Option<u32>,
    key: Option<String>,
    variant_len: usize,
) -> Result<PickSpecifier> {
    match (variant, key) {
        (Some(_), Some(_)) => {
            anyhow::bail!("use either --variant or --key (not both)");
        }
        (None, None) => {
            anyhow::bail!("missing required flag: --variant or --key");
        }
        (Some(variant), None) => {
            if variant == 0 {
                anyhow::bail!("variant is 1-based (use --variant 1..{})", variant_len);
            }
            let idx = (variant - 1) as usize;
            if idx >= variant_len {
                anyhow::bail!("variant out of range (variants: {})", variant_len);
            }
            Ok(PickSpecifier::VariantIndex(idx))
        }
        (None, Some(key_json)) => Ok(PickSpecifier::KeyJson(key_json)),
    }
}

pub(super) fn handle_resolve_pick(
    ws: &Workspace,
    client: &RemoteClient,
    bundle_id: String,
    path: String,
    variant: Option<u32>,
    key: Option<String>,
    json: bool,
) -> Result<()> {
    let bundle = client.get_bundle(&bundle_id)?;
    let root = converge::model::ObjectId(bundle.root_manifest.clone());
    client.fetch_manifest_tree(&ws.store, &root)?;

    let variants = converge::resolve::superposition_variants(&ws.store, &root)?;
    let Some(vs) = variants.get(&path) else {
        anyhow::bail!("no superposition at path {}", path);
    };
    let vlen = vs.len();

    let decision = match parse_pick_specifier(variant, key, vlen)? {
        PickSpecifier::VariantIndex(idx) => converge::model::ResolutionDecision::Key(vs[idx].key()),
        PickSpecifier::KeyJson(key_json) => {
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

    Ok(())
}

pub(super) fn handle_resolve_clear(
    ws: &Workspace,
    bundle_id: String,
    path: String,
    json: bool,
) -> Result<()> {
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

    Ok(())
}

pub(super) fn handle_resolve_show(
    ws: &Workspace,
    client: &RemoteClient,
    bundle_id: String,
    json: bool,
) -> Result<()> {
    let r = ws.store.get_resolution(&bundle_id)?;

    // Best-effort fetch so we can enumerate current conflicts.
    let _ = client.fetch_manifest_tree(&ws.store, &r.root_manifest);

    let variants =
        converge::resolve::superposition_variants(&ws.store, &r.root_manifest).unwrap_or_default();
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

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pick_specifier_rejects_conflicting_inputs() {
        let err = parse_pick_specifier(Some(1), Some("{}".to_string()), 2).unwrap_err();
        assert!(
            err.to_string().contains("use either --variant or --key"),
            "{}",
            err
        );
    }

    #[test]
    fn parse_pick_specifier_rejects_missing_inputs() {
        let err = parse_pick_specifier(None, None, 2).unwrap_err();
        assert!(
            err.to_string()
                .contains("missing required flag: --variant or --key"),
            "{}",
            err
        );
    }

    #[test]
    fn parse_pick_specifier_rejects_out_of_range_variants() {
        let err = parse_pick_specifier(Some(3), None, 2).unwrap_err();
        assert!(
            err.to_string()
                .contains("variant out of range (variants: 2)"),
            "{}",
            err
        );
    }

    #[test]
    fn parse_pick_specifier_accepts_index_and_key_forms() {
        match parse_pick_specifier(Some(2), None, 3).expect("parse variant") {
            PickSpecifier::VariantIndex(i) => assert_eq!(i, 1),
            PickSpecifier::KeyJson(_) => panic!("expected variant index"),
        }

        match parse_pick_specifier(None, Some("{\"source\":\"x\"}".to_string()), 3)
            .expect("parse key")
        {
            PickSpecifier::VariantIndex(_) => panic!("expected key json"),
            PickSpecifier::KeyJson(key) => assert_eq!(key, "{\"source\":\"x\"}"),
        }
    }
}
