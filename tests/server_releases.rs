use anyhow::{Context, Result};

mod common;

#[test]
fn release_endpoints_create_list_and_show() -> Result<()> {
    let server = common::spawn_server()?;
    let client = reqwest::blocking::Client::new();
    let auth = common::auth_header(&server.token);

    // Create repo.
    client
        .post(format!("{}/repos", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .json(&serde_json::json!({"id": "test"}))
        .send()
        .context("create repo")?
        .error_for_status()
        .context("create repo status")?;

    // Enable metadata-only publications for dev-intake.
    let mut graph: serde_json::Value = client
        .get(format!("{}/repos/test/gate-graph", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .send()
        .context("get gate graph")?
        .error_for_status()
        .context("get gate graph status")?
        .json()
        .context("parse gate graph")?;

    let gates = graph
        .get_mut("gates")
        .and_then(|v| v.as_array_mut())
        .context("gate graph gates missing")?;
    for g in gates.iter_mut() {
        if g.get("id") == Some(&serde_json::Value::String("dev-intake".to_string())) {
            g["allow_metadata_only_publications"] = serde_json::Value::Bool(true);
        }
    }

    client
        .put(format!("{}/repos/test/gate-graph", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .json(&graph)
        .send()
        .context("put gate graph")?
        .error_for_status()
        .context("put gate graph status")?;

    // Upload a manifest and snap that references a missing blob.
    let missing_blob = "1".repeat(64);
    let manifest = converge::model::Manifest {
        version: 1,
        entries: vec![converge::model::ManifestEntry {
            name: "f.txt".to_string(),
            kind: converge::model::ManifestEntryKind::File {
                blob: converge::model::ObjectId(missing_blob),
                mode: 0o100644,
                size: 1,
            },
        }],
    };
    let manifest_bytes = serde_json::to_vec(&manifest).context("serialize manifest")?;
    let manifest_id = blake3::hash(&manifest_bytes).to_hex().to_string();

    client
        .put(format!(
            "{}/repos/test/objects/manifests/{}?allow_missing_blobs=true",
            server.base_url, manifest_id
        ))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .body(manifest_bytes)
        .send()
        .context("put manifest")?
        .error_for_status()
        .context("put manifest status")?;

    let created_at = "2026-01-25T00:00:00Z";
    let root_manifest = converge::model::ObjectId(manifest_id);
    let snap_id = converge::model::compute_snap_id(created_at, &root_manifest);
    let snap = converge::model::SnapRecord {
        version: 1,
        id: snap_id.clone(),
        created_at: created_at.to_string(),
        root_manifest,
        message: None,
        stats: converge::model::SnapStats::default(),
    };

    client
        .put(format!(
            "{}/repos/test/objects/snaps/{}",
            server.base_url, snap_id
        ))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .json(&snap)
        .send()
        .context("put snap")?
        .error_for_status()
        .context("put snap status")?;

    // Create metadata-only publication.
    let pubrec: serde_json::Value = client
        .post(format!("{}/repos/test/publications", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .json(&serde_json::json!({
            "snap_id": snap.id,
            "scope": "main",
            "gate": "dev-intake",
            "metadata_only": true
        }))
        .send()
        .context("create publication")?
        .error_for_status()
        .context("create publication status")?
        .json()
        .context("parse publication")?;
    let pub_id = pubrec
        .get("id")
        .and_then(|v| v.as_str())
        .context("missing publication id")?
        .to_string();

    // Create bundle.
    let bundle: serde_json::Value = client
        .post(format!("{}/repos/test/bundles", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .json(&serde_json::json!({
            "scope": "main",
            "gate": "dev-intake",
            "input_publications": [pub_id]
        }))
        .send()
        .context("create bundle")?
        .error_for_status()
        .context("create bundle status")?
        .json()
        .context("parse bundle")?;
    let bundle_id = bundle
        .get("id")
        .and_then(|v| v.as_str())
        .context("missing bundle id")?
        .to_string();

    // Create release.
    let rel: serde_json::Value = client
        .post(format!("{}/repos/test/releases", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .json(&serde_json::json!({
            "channel": "stable",
            "bundle_id": bundle_id
        }))
        .send()
        .context("create release")?
        .error_for_status()
        .context("create release status")?
        .json()
        .context("parse release")?;

    assert_eq!(rel.get("channel").and_then(|v| v.as_str()), Some("stable"));

    // List.
    let list: serde_json::Value = client
        .get(format!("{}/repos/test/releases", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .send()
        .context("list releases")?
        .error_for_status()
        .context("list releases status")?
        .json()
        .context("parse releases list")?;
    let arr = list.as_array().context("releases list not array")?;
    assert!(!arr.is_empty());

    // Show channel.
    let latest: serde_json::Value = client
        .get(format!("{}/repos/test/releases/stable", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .send()
        .context("get release channel")?
        .error_for_status()
        .context("get release channel status")?
        .json()
        .context("parse release")?;
    assert_eq!(
        latest.get("channel").and_then(|v| v.as_str()),
        Some("stable")
    );

    // Smoke: GC should keep the released bundle.
    let gc: serde_json::Value = client
        .post(format!("{}/repos/test/gc", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &auth)
        .send()
        .context("gc")?
        .error_for_status()
        .context("gc status")?
        .json()
        .context("parse gc")?;
    let kept_bundles = gc
        .get("kept")
        .and_then(|v| v.get("bundles"))
        .and_then(|v| v.as_u64())
        .context("missing kept.bundles")?;
    assert!(kept_bundles >= 1);

    Ok(())
}
