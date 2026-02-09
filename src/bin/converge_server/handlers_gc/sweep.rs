use super::*;

pub(super) fn sweep_ids(
    dir: &std::path::Path,
    ext: Option<&str>,
    keep: &HashSet<String>,
    dry_run: bool,
) -> Result<(usize, usize), Response> {
    if !dir.is_dir() {
        return Ok((0, 0));
    }
    let mut deleted = 0;
    let mut kept = 0;
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("read {}", dir.display()))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?
    {
        let entry = entry
            .with_context(|| format!("read {} entry", dir.display()))
            .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let id = match ext {
            None => path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string()),
            Some(e) => {
                if path.extension().and_then(|s| s.to_str()) != Some(e) {
                    continue;
                }
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string())
            }
        };
        let Some(id) = id else {
            continue;
        };
        if id.len() != 64 {
            continue;
        }
        if keep.contains(&id) {
            kept += 1;
            continue;
        }
        deleted += 1;
        if !dry_run {
            std::fs::remove_file(&path)
                .with_context(|| format!("remove {}", path.display()))
                .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
        }
    }
    Ok((deleted, kept))
}
