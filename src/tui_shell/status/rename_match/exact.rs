use std::collections::{HashMap, HashSet};

use super::IdentityKey;

pub(super) fn detect_exact_renames(
    added: &[String],
    deleted: &[String],
    base_ids: &HashMap<String, IdentityKey>,
    cur_ids: &HashMap<String, IdentityKey>,
    consumed_added: &mut HashSet<String>,
    consumed_deleted: &mut HashSet<String>,
    renames: &mut Vec<(String, String, bool)>,
) {
    let mut added_by_id: HashMap<IdentityKey, Vec<String>> = HashMap::new();
    for path in added {
        if let Some(id) = cur_ids.get(path) {
            added_by_id
                .entry(id.clone())
                .or_default()
                .push(path.clone());
        }
    }

    let mut deleted_by_id: HashMap<IdentityKey, Vec<String>> = HashMap::new();
    for path in deleted {
        if let Some(id) = base_ids.get(path) {
            deleted_by_id
                .entry(id.clone())
                .or_default()
                .push(path.clone());
        }
    }

    for (id, deleted_paths) in &deleted_by_id {
        let Some(added_paths) = added_by_id.get(id) else {
            continue;
        };
        if deleted_paths.len() == 1 && added_paths.len() == 1 {
            let from = deleted_paths[0].clone();
            let to = added_paths[0].clone();
            consumed_deleted.insert(from.clone());
            consumed_added.insert(to.clone());
            renames.push((from, to, false));
        }
    }
}
