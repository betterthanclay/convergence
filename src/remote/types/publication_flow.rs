#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MissingObjectsResponse {
    pub missing_blobs: Vec<String>,
    pub missing_manifests: Vec<String>,
    pub missing_recipes: Vec<String>,
    pub missing_snaps: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Pins {
    pub bundles: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Publication {
    pub id: String,
    pub snap_id: String,
    pub scope: String,
    pub gate: String,
    pub publisher: String,
    pub created_at: String,

    #[serde(default)]
    pub resolution: Option<PublicationResolution>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PublicationResolution {
    pub bundle_id: String,
    pub root_manifest: String,
    pub resolved_root_manifest: String,
    pub created_at: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Bundle {
    pub id: String,
    pub scope: String,
    pub gate: String,
    pub root_manifest: String,
    pub input_publications: Vec<String>,
    pub created_by: String,
    pub created_at: String,
    pub promotable: bool,
    pub reasons: Vec<String>,

    #[serde(default)]
    pub approvals: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Promotion {
    pub id: String,
    pub bundle_id: String,
    pub scope: String,
    pub from_gate: String,
    pub to_gate: String,
    pub promoted_by: String,
    pub promoted_at: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Release {
    pub id: String,
    pub channel: String,
    pub bundle_id: String,
    pub scope: String,
    pub gate: String,
    pub released_by: String,

    #[serde(default)]
    pub released_by_user_id: Option<String>,

    pub released_at: String,

    #[serde(default)]
    pub notes: Option<String>,
}
