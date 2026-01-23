mod common;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

use converge::model::{ManifestEntryKind, ObjectId, Resolution};
use converge::remote::RemoteClient;
use converge::store::LocalStore;

fn run_converge(cwd: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new(env!("CARGO_BIN_EXE_converge"))
        .current_dir(cwd)
        .args(args)
        .output()
        .with_context(|| format!("run converge {:?} in {}", args, cwd.display()))?;

    if !out.status.success() {
        anyhow::bail!(
            "converge {:?} failed (status {:?})\nstdout:\n{}\nstderr:\n{}",
            args,
            out.status,
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

#[derive(Debug, serde::Deserialize)]
struct Bundle {
    id: String,
    root_manifest: String,
    promotable: bool,
    reasons: Vec<String>,
}

fn setup_workspace(ws: &Path, base_url: &str, token: &str) -> Result<()> {
    run_converge(ws, &["init"])?;
    run_converge(
        ws,
        &[
            "remote",
            "set",
            "--url",
            base_url,
            "--token",
            token,
            "--repo",
            "test",
            "--scope",
            "main",
            "--gate",
            "dev-intake",
        ],
    )?;
    Ok(())
}

fn collect_superposition_paths(store: &LocalStore, root: &ObjectId) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let mut stack = vec![(String::new(), root.clone())];

    while let Some((prefix, mid)) = stack.pop() {
        let manifest = store.get_manifest(&mid)?;
        for e in manifest.entries {
            let path = if prefix.is_empty() {
                e.name.clone()
            } else {
                format!("{}/{}", prefix, e.name)
            };

            match e.kind {
                ManifestEntryKind::Dir { manifest } => {
                    stack.push((path, manifest));
                }
                ManifestEntryKind::Superposition { .. } => {
                    out.push(path);
                }
                ManifestEntryKind::File { .. } | ManifestEntryKind::Symlink { .. } => {}
            }
        }
    }

    out.sort();
    out.dedup();
    Ok(out)
}

#[test]
fn phase6_e2e_resolve_superpositions_produces_promotable_bundle() -> Result<()> {
    let server = common::spawn_server()?;
    let base_url = server.base_url.clone();
    let token = server.token.clone();

    let ws1 = tempfile::tempdir().context("create ws1")?;
    let ws2 = tempfile::tempdir().context("create ws2")?;

    setup_workspace(ws1.path(), &base_url, &token)?;
    setup_workspace(ws2.path(), &base_url, &token)?;

    run_converge(ws1.path(), &["remote", "create-repo"])?;

    // Configure a simple 2-gate graph: dev-intake -> team
    let client = reqwest::blocking::Client::new();
    client
        .put(format!("{}/repos/test/gate-graph", base_url))
        .header(reqwest::header::AUTHORIZATION, common::auth_header(&token))
        .json(&serde_json::json!({
            "version": 1,
            "terminal_gate": "team",
            "gates": [
                {"id": "dev-intake", "name": "Dev Intake", "upstream": [], "allow_superpositions": false, "required_approvals": 0},
                {"id": "team", "name": "Team", "upstream": ["dev-intake"], "allow_superpositions": false, "required_approvals": 0}
            ]
        }))
        .send()
        .context("put gate graph")?
        .error_for_status()
        .context("put gate graph status")?;

    // Publish two conflicting snaps.
    fs::write(ws1.path().join("a.txt"), b"one\n").context("write a.txt ws1")?;
    let snap1 = run_converge(ws1.path(), &["snap", "-m", "one"])?;
    run_converge(ws1.path(), &["publish", "--snap-id", &snap1])?;

    fs::write(ws2.path().join("a.txt"), b"two\n").context("write a.txt ws2")?;
    let snap2 = run_converge(ws2.path(), &["snap", "-m", "two"])?;
    run_converge(ws2.path(), &["publish", "--snap-id", &snap2])?;

    // Bundle should be blocked due to superpositions.
    let bundle_json = run_converge(ws1.path(), &["bundle", "--json"])?;
    let bundle: Bundle = serde_json::from_str(&bundle_json).context("parse bundle")?;
    assert!(!bundle.promotable);
    assert!(bundle.reasons.iter().any(|r| r == "superpositions_present"));

    // Create a resolution file by choosing variant #1 for every conflicted path.
    let store = LocalStore::open(ws1.path())?;
    let cfg = store.read_config()?;
    let remote = cfg.remote.context("missing remote config")?;
    let rc = RemoteClient::new(remote)?;

    let root = ObjectId(bundle.root_manifest.clone());
    rc.fetch_manifest_tree(&store, &root)?;
    let paths = collect_superposition_paths(&store, &root)?;
    assert!(
        !paths.is_empty(),
        "expected at least one superposition path"
    );

    let mut decisions = BTreeMap::new();
    for p in &paths {
        decisions.insert(p.clone(), 0);
    }

    let resolution = Resolution {
        version: 1,
        bundle_id: bundle.id.clone(),
        root_manifest: root.clone(),
        created_at: "2026-01-23T00:00:00Z".to_string(),
        decisions,
    };
    store.put_resolution(&resolution)?;

    // Apply (twice) and ensure resolved root manifest is deterministic.
    let out1 = run_converge(
        ws1.path(),
        &["resolve", "apply", "--bundle-id", &bundle.id, "--json"],
    )?;
    let json1: serde_json::Value = serde_json::from_str(&out1).context("parse resolve json 1")?;
    let root1 = json1
        .get("snap")
        .and_then(|s| s.get("root_manifest"))
        .and_then(|v| v.as_str())
        .context("missing snap.root_manifest (1)")?
        .to_string();

    let out2 = run_converge(
        ws1.path(),
        &["resolve", "apply", "--bundle-id", &bundle.id, "--json"],
    )?;
    let json2: serde_json::Value = serde_json::from_str(&out2).context("parse resolve json 2")?;
    let root2 = json2
        .get("snap")
        .and_then(|s| s.get("root_manifest"))
        .and_then(|v| v.as_str())
        .context("missing snap.root_manifest (2)")?
        .to_string();
    assert_eq!(
        root1, root2,
        "resolved root manifest should be deterministic"
    );

    // Apply+publish and get the resulting publication id.
    let out3 = run_converge(
        ws1.path(),
        &[
            "resolve",
            "apply",
            "--bundle-id",
            &bundle.id,
            "--publish",
            "--json",
        ],
    )?;
    let json3: serde_json::Value = serde_json::from_str(&out3).context("parse resolve json 3")?;
    let pub_id = json3
        .get("published_publication_id")
        .and_then(|v| v.as_str())
        .context("missing published_publication_id")?
        .to_string();

    // Bundle only the resolved publication; it should be promotable.
    let bundle2_json = run_converge(ws1.path(), &["bundle", "--publication", &pub_id, "--json"])?;
    let bundle2: Bundle = serde_json::from_str(&bundle2_json).context("parse bundle2")?;
    assert!(bundle2.promotable, "expected promotable resolved bundle");
    assert!(bundle2.reasons.is_empty(), "expected no blockers");

    Ok(())
}
