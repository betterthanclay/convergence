use super::*;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct CreateRepoRequest {
    id: String,
}

pub(crate) async fn create_repo(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Json(payload): Json<CreateRepoRequest>,
) -> Result<Json<Repo>, Response> {
    validate_repo_id(&payload.id).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    if repos.contains_key(&payload.id) {
        return Err(conflict("repo already exists"));
    }

    let mut readers = HashSet::new();
    readers.insert(subject.user.clone());
    let mut reader_user_ids = HashSet::new();
    reader_user_ids.insert(subject.user_id.clone());

    let mut publishers = HashSet::new();
    publishers.insert(subject.user.clone());
    let mut publisher_user_ids = HashSet::new();
    publisher_user_ids.insert(subject.user_id.clone());

    let mut members = HashSet::new();
    members.insert(subject.user.clone());
    let mut member_user_ids = HashSet::new();
    member_user_ids.insert(subject.user_id.clone());
    let default_lane = Lane {
        id: "default".to_string(),
        members,
        member_user_ids,
        heads: HashMap::new(),
        head_history: HashMap::new(),
    };
    let mut lanes = HashMap::new();
    lanes.insert(default_lane.id.clone(), default_lane);

    let gate_graph = GateGraph {
        version: 1,
        gates: vec![GateDef {
            id: "dev-intake".to_string(),
            name: "Dev Intake".to_string(),
            upstream: vec![],
            allow_releases: true,
            allow_superpositions: false,
            allow_metadata_only_publications: false,
            required_approvals: 0,
        }],
    };

    let mut scopes = HashSet::new();
    scopes.insert("main".to_string());

    let snaps = HashSet::new();
    let publications = Vec::new();
    let bundles = Vec::new();
    let pinned_bundles = HashSet::new();
    let promotions = Vec::new();
    let promotion_state = HashMap::new();
    let releases = Vec::new();

    let repo = Repo {
        id: payload.id.clone(),
        owner: subject.user.clone(),
        owner_user_id: Some(subject.user_id.clone()),
        readers,
        reader_user_ids,
        publishers,
        publisher_user_ids,
        lanes,
        gate_graph,
        scopes,
        snaps,
        publications,
        bundles,
        pinned_bundles,
        promotions,
        promotion_state,
        releases,
    };
    repos.insert(repo.id.clone(), repo.clone());

    std::fs::create_dir_all(repo_data_dir(&state, &repo.id))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    std::fs::create_dir_all(repo_data_dir(&state, &repo.id).join("bundles"))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    std::fs::create_dir_all(repo_data_dir(&state, &repo.id).join("promotions"))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    std::fs::create_dir_all(repo_data_dir(&state, &repo.id).join("releases"))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    persist_repo(state.as_ref(), &repo).map_err(internal_error)?;

    Ok(Json(repo))
}
