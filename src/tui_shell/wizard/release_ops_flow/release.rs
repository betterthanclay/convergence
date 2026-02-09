use super::*;

impl App {
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
}
