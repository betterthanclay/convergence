use super::super::*;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct BootstrapRequest {
    handle: String,

    #[serde(default)]
    display_name: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct BootstrapResponse {
    user: User,
    token: CreateTokenResponse,
}

pub(super) async fn bootstrap(
    State(state): State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<BootstrapRequest>,
) -> Result<Json<BootstrapResponse>, Response> {
    let Some(expected_hash) = state.bootstrap_token_hash.as_deref() else {
        return Err(not_found());
    };

    let Some(value) = headers.get(header::AUTHORIZATION) else {
        return Err(unauthorized());
    };
    let Ok(value) = value.to_str() else {
        return Err(unauthorized());
    };
    let Some(token) = value.strip_prefix("Bearer ") else {
        return Err(unauthorized());
    };
    if hash_token(token) != expected_hash {
        return Err(unauthorized());
    }

    validate_user_handle(&payload.handle).map_err(bad_request)?;
    let created_at = now_ts();
    let user_id = generate_token_secret().map_err(internal_error)?;

    let user = User {
        id: user_id.clone(),
        handle: payload.handle.clone(),
        display_name: payload.display_name,
        admin: true,
        created_at: created_at.clone(),
    };

    // Enforce one-time semantics per data_dir: only allow bootstrapping if no admin exists.
    {
        let users = state.users.read().await;
        if users.values().any(|u| u.admin) {
            return Err(conflict("already bootstrapped"));
        }
    }

    {
        let mut users = state.users.write().await;
        if users.values().any(|u| u.handle == payload.handle) {
            return Err(conflict("user handle already exists"));
        }
        // Re-check under write lock.
        if users.values().any(|u| u.admin) {
            return Err(conflict("already bootstrapped"));
        }
        users.insert(user_id.clone(), user.clone());
    }

    let token_secret = generate_token_secret().map_err(internal_error)?;
    let token_hash = hash_token(&token_secret);
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
                user_id: user_id.clone(),
                token_hash: token_hash.clone(),
                label: Some("bootstrap".to_string()),
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

    Ok(Json(BootstrapResponse {
        user,
        token: CreateTokenResponse {
            id: token_id,
            token: token_secret,
            created_at,
        },
    }))
}
