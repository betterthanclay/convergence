pub(super) mod bundles;
pub(super) mod inbox;
pub(super) mod lanes;
pub(super) mod releases;
pub(super) mod snaps;

pub(in crate::tui_shell) use bundles::BundlesView;
pub(in crate::tui_shell) use inbox::InboxView;
pub(in crate::tui_shell) use lanes::{LaneHeadItem, LanesView};
pub(in crate::tui_shell) use releases::ReleasesView;
pub(in crate::tui_shell) use snaps::SnapsView;
