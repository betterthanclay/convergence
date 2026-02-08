use super::super::*;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct UpdateLaneHeadRequest {
    snap_id: String,

    #[serde(default)]
    client_id: Option<String>,
}

pub(crate) async fn update_lane_head_me(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, lane_id)): Path<(String, String)>,
    Json(payload): Json<UpdateLaneHeadRequest>,
) -> Result<Json<LaneHead>, Response> {
    validate_lane_id(&lane_id).map_err(bad_request)?;
    validate_object_id(&payload.snap_id).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }

    let lane = repo.lanes.get_mut(&lane_id).ok_or_else(not_found)?;
    if !lane.members.contains(&subject.user) && !lane.member_user_ids.contains(&subject.user_id) {
        return Err(forbidden());
    }

    if !repo.snaps.contains(&payload.snap_id) {
        return Err(bad_request(anyhow::anyhow!(
            "unknown snap (upload snap first)"
        )));
    }

    let updated_at = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    let head = LaneHead {
        snap_id: payload.snap_id,
        updated_at,
        client_id: payload.client_id,
    };
    lane.heads.insert(subject.user.clone(), head.clone());

    let hist = lane.head_history.entry(subject.user.clone()).or_default();
    // Keep newest first.
    hist.insert(0, head.clone());
    if hist.len() > LANE_HEAD_HISTORY_KEEP_LAST {
        hist.truncate(LANE_HEAD_HISTORY_KEEP_LAST);
    }
    persist_repo(state.as_ref(), repo).map_err(internal_error)?;
    Ok(Json(head))
}

pub(crate) async fn get_lane_head(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, lane_id, user)): Path<(String, String, String)>,
) -> Result<Json<LaneHead>, Response> {
    validate_lane_id(&lane_id).map_err(bad_request)?;

    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }
    let lane = repo.lanes.get(&lane_id).ok_or_else(not_found)?;
    if !lane.members.contains(&subject.user) && !lane.member_user_ids.contains(&subject.user_id) {
        return Err(forbidden());
    }

    let head = lane.heads.get(&user).ok_or_else(not_found)?;
    Ok(Json(head.clone()))
}
