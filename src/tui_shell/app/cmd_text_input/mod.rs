use super::*;

mod direct_commands;
mod settings_actions;
mod wizard_routes;

impl App {
    pub(in crate::tui_shell) fn submit_text_input(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if settings_actions::is_settings_action(&action) {
            settings_actions::apply_settings_text_input(self, action, value);
            return;
        }

        if direct_commands::is_direct_command_action(&action) {
            direct_commands::apply_direct_command_text_input(self, action, value);
            return;
        }

        wizard_routes::submit_wizard_text_input(self, action, value);
    }
}
