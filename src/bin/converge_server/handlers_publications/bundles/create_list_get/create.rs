use super::super::super::super::*;

use super::types::CreateBundleRequest;

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
