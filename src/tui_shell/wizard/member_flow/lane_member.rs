use super::*;
use crate::tui_shell::App;

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
            None => {
                self.open_text_input_modal(
                    "Lane Member",
                    "action> ",
                    TextInputAction::LaneMemberAction,
                    Some("add".to_string()),
                    vec!["add | remove".to_string()],
                );
            }
            Some(_) => {
                self.open_text_input_modal(
                    "Lane Member",
                    "lane> ",
                    TextInputAction::LaneMemberLane,
                    Some("default".to_string()),
                    vec!["Lane id".to_string()],
                );
            }
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
            TextInputAction::LaneMemberAction => {
                let v = value.trim().to_lowercase();
                let act = match v.as_str() {
                    "add" => Some(MemberAction::Add),
                    "remove" | "rm" | "del" => Some(MemberAction::Remove),
                    _ => None,
                };
                let Some(act) = act else {
                    self.open_text_input_modal(
                        "Lane Member",
                        "action> ",
                        TextInputAction::LaneMemberAction,
                        Some("add".to_string()),
                        vec!["error: choose add | remove".to_string()],
                    );
                    return;
                };
                if let Some(w) = self.lane_member_wizard.as_mut() {
                    w.action = Some(act);
                }
                self.open_text_input_modal(
                    "Lane Member",
                    "lane> ",
                    TextInputAction::LaneMemberLane,
                    Some("default".to_string()),
                    vec!["Lane id".to_string()],
                );
            }
            TextInputAction::LaneMemberLane => {
                let lane = value.trim().to_string();
                if lane.is_empty() {
                    self.open_text_input_modal(
                        "Lane Member",
                        "lane> ",
                        TextInputAction::LaneMemberLane,
                        Some("default".to_string()),
                        vec!["error: value required".to_string()],
                    );
                    return;
                }
                if let Some(w) = self.lane_member_wizard.as_mut() {
                    w.lane = Some(lane);
                }
                self.open_text_input_modal(
                    "Lane Member",
                    "handle> ",
                    TextInputAction::LaneMemberHandle,
                    None,
                    vec!["User handle".to_string()],
                );
            }
            TextInputAction::LaneMemberHandle => {
                let handle = value.trim().to_string();
                if handle.is_empty() {
                    self.open_text_input_modal(
                        "Lane Member",
                        "handle> ",
                        TextInputAction::LaneMemberHandle,
                        None,
                        vec!["error: value required".to_string()],
                    );
                    return;
                }
                if let Some(w) = self.lane_member_wizard.as_mut() {
                    w.handle = Some(handle);
                }
                self.finish_lane_member_wizard();
            }
            _ => {
                self.push_error("unexpected lane-member wizard input".to_string());
            }
        }
    }

    pub(in crate::tui_shell) fn finish_lane_member_wizard(&mut self) {
        let Some(w) = self.lane_member_wizard.clone() else {
            self.push_error("lane-member wizard not active".to_string());
            return;
        };
        self.lane_member_wizard = None;

        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        let Some(action) = w.action else {
            self.push_error("lane-member: missing action".to_string());
            return;
        };
        let Some(lane) = w.lane else {
            self.push_error("lane-member: missing lane".to_string());
            return;
        };
        let Some(handle) = w.handle else {
            self.push_error("lane-member: missing handle".to_string());
            return;
        };

        match action {
            MemberAction::Add => match client.add_lane_member(&lane, &handle) {
                Ok(()) => {
                    self.push_output(vec![format!("added {} to lane {}", handle, lane)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("lane-member add: {:#}", err)),
            },
            MemberAction::Remove => match client.remove_lane_member(&lane, &handle) {
                Ok(()) => {
                    self.push_output(vec![format!("removed {} from lane {}", handle, lane)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("lane-member remove: {:#}", err)),
            },
        }
    }
}
