#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Publication {
    pub(crate) id: String,
    pub(crate) snap_id: String,
    pub(crate) scope: String,
    pub(crate) gate: String,
    pub(crate) publisher: String,

    #[serde(default)]
    pub(crate) publisher_user_id: Option<String>,
    pub(crate) created_at: String,

    #[serde(default)]
    pub(crate) resolution: Option<PublicationResolution>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct PublicationResolution {
    pub(crate) bundle_id: String,
    pub(crate) root_manifest: String,
    pub(crate) resolved_root_manifest: String,
    pub(crate) created_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Bundle {
    pub(crate) id: String,
    pub(crate) scope: String,
    pub(crate) gate: String,
    pub(crate) root_manifest: String,
    pub(crate) input_publications: Vec<String>,
    pub(crate) created_by: String,

    #[serde(default)]
    pub(crate) created_by_user_id: Option<String>,
    pub(crate) created_at: String,

    pub(crate) promotable: bool,
    pub(crate) reasons: Vec<String>,

    #[serde(default)]
    pub(crate) approvals: Vec<String>,

    #[serde(default)]
    pub(crate) approval_user_ids: Vec<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Promotion {
    pub(crate) id: String,
    pub(crate) bundle_id: String,
    pub(crate) scope: String,
    pub(crate) from_gate: String,
    pub(crate) to_gate: String,
    pub(crate) promoted_by: String,

    #[serde(default)]
    pub(crate) promoted_by_user_id: Option<String>,
    pub(crate) promoted_at: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Release {
    pub(crate) id: String,
    pub(crate) channel: String,
    pub(crate) bundle_id: String,
    pub(crate) scope: String,
    pub(crate) gate: String,

    pub(crate) released_by: String,

    #[serde(default)]
    pub(crate) released_by_user_id: Option<String>,

    pub(crate) released_at: String,

    #[serde(default)]
    pub(crate) notes: Option<String>,
}
