use std::collections::{HashMap, HashSet};

use crate::store::LocalStore;

pub(super) use super::rename_helpers::IdentityKey;

mod blob_edits;
mod exact;
mod recipe_edits;

use self::blob_edits::detect_blob_edit_renames;
use self::exact::detect_exact_renames;
use self::recipe_edits::detect_recipe_edit_renames;

pub(super) struct RenameDetection {
    pub(super) renames: Vec<(String, String, bool)>,
    pub(super) consumed_added: HashSet<String>,
    pub(super) consumed_deleted: HashSet<String>,
}

pub(super) fn detect_renames(
    store: &LocalStore,
    workspace_root: Option<&std::path::Path>,
    chunk_size_bytes: usize,
    added: &[String],
    deleted: &[String],
    base_ids: &HashMap<String, IdentityKey>,
    cur_ids: &HashMap<String, IdentityKey>,
) -> RenameDetection {
    let mut renames = Vec::new();
    let mut consumed_added: HashSet<String> = HashSet::new();
    let mut consumed_deleted: HashSet<String> = HashSet::new();

    detect_exact_renames(
        added,
        deleted,
        base_ids,
        cur_ids,
        &mut consumed_added,
        &mut consumed_deleted,
        &mut renames,
    );

    let mut ctx = MatchCtx {
        store,
        workspace_root,
        chunk_size_bytes,
        added,
        deleted,
        base_ids,
        cur_ids,
        consumed_added: &mut consumed_added,
        consumed_deleted: &mut consumed_deleted,
        renames: &mut renames,
    };

    detect_blob_edit_renames(&mut ctx);
    detect_recipe_edit_renames(&mut ctx);

    RenameDetection {
        renames,
        consumed_added,
        consumed_deleted,
    }
}

struct MatchCtx<'a> {
    store: &'a LocalStore,
    workspace_root: Option<&'a std::path::Path>,
    chunk_size_bytes: usize,
    added: &'a [String],
    deleted: &'a [String],
    base_ids: &'a HashMap<String, IdentityKey>,
    cur_ids: &'a HashMap<String, IdentityKey>,
    consumed_added: &'a mut HashSet<String>,
    consumed_deleted: &'a mut HashSet<String>,
    renames: &'a mut Vec<(String, String, bool)>,
}
