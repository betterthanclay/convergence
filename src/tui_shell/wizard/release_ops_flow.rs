use super::super::TextInputAction;
use super::types::{PinWizard, PromoteWizard, ReleaseWizard};

impl super::super::App {
    pub(in crate::tui_shell) fn start_release_wizard(&mut self, bundle_id: String) {
        self.release_wizard = Some(ReleaseWizard {
            bundle_id,
            channel: "main".to_string(),
            notes: None,
        });

        self.open_text_input_modal(
            "Release",
            "channel> ",
            TextInputAction::ReleaseChannel,
            Some("main".to_string()),
            vec![
                "Release channel name (example: main).".to_string(),
                "Esc cancels.".to_string(),
            ],
        );
    }

    pub(in crate::tui_shell) fn continue_release_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.release_wizard.is_none() {
            self.push_error("release wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::ReleaseChannel => {
                let v = value.trim().to_string();
                if let Some(w) = self.release_wizard.as_mut()
                    && !v.is_empty()
                {
                    w.channel = v;
                }

                self.open_text_input_modal(
                    "Release",
                    "notes (blank=none)> ",
                    TextInputAction::ReleaseNotes,
                    None,
                    vec!["Optional release notes.".to_string()],
                );
            }

            TextInputAction::ReleaseNotes => {
                let v = value.trim().to_string();
                if let Some(w) = self.release_wizard.as_mut() {
                    w.notes = if v.is_empty() { None } else { Some(v) };
                }
                self.finish_release_wizard();
            }

            _ => {
                self.push_error("unexpected release wizard input".to_string());
            }
        }
    }

    pub(in crate::tui_shell) fn finish_release_wizard(&mut self) {
        let Some(w) = self.release_wizard.clone() else {
            self.push_error("release wizard not active".to_string());
            return;
        };
        self.release_wizard = None;

        let mut argv = vec![
            "--channel".to_string(),
            w.channel,
            "--bundle-id".to_string(),
            w.bundle_id,
        ];
        if let Some(n) = w.notes {
            argv.extend(["--notes".to_string(), n]);
        }
        self.cmd_release(&argv);
    }

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
