use crate::model::{FileRecipe, FileRecipeChunk, ObjectId};
use crate::store::LocalStore;

pub(super) fn load_blob_bytes(
    store: &LocalStore,
    workspace_root: Option<&std::path::Path>,
    rel_path: &str,
    blob_id: &str,
) -> Option<Vec<u8>> {
    let oid = ObjectId(blob_id.to_string());
    if store.has_blob(&oid) {
        return store.get_blob(&oid).ok();
    }
    let root = workspace_root?;
    let bytes = std::fs::read(root.join(std::path::Path::new(rel_path))).ok()?;
    if crate::store::hash_bytes(&bytes).as_str() != blob_id {
        return None;
    }
    Some(bytes)
}

pub(super) fn load_recipe(
    store: &LocalStore,
    workspace_root: Option<&std::path::Path>,
    rel_path: &str,
    recipe_id: &str,
    chunk_size_bytes: usize,
) -> Option<FileRecipe> {
    let oid = ObjectId(recipe_id.to_string());
    if store.has_recipe(&oid) {
        return store.get_recipe(&oid).ok();
    }

    let root = workspace_root?;
    let abs = root.join(std::path::Path::new(rel_path));
    let meta = std::fs::symlink_metadata(&abs).ok()?;
    let size = meta.len();
    let f = std::fs::File::open(&abs).ok()?;
    let mut r = std::io::BufReader::new(f);

    let mut buf = vec![0u8; chunk_size_bytes.max(64 * 1024)];
    let mut chunks = Vec::new();
    let mut total: u64 = 0;
    loop {
        let n = std::io::Read::read(&mut r, &mut buf).ok()?;
        if n == 0 {
            break;
        }
        total += n as u64;
        let blob = crate::store::hash_bytes(&buf[..n]);
        chunks.push(FileRecipeChunk {
            blob,
            size: n as u32,
        });
    }
    if total != size {
        return None;
    }
    let recipe = FileRecipe {
        version: 1,
        size,
        chunks,
    };
    let bytes = serde_json::to_vec(&recipe).ok()?;
    if crate::store::hash_bytes(&bytes).as_str() != recipe_id {
        return None;
    }
    Some(recipe)
}
