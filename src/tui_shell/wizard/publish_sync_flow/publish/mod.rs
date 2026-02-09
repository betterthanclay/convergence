use super::*;
use crate::tui_shell::App;

mod finish;
mod prompts;
mod transitions;

impl App {
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
            prompts::open_publish_snap_prompt(self, true);
        } else {
            prompts::open_publish_start_prompt(self, &cfg.scope, &cfg.gate);
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
            TextInputAction::PublishStart => transitions::on_publish_start(self, value),
            TextInputAction::PublishSnap => transitions::on_publish_snap(self, value),
            TextInputAction::PublishScope => transitions::on_publish_scope(self, value),
            TextInputAction::PublishGate => transitions::on_publish_gate(self, value),
            TextInputAction::PublishMeta => transitions::on_publish_meta(self, value),
            _ => self.push_error("unexpected publish wizard input".to_string()),
        }
    }

    pub(in crate::tui_shell) fn finish_publish_wizard(&mut self) {
        finish::finish_publish_wizard(self);
    }
}
