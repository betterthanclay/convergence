use std::collections::{BTreeMap, HashMap};

use anyhow::{Context, Result};

use crate::model::{Manifest, ManifestEntryKind, ObjectId};
use crate::store::LocalStore;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind")]
pub enum EntrySig {
    File {
        blob: String,
        mode: u32,
        size: u64,
    },
    FileChunks {
        recipe: String,
        mode: u32,
        size: u64,
    },
    Symlink {
        target: String,
    },
    Superposition {
        variants: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "status")]
pub enum DiffLine {
    Added {
        path: String,
        to: EntrySig,
    },
    Deleted {
        path: String,
        from: EntrySig,
    },
    Modified {
        path: String,
        from: EntrySig,
        to: EntrySig,
    },
}

fn join_path(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{}/{}", prefix, name)
    }
}

fn sig_for_kind(kind: &ManifestEntryKind) -> Result<EntrySig> {
    match kind {
        ManifestEntryKind::File { blob, mode, size } => Ok(EntrySig::File {
            blob: blob.as_str().to_string(),
            mode: *mode,
            size: *size,
        }),
        ManifestEntryKind::FileChunks { recipe, mode, size } => Ok(EntrySig::FileChunks {
            recipe: recipe.as_str().to_string(),
            mode: *mode,
            size: *size,
        }),
        ManifestEntryKind::Symlink { target } => Ok(EntrySig::Symlink {
            target: target.clone(),
        }),
        ManifestEntryKind::Superposition { variants } => Ok(EntrySig::Superposition {
            variants: variants.len(),
        }),
        ManifestEntryKind::Dir { .. } => {
            anyhow::bail!("dir entry should be handled by traversal")
        }
    }
}

pub fn tree_from_store(store: &LocalStore, root: &ObjectId) -> Result<BTreeMap<String, EntrySig>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![(root.clone(), String::new())];
    while let Some((mid, prefix)) = stack.pop() {
        let m = store.get_manifest(&mid)?;
        walk_manifest(&m, &prefix, &mut out, |child| {
            stack.push(child);
        })?;
    }
    Ok(out)
}

pub fn tree_from_memory(
    manifests: &HashMap<ObjectId, Manifest>,
    root: &ObjectId,
) -> Result<BTreeMap<String, EntrySig>> {
    let mut out = BTreeMap::new();
    let mut stack = vec![(root.clone(), String::new())];
    while let Some((mid, prefix)) = stack.pop() {
        let m = manifests
            .get(&mid)
            .with_context(|| format!("manifest missing: {}", mid.as_str()))?;
        walk_manifest(m, &prefix, &mut out, |child| {
            stack.push(child);
        })?;
    }
    Ok(out)
}

fn walk_manifest(
    m: &Manifest,
    prefix: &str,
    out: &mut BTreeMap<String, EntrySig>,
    mut push_dir: impl FnMut((ObjectId, String)),
) -> Result<()> {
    for e in &m.entries {
        let path = join_path(prefix, &e.name);
        match &e.kind {
            ManifestEntryKind::Dir { manifest } => {
                push_dir((manifest.clone(), path));
            }
            other => {
                out.insert(path, sig_for_kind(other)?);
            }
        }
    }
    Ok(())
}

pub fn diff_trees(
    from: &BTreeMap<String, EntrySig>,
    to: &BTreeMap<String, EntrySig>,
) -> Vec<DiffLine> {
    let mut out = Vec::new();

    for (path, from_sig) in from {
        match to.get(path) {
            None => out.push(DiffLine::Deleted {
                path: path.clone(),
                from: from_sig.clone(),
            }),
            Some(to_sig) => {
                if from_sig != to_sig {
                    out.push(DiffLine::Modified {
                        path: path.clone(),
                        from: from_sig.clone(),
                        to: to_sig.clone(),
                    });
                }
            }
        }
    }

    for (path, to_sig) in to {
        if !from.contains_key(path) {
            out.push(DiffLine::Added {
                path: path.clone(),
                to: to_sig.clone(),
            });
        }
    }

    out.sort_by(|a, b| a_path(a).cmp(a_path(b)));
    out
}

fn a_path(d: &DiffLine) -> &str {
    match d {
        DiffLine::Added { path, .. } => path,
        DiffLine::Deleted { path, .. } => path,
        DiffLine::Modified { path, .. } => path,
    }
}
