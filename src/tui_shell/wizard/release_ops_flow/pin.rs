use super::*;

impl App {
    pub(in crate::tui_shell) fn start_pin_wizard(&mut self) {
        if self.remote_client().is_none() {
            self.start_login_wizard();
            return;
        }

        self.pin_wizard = Some(PinWizard { bundle_id: None });
        self.open_text_input_modal(
            "Pin",
            "bundle id> ",
            TextInputAction::PinBundleId,
            None,
            vec!["Bundle id".to_string()],
        );
    }

    pub(in crate::tui_shell) fn finish_pin_wizard(&mut self, value: String) {
        let Some(w) = self.pin_wizard.clone() else {
            self.push_error("pin wizard not active".to_string());
            return;
        };

        let bundle_id = match w.bundle_id {
            Some(id) if !id.trim().is_empty() => id,
            _ => {
                self.pin_wizard = None;
                self.push_error("pin: missing bundle id".to_string());
                return;
            }
        };

        let v = value.trim().to_lowercase();
        let unpin = matches!(v.as_str(), "unpin" | "u" | "rm" | "remove");

        self.pin_wizard = None;

        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        let res = if unpin {
            client.unpin_bundle(&bundle_id)
        } else {
            client.pin_bundle(&bundle_id)
        };
        match res {
            Ok(()) => {
                if unpin {
                    self.push_output(vec![format!("unpinned {}", bundle_id)]);
                } else {
                    self.push_output(vec![format!("pinned {}", bundle_id)]);
                }
                self.refresh_root_view();
            }
            Err(err) => {
                self.push_error(format!("pin: {:#}", err));
            }
        }
    }
}
