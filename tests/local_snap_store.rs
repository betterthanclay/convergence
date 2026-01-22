use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use converge::workspace::Workspace;

#[test]
fn snap_and_restore_roundtrip() -> Result<()> {
    let tmp = tempfile::tempdir().context("create tempdir")?;
    let root = tmp.path();

    fs::create_dir_all(root.join("sub")).context("create sub dir")?;
    fs::write(root.join("a.txt"), b"hello\n").context("write a.txt")?;
    fs::write(root.join("sub/b.bin"), b"\x00\x01\x02").context("write b.bin")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink("a.txt", root.join("link.txt")).context("create symlink")?;
    }

    let ws = Workspace::init(root, false)?;
    let expected = capture_tree(root)?;
    let snap = ws.create_snap(Some("test snap".to_string()))?;

    // Mutate working directory.
    fs::remove_file(root.join("a.txt")).context("remove a.txt")?;
    fs::write(root.join("new.txt"), b"new\n").context("write new.txt")?;

    ws.restore_snap(&snap.id, true)?;
    let actual = capture_tree(root)?;

    assert_eq!(expected, actual);
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    File { bytes: Vec<u8>, mode: u32 },
    Symlink { target: String },
}

fn capture_tree(root: &Path) -> Result<BTreeMap<PathBuf, Node>> {
    let mut out = BTreeMap::new();
    capture_dir(root, Path::new(""), &mut out)?;
    Ok(out)
}

fn capture_dir(root: &Path, rel: &Path, out: &mut BTreeMap<PathBuf, Node>) -> Result<()> {
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

        if ft.is_symlink() {
            let target = fs::read_link(&path)?;
            let target = target
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("non-utf8 symlink target"))?
                .to_string();
            out.insert(child_rel, Node::Symlink { target });
            continue;
        }

        let bytes = fs::read(&path)?;
        let mode = file_mode(&path)?;
        out.insert(child_rel, Node::File { bytes, mode });
    }
    Ok(())
}

fn file_mode(path: &Path) -> Result<u32> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let meta = fs::symlink_metadata(path)?;
        Ok(meta.permissions().mode())
    }

    #[cfg(not(unix))]
    {
        let _ = path;
        Ok(0)
    }
}
