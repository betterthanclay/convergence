use super::*;
use crate::model::{
    FileRecipe, FileRecipeChunk, Manifest, ManifestEntry, ManifestEntryKind, ObjectId,
};
use crate::store::LocalStore;
use tempfile::tempdir;

fn setup_store() -> anyhow::Result<(tempfile::TempDir, LocalStore)> {
    let dir = tempdir()?;
    let store = LocalStore::init(dir.path(), false)?;
    Ok((dir, store))
}

fn manifest_with_file(name: &str, blob: &ObjectId, size: u64) -> Manifest {
    Manifest {
        version: 1,
        entries: vec![ManifestEntry {
            name: name.to_string(),
            kind: ManifestEntryKind::File {
                blob: blob.clone(),
                mode: 0o100644,
                size,
            },
        }],
    }
}

fn manifest_with_chunked_file(name: &str, recipe: &ObjectId, size: u64) -> Manifest {
    Manifest {
        version: 1,
        entries: vec![ManifestEntry {
            name: name.to_string(),
            kind: ManifestEntryKind::FileChunks {
                recipe: recipe.clone(),
                mode: 0o100644,
                size,
            },
        }],
    }
}

#[test]
fn detects_exact_rename_for_same_blob() -> anyhow::Result<()> {
    let (_dir, store) = setup_store()?;

    let blob = store.put_blob(b"hello\n")?;
    let base_manifest = manifest_with_file("a.txt", &blob, 6);
    let base_root = store.put_manifest(&base_manifest)?;

    let cur_manifest = manifest_with_file("b.txt", &blob, 6);
    let cur_root = store.put_manifest(&cur_manifest)?;
    let mut cur_manifests = std::collections::HashMap::new();
    cur_manifests.insert(cur_root.clone(), cur_manifest);

    let out = diff_trees_with_renames(
        &store,
        Some(&base_root),
        &cur_root,
        &cur_manifests,
        None,
        default_chunk_size_bytes(),
    )?;
    assert_eq!(out.len(), 1);
    match &out[0] {
        StatusChange::Renamed { from, to, modified } => {
            assert_eq!(from, "a.txt");
            assert_eq!(to, "b.txt");
            assert!(!modified);
        }
        other => anyhow::bail!("unexpected diff: {:?}", other),
    }
    Ok(())
}

#[test]
fn detects_rename_with_small_edit_for_blobs() -> anyhow::Result<()> {
    let (_dir, store) = setup_store()?;

    let blob_old = store.put_blob(b"hello world\n")?;
    let blob_new = store.put_blob(b"hello world!\n")?;

    let base_manifest = manifest_with_file("a.txt", &blob_old, 12);
    let base_root = store.put_manifest(&base_manifest)?;

    let cur_manifest = manifest_with_file("b.txt", &blob_new, 13);
    let cur_root = store.put_manifest(&cur_manifest)?;
    let mut cur_manifests = std::collections::HashMap::new();
    cur_manifests.insert(cur_root.clone(), cur_manifest);

    let out = diff_trees_with_renames(
        &store,
        Some(&base_root),
        &cur_root,
        &cur_manifests,
        None,
        default_chunk_size_bytes(),
    )?;
    assert_eq!(out.len(), 1);
    match &out[0] {
        StatusChange::Renamed { from, to, modified } => {
            assert_eq!(from, "a.txt");
            assert_eq!(to, "b.txt");
            assert!(*modified);
        }
        other => anyhow::bail!("unexpected diff: {:?}", other),
    }
    Ok(())
}

#[test]
fn detects_rename_with_small_edit_for_recipes() -> anyhow::Result<()> {
    let (_dir, store) = setup_store()?;

    // Fake chunk ids (we don't need actual blobs for recipe storage).
    let c1 = ObjectId("1".repeat(64));
    let c2 = ObjectId("2".repeat(64));
    let c3 = ObjectId("3".repeat(64));
    let c4 = ObjectId("4".repeat(64));
    let c5 = ObjectId("5".repeat(64));
    let c6 = ObjectId("6".repeat(64));
    let c7 = ObjectId("7".repeat(64));
    let c8 = ObjectId("8".repeat(64));
    let c9 = ObjectId("9".repeat(64));
    let ca = ObjectId("a".repeat(64));
    let cb = ObjectId("b".repeat(64));

    let r_old = FileRecipe {
        version: 1,
        size: 40,
        chunks: vec![
            FileRecipeChunk {
                blob: c1.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c2.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c3.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c4.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c5.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c6.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c7.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c8.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c9.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: ca.clone(),
                size: 4,
            },
        ],
    };
    let r_new = FileRecipe {
        version: 1,
        size: 40,
        chunks: vec![
            FileRecipeChunk {
                blob: c1.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c2.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c3.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c4.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: cb.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c6.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c7.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c8.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: c9.clone(),
                size: 4,
            },
            FileRecipeChunk {
                blob: ca.clone(),
                size: 4,
            },
        ],
    };

    let rid_old = store.put_recipe(&r_old)?;
    let rid_new = store.put_recipe(&r_new)?;

    let base_manifest = manifest_with_chunked_file("a.bin", &rid_old, 40);
    let base_root = store.put_manifest(&base_manifest)?;

    let cur_manifest = manifest_with_chunked_file("b.bin", &rid_new, 40);
    let cur_root = store.put_manifest(&cur_manifest)?;
    let mut cur_manifests = std::collections::HashMap::new();
    cur_manifests.insert(cur_root.clone(), cur_manifest);

    let out = diff_trees_with_renames(
        &store,
        Some(&base_root),
        &cur_root,
        &cur_manifests,
        None,
        default_chunk_size_bytes(),
    )?;
    assert_eq!(out.len(), 1);
    match &out[0] {
        StatusChange::Renamed { from, to, modified } => {
            assert_eq!(from, "a.bin");
            assert_eq!(to, "b.bin");
            assert!(*modified);
        }
        other => anyhow::bail!("unexpected diff: {:?}", other),
    }

    Ok(())
}
