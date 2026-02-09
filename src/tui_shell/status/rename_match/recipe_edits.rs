use std::collections::HashSet;

use super::super::rename_helpers::{
    IdentityKey, min_recipe_rename_matched_chunks, min_recipe_rename_score,
    recipe_prefix_suffix_score,
};
use super::super::rename_io::load_recipe;
use super::MatchCtx;

pub(super) fn detect_recipe_edit_renames(ctx: &mut MatchCtx<'_>) {
    const MAX_CHUNKS: usize = 2048;

    let mut remaining_added_recipes = Vec::new();
    for path in ctx.added {
        if ctx.consumed_added.contains(path) {
            continue;
        }
        let Some(id) = ctx.cur_ids.get(path) else {
            continue;
        };
        let IdentityKey::Recipe(recipe) = id else {
            continue;
        };
        remaining_added_recipes.push((path.clone(), recipe.clone()));
    }

    let mut remaining_deleted_recipes = Vec::new();
    for path in ctx.deleted {
        if ctx.consumed_deleted.contains(path) {
            continue;
        }
        let Some(id) = ctx.base_ids.get(path) else {
            continue;
        };
        let IdentityKey::Recipe(recipe) = id else {
            continue;
        };
        remaining_deleted_recipes.push((path.clone(), recipe.clone()));
    }

    let mut used_added_recipe_paths: HashSet<String> = HashSet::new();
    for (from_path, from_recipe) in remaining_deleted_recipes {
        let Some(from_recipe_obj) =
            load_recipe(ctx.store, None, "", &from_recipe, ctx.chunk_size_bytes)
        else {
            continue;
        };
        if from_recipe_obj.chunks.len() > MAX_CHUNKS {
            continue;
        }

        let mut best: Option<(String, String, f64)> = None;
        for (to_path, to_recipe) in &remaining_added_recipes {
            if used_added_recipe_paths.contains(to_path) {
                continue;
            }
            let Some(to_recipe_obj) = load_recipe(
                ctx.store,
                ctx.workspace_root,
                to_path,
                to_recipe,
                ctx.chunk_size_bytes,
            ) else {
                continue;
            };
            if to_recipe_obj.chunks.len() > MAX_CHUNKS {
                continue;
            }

            let diff = from_recipe_obj
                .chunks
                .len()
                .abs_diff(to_recipe_obj.chunks.len());
            let max = from_recipe_obj.chunks.len().max(to_recipe_obj.chunks.len());
            if diff > 4 && (diff as f64) / (max as f64) > 0.20 {
                continue;
            }

            let (prefix, suffix, max_chunks, score) =
                recipe_prefix_suffix_score(&from_recipe_obj, &to_recipe_obj);
            let min_score = min_recipe_rename_score(max_chunks);
            let min_matched = min_recipe_rename_matched_chunks(max_chunks);
            if score >= min_score && (prefix + suffix) >= min_matched {
                match &best {
                    None => best = Some((to_path.clone(), to_recipe.clone(), score)),
                    Some((_, _, best_score)) if score > *best_score => {
                        best = Some((to_path.clone(), to_recipe.clone(), score))
                    }
                    _ => {}
                }
            }
        }

        if let Some((to_path, _to_recipe, _score)) = best {
            used_added_recipe_paths.insert(to_path.clone());
            ctx.consumed_deleted.insert(from_path.clone());
            ctx.consumed_added.insert(to_path.clone());
            ctx.renames.push((from_path, to_path, true));
        }
    }
}
