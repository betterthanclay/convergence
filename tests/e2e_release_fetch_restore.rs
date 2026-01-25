use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

mod common;

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

fn write_fixture(dir: &Path) -> Result<()> {
    fs::create_dir_all(dir.join("sub")).context("create sub dir")?;
    fs::write(dir.join("a.txt"), b"hello\n").context("write a.txt")?;
    fs::write(dir.join("sub/b.txt"), b"world\n").context("write b.txt")?;
    Ok(())
}

fn capture_files(dir: &Path) -> Result<Vec<(PathBuf, Vec<u8>)>> {
    let mut out = Vec::new();
    capture_dir(dir, Path::new(""), &mut out)?;
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

fn capture_dir(root: &Path, rel: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) -> Result<()> {
    let dir = root.join(rel);
    for entry in fs::read_dir(&dir).with_context(|| format!("read dir {}", dir.display()))? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".converge" {
            continue;
        }
        let name = name
            .into_string()
            .map_err(|_| anyhow::anyhow!("non-utf8 filename"))?;

        let child_rel = rel.join(&name);
        let path = root.join(&child_rel);
        let ft = entry.file_type()?;
        if ft.is_dir() {
            capture_dir(root, &child_rel, out)?;
            continue;
        }
        if ft.is_file() {
            out.push((child_rel, fs::read(&path)?));
        }
    }
    Ok(())
}

#[test]
fn e2e_release_create_and_fetch_restore() -> Result<()> {
    let server = common::spawn_server()?;

    let ws1 = tempfile::tempdir().context("create ws1")?;
    let ws2 = tempfile::tempdir().context("create ws2")?;

    // Workspace 1: publish and bundle.
    run_converge(ws1.path(), &["init"])?;
    run_converge(
        ws1.path(),
        &[
            "login",
            "--url",
            &server.base_url,
            "--token",
            &server.token,
            "--repo",
            "test",
            "--scope",
            "main",
            "--gate",
            "dev-intake",
        ],
    )?;
    run_converge(ws1.path(), &["remote", "create-repo"])?;

    write_fixture(ws1.path())?;
    let expected = capture_files(ws1.path())?;

    let snap_id = run_converge(ws1.path(), &["snap", "-m", "e2e-release"])?
        .trim()
        .to_string();
    run_converge(ws1.path(), &["publish", "--snap-id", &snap_id])?;
    let bundle_id = run_converge(ws1.path(), &["bundle"])?.trim().to_string();

    run_converge(
        ws1.path(),
        &[
            "release",
            "create",
            "--channel",
            "stable",
            "--bundle-id",
            &bundle_id,
        ],
    )?;

    // Workspace 2: fetch release and restore to a directory.
    run_converge(ws2.path(), &["init"])?;
    run_converge(
        ws2.path(),
        &[
            "login",
            "--url",
            &server.base_url,
            "--token",
            &server.token,
            "--repo",
            "test",
        ],
    )?;

    let out_dir = ws2.path().join("out");
    run_converge(
        ws2.path(),
        &[
            "fetch",
            "--release",
            "stable",
            "--restore",
            "--into",
            out_dir.to_str().unwrap(),
            "--force",
        ],
    )?;

    let actual = capture_files(&out_dir)?;
    assert_eq!(expected, actual);
    Ok(())
}
