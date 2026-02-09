#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GateGraph {
    pub version: u32,
    pub gates: Vec<GateDef>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GateDef {
    pub id: String,
    pub name: String,
    pub upstream: Vec<String>,

    #[serde(default = "default_true")]
    pub allow_releases: bool,

    #[serde(default)]
    pub allow_superpositions: bool,

    #[serde(default)]
    pub allow_metadata_only_publications: bool,

    #[serde(default)]
    pub required_approvals: u32,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct GateGraphValidationError {
    pub(crate) error: String,
    #[serde(default)]
    pub(crate) issues: Vec<GateGraphIssueView>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct GateGraphIssueView {
    pub(crate) code: String,
    pub(crate) message: String,

    #[serde(default)]
    pub(crate) gate: Option<String>,
    #[serde(default)]
    pub(crate) upstream: Option<String>,
}

fn default_true() -> bool {
    true
}
