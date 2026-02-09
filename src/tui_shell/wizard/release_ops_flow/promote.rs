use super::*;

impl App {
    pub(in crate::tui_shell) fn start_promote_wizard(
        &mut self,
        bundle_id: String,
        candidates: Vec<String>,
        initial: Option<String>,
    ) {
        let initial = initial.or_else(|| candidates.first().cloned());
        let preview = candidates
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");

        self.promote_wizard = Some(PromoteWizard {
            bundle_id,
            candidates,
        });

        self.open_text_input_modal(
            "Promote",
            "to gate> ",
            TextInputAction::PromoteToGate,
            initial,
            vec![
                "Choose a destination gate.".to_string(),
                format!("candidates: {}", preview),
            ],
        );
    }

    pub(in crate::tui_shell) fn continue_promote_wizard(&mut self, value: String) {
        let Some(w) = self.promote_wizard.clone() else {
            self.push_error("promote wizard not active".to_string());
            return;
        };
        let gate = value.trim().to_string();
        if gate.is_empty() {
            self.start_promote_wizard(w.bundle_id, w.candidates, None);
            self.push_error("missing to gate".to_string());
            return;
        }
        if !w.candidates.iter().any(|g| g == &gate) {
            self.start_promote_wizard(w.bundle_id, w.candidates, Some(gate));
            self.push_error("invalid gate (not a candidate)".to_string());
            return;
        }

        self.promote_wizard = None;
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        match client.promote_bundle(&w.bundle_id, &gate) {
            Ok(_) => {
                self.push_output(vec![format!("promoted {} -> {}", w.bundle_id, gate)]);
                self.refresh_root_view();
            }
            Err(err) => self.push_error(format!("promote: {:#}", err)),
        }
    }
}
