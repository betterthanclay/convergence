use anyhow::{Context, Result};

mod common;

fn auth_header(token: &str) -> String {
    format!("Bearer {}", token)
}

fn upload_metadata_only_snap(
    client: &reqwest::blocking::Client,
    base_url: &str,
    auth: &str,
    repo_id: &str,
    created_at: &str,
    filename: &str,
    missing_blob: &str,
) -> Result<String> {
    let manifest = converge::model::Manifest {
        version: 1,
        entries: vec![converge::model::ManifestEntry {
            name: filename.to_string(),
            kind: converge::model::ManifestEntryKind::File {
                blob: converge::model::ObjectId(missing_blob.to_string()),
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

fn enable_metadata_only_publications(
    client: &reqwest::blocking::Client,
    base_url: &str,
    auth: &str,
    repo_id: &str,
    gate_id: &str,
) -> Result<()> {
    let mut graph: serde_json::Value = client
        .get(format!("{}/repos/{}/gate-graph", base_url, repo_id))
        .header(reqwest::header::AUTHORIZATION, auth)
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
        if g.get("id") == Some(&serde_json::Value::String(gate_id.to_string())) {
            g["allow_metadata_only_publications"] = serde_json::Value::Bool(true);
        }
    }

    client
        .put(format!("{}/repos/{}/gate-graph", base_url, repo_id))
        .header(reqwest::header::AUTHORIZATION, auth)
        .json(&graph)
        .send()
        .context("put gate graph")?
        .error_for_status()
        .context("put gate graph status")?;

    Ok(())
}

#[test]
fn provenance_records_distinct_user_ids_for_publications() -> Result<()> {
    let server = common::spawn_server()?;

    let client = reqwest::blocking::Client::new();
    let base_url = server.base_url.clone();

    let dev_token = server.token.clone();
    let dev_auth = auth_header(&dev_token);

    // Create repo.
    client
        .post(format!("{}/repos", base_url))
        .header(reqwest::header::AUTHORIZATION, &dev_auth)
        .json(&serde_json::json!({"id": "test"}))
        .send()
        .context("create repo")?
        .error_for_status()
        .context("create repo status")?;

    enable_metadata_only_publications(&client, &server.base_url, &dev_auth, "test", "dev-intake")?;

    // Create second user and token.
    let user: serde_json::Value = client
        .post(format!("{}/users", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &dev_auth)
        .json(&serde_json::json!({"handle": "alice"}))
        .send()
        .context("create user")?
        .error_for_status()
        .context("create user status")?
        .json()
        .context("parse create user response")?;
    let alice_id = user
        .get("id")
        .and_then(|v| v.as_str())
        .context("missing user id")?
        .to_string();

    let tok: serde_json::Value = client
        .post(format!("{}/users/{}/tokens", server.base_url, alice_id))
        .header(reqwest::header::AUTHORIZATION, &dev_auth)
        .json(&serde_json::json!({"label": "alice"}))
        .send()
        .context("mint alice token")?
        .error_for_status()
        .context("mint alice token status")?
        .json()
        .context("parse alice token response")?;
    let alice_token = tok
        .get("token")
        .and_then(|v| v.as_str())
        .context("missing alice token")?
        .to_string();
    let alice_auth = auth_header(&alice_token);

    // Add alice as publisher.
    client
        .post(format!("{}/repos/test/members", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &dev_auth)
        .json(&serde_json::json!({"handle": "alice", "role": "publish"}))
        .send()
        .context("add alice repo member")?
        .error_for_status()
        .context("add alice repo member status")?;

    // Get whoami identities.
    let dev_who: serde_json::Value = client
        .get(format!("{}/whoami", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &dev_auth)
        .send()
        .context("dev whoami")?
        .error_for_status()
        .context("dev whoami status")?
        .json()
        .context("parse dev whoami")?;
    let dev_id = dev_who
        .get("user_id")
        .and_then(|v| v.as_str())
        .context("missing dev user_id")?
        .to_string();

    let alice_who: serde_json::Value = client
        .get(format!("{}/whoami", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &alice_auth)
        .send()
        .context("alice whoami")?
        .error_for_status()
        .context("alice whoami status")?
        .json()
        .context("parse alice whoami")?;
    let alice_who_id = alice_who
        .get("user_id")
        .and_then(|v| v.as_str())
        .context("missing alice user_id")?
        .to_string();

    anyhow::ensure!(alice_who_id == alice_id, "alice user_id mismatch");
    anyhow::ensure!(dev_id != alice_who_id, "dev/alice user_id should differ");

    // Create snaps + metadata-only publications as each identity.
    let missing_blob_dev = "1".repeat(64);
    let snap_dev = upload_metadata_only_snap(
        &client,
        &server.base_url,
        &dev_auth,
        "test",
        "2026-01-25T01:00:00Z",
        "dev.txt",
        &missing_blob_dev,
    )?;
    let pub_dev: serde_json::Value = client
        .post(format!("{}/repos/test/publications", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &dev_auth)
        .json(&serde_json::json!({
            "snap_id": snap_dev,
            "scope": "main",
            "gate": "dev-intake",
            "metadata_only": true
        }))
        .send()
        .context("create dev publication")?
        .error_for_status()
        .context("create dev publication status")?
        .json()
        .context("parse dev publication")?;

    let missing_blob_alice = "2".repeat(64);
    let snap_alice = upload_metadata_only_snap(
        &client,
        &server.base_url,
        &alice_auth,
        "test",
        "2026-01-25T02:00:00Z",
        "alice.txt",
        &missing_blob_alice,
    )?;
    let pub_alice: serde_json::Value = client
        .post(format!("{}/repos/test/publications", server.base_url))
        .header(reqwest::header::AUTHORIZATION, &alice_auth)
        .json(&serde_json::json!({
            "snap_id": snap_alice,
            "scope": "main",
            "gate": "dev-intake",
            "metadata_only": true
        }))
        .send()
        .context("create alice publication")?
        .error_for_status()
        .context("create alice publication status")?
        .json()
        .context("parse alice publication")?;

    // Assert provenance contains user ids.
    let dev_pub_uid = pub_dev
        .get("publisher_user_id")
        .and_then(|v| v.as_str())
        .context("missing dev publisher_user_id")?;
    let dev_pub_handle = pub_dev
        .get("publisher")
        .and_then(|v| v.as_str())
        .context("missing dev publisher")?;

    let alice_pub_uid = pub_alice
        .get("publisher_user_id")
        .and_then(|v| v.as_str())
        .context("missing alice publisher_user_id")?;
    let alice_pub_handle = pub_alice
        .get("publisher")
        .and_then(|v| v.as_str())
        .context("missing alice publisher")?;

    anyhow::ensure!(dev_pub_handle == "dev");
    anyhow::ensure!(alice_pub_handle == "alice");
    anyhow::ensure!(dev_pub_uid == dev_id);
    anyhow::ensure!(alice_pub_uid == alice_who_id);
    anyhow::ensure!(dev_pub_uid != alice_pub_uid);

    Ok(())
}
