use super::*;
use crate::tui_shell::App;

mod finish;
mod prompts;
mod transitions;

impl App {
    pub(in crate::tui_shell) fn start_lane_member_wizard(&mut self, action: Option<MemberAction>) {
        if self.remote_client().is_none() {
            self.start_login_wizard();
            return;
        }

        self.lane_member_wizard = Some(LaneMemberWizard {
            action,
            lane: None,
            handle: None,
        });

        match action {
            None => prompts::open_lane_member_action_prompt(self, None),
            Some(_) => prompts::open_lane_member_lane_prompt(self, None),
        }
    }

    pub(in crate::tui_shell) fn continue_lane_member_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.lane_member_wizard.is_none() {
            self.push_error("lane-member wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::LaneMemberAction => transitions::on_lane_member_action(self, value),
            TextInputAction::LaneMemberLane => transitions::on_lane_member_lane(self, value),
            TextInputAction::LaneMemberHandle => transitions::on_lane_member_handle(self, value),
            _ => self.push_error("unexpected lane-member wizard input".to_string()),
        }
    }

    pub(in crate::tui_shell) fn finish_lane_member_wizard(&mut self) {
        finish::finish_lane_member_wizard(self);
    }
}
