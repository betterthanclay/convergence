use super::*;

mod auth;
mod bootstrap;

pub(super) async fn require_bearer(
    state: State<Arc<AppState>>,
    req: axum::extract::Request,
    next: Next,
) -> Response {
    auth::require_bearer(state, req, next).await
}

pub(super) async fn bootstrap(
    state: State<Arc<AppState>>,
    headers: axum::http::HeaderMap,
    payload: Json<bootstrap::BootstrapRequest>,
) -> Result<Json<bootstrap::BootstrapResponse>, Response> {
    bootstrap::bootstrap(state, headers, payload).await
}

pub(super) async fn healthz() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}
