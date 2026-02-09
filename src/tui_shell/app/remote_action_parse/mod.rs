#[derive(Debug, Default)]
pub(super) struct BundleArgs {
    pub(super) scope: Option<String>,
    pub(super) gate: Option<String>,
    pub(super) publications: Vec<String>,
}

#[derive(Debug, Default)]
pub(super) struct PinArgs {
    pub(super) bundle_id: Option<String>,
    pub(super) unpin: bool,
}

#[derive(Debug, Default)]
pub(super) struct ApproveArgs {
    pub(super) bundle_id: Option<String>,
}

#[derive(Debug, Default)]
pub(super) struct PromoteArgs {
    pub(super) bundle_id: Option<String>,
    pub(super) to_gate: Option<String>,
}

#[derive(Debug, Default)]
pub(super) struct ReleaseArgs {
    pub(super) channel: Option<String>,
    pub(super) bundle_id: Option<String>,
    pub(super) notes: Option<String>,
}

#[derive(Debug, Default)]
pub(super) struct SuperpositionsArgs {
    pub(super) bundle_id: Option<String>,
    pub(super) filter: Option<String>,
}

mod approve;
mod bundle;
mod pin;
mod promote;
mod release;
mod superpositions;

pub(super) use self::approve::parse_approve_args;
pub(super) use self::bundle::parse_bundle_args;
pub(super) use self::pin::parse_pin_args;
pub(super) use self::promote::parse_promote_args;
pub(super) use self::release::parse_release_args;
pub(super) use self::superpositions::parse_superpositions_args;
