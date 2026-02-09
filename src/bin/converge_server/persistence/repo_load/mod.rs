use super::super::*;

mod collection_loaders;
mod hydrate;
mod promotion_state;

pub(crate) fn load_repos_from_disk(
    state: &AppState,
    handle_to_id: &HashMap<String, String>,
) -> Result<HashMap<String, Repo>> {
    hydrate::load_repos_from_disk(state, handle_to_id)
}
