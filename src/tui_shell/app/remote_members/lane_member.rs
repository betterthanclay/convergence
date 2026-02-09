use super::*;

impl App {
    pub(in crate::tui_shell::app) fn cmd_lane_member(&mut self, args: &[String]) {
        if args.is_empty() {
            self.start_lane_member_wizard(None);
            return;
        }

        // Prompt-first UX:
        // - `lane-member` -> wizard
        // - `lane-member add` / `lane-member remove` -> wizard
        // - `lane-member add <lane> <handle>`
        // - `lane-member remove <lane> <handle>`
        let sub = args[0].as_str();
        if matches!(sub, "add" | "remove" | "rm") {
            let action = if sub == "add" {
                Some(MemberAction::Add)
            } else {
                Some(MemberAction::Remove)
            };
            if args.len() < 3 {
                self.start_lane_member_wizard(action);
                return;
            }
            let lane = args[1].trim().to_string();
            let handle = args[2].trim().to_string();
            if lane.is_empty() || handle.is_empty() {
                self.start_lane_member_wizard(action);
                return;
            }

            let client = match self.remote_client() {
                Some(c) => c,
                None => {
                    self.start_login_wizard();
                    return;
                }
            };
            match action {
                Some(MemberAction::Add) => match client.add_lane_member(&lane, &handle) {
                    Ok(()) => {
                        self.push_output(vec![format!("added {} to lane {}", handle, lane)]);
                        self.refresh_root_view();
                    }
                    Err(err) => self.push_error(format!("lane-member add: {:#}", err)),
                },
                Some(MemberAction::Remove) => match client.remove_lane_member(&lane, &handle) {
                    Ok(()) => {
                        self.push_output(vec![format!("removed {} from lane {}", handle, lane)]);
                        self.refresh_root_view();
                    }
                    Err(err) => self.push_error(format!("lane-member remove: {:#}", err)),
                },
                None => self.start_lane_member_wizard(None),
            }
            return;
        }

        // Back-compat: accept legacy flag form.
        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let sub = &args[0];
        let mut lane: Option<String> = None;
        let mut handle: Option<String> = None;

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--lane" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --lane".to_string());
                        return;
                    }
                    lane = Some(args[i].clone());
                }
                "--handle" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --handle".to_string());
                        return;
                    }
                    handle = Some(args[i].clone());
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let Some(lane) = lane else {
            self.push_error("missing --lane".to_string());
            return;
        };
        let Some(handle) = handle else {
            self.push_error("missing --handle".to_string());
            return;
        };

        match sub.as_str() {
            "add" => match client.add_lane_member(&lane, &handle) {
                Ok(()) => {
                    self.push_output(vec![format!("added {} to lane {}", handle, lane)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("lane-member add: {:#}", err)),
            },
            "remove" | "rm" => match client.remove_lane_member(&lane, &handle) {
                Ok(()) => {
                    self.push_output(vec![format!("removed {} from lane {}", handle, lane)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("lane-member remove: {:#}", err)),
            },
            _ => self.start_lane_member_wizard(None),
        }
    }
}
