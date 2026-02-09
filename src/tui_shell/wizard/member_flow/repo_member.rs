use super::*;
use crate::tui_shell::App;

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
            None => {
                self.open_text_input_modal(
                    "Member",
                    "action> ",
                    TextInputAction::MemberAction,
                    Some("add".to_string()),
                    vec!["add | remove".to_string()],
                );
            }
            Some(_) => {
                self.open_text_input_modal(
                    "Member",
                    "handle> ",
                    TextInputAction::MemberHandle,
                    None,
                    vec!["GitHub handle / user handle".to_string()],
                );
            }
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
            TextInputAction::MemberAction => {
                let v = value.trim().to_lowercase();
                let act = match v.as_str() {
                    "add" => Some(MemberAction::Add),
                    "remove" | "rm" | "del" => Some(MemberAction::Remove),
                    _ => None,
                };
                let Some(act) = act else {
                    self.open_text_input_modal(
                        "Member",
                        "action> ",
                        TextInputAction::MemberAction,
                        Some("add".to_string()),
                        vec!["error: choose add | remove".to_string()],
                    );
                    return;
                };
                if let Some(w) = self.member_wizard.as_mut() {
                    w.action = Some(act);
                }
                self.open_text_input_modal(
                    "Member",
                    "handle> ",
                    TextInputAction::MemberHandle,
                    None,
                    vec!["GitHub handle / user handle".to_string()],
                );
            }
            TextInputAction::MemberHandle => {
                let handle = value.trim().to_string();
                if handle.is_empty() {
                    self.open_text_input_modal(
                        "Member",
                        "handle> ",
                        TextInputAction::MemberHandle,
                        None,
                        vec!["error: value required".to_string()],
                    );
                    return;
                }
                let act = self.member_wizard.as_ref().and_then(|w| w.action);
                if let Some(w) = self.member_wizard.as_mut() {
                    w.handle = Some(handle);
                }
                match act {
                    Some(MemberAction::Add) => {
                        self.open_text_input_modal(
                            "Member",
                            "role (read/publish)> ",
                            TextInputAction::MemberRole,
                            Some("read".to_string()),
                            vec!["Default: read".to_string()],
                        );
                    }
                    Some(MemberAction::Remove) => {
                        self.finish_member_wizard();
                    }
                    None => {
                        self.start_member_wizard(None);
                    }
                }
            }
            TextInputAction::MemberRole => {
                let role = value.trim().to_lowercase();
                let role = if role.is_empty() {
                    "read".to_string()
                } else {
                    role
                };
                if role != "read" && role != "publish" {
                    self.open_text_input_modal(
                        "Member",
                        "role (read/publish)> ",
                        TextInputAction::MemberRole,
                        Some(role),
                        vec!["error: role must be read or publish".to_string()],
                    );
                    return;
                }
                if let Some(w) = self.member_wizard.as_mut() {
                    w.role = role;
                }
                self.finish_member_wizard();
            }
            _ => {
                self.push_error("unexpected member wizard input".to_string());
            }
        }
    }

    pub(in crate::tui_shell) fn finish_member_wizard(&mut self) {
        let Some(w) = self.member_wizard.clone() else {
            self.push_error("member wizard not active".to_string());
            return;
        };
        self.member_wizard = None;

        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };
        let Some(action) = w.action else {
            self.push_error("member: missing action".to_string());
            return;
        };
        let Some(handle) = w.handle else {
            self.push_error("member: missing handle".to_string());
            return;
        };

        match action {
            MemberAction::Add => match client.add_repo_member(&handle, &w.role) {
                Ok(()) => {
                    self.push_output(vec![format!("added {} ({})", handle, w.role)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("member add: {:#}", err)),
            },
            MemberAction::Remove => match client.remove_repo_member(&handle) {
                Ok(()) => {
                    self.push_output(vec![format!("removed {}", handle)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("member remove: {:#}", err)),
            },
        }
    }
}
