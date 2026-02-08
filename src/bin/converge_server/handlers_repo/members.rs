use super::super::*;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MemberHandleRequest {
    pub(crate) handle: String,

    #[serde(default)]
    pub(crate) role: Option<String>,
}

pub(crate) async fn list_repo_members(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
) -> Result<Json<serde_json::Value>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !subject.admin && repo.owner_user_id.as_ref() != Some(&subject.user_id) {
        return Err(forbidden());
    }
    Ok(Json(serde_json::json!({
        "owner": repo.owner,
        "readers": repo.readers,
        "publishers": repo.publishers,
        "owner_user_id": repo.owner_user_id,
        "reader_user_ids": repo.reader_user_ids,
        "publisher_user_ids": repo.publisher_user_ids,
    })))
}

pub(crate) async fn add_repo_member(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Json(payload): Json<MemberHandleRequest>,
) -> Result<Json<serde_json::Value>, Response> {
    validate_user_handle(&payload.handle).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !subject.admin && repo.owner_user_id.as_ref() != Some(&subject.user_id) {
        return Err(forbidden());
    }

    let users = state.users.read().await;
    let (user_id, handle) = users
        .values()
        .find(|u| u.handle == payload.handle)
        .map(|u| (u.id.clone(), u.handle.clone()))
        .ok_or_else(|| bad_request(anyhow::anyhow!("unknown user handle")))?;
    drop(users);

    let role = payload.role.unwrap_or_else(|| "read".to_string());
    match role.as_str() {
        "read" => {
            repo.readers.insert(handle);
            repo.reader_user_ids.insert(user_id);
        }
        "publish" => {
            repo.readers.insert(handle.clone());
            repo.reader_user_ids.insert(user_id.clone());
            repo.publishers.insert(handle);
            repo.publisher_user_ids.insert(user_id);
        }
        _ => return Err(bad_request(anyhow::anyhow!("unknown role"))),
    }

    persist_repo(state.as_ref(), repo).map_err(internal_error)?;
    Ok(Json(serde_json::json!({"ok": true})))
}

pub(crate) async fn remove_repo_member(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, handle)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, Response> {
    validate_user_handle(&handle).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !subject.admin && repo.owner_user_id.as_ref() != Some(&subject.user_id) {
        return Err(forbidden());
    }

    let users = state.users.read().await;
    let uid = users
        .values()
        .find(|u| u.handle == handle)
        .map(|u| u.id.clone());
    drop(users);

    repo.readers.remove(&handle);
    repo.publishers.remove(&handle);
    if let Some(uid) = uid {
        repo.reader_user_ids.remove(&uid);
        repo.publisher_user_ids.remove(&uid);
    }

    persist_repo(state.as_ref(), repo).map_err(internal_error)?;
    Ok(Json(serde_json::json!({"ok": true})))
}
