use anyhow::{Context, Result};

#[test]
fn server_bootstrap_creates_first_admin_once() -> Result<()> {
    let data_dir = tempfile::tempdir().context("create server tempdir")?;
    let addr_file = data_dir.path().join("addr.txt");

    let bootstrap_token = "bootstrap-secret";

    let mut child = std::process::Command::new(env!("CARGO_BIN_EXE_converge-server"))
        .args([
            "--addr",
            "127.0.0.1:0",
            "--addr-file",
            addr_file.to_str().unwrap(),
            "--data-dir",
            data_dir.path().to_str().unwrap(),
            "--bootstrap-token",
            bootstrap_token,
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .context("spawn converge-server")?;

    let base_url = {
        // Reuse the logic from common::wait_for_healthz but we need the addr file parsing.
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > std::time::Duration::from_secs(5) {
                let _ = child.kill();
                anyhow::bail!("addr file not written at {}", addr_file.display());
            }
            if let Ok(s) = std::fs::read_to_string(&addr_file) {
                let s = s.trim();
                if !s.is_empty() {
                    break format!("http://{}", s);
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    };
    wait_for_healthz(&base_url)?;

    let client = reqwest::blocking::Client::new();

    // Bootstrap first admin.
    let resp = client
        .post(format!("{}/bootstrap", base_url))
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", bootstrap_token),
        )
        .json(&serde_json::json!({"handle": "admin"}))
        .send()
        .context("POST /bootstrap")?;
    anyhow::ensure!(resp.status().is_success(), "bootstrap failed");
    let out: serde_json::Value = resp.json().context("parse bootstrap response")?;
    let token = out
        .get("token")
        .and_then(|t| t.get("token"))
        .and_then(|t| t.as_str())
        .context("missing bootstrap token")?
        .to_string();

    // Verify whoami works with minted token.
    let resp = client
        .get(format!("{}/whoami", base_url))
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {}", token))
        .send()
        .context("GET /whoami")?;
    anyhow::ensure!(resp.status().is_success(), "whoami failed");
    let who: serde_json::Value = resp.json().context("parse whoami")?;
    anyhow::ensure!(who.get("user").and_then(|v| v.as_str()) == Some("admin"));
    anyhow::ensure!(who.get("admin").and_then(|v| v.as_bool()) == Some(true));

    // Second bootstrap should be rejected.
    let resp = client
        .post(format!("{}/bootstrap", base_url))
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", bootstrap_token),
        )
        .json(&serde_json::json!({"handle": "admin2"}))
        .send()
        .context("POST /bootstrap (second)")?;
    anyhow::ensure!(resp.status() == reqwest::StatusCode::CONFLICT);

    let _ = child.kill();
    let _ = child.wait();

    Ok(())
}

fn wait_for_healthz(base_url: &str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let start = std::time::Instant::now();
    loop {
        if start.elapsed() > std::time::Duration::from_secs(5) {
            anyhow::bail!("server did not become healthy at {}/healthz", base_url);
        }
        match client.get(format!("{}/healthz", base_url)).send() {
            Ok(resp) if resp.status().is_success() => return Ok(()),
            _ => {
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
        }
    }
}
