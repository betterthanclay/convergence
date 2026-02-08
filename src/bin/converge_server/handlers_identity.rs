use super::*;

pub(super) async fn whoami(Extension(subject): Extension<Subject>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "user": subject.user,
        "user_id": subject.user_id,
        "admin": subject.admin,
    }))
}

pub(super) async fn list_users(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<Json<Vec<User>>, Response> {
    if !subject.admin {
        return Err(forbidden());
    }
    let users = state.users.read().await;
    let mut out: Vec<User> = users.values().cloned().collect();
    out.sort_by(|a, b| a.handle.cmp(&b.handle));
    Ok(Json(out))
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct CreateUserRequest {
    handle: String,

    #[serde(default)]
    display_name: Option<String>,

    #[serde(default)]
    admin: bool,
}

pub(super) async fn create_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, Response> {
    if !subject.admin {
        return Err(forbidden());
    }
    validate_user_handle(&payload.handle).map_err(bad_request)?;

    let created_at = now_ts();
    let user_id = generate_token_secret().map_err(internal_error)?;
    let user = User {
        id: user_id.clone(),
        handle: payload.handle.clone(),
        display_name: payload.display_name,
        admin: payload.admin,
        created_at,
    };

    {
        let mut users = state.users.write().await;
        if users.values().any(|u| u.handle == payload.handle) {
            return Err(conflict("user handle already exists"));
        }
        users.insert(user_id, user.clone());
    }

    {
        let users = state.users.read().await;
        let tokens = state.tokens.read().await;
        if let Err(err) = persist_identity_to_disk(&state.data_dir, &users, &tokens) {
            return Err(internal_error(err));
        }
    }

    Ok(Json(user))
}

#[derive(Debug, serde::Serialize)]
pub(super) struct TokenView {
    id: String,
    label: Option<String>,
    created_at: String,
    last_used_at: Option<String>,
    revoked_at: Option<String>,
    expires_at: Option<String>,
}

pub(super) async fn list_tokens(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<Json<Vec<TokenView>>, Response> {
    let tokens = state.tokens.read().await;
    let mut out = Vec::new();
    for t in tokens.values() {
        if t.user_id != subject.user_id {
            continue;
        }
        out.push(TokenView {
            id: t.id.clone(),
            label: t.label.clone(),
            created_at: t.created_at.clone(),
            last_used_at: t.last_used_at.clone(),
            revoked_at: t.revoked_at.clone(),
            expires_at: t.expires_at.clone(),
        });
    }
    out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(Json(out))
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct CreateTokenRequest {
    #[serde(default)]
    label: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub(super) struct CreateTokenResponse {
    pub(super) id: String,
    pub(super) token: String,
    pub(super) created_at: String,
}

pub(super) async fn mint_token(
    state: &Arc<AppState>,
    user_id: &str,
    label: Option<String>,
) -> Result<CreateTokenResponse, Response> {
    let created_at = now_ts();

    let token = generate_token_secret().map_err(internal_error)?;
    let token_hash = hash_token(&token);
    let token_id = {
        let mut h = blake3::Hasher::new();
        h.update(user_id.as_bytes());
        h.update(b"\n");
        h.update(token_hash.as_bytes());
        h.update(b"\n");
        h.update(created_at.as_bytes());
        h.finalize().to_hex().to_string()
    };

    {
        let mut tokens = state.tokens.write().await;
        tokens.insert(
            token_id.clone(),
            AccessToken {
                id: token_id.clone(),
                user_id: user_id.to_string(),
                token_hash: token_hash.clone(),
                label,
                created_at: created_at.clone(),
                last_used_at: None,
                revoked_at: None,
                expires_at: None,
            },
        );
    }
    {
        let mut idx = state.token_hash_index.write().await;
        idx.insert(token_hash, token_id.clone());
    }

    {
        let users = state.users.read().await;
        let tokens = state.tokens.read().await;
        if let Err(err) = persist_identity_to_disk(&state.data_dir, &users, &tokens) {
            return Err(internal_error(err));
        }
    }

    Ok(CreateTokenResponse {
        id: token_id,
        token,
        created_at,
    })
}

pub(super) async fn create_token(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, Response> {
    let out = mint_token(&state, &subject.user_id, payload.label).await?;
    Ok(Json(out))
}

pub(super) async fn create_token_for_user(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(user_id): Path<String>,
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<CreateTokenResponse>, Response> {
    if !subject.admin && subject.user_id != user_id {
        return Err(forbidden());
    }
    {
        let users = state.users.read().await;
        if !users.contains_key(&user_id) {
            return Err(not_found());
        }
    }
    let out = mint_token(&state, &user_id, payload.label).await?;
    Ok(Json(out))
}

pub(super) async fn revoke_token(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(token_id): Path<String>,
) -> Result<Json<serde_json::Value>, Response> {
    let revoked_at = now_ts();

    {
        let mut tokens = state.tokens.write().await;
        let Some(t) = tokens.get_mut(&token_id) else {
            return Err(not_found());
        };
        if t.user_id != subject.user_id && !subject.admin {
            return Err(forbidden());
        }
        t.revoked_at = Some(revoked_at.clone());
    }

    {
        let users = state.users.read().await;
        let tokens = state.tokens.read().await;
        if let Err(err) = persist_identity_to_disk(&state.data_dir, &users, &tokens) {
            return Err(internal_error(err));
        }
    }

    Ok(Json(
        serde_json::json!({"revoked": true, "token_id": token_id, "revoked_at": revoked_at}),
    ))
}
