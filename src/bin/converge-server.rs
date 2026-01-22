use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::{collections::HashMap, collections::HashSet};

use anyhow::{Context, Result};
use axum::extract::{Extension, State};
use axum::http::{header, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{extract::Path, Json, Router};
use clap::Parser;
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
struct Subject {
    user: String,
}

#[derive(Clone)]
struct AppState {
    user: String,
    token: String,

    repos: Arc<RwLock<HashMap<String, Repo>>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Repo {
    id: String,
    owner: String,
    readers: HashSet<String>,
    publishers: HashSet<String>,
    lanes: HashMap<String, Lane>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct Lane {
    id: String,
    members: HashSet<String>,
}

fn can_read(repo: &Repo, user: &str) -> bool {
    repo.owner == user || repo.readers.contains(user)
}

fn can_publish(repo: &Repo, user: &str) -> bool {
    repo.owner == user || repo.publishers.contains(user)
}

#[derive(Parser)]
#[command(name = "converge-server")]
#[command(about = "Convergence central authority (development)", long_about = None)]
struct Args {
    /// Address to listen on
    #[arg(long, default_value = "127.0.0.1:8080")]
    addr: SocketAddr,

    /// Data directory (future use)
    #[arg(long, default_value = "./converge-data")]
    data_dir: PathBuf,

    /// Database URL (future use)
    #[arg(long)]
    db_url: Option<String>,

    /// Development user name
    #[arg(long, default_value = "dev")]
    dev_user: String,

    /// Development bearer token (dev-only)
    #[arg(long, default_value = "dev")]
    dev_token: String,
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let _ = args.db_url;
    std::fs::create_dir_all(&args.data_dir)
        .with_context(|| format!("create data dir {}", args.data_dir.display()))?;

    let state = Arc::new(AppState {
        user: args.dev_user,
        token: args.dev_token,
        repos: Arc::new(RwLock::new(HashMap::new())),
    });

    let authed = Router::new()
        .route("/whoami", get(whoami))
        .route("/repos", get(list_repos).post(create_repo))
        .route("/repos/:repo_id", get(get_repo))
        .route("/repos/:repo_id/permissions", get(get_repo_permissions))
        .route("/repos/:repo_id/lanes", get(list_lanes))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            require_bearer,
        ));

    let app = Router::new()
        .route("/healthz", get(healthz))
        .merge(authed)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(args.addr)
        .await
        .with_context(|| format!("bind {}", args.addr))?;

    let local_addr = listener
        .local_addr()
        .context("read listener local addr")?;
    eprintln!("converge-server listening on {}", local_addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}

async fn require_bearer(
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

    if token != state.token {
        return unauthorized();
    }

    let mut req = req;
    req.extensions_mut()
        .insert(Subject { user: state.user.clone() });
    next.run(req).await
}

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({"error": "unauthorized"})),
    )
        .into_response()
}

async fn healthz() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

async fn whoami(Extension(subject): Extension<Subject>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"user": subject.user}))
}

#[derive(Debug, serde::Deserialize)]
struct CreateRepoRequest {
    id: String,
}

async fn create_repo(
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
    let mut publishers = HashSet::new();
    publishers.insert(subject.user.clone());

    let mut members = HashSet::new();
    members.insert(subject.user.clone());
    let default_lane = Lane {
        id: "default".to_string(),
        members,
    };
    let mut lanes = HashMap::new();
    lanes.insert(default_lane.id.clone(), default_lane);

    let repo = Repo {
        id: payload.id.clone(),
        owner: subject.user.clone(),
        readers,
        publishers,
        lanes,
    };
    repos.insert(repo.id.clone(), repo.clone());

    Ok(Json(repo))
}

async fn list_repos(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
) -> Result<Json<Vec<Repo>>, Response> {
    let repos = state.repos.read().await;
    let mut out = Vec::new();
    for repo in repos.values() {
        if can_read(repo, &subject.user) {
            out.push(repo.clone());
        }
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(Json(out))
}

async fn get_repo(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
) -> Result<Json<Repo>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject.user) {
        return Err(forbidden());
    }
    Ok(Json(repo.clone()))
}

async fn get_repo_permissions(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
) -> Result<Json<serde_json::Value>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    Ok(Json(serde_json::json!({
        "read": can_read(repo, &subject.user),
        "publish": can_publish(repo, &subject.user)
    })))
}

async fn list_lanes(
    State(state): State<Arc<AppState>>,
    Extension(subject): Extension<Subject>,
    Path(repo_id): Path<String>,
) -> Result<Json<Vec<Lane>>, Response> {
    let repos = state.repos.read().await;
    let repo = repos.get(&repo_id).ok_or_else(not_found)?;
    if !can_read(repo, &subject.user) {
        return Err(forbidden());
    }

    let mut out: Vec<Lane> = repo.lanes.values().cloned().collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(Json(out))
}

fn validate_repo_id(id: &str) -> Result<()> {
    if id.is_empty() {
        return Err(anyhow::anyhow!("repo id cannot be empty"));
    }
    if !id
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(anyhow::anyhow!(
            "repo id must be lowercase alnum or '-'"
        ));
    }
    Ok(())
}

fn bad_request(err: anyhow::Error) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({"error": err.to_string()})),
    )
        .into_response()
}

fn forbidden() -> Response {
    (
        StatusCode::FORBIDDEN,
        Json(serde_json::json!({"error": "forbidden"})),
    )
        .into_response()
}

fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({"error": "not found"})),
    )
        .into_response()
}

fn conflict(msg: &str) -> Response {
    (
        StatusCode::CONFLICT,
        Json(serde_json::json!({"error": msg})),
    )
        .into_response()
}
