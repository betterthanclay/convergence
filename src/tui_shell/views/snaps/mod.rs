use super::super::RenderCtx;
use super::super::status::ChangeSummary;

mod details;
mod render;
mod rows;
mod state;

#[derive(Debug)]
pub(in crate::tui_shell) struct SnapsView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) filter: Option<String>,
    pub(in crate::tui_shell) all_items: Vec<crate::model::SnapRecord>,
    pub(in crate::tui_shell) items: Vec<crate::model::SnapRecord>,
    pub(in crate::tui_shell) selected_row: usize,

    pub(in crate::tui_shell) head_id: Option<String>,

    pub(in crate::tui_shell) pending_changes: Option<ChangeSummary>,
}
