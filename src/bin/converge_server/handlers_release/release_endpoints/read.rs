use super::*;

pub(crate) async fn list_releases(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
) -> Result<Json<Vec<Release>>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }
    let mut out = repo.releases.clone();
    out.sort_by(|a, b| b.released_at.cmp(&a.released_at));
    Ok(Json(out))
}

pub(crate) async fn get_release_channel(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, channel)): Path<(String, String)>,
) -> Result<Json<Release>, Response> {
    validate_release_channel(&channel).map_err(bad_request)?;

    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }

    let mut best: Option<Release> = None;
    for r in &repo.releases {
        if r.channel != channel {
            continue;
        }
        match best.as_ref() {
            None => best = Some(r.clone()),
            Some(prev) => {
                if r.released_at > prev.released_at {
                    best = Some(r.clone());
                }
            }
        }
    }
    let Some(best) = best else {
        return Err(not_found());
    };
    Ok(Json(best))
}
