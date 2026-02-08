use super::*;

pub(super) async fn require_bearer(
    State(state): State<Arc<AppState>>,
    req: axum::extract::Request,
    next: Next,
) -> Response {
    let Some(value) = req.headers().get(header::AUTHORIZATION) else {
        return unauthorized();
    };

    let Ok(value) = value.to_str() else {
        return unauthorized();
    };

    let Some(token) = value.strip_prefix("Bearer ") else {
        return unauthorized();
    };

    let token_hash = hash_token(token);

    let token_id = {
        let idx = state.token_hash_index.read().await;
        idx.get(&token_hash).cloned()
    };
    let Some(token_id) = token_id else {
        return unauthorized();
    };

    let (user_id, handle, admin) = {
        let tokens = state.tokens.read().await;
        let Some(t) = tokens.get(&token_id) else {
            return unauthorized();
        };
        if t.revoked_at.is_some() {
            return unauthorized();
        }
        if let Some(exp) = &t.expires_at
            && let Ok(exp) =
                time::OffsetDateTime::parse(exp, &time::format_description::well_known::Rfc3339)
            && time::OffsetDateTime::now_utc() > exp
        {
            return unauthorized();
        }

        let users = state.users.read().await;
        let Some(u) = users.get(&t.user_id) else {
            return unauthorized();
        };
        (u.id.clone(), u.handle.clone(), u.admin)
    };

    // Best-effort last_used tracking (in-memory only).
    {
        let mut tokens = state.tokens.write().await;
        if let Some(t) = tokens.get_mut(&token_id) {
            t.last_used_at = Some(now_ts());
        }
    }

    let mut req = req;
    req.extensions_mut().insert(Subject {
        user_id,
        user: handle,
        admin,
    });
    next.run(req).await
}

pub(super) async fn healthz() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct BootstrapRequest {
    handle: String,

    #[serde(default)]
    display_name: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub(super) struct BootstrapResponse {
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
