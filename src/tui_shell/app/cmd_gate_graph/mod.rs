use super::*;

mod actions;
mod apply;
mod text_input;

impl App {
    pub(in crate::tui_shell) fn cmd_gate_graph(&mut self, args: &[String]) {
        if !args.is_empty() {
            self.push_error("usage: gates".to_string());
            return;
        }
        self.open_gate_graph_view();
    }

    pub(in crate::tui_shell) fn cmd_gate_graph_add_gate(&mut self) {
        self.gate_graph_new_gate_id = None;
        self.gate_graph_new_gate_name = None;
        self.open_text_input_modal(
            "Gate Graph",
            "new gate id> ",
            TextInputAction::GateGraphAddGateId,
            None,
            vec!["Enter a new gate id (lowercase, 0-9, -).".to_string()],
        );
    }

    pub(in crate::tui_shell) fn open_gate_graph_view(&mut self) {
        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let graph = match client.get_gate_graph() {
            Ok(g) => g,
            Err(err) => {
                self.push_error(format!("gates: {:#}", err));
                return;
            }
        };

        if self.mode() == UiMode::GateGraph {
            if let Some(frame) = self.frames.last_mut() {
                frame.view = Box::new(GateGraphView::new(graph));
            }
            self.push_output(vec!["refreshed gates".to_string()]);
        } else {
            self.push_view(GateGraphView::new(graph));
            self.push_output(vec!["opened gates".to_string()]);
        }
    }
}
