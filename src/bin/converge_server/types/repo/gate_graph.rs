#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Gate {
    pub(crate) id: String,
    pub(crate) name: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct GateGraph {
    pub(crate) version: u32,
    pub(crate) gates: Vec<GateDef>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct GateDef {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) upstream: Vec<String>,

    #[serde(default = "default_true")]
    pub(crate) allow_releases: bool,

    #[serde(default)]
    pub(crate) allow_superpositions: bool,

    #[serde(default)]
    pub(crate) allow_metadata_only_publications: bool,

    #[serde(default)]
    pub(crate) required_approvals: u32,
}

fn default_true() -> bool {
    true
}
