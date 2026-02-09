use std::collections::HashSet;

use anyhow::Result;

use crate::model::ObjectId;
use crate::store::LocalStore;

pub(crate) fn collect_objects(
    store: &LocalStore,
    root: &ObjectId,
) -> Result<(HashSet<String>, HashSet<String>, HashSet<String>)> {
    let mut blobs = HashSet::new();
    let mut manifests = HashSet::new();
    let mut recipes = HashSet::new();
    let mut stack = vec![root.clone()];

    while let Some(mid) = stack.pop() {
        if !manifests.insert(mid.as_str().to_string()) {
            continue;
        }
        let m = store.get_manifest(&mid)?;
        for e in m.entries {
            match e.kind {
                crate::model::ManifestEntryKind::File { blob, .. } => {
                    blobs.insert(blob.as_str().to_string());
                }
                crate::model::ManifestEntryKind::FileChunks { recipe, .. } => {
                    recipes.insert(recipe.as_str().to_string());
                    let r = store.get_recipe(&recipe)?;
                    for c in r.chunks {
                        blobs.insert(c.blob.as_str().to_string());
                    }
                }
                crate::model::ManifestEntryKind::Dir { manifest } => {
                    stack.push(manifest);
                }
                crate::model::ManifestEntryKind::Symlink { .. } => {}
                crate::model::ManifestEntryKind::Superposition { .. } => {
                    anyhow::bail!("cannot publish snap containing superpositions");
                }
            }
        }
    }

    Ok((blobs, manifests, recipes))
}

pub(crate) fn manifest_postorder(store: &LocalStore, root: &ObjectId) -> Result<Vec<ObjectId>> {
    fn visit(
        store: &LocalStore,
        id: &ObjectId,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        out: &mut Vec<ObjectId>,
    ) -> Result<()> {
        let key = id.as_str().to_string();
        if visited.contains(&key) {
            return Ok(());
        }
        if !visiting.insert(key.clone()) {
            anyhow::bail!("cycle detected in manifest graph at {}", id.as_str());
        }

        let manifest = store.get_manifest(id)?;
        for e in manifest.entries {
            if let crate::model::ManifestEntryKind::Dir { manifest } = e.kind {
                visit(store, &manifest, visiting, visited, out)?;
            }
        }

        visiting.remove(&key);
        visited.insert(key);
        out.push(id.clone());
        Ok(())
    }

    let mut out = Vec::new();
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    visit(store, root, &mut visiting, &mut visited, &mut out)?;
    Ok(out)
}
