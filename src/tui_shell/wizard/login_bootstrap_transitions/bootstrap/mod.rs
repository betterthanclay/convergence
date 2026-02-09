use super::*;

mod prompts;
mod transitions;

impl crate::tui_shell::App {
    pub(in crate::tui_shell) fn continue_bootstrap_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.bootstrap_wizard.is_none() {
            self.push_error("bootstrap wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::BootstrapUrl => transitions::on_bootstrap_url(self, value),
            TextInputAction::BootstrapRepo => transitions::on_bootstrap_repo(self, value),
            TextInputAction::BootstrapScope => transitions::on_bootstrap_scope(self, value),
            TextInputAction::BootstrapGate => transitions::on_bootstrap_gate(self, value),
            TextInputAction::BootstrapToken => transitions::on_bootstrap_token(self, value),
            TextInputAction::BootstrapHandle => transitions::on_bootstrap_handle(self, value),
            TextInputAction::BootstrapDisplayName => {
                transitions::on_bootstrap_display_name(self, value)
            }
            _ => self.push_error("unexpected bootstrap wizard input".to_string()),
        }
    }
}
