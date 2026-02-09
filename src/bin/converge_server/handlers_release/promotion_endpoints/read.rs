use super::*;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ListPromotionsQuery {
    scope: Option<String>,
    to_gate: Option<String>,
}

pub(crate) async fn list_promotions(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Query(q): Query<ListPromotionsQuery>,
) -> Result<Json<Vec<Promotion>>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }

    let mut out = Vec::new();
    for p in &repo.promotions {
        if let Some(scope) = &q.scope
            && &p.scope != scope
        {
            continue;
        }
        if let Some(to_gate) = &q.to_gate
            && &p.to_gate != to_gate
        {
            continue;
        }
        out.push(p.clone());
    }
    Ok(Json(out))
}
