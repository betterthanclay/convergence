use anyhow::{Context, Result};

mod common;

fn create_user_and_token(
    client: &reqwest::blocking::Client,
    base_url: &str,
    admin_auth: &str,
    handle: &str,
) -> Result<String> {
    let user: serde_json::Value = client
        .post(format!("{}/users", base_url))
        .header(reqwest::header::AUTHORIZATION, admin_auth)
        .json(&serde_json::json!({"handle": handle}))
        .send()
        .context("create user")?
        .error_for_status()
        .context("create user status")?
        .json()
        .context("parse create user")?;
    let user_id = user
        .get("id")
        .and_then(|v| v.as_str())
        .context("missing user id")?
        .to_string();

    let tok: serde_json::Value = client
        .post(format!("{}/users/{}/tokens", base_url, user_id))
        .header(reqwest::header::AUTHORIZATION, admin_auth)
        .json(&serde_json::json!({"label": handle}))
        .send()
        .context("mint token")?
        .error_for_status()
        .context("mint token status")?
        .json()
        .context("parse token")?;
    let token = tok
        .get("token")
        .and_then(|v| v.as_str())
        .context("missing token")?
        .to_string();
    Ok(token)
}

fn upload_metadata_only_snap(
    client: &reqwest::blocking::Client,
    base_url: &str,
    auth: &str,
    repo_id: &str,
    created_at: &str,
) -> Result<String> {
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
            "{}/repos/{}/objects/manifests/{}?allow_missing_blobs=true",
            base_url, repo_id, manifest_id
        ))
        .header(reqwest::header::AUTHORIZATION, auth)
        .body(manifest_bytes)
        .send()
        .context("put manifest")?
        .error_for_status()
        .context("put manifest status")?;

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
            "{}/repos/{}/objects/snaps/{}",
            base_url, repo_id, snap_id
        ))
        .header(reqwest::header::AUTHORIZATION, auth)
        .json(&snap)
        .send()
        .context("put snap")?
        .error_for_status()
        .context("put snap status")?;

    Ok(snap_id)
}

#[test]
fn non_terminal_release_requires_admin() -> Result<()> {
    let server = common::spawn_server()?;
    let client = reqwest::blocking::Client::new();
    let admin_auth = common::auth_header(&server.token);

    // Create repo.
    client
        .post(format!("{}/repos", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &admin_auth)
        .json(&serde_json::json!({"id": "test"}))
        .send()
        .context("create repo")?
        .error_for_status()
        .context("create repo status")?;

    // Configure a 2-gate graph: dev-intake -> rc, terminal=rc.
    let mut graph: serde_json::Value = client
        .get(format!("{}/repos/test/gate-graph", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &admin_auth)
        .send()
        .context("get gate graph")?
        .error_for_status()
        .context("get gate graph status")?
        .json()
        .context("parse gate graph")?;

    graph["terminal_gate"] = serde_json::Value::String("rc".to_string());
    let gates = graph
        .get_mut("gates")
        .and_then(|v| v.as_array_mut())
        .context("gate graph gates missing")?;
    for g in gates.iter_mut() {
        if g.get("id") == Some(&serde_json::Value::String("dev-intake".to_string())) {
            g["allow_metadata_only_publications"] = serde_json::Value::Bool(true);
        }
    }
    gates.push(serde_json::json!({
        "id": "rc",
        "name": "Release Candidate",
        "upstream": ["dev-intake"],
        "allow_superpositions": false,
        "allow_metadata_only_publications": false,
        "required_approvals": 0
    }));

    client
        .put(format!("{}/repos/test/gate-graph", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &admin_auth)
        .json(&graph)
        .send()
        .context("put gate graph")?
        .error_for_status()
        .context("put gate graph status")?;

    // Create non-admin publisher.
    let alice_token = create_user_and_token(&client, &server.base_url, &admin_auth, "alice")?;
    client
        .post(format!("{}/repos/test/members", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &admin_auth)
        .json(&serde_json::json!({"handle": "alice", "role": "publish"}))
        .send()
        .context("add alice")?
        .error_for_status()
        .context("add alice status")?;

    let alice_auth = common::auth_header(&alice_token);

    // Create publication and bundle at dev-intake (non-terminal).
    let snap_id = upload_metadata_only_snap(
        &client,
        &server.base_url,
        &alice_auth,
        "test",
        "2026-01-25T03:00:00Z",
    )?;
    let pubrec: serde_json::Value = client
        .post(format!("{}/repos/test/publications", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &alice_auth)
        .json(&serde_json::json!({
            "snap_id": snap_id,
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
        .context("missing pub id")?
        .to_string();

    let bundle: serde_json::Value = client
        .post(format!("{}/repos/test/bundles", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &alice_auth)
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

    // Non-admin cannot release from dev-intake now that terminal is rc.
    let resp = client
        .post(format!("{}/repos/test/releases", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &alice_auth)
        .json(&serde_json::json!({"channel": "stable", "bundle_id": bundle_id.clone()}))
        .send()
        .context("create release (alice)")?;
    assert_eq!(resp.status(), reqwest::StatusCode::BAD_REQUEST);

    // Admin can.
    client
        .post(format!("{}/repos/test/releases", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &admin_auth)
        .json(&serde_json::json!({"channel": "stable", "bundle_id": bundle_id}))
        .send()
        .context("create release (admin)")?
        .error_for_status()
        .context("create release (admin) status")?;

    Ok(())
}
