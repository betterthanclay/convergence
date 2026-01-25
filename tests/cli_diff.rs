use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

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

#[test]
fn diff_workspace_vs_head_and_snap_vs_snap() -> Result<()> {
    let ws = tempfile::tempdir().context("create ws")?;
    run_converge(ws.path(), &["init"])?;

    fs::write(ws.path().join("a.txt"), b"one\n").context("write a.txt")?;
    let snap1 = run_converge(ws.path(), &["snap", "-m", "s1"])?
        .trim()
        .to_string();

    // Modify and add.
    fs::write(ws.path().join("a.txt"), b"two\n").context("rewrite a.txt")?;
    fs::write(ws.path().join("b.txt"), b"hello\n").context("write b.txt")?;

    let diff_json = run_converge(ws.path(), &["diff", "--json"])?;
    let v: serde_json::Value = serde_json::from_str(&diff_json).context("parse diff json")?;
    let arr = v.as_array().context("diff json not array")?;
    assert_eq!(arr.len(), 2);
    assert!(arr.iter().any(|x| x.get("status")
        == Some(&serde_json::Value::String("Modified".to_string()))
        && x.get("path") == Some(&serde_json::Value::String("a.txt".to_string()))));
    assert!(arr.iter().any(|x| x.get("status")
        == Some(&serde_json::Value::String("Added".to_string()))
        && x.get("path") == Some(&serde_json::Value::String("b.txt".to_string()))));

    let snap2 = run_converge(ws.path(), &["snap", "-m", "s2"])?
        .trim()
        .to_string();

    let diff_json = run_converge(
        ws.path(),
        &["diff", "--from", &snap1, "--to", &snap2, "--json"],
    )?;
    let v: serde_json::Value = serde_json::from_str(&diff_json).context("parse diff json")?;
    let arr = v.as_array().context("diff json not array")?;
    assert_eq!(arr.len(), 2);

    Ok(())
}
