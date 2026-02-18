use super::*;

impl App {
    fn current_workflow_profile(&self) -> crate::model::WorkflowProfile {
        self.workspace
            .as_ref()
            .and_then(|ws| ws.store.read_config().ok())
            .map(|cfg| cfg.workflow_profile)
            .unwrap_or_default()
    }

    fn default_release_channel(profile: crate::model::WorkflowProfile) -> &'static str {
        match profile {
            crate::model::WorkflowProfile::Software => "main",
            crate::model::WorkflowProfile::Daw => "master",
            crate::model::WorkflowProfile::GameAssets => "internal",
        }
    }

    pub(in crate::tui_shell) fn start_release_wizard(&mut self, bundle_id: String) {
        let profile = self.current_workflow_profile();
        let default_channel = Self::default_release_channel(profile).to_string();
        self.release_wizard = Some(ReleaseWizard {
            bundle_id,
            channel: default_channel.clone(),
            notes: None,
        });

        self.open_text_input_modal(
            "Release",
            "channel> ",
            TextInputAction::ReleaseChannel,
            Some(default_channel.clone()),
            vec![
                "Release channel name (example: main).".to_string(),
                format!("profile: {}", profile.as_str()),
                format!("default: {}", default_channel),
                profile.release_hint().to_string(),
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
