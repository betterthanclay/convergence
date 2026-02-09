use super::*;

impl App {
    pub(in crate::tui_shell) fn apply_gate_graph_edit(
        &mut self,
        keep_selected: Option<String>,
        f: impl FnOnce(&mut crate::remote::GateGraph) -> anyhow::Result<()>,
    ) {
        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let Some(v) = self.current_view::<GateGraphView>() else {
            self.push_error("not in gates mode".to_string());
            return;
        };
        let mut graph = v.graph.clone();

        if let Err(err) = f(&mut graph) {
            self.push_error(err.to_string());
            return;
        }

        let updated = match client.put_gate_graph(&graph) {
            Ok(g) => g,
            Err(err) => {
                self.push_error(format!("gates: {:#}", err));
                return;
            }
        };

        if let Some(v) = self.current_view_mut::<GateGraphView>() {
            let mut updated = updated;
            updated.gates.sort_by(|a, b| a.id.cmp(&b.id));
            v.graph = updated;
            v.updated_at = now_ts();
            if let Some(id) = keep_selected
                && let Some(i) = v.graph.gates.iter().position(|g| g.id == id)
            {
                v.selected = i;
            }
        }
        self.refresh_root_view();
    }
}
