use super::*;

pub(super) fn handle_init_command(force: bool, path: Option<std::path::PathBuf>) -> Result<()> {
    let root = path.unwrap_or(std::env::current_dir().context("get current dir")?);
    Workspace::init(&root, force)?;
    println!("Initialized Convergence workspace at {}", root.display());
    Ok(())
}

pub(super) fn handle_snap_command(message: Option<String>, json: bool) -> Result<()> {
    let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
    let snap = ws.create_snap(message)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&snap).context("serialize snap json")?
        );
    } else {
        println!("{}", snap.id);
    }
    Ok(())
}

pub(super) fn handle_snaps_command(json: bool) -> Result<()> {
    let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
    let snaps = ws.list_snaps()?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&snaps).context("serialize snaps json")?
        );
    } else {
        for snap in snaps {
            let short = snap.id.chars().take(8).collect::<String>();
            let msg = snap.message.unwrap_or_default();
            if msg.is_empty() {
                println!("{} {}", short, snap.created_at);
            } else {
                println!("{} {} {}", short, snap.created_at, msg);
            }
        }
    }
    Ok(())
}

pub(super) fn handle_show_command(snap_id: String, json: bool) -> Result<()> {
    let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
    let snap = ws.show_snap(&snap_id)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&snap).context("serialize snap")?
        );
    } else {
        println!("id: {}", snap.id);
        println!("created_at: {}", snap.created_at);
        if let Some(msg) = snap.message
            && !msg.is_empty()
        {
            println!("message: {}", msg);
        }
        println!("root_manifest: {}", snap.root_manifest.as_str());
        println!(
            "stats: files={} dirs={} symlinks={} bytes={}",
            snap.stats.files, snap.stats.dirs, snap.stats.symlinks, snap.stats.bytes
        );
    }
    Ok(())
}

pub(super) fn handle_restore_command(snap_id: String, force: bool) -> Result<()> {
    let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
    ws.restore_snap(&snap_id, force)?;
    println!("Restored {}", snap_id);
    Ok(())
}

pub(super) fn handle_diff_command(
    from: Option<String>,
    to: Option<String>,
    json: bool,
) -> Result<()> {
    let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;

    let diffs = match (from.as_deref(), to.as_deref()) {
        (None, None) => {
            let head = ws.store.get_head()?.context("no HEAD snap")?;
            let head_snap = ws.store.get_snap(&head)?;
            let from_tree = converge::diff::tree_from_store(&ws.store, &head_snap.root_manifest)?;

            let (cur_root, cur_manifests, _stats) = ws.current_manifest_tree()?;
            let to_tree = converge::diff::tree_from_memory(&cur_manifests, &cur_root)?;

            converge::diff::diff_trees(&from_tree, &to_tree)
        }
        (Some(_), None) | (None, Some(_)) => {
            anyhow::bail!(
                "use both --from and --to for snap diffs, or omit both for workspace vs HEAD"
            )
        }
        (Some(from), Some(to)) => {
            let from_snap = ws.store.get_snap(from)?;
            let to_snap = ws.store.get_snap(to)?;
            let from_tree = converge::diff::tree_from_store(&ws.store, &from_snap.root_manifest)?;
            let to_tree = converge::diff::tree_from_store(&ws.store, &to_snap.root_manifest)?;
            converge::diff::diff_trees(&from_tree, &to_tree)
        }
    };

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&diffs).context("serialize diff json")?
        );
    } else {
        for d in &diffs {
            match d {
                converge::diff::DiffLine::Added { path, .. } => println!("A {}", path),
                converge::diff::DiffLine::Deleted { path, .. } => println!("D {}", path),
                converge::diff::DiffLine::Modified { path, .. } => println!("M {}", path),
            }
        }
        println!("{} changes", diffs.len());
    }
    Ok(())
}

pub(super) fn handle_mv_command(from: String, to: String) -> Result<()> {
    let ws = Workspace::discover(&std::env::current_dir().context("get current dir")?)?;
    ws.move_path(std::path::Path::new(&from), std::path::Path::new(&to))?;
    println!("Moved {} -> {}", from, to);
    Ok(())
}
