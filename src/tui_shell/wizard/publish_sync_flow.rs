use super::super::TextInputAction;
use super::types::{PublishWizard, SyncWizard};

impl super::super::App {
    pub(in crate::tui_shell) fn start_publish_wizard(&mut self, edit: bool) {
        let Some(_) = self.require_workspace() else {
            return;
        };
        let Some(cfg) = self.remote_config() else {
            self.start_login_wizard();
            return;
        };

        self.publish_wizard = Some(PublishWizard {
            snap: None,
            scope: Some(cfg.scope.clone()),
            gate: Some(cfg.gate.clone()),
            meta: false,
        });

        if edit {
            self.open_text_input_modal(
                "Publish",
                "snap (blank=latest)> ",
                TextInputAction::PublishSnap,
                None,
                vec![
                    "Optional: snap id (leave blank to publish latest).".to_string(),
                    "Esc cancels.".to_string(),
                ],
            );
        } else {
            self.open_text_input_modal(
                "Publish",
                "publish> ",
                TextInputAction::PublishStart,
                None,
                vec![
                    format!("Default: latest snap -> {}/{}", cfg.scope, cfg.gate),
                    "Enter: publish now".to_string(),
                    "Type `edit` to customize (snap/scope/gate/meta).".to_string(),
                ],
            );
        }
    }

    pub(in crate::tui_shell) fn continue_publish_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.publish_wizard.is_none() {
            self.push_error("publish wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::PublishStart => {
                let v = value.trim().to_string();
                if v.is_empty() {
                    self.publish_wizard = None;
                    self.cmd_publish_impl(&[]);
                    return;
                }

                let v_lc = v.to_lowercase();
                if matches!(v_lc.as_str(), "edit" | "prompt" | "custom") {
                    // Jump into the snap prompt (blank=latest).
                    self.open_text_input_modal(
                        "Publish",
                        "snap (blank=latest)> ",
                        TextInputAction::PublishSnap,
                        None,
                        vec!["Optional: snap id".to_string()],
                    );
                    return;
                }

                // Treat any other input as a snap id override.
                if let Some(w) = self.publish_wizard.as_mut() {
                    w.snap = Some(v);
                }

                let initial = self.publish_wizard.as_ref().and_then(|w| w.scope.clone());
                self.open_text_input_modal(
                    "Publish",
                    "scope> ",
                    TextInputAction::PublishScope,
                    initial,
                    vec!["Scope id (Enter keeps default).".to_string()],
                );
            }
            TextInputAction::PublishSnap => {
                let v = value.trim().to_string();
                if let Some(w) = self.publish_wizard.as_mut() {
                    w.snap = if v.is_empty() { None } else { Some(v) };
                }

                let initial = self.publish_wizard.as_ref().and_then(|w| w.scope.clone());
                self.open_text_input_modal(
                    "Publish",
                    "scope> ",
                    TextInputAction::PublishScope,
                    initial,
                    vec!["Scope id (Enter keeps default).".to_string()],
                );
            }
            TextInputAction::PublishScope => {
                let v = value.trim().to_string();
                if let Some(w) = self.publish_wizard.as_mut() {
                    w.scope = if v.is_empty() { None } else { Some(v) };
                }

                let initial = self.publish_wizard.as_ref().and_then(|w| w.gate.clone());
                self.open_text_input_modal(
                    "Publish",
                    "gate> ",
                    TextInputAction::PublishGate,
                    initial,
                    vec!["Gate id (Enter keeps default).".to_string()],
                );
            }
            TextInputAction::PublishGate => {
                let v = value.trim().to_string();
                if let Some(w) = self.publish_wizard.as_mut() {
                    w.gate = if v.is_empty() { None } else { Some(v) };
                }

                self.open_text_input_modal(
                    "Publish",
                    "metadata-only? (y/N)> ",
                    TextInputAction::PublishMeta,
                    Some("n".to_string()),
                    vec![
                        "If yes, publish metadata only (objects may be missing until later)."
                            .to_string(),
                    ],
                );
            }
            TextInputAction::PublishMeta => {
                let v = value.trim().to_lowercase();
                let meta = matches!(v.as_str(), "y" | "yes" | "true" | "1");
                if let Some(w) = self.publish_wizard.as_mut() {
                    w.meta = meta;
                }
                self.finish_publish_wizard();
            }
            _ => {
                self.push_error("unexpected publish wizard input".to_string());
            }
        }
    }

    pub(in crate::tui_shell) fn finish_publish_wizard(&mut self) {
        let Some(w) = self.publish_wizard.clone() else {
            self.push_error("publish wizard not active".to_string());
            return;
        };
        self.publish_wizard = None;

        let mut argv: Vec<String> = Vec::new();
        if let Some(s) = w.snap {
            argv.extend(["--snap-id".to_string(), s]);
        }
        if let Some(s) = w.scope {
            argv.extend(["--scope".to_string(), s]);
        }
        if let Some(g) = w.gate {
            argv.extend(["--gate".to_string(), g]);
        }
        if w.meta {
            argv.push("--metadata-only".to_string());
        }

        self.cmd_publish_impl(&argv);
    }

    pub(in crate::tui_shell) fn start_sync_wizard(&mut self, edit: bool) {
        let Some(_) = self.require_workspace() else {
            return;
        };
        if self.remote_config().is_none() {
            self.start_login_wizard();
            return;
        }

        self.sync_wizard = Some(SyncWizard {
            snap: None,
            lane: "default".to_string(),
            client: None,
        });

        if edit {
            self.open_text_input_modal(
                "Sync",
                "lane> ",
                TextInputAction::SyncLane,
                Some("default".to_string()),
                vec!["Lane id (Enter keeps default).".to_string()],
            );
        } else {
            self.open_text_input_modal(
                "Sync",
                "sync> ",
                TextInputAction::SyncStart,
                None,
                vec![
                    "Default: latest snap -> lane=default".to_string(),
                    "Enter: sync now".to_string(),
                    "Type a lane id, or `edit` to customize (lane/client/snap).".to_string(),
                ],
            );
        }
    }

    pub(in crate::tui_shell) fn continue_sync_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.sync_wizard.is_none() {
            self.push_error("sync wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::SyncStart => {
                let v = value.trim().to_string();
                if v.is_empty() {
                    self.sync_wizard = None;
                    self.cmd_sync_impl(&[]);
                    return;
                }

                let v_lc = v.to_lowercase();
                if matches!(v_lc.as_str(), "edit" | "prompt" | "custom") {
                    self.open_text_input_modal(
                        "Sync",
                        "lane> ",
                        TextInputAction::SyncLane,
                        Some("default".to_string()),
                        vec!["Lane id (Enter keeps default).".to_string()],
                    );
                    return;
                }

                if let Some(w) = self.sync_wizard.as_mut() {
                    w.lane = v;
                }
                self.open_text_input_modal(
                    "Sync",
                    "client (blank=auto)> ",
                    TextInputAction::SyncClient,
                    None,
                    vec!["Optional: client id (rarely needed).".to_string()],
                );
            }

            TextInputAction::SyncLane => {
                let v = value.trim().to_string();
                if let Some(w) = self.sync_wizard.as_mut()
                    && !v.is_empty()
                {
                    w.lane = v;
                }
                self.open_text_input_modal(
                    "Sync",
                    "client (blank=auto)> ",
                    TextInputAction::SyncClient,
                    None,
                    vec!["Optional: client id (rarely needed).".to_string()],
                );
            }

            TextInputAction::SyncClient => {
                let v = value.trim().to_string();
                if let Some(w) = self.sync_wizard.as_mut() {
                    w.client = if v.is_empty() { None } else { Some(v) };
                }
                self.open_text_input_modal(
                    "Sync",
                    "snap (blank=latest)> ",
                    TextInputAction::SyncSnap,
                    None,
                    vec!["Optional: snap id (leave blank for latest).".to_string()],
                );
            }

            TextInputAction::SyncSnap => {
                let v = value.trim().to_string();
                if let Some(w) = self.sync_wizard.as_mut() {
                    w.snap = if v.is_empty() { None } else { Some(v) };
                }
                self.finish_sync_wizard();
            }

            _ => {
                self.push_error("unexpected sync wizard input".to_string());
            }
        }
    }

    pub(in crate::tui_shell) fn finish_sync_wizard(&mut self) {
        let Some(w) = self.sync_wizard.clone() else {
            self.push_error("sync wizard not active".to_string());
            return;
        };
        self.sync_wizard = None;

        let mut argv: Vec<String> = Vec::new();
        if let Some(s) = w.snap {
            argv.extend(["--snap-id".to_string(), s]);
        }
        if !w.lane.trim().is_empty() {
            argv.extend(["--lane".to_string(), w.lane]);
        }
        if let Some(c) = w.client {
            argv.extend(["--client-id".to_string(), c]);
        }
        self.cmd_sync_impl(&argv);
    }
}
