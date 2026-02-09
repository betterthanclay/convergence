use super::publication_flow::PublicationResolution;

fn is_false(v: &bool) -> bool {
    !*v
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct MissingObjectsRequest {
    pub(crate) blobs: Vec<String>,
    pub(crate) manifests: Vec<String>,
    pub(crate) recipes: Vec<String>,
    pub(crate) snaps: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct CreatePublicationRequest {
    pub(crate) snap_id: String,
    pub(crate) scope: String,
    pub(crate) gate: String,

    #[serde(default, skip_serializing_if = "is_false")]
    pub(crate) metadata_only: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) resolution: Option<PublicationResolution>,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct CreateRepoRequest {
    pub(crate) id: String,
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct UpdateLaneHeadRequest {
    pub(crate) snap_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) client_id: Option<String>,
}
