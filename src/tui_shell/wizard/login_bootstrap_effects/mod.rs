use crate::model::RemoteConfig;

use super::login_bootstrap_validate::parse_bootstrap_inputs;

mod bootstrap;
mod login;
mod repo;

impl super::super::App {
    pub(in crate::tui_shell) fn apply_login_config(
        &mut self,
        base_url: String,
        token: String,
        repo_id: String,
        scope: String,
        gate: String,
    ) {
        login::apply_login_config(self, base_url, token, repo_id, scope, gate);
    }

    pub(in crate::tui_shell) fn finish_bootstrap_wizard(&mut self) {
        bootstrap::finish_bootstrap_wizard(self);
    }
}
