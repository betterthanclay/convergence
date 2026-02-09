#[derive(Debug, serde::Deserialize)]
pub(crate) struct GcQuery {
    #[serde(default = "default_true")]
    pub(crate) dry_run: bool,
    #[serde(default = "default_true")]
    pub(crate) prune_metadata: bool,

    /// If set, prune release history by keeping only the latest N releases per channel.
    ///
    /// This affects GC roots: pruned releases stop retaining their referenced bundles/objects.
    #[serde(default)]
    pub(crate) prune_releases_keep_last: Option<usize>,
}

fn default_true() -> bool {
    true
}
