use super::*;

#[derive(Debug, serde::Deserialize)]
pub(super) struct MissingObjectsRequest {
    blobs: Vec<String>,
    manifests: Vec<String>,
    recipes: Vec<String>,
    snaps: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub(super) struct MissingObjectsResponse {
    missing_blobs: Vec<String>,
    missing_manifests: Vec<String>,
    missing_recipes: Vec<String>,
    missing_snaps: Vec<String>,
}

pub(super) async fn find_missing_objects(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Json(req): Json<MissingObjectsRequest>,
) -> Result<Json<MissingObjectsResponse>, Response> {
    {
        let repos = state.repos.read().await;
        let repo = repos.get(&repo_id).ok_or_else(not_found)?;
        if !can_publish(repo, &subject) {
            return Err(forbidden());
        }
    }

    for id in req
        .blobs
        .iter()
        .chain(req.manifests.iter())
        .chain(req.recipes.iter())
        .chain(req.snaps.iter())
    {
        validate_object_id(id).map_err(bad_request)?;
    }

    let root = repo_data_dir(&state, &repo_id).join("objects");

    let mut missing_blobs = Vec::new();
    for id in req.blobs {
        let p = root.join("blobs").join(&id);
        if !p.exists() {
            missing_blobs.push(id);
        }
    }

    let mut missing_manifests = Vec::new();
    for id in req.manifests {
        let p = root.join("manifests").join(format!("{}.json", id));
        if !p.exists() {
            missing_manifests.push(id);
        }
    }

    let mut missing_recipes = Vec::new();
    for id in req.recipes {
        let p = root.join("recipes").join(format!("{}.json", id));
        if !p.exists() {
            missing_recipes.push(id);
        }
    }

    let mut missing_snaps = Vec::new();
    for id in req.snaps {
        let p = root.join("snaps").join(format!("{}.json", id));
        if !p.exists() {
            missing_snaps.push(id);
        }
    }

    Ok(Json(MissingObjectsResponse {
        missing_blobs,
        missing_manifests,
        missing_recipes,
        missing_snaps,
    }))
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct CreatePublicationRequest {
    snap_id: String,
    scope: String,
    gate: String,

    #[serde(default)]
    metadata_only: bool,

    #[serde(default)]
    resolution: Option<PublicationResolution>,
}

pub(super) async fn create_publication(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Json(payload): Json<CreatePublicationRequest>,
) -> Result<Json<Publication>, Response> {
    validate_object_id(&payload.snap_id).map_err(bad_request)?;
    validate_scope_id(&payload.scope).map_err(bad_request)?;
    validate_gate_id(&payload.gate).map_err(bad_request)?;

    let created_at = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    let id = {
        let mut hasher = blake3::Hasher::new();
        hasher.update(repo_id.as_bytes());
        hasher.update(b"\n");
        hasher.update(payload.snap_id.as_bytes());
        hasher.update(b"\n");
        hasher.update(payload.scope.as_bytes());
        hasher.update(b"\n");
        hasher.update(payload.gate.as_bytes());
        hasher.update(b"\n");
        hasher.update(subject.user.as_bytes());
        hasher.update(b"\n");
        hasher.update(created_at.as_bytes());
        hasher.finalize().to_hex().to_string()
    };

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }
    if !repo.scopes.contains(&payload.scope) {
        return Err(bad_request(anyhow::anyhow!("unknown scope")));
    }
    if !repo.gate_graph.gates.iter().any(|g| g.id == payload.gate) {
        return Err(bad_request(anyhow::anyhow!("unknown gate")));
    }

    // Enforce at-most-once publication for a given snap+scope+gate.
    // If you need to publish again, create a new snap.
    if repo
        .publications
        .iter()
        .any(|p| p.snap_id == payload.snap_id && p.scope == payload.scope && p.gate == payload.gate)
    {
        return Err(conflict("snap already published to this scope/gate"));
    }

    let gate_def = repo
        .gate_graph
        .gates
        .iter()
        .find(|g| g.id == payload.gate)
        .ok_or_else(|| bad_request(anyhow::anyhow!("unknown gate")))?;
    if payload.metadata_only && !gate_def.allow_metadata_only_publications {
        return Err(bad_request(anyhow::anyhow!(
            "metadata-only publications not allowed in this gate"
        )));
    }
    if !repo.snaps.contains(&payload.snap_id) {
        return Err(bad_request(anyhow::anyhow!(
            "unknown snap (upload snap first)"
        )));
    }

    // For non-metadata-only publications, require full availability of referenced objects.
    // For metadata-only publications, we still require the manifest structure to be present
    // (snaps/manifests/recipes), but allow blob bytes to be pending.
    let snap = read_snap(state.as_ref(), &repo_id, &payload.snap_id)?;
    validate_manifest_tree_availability(
        state.as_ref(),
        &repo_id,
        snap.root_manifest.as_str(),
        !payload.metadata_only,
    )?;

    let pubrec = Publication {
        id,
        snap_id: payload.snap_id,
        scope: payload.scope,
        gate: payload.gate,
        publisher: subject.user,
        publisher_user_id: Some(subject.user_id),
        created_at,
        resolution: payload.resolution,
    };
    repo.publications.push(pubrec.clone());

    persist_repo(state.as_ref(), repo).map_err(internal_error)?;

    Ok(Json(pubrec))
}

pub(super) async fn list_publications(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
) -> Result<Json<Vec<Publication>>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }
    Ok(Json(repo.publications.clone()))
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct CreateBundleRequest {
    scope: String,
    gate: String,
    input_publications: Vec<String>,
}

pub(super) async fn create_bundle(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Json(payload): Json<CreateBundleRequest>,
) -> Result<Json<Bundle>, Response> {
    validate_scope_id(&payload.scope).map_err(bad_request)?;
    validate_gate_id(&payload.gate).map_err(bad_request)?;
    if payload.input_publications.is_empty() {
        return Err(bad_request(anyhow::anyhow!(
            "bundle must include at least one input publication"
        )));
    }
    for pid in &payload.input_publications {
        validate_object_id(pid).map_err(bad_request)?;
    }

    let created_at = time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;

    let mut input_publications = payload.input_publications;
    input_publications.sort();
    input_publications.dedup();

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }
    if !repo.scopes.contains(&payload.scope) {
        return Err(bad_request(anyhow::anyhow!("unknown scope")));
    }
    if !repo.gate_graph.gates.iter().any(|g| g.id == payload.gate) {
        return Err(bad_request(anyhow::anyhow!("unknown gate")));
    }

    // Resolve and validate publication ids; gather input snap roots.
    let mut input_roots: Vec<(String, String)> = Vec::new();
    for pid in &input_publications {
        let Some(p) = repo.publications.iter().find(|p| &p.id == pid) else {
            return Err(bad_request(anyhow::anyhow!("unknown publication {}", pid)));
        };
        if p.scope != payload.scope {
            return Err(bad_request(anyhow::anyhow!(
                "publication {} has mismatched scope",
                pid
            )));
        }
        if p.gate != payload.gate {
            return Err(bad_request(anyhow::anyhow!(
                "publication {} has mismatched gate",
                pid
            )));
        }

        let snap = read_snap(&state, &repo_id, &p.snap_id)?;
        input_roots.push((pid.clone(), snap.root_manifest.as_str().to_string()));
    }

    // Derive a new root manifest by coalescing input snap trees.
    let root_manifest = coalesce_root_manifest(&state, &repo_id, &input_roots)?;

    let gate_def = repo
        .gate_graph
        .gates
        .iter()
        .find(|g| g.id == payload.gate)
        .ok_or_else(|| bad_request(anyhow::anyhow!("unknown gate")))?;

    let has_superpositions = manifest_has_superpositions(&state, &repo_id, &root_manifest)?;
    let (promotable, reasons) = compute_promotability(gate_def, has_superpositions, 0);

    let id = {
        let mut hasher = blake3::Hasher::new();
        hasher.update(repo_id.as_bytes());
        hasher.update(b"\n");
        hasher.update(payload.scope.as_bytes());
        hasher.update(b"\n");
        hasher.update(payload.gate.as_bytes());
        hasher.update(b"\n");
        hasher.update(root_manifest.as_bytes());
        hasher.update(b"\n");
        for pid in &input_publications {
            hasher.update(pid.as_bytes());
            hasher.update(b"\n");
        }
        hasher.update(subject.user.as_bytes());
        hasher.update(b"\n");
        hasher.update(created_at.as_bytes());
        hasher.finalize().to_hex().to_string()
    };

    let bundle = Bundle {
        id: id.clone(),
        scope: payload.scope,
        gate: payload.gate,
        root_manifest,
        input_publications,
        created_by: subject.user,
        created_by_user_id: Some(subject.user_id),
        created_at,

        promotable,
        reasons,

        approvals: Vec::new(),
        approval_user_ids: Vec::new(),
    };

    let bytes =
        serde_json::to_vec_pretty(&bundle).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let path = repo_data_dir(&state, &repo_id)
        .join("bundles")
        .join(format!("{}.json", id));
    write_if_absent(&path, &bytes).map_err(internal_error)?;

    repo.bundles.push(bundle.clone());
    persist_repo(state.as_ref(), repo).map_err(internal_error)?;
    Ok(Json(bundle))
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct ListBundlesQuery {
    scope: Option<String>,
    gate: Option<String>,
}

pub(super) async fn list_bundles(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
    Query(q): Query<ListBundlesQuery>,
) -> Result<Json<Vec<Bundle>>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }

    let mut out = Vec::new();
    for b in &repo.bundles {
        if let Some(scope) = &q.scope
            && &b.scope != scope
        {
            continue;
        }
        if let Some(gate) = &q.gate
            && &b.gate != gate
        {
            continue;
        }
        out.push(b.clone());
    }
    Ok(Json(out))
}

pub(super) async fn get_bundle(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, bundle_id)): Path<(String, String)>,
) -> Result<Json<Bundle>, Response> {
    validate_object_id(&bundle_id).map_err(bad_request)?;

    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }

    if let Some(b) = repo.bundles.iter().find(|b| b.id == bundle_id) {
        return Ok(Json(b.clone()));
    }

    // Best-effort disk fallback.
    let path = repo_data_dir(&state, &repo_id)
        .join("bundles")
        .join(format!("{}.json", bundle_id));
    if !path.exists() {
        return Err(not_found());
    }
    let bytes = std::fs::read(&path)
        .with_context(|| format!("read {}", path.display()))
        .map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let bundle: Bundle =
        serde_json::from_slice(&bytes).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    Ok(Json(bundle))
}

pub(super) async fn approve_bundle(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, bundle_id)): Path<(String, String)>,
) -> Result<Json<Bundle>, Response> {
    validate_object_id(&bundle_id).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }

    // Load bundle.
    let mut bundle = if let Some(b) = repo.bundles.iter().find(|b| b.id == bundle_id) {
        b.clone()
    } else {
        load_bundle_from_disk(state.as_ref(), &repo_id, &bundle_id)?
    };

    if !bundle.approvals.contains(&subject.user) {
        bundle.approvals.push(subject.user.clone());
        bundle.approvals.sort();
        bundle.approvals.dedup();
    }

    if !bundle
        .approval_user_ids
        .iter()
        .any(|u| u == &subject.user_id)
    {
        bundle.approval_user_ids.push(subject.user_id.clone());
        bundle.approval_user_ids.sort();
        bundle.approval_user_ids.dedup();
    }

    let gate_def = repo
        .gate_graph
        .gates
        .iter()
        .find(|g| g.id == bundle.gate)
        .ok_or_else(|| internal_error(anyhow::anyhow!("bundle gate not found")))?;

    let has_superpositions =
        manifest_has_superpositions(state.as_ref(), &repo_id, &bundle.root_manifest)?;
    let (promotable, reasons) =
        compute_promotability(gate_def, has_superpositions, bundle.approvals.len());
    bundle.promotable = promotable;
    bundle.reasons = reasons;

    // Persist updated bundle.
    let bytes =
        serde_json::to_vec_pretty(&bundle).map_err(|e| internal_error(anyhow::anyhow!(e)))?;
    let path = repo_data_dir(state.as_ref(), &repo_id)
        .join("bundles")
        .join(format!("{}.json", bundle.id));
    write_atomic_overwrite(&path, &bytes).map_err(internal_error)?;

    // Update in-memory copy if present.
    if let Some(existing) = repo.bundles.iter_mut().find(|b| b.id == bundle.id) {
        *existing = bundle.clone();
    } else {
        repo.bundles.push(bundle.clone());
    }

    persist_repo(state.as_ref(), repo).map_err(internal_error)?;

    Ok(Json(bundle))
}

pub(super) async fn list_pins(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
) -> Result<Json<serde_json::Value>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject) {
        return Err(forbidden());
    }

    let mut bundles: Vec<String> = repo.pinned_bundles.iter().cloned().collect();
    bundles.sort();
    Ok(Json(serde_json::json!({"bundles": bundles})))
}

pub(super) async fn pin_bundle(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, bundle_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, Response> {
    validate_object_id(&bundle_id).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }

    // Ensure bundle exists (in memory or on disk).
    let _ = if repo.bundles.iter().any(|b| b.id == bundle_id) {
        None
    } else {
        Some(load_bundle_from_disk(state.as_ref(), &repo_id, &bundle_id)?)
    };

    repo.pinned_bundles.insert(bundle_id.clone());
    persist_repo(state.as_ref(), repo).map_err(internal_error)?;

    Ok(Json(
        serde_json::json!({"bundle_id": bundle_id, "pinned": true}),
    ))
}

pub(super) async fn unpin_bundle(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path((repo_id, bundle_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, Response> {
    validate_object_id(&bundle_id).map_err(bad_request)?;

    let mut repos = state.repos.write().await;
    let repo = repos.get_mut(&repo_id).ok_or_else(not_found)?;
    if !can_publish(repo, &subject) {
        return Err(forbidden());
    }

    repo.pinned_bundles.remove(&bundle_id);
    persist_repo(state.as_ref(), repo).map_err(internal_error)?;
    Ok(Json(
        serde_json::json!({"bundle_id": bundle_id, "pinned": false}),
    ))
}
