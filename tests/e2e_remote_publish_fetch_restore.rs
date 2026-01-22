use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

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

fn wait_for_healthz(base_url: &str) -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let start = Instant::now();
    loop {
        if start.elapsed() > Duration::from_secs(5) {
            anyhow::bail!("server did not become healthy at {}/healthz", base_url);
        }
        match client.get(format!("{}/healthz", base_url)).send() {
            Ok(resp) if resp.status().is_success() => return Ok(()),
            _ => {
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
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
fn e2e_publish_fetch_restore() -> Result<()> {
    let server_data = tempfile::tempdir().context("create server tempdir")?;

    let listener = TcpListener::bind("127.0.0.1:0").context("bind ephemeral port")?;
    let port = listener.local_addr()?.port();
    drop(listener);

    let token = "dev";
    let base_url = format!("http://127.0.0.1:{}", port);

    let server = Command::new(env!("CARGO_BIN_EXE_converge-server"))
        .args([
            "--addr",
            &format!("127.0.0.1:{}", port),
            "--data-dir",
            server_data.path().to_str().unwrap(),
            "--dev-token",
            token,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("spawn converge-server")?;

    struct KillOnDrop(std::process::Child);
    impl Drop for KillOnDrop {
        fn drop(&mut self) {
            let _ = self.0.kill();
        }
    }
    let _guard = KillOnDrop(server);

    wait_for_healthz(&base_url)?;

    let ws1 = tempfile::tempdir().context("create ws1")?;
    let ws2 = tempfile::tempdir().context("create ws2")?;

    // Workspace 1: create snap and publish.
    run_converge(ws1.path(), &["init"])?;
    run_converge(
        ws1.path(),
        &[
            "remote",
            "set",
            "--url",
            &base_url,
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
    run_converge(ws1.path(), &["remote", "create-repo"])?;

    write_fixture(ws1.path())?;
    let expected = capture_files(ws1.path())?;

    let snap_id = run_converge(ws1.path(), &["snap", "-m", "e2e"])?
        .trim()
        .to_string();
    run_converge(ws1.path(), &["publish", "--snap-id", &snap_id])?;

    // Workspace 2: fetch and restore.
    run_converge(ws2.path(), &["init"])?;
    run_converge(
        ws2.path(),
        &[
            "remote",
            "set",
            "--url",
            &base_url,
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

    run_converge(ws2.path(), &["fetch", "--snap-id", &snap_id])?;
    run_converge(ws2.path(), &["restore", &snap_id, "--force"])?;

    let actual = capture_files(ws2.path())?;
    assert_eq!(expected, actual);
    Ok(())
}
