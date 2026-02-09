#[derive(Debug, serde::Deserialize)]
pub(crate) struct CreateBundleRequest {
    pub(crate) scope: String,
    pub(crate) gate: String,
    pub(crate) input_publications: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ListBundlesQuery {
    pub(crate) scope: Option<String>,
    pub(crate) gate: Option<String>,
}
