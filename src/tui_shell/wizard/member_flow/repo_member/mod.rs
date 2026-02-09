use super::*;
use crate::tui_shell::App;

mod finish;
mod prompts;
mod transitions;

impl App {
    pub(in crate::tui_shell) fn start_member_wizard(&mut self, action: Option<MemberAction>) {
        if self.remote_client().is_none() {
            self.start_login_wizard();
            return;
        }

        self.member_wizard = Some(MemberWizard {
            action,
            handle: None,
            role: "read".to_string(),
        });

        match action {
            None => prompts::open_member_action_prompt(self, None),
            Some(_) => prompts::open_member_handle_prompt(self, None),
        }
    }

    pub(in crate::tui_shell) fn continue_member_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.member_wizard.is_none() {
            self.push_error("member wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::MemberAction => transitions::on_member_action(self, value),
            TextInputAction::MemberHandle => transitions::on_member_handle(self, value),
            TextInputAction::MemberRole => transitions::on_member_role(self, value),
            _ => self.push_error("unexpected member wizard input".to_string()),
        }
    }

    pub(in crate::tui_shell) fn finish_member_wizard(&mut self) {
        finish::finish_member_wizard(self);
    }
}
