use clap::Args;

#[derive(Args)]
pub(crate) struct PublishArgs {
    /// Snap id to publish (defaults to latest)
    #[arg(long)]
    pub(crate) snap_id: Option<String>,
    /// Override scope (defaults to remote config)
    #[arg(long)]
    pub(crate) scope: Option<String>,
    /// Override gate (defaults to remote config)
    #[arg(long)]
    pub(crate) gate: Option<String>,
    /// Create a metadata-only publication (skip uploading blobs)
    #[arg(long)]
    pub(crate) metadata_only: bool,
    /// Emit JSON
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Args)]
pub(crate) struct SyncArgs {
    /// Snap id to sync (defaults to latest)
    #[arg(long)]
    pub(crate) snap_id: Option<String>,
    /// Lane id (defaults to "default")
    #[arg(long, default_value = "default")]
    pub(crate) lane: String,
    /// Optional client identifier
    #[arg(long)]
    pub(crate) client_id: Option<String>,
    /// Emit JSON
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Args)]
pub(crate) struct LanesArgs {
    /// Emit JSON
    #[arg(long)]
    pub(crate) json: bool,
}
