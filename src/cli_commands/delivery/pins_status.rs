use clap::Args;

#[derive(Args)]
pub(crate) struct PinsArgs {
    /// Emit JSON
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Args)]
pub(crate) struct PinArgs {
    /// Bundle id to pin/unpin
    #[arg(long)]
    pub(crate) bundle_id: String,
    /// Unpin instead of pin
    #[arg(long)]
    pub(crate) unpin: bool,
    /// Emit JSON
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Args)]
pub(crate) struct StatusArgs {
    /// Emit JSON
    #[arg(long)]
    pub(crate) json: bool,
    /// Limit number of publications shown
    #[arg(long, default_value_t = 10)]
    pub(crate) limit: usize,
}
