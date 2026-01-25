use std::process::Command;

use anyhow::{Context, Result};

mod common;

#[test]
fn cli_remote_gc_runs_and_returns_json() -> Result<()> {
    let server = common::spawn_server()?;

    let ws = tempfile::tempdir().context("create ws")?;
    let out = Command::new(env!("CARGO_BIN_EXE_converge"))
        .current_dir(ws.path())
        .args(["init"])
        .output()
        .context("init")?;
    anyhow::ensure!(
        out.status.success(),
        "init failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let out = Command::new(env!("CARGO_BIN_EXE_converge"))
        .current_dir(ws.path())
        .args([
            "login",
            "--url",
            &server.base_url,
            "--token",
            &server.token,
            "--repo",
            "test",
        ])
        .output()
        .context("login")?;
    anyhow::ensure!(out.status.success(), "login failed");

    let out = Command::new(env!("CARGO_BIN_EXE_converge"))
        .current_dir(ws.path())
        .args(["remote", "create-repo"])
        .output()
        .context("create repo")?;
    anyhow::ensure!(out.status.success(), "create-repo failed");

    let out = Command::new(env!("CARGO_BIN_EXE_converge"))
        .current_dir(ws.path())
        .args(["remote", "gc", "--json"])
        .output()
        .context("remote gc")?;
    anyhow::ensure!(
        out.status.success(),
        "remote gc failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    let s = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&s).context("parse gc json")?;
    anyhow::ensure!(v.get("kept").is_some());
    anyhow::ensure!(v.get("deleted").is_some());

    Ok(())
}
