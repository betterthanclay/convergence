use super::*;

mod fetch_id;
mod fetch_kind;
mod options;
mod prompts;

impl crate::tui_shell::App {
    pub(in crate::tui_shell) fn continue_fetch_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.fetch_wizard.is_none() {
            self.push_error("fetch wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::FetchKind => fetch_kind::on_fetch_kind(self, value),
            TextInputAction::FetchId => fetch_id::on_fetch_id(self, value),
            TextInputAction::FetchUser => options::on_fetch_user(self, value),
            TextInputAction::FetchOptions => options::on_fetch_options(self, value),
            _ => self.push_error("unexpected fetch wizard input".to_string()),
        }
    }
}
