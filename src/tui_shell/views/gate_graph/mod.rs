use super::super::{RenderCtx, UiMode, View, fmt_ts_ui, render_view_chrome};

mod details;
mod render;
mod rows;

#[derive(Debug)]
pub(in crate::tui_shell) struct GateGraphView {
    pub(in crate::tui_shell) updated_at: String,
    pub(in crate::tui_shell) graph: crate::remote::GateGraph,
    pub(in crate::tui_shell) selected: usize,
}

impl GateGraphView {
    pub(in crate::tui_shell) fn new(mut graph: crate::remote::GateGraph) -> Self {
        graph.gates.sort_by(|a, b| a.id.cmp(&b.id));
        Self {
            updated_at: super::super::app::now_ts(),
            graph,
            selected: 0,
        }
    }
}
