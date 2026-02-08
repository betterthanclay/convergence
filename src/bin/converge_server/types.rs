use super::*;

#[derive(Clone, Debug)]
pub(super) struct Subject {
    pub(super) user_id: String,
    pub(super) user: String,

    #[allow(dead_code)]
    pub(super) admin: bool,
}

#[derive(Clone)]
pub(super) struct AppState {
    // Used only for best-effort defaults when hydrating old on-disk repos.
    pub(super) default_user: String,

    pub(super) data_dir: PathBuf,

    pub(super) repos: Arc<RwLock<HashMap<String, Repo>>>,

    pub(super) users: Arc<RwLock<HashMap<String, User>>>,
    pub(super) tokens: Arc<RwLock<HashMap<String, AccessToken>>>,
    pub(super) token_hash_index: Arc<RwLock<HashMap<String, String>>>,

    // Optional one-time bootstrap token (hash) used to create the first admin.
    // Enabled only when the server is started with `--bootstrap-token`.
    pub(super) bootstrap_token_hash: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct User {
    pub(super) id: String,
    pub(super) handle: String,

    #[serde(default)]
    pub(super) display_name: Option<String>,

    #[serde(default)]
    pub(super) admin: bool,

    pub(super) created_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct AccessToken {
    pub(super) id: String,
    pub(super) user_id: String,

    // Stored hash of the bearer token secret.
    pub(super) token_hash: String,

    #[serde(default)]
    pub(super) label: Option<String>,

    pub(super) created_at: String,

    #[serde(default)]
    pub(super) last_used_at: Option<String>,

    #[serde(default)]
    pub(super) revoked_at: Option<String>,

    #[serde(default)]
    pub(super) expires_at: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct Repo {
    pub(super) id: String,
    pub(super) owner: String,

    #[serde(default)]
    pub(super) owner_user_id: Option<String>,

    pub(super) readers: HashSet<String>,

    #[serde(default)]
    pub(super) reader_user_ids: HashSet<String>,

    pub(super) publishers: HashSet<String>,

    #[serde(default)]
    pub(super) publisher_user_ids: HashSet<String>,

    pub(super) lanes: HashMap<String, Lane>,

    pub(super) gate_graph: GateGraph,
    pub(super) scopes: HashSet<String>,

    pub(super) snaps: HashSet<String>,
    pub(super) publications: Vec<Publication>,

    pub(super) bundles: Vec<Bundle>,

    #[serde(default)]
    pub(super) pinned_bundles: HashSet<String>,

    pub(super) promotions: Vec<Promotion>,
    pub(super) promotion_state: HashMap<String, HashMap<String, String>>,

    #[serde(default)]
    pub(super) releases: Vec<Release>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct Gate {
    pub(super) id: String,
    pub(super) name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct GateGraph {
    pub(super) version: u32,
    pub(super) gates: Vec<GateDef>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct GateDef {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) upstream: Vec<String>,

    #[serde(default = "default_true")]
    pub(super) allow_releases: bool,

    #[serde(default)]
    pub(super) allow_superpositions: bool,

    #[serde(default)]
    pub(super) allow_metadata_only_publications: bool,

    #[serde(default)]
    pub(super) required_approvals: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct Publication {
    pub(super) id: String,
    pub(super) snap_id: String,
    pub(super) scope: String,
    pub(super) gate: String,
    pub(super) publisher: String,

    #[serde(default)]
    pub(super) publisher_user_id: Option<String>,
    pub(super) created_at: String,

    #[serde(default)]
    pub(super) resolution: Option<PublicationResolution>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct PublicationResolution {
    pub(super) bundle_id: String,
    pub(super) root_manifest: String,
    pub(super) resolved_root_manifest: String,
    pub(super) created_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct Bundle {
    pub(super) id: String,
    pub(super) scope: String,
    pub(super) gate: String,
    pub(super) root_manifest: String,
    pub(super) input_publications: Vec<String>,
    pub(super) created_by: String,

    #[serde(default)]
    pub(super) created_by_user_id: Option<String>,
    pub(super) created_at: String,

    pub(super) promotable: bool,
    pub(super) reasons: Vec<String>,

    #[serde(default)]
    pub(super) approvals: Vec<String>,

    #[serde(default)]
    pub(super) approval_user_ids: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct Promotion {
    pub(super) id: String,
    pub(super) bundle_id: String,
    pub(super) scope: String,
    pub(super) from_gate: String,
    pub(super) to_gate: String,
    pub(super) promoted_by: String,

    #[serde(default)]
    pub(super) promoted_by_user_id: Option<String>,
    pub(super) promoted_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct Release {
    pub(super) id: String,
    pub(super) channel: String,
    pub(super) bundle_id: String,
    pub(super) scope: String,
    pub(super) gate: String,

    pub(super) released_by: String,

    #[serde(default)]
    pub(super) released_by_user_id: Option<String>,

    pub(super) released_at: String,

    #[serde(default)]
    pub(super) notes: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct Lane {
    pub(super) id: String,
    pub(super) members: HashSet<String>,

    #[serde(default)]
    pub(super) member_user_ids: HashSet<String>,

    #[serde(default)]
    pub(super) heads: HashMap<String, LaneHead>,

    // Retention roots for unpublished collaboration. We keep a bounded history of head
    // updates so the server can GC aggressively without losing recent WIP context.
    #[serde(default)]
    pub(super) head_history: HashMap<String, Vec<LaneHead>>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(super) struct LaneHead {
    pub(super) snap_id: String,
    pub(super) updated_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) client_id: Option<String>,
}

pub(super) const LANE_HEAD_HISTORY_KEEP_LAST: usize = 5;

fn default_true() -> bool {
    true
}
