use super::*;

impl App {
    pub(super) fn cmd_members(&mut self, args: &[String]) {
        let _ = args;
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        let members = match client.list_repo_members() {
            Ok(m) => m,
            Err(err) => {
                self.push_error(format!("members: {:#}", err));
                return;
            }
        };

        let lanes = client.list_lanes().ok();

        let mut lines = Vec::new();
        lines.push("Repo".to_string());
        lines.push(format!("owner: {}", members.owner));

        let publishers: std::collections::HashSet<String> =
            members.publishers.iter().cloned().collect();
        let mut readers = members.readers;
        readers.sort();
        lines.push("".to_string());
        lines.push("members:".to_string());
        for h in readers {
            let role = if publishers.contains(&h) {
                "publish"
            } else {
                "read"
            };
            lines.push(format!("- {} {}", h, role));
        }

        if let Some(mut lanes) = lanes {
            lanes.sort_by(|a, b| a.id.cmp(&b.id));
            lines.push("".to_string());
            lines.push("Lanes".to_string());
            for l in lanes {
                let mut m = l.members.into_iter().collect::<Vec<_>>();
                m.sort();
                lines.push(format!("lane {} ({})", l.id, m.len()));
                if !m.is_empty() {
                    let preview = m.into_iter().take(10).collect::<Vec<_>>().join(", ");
                    lines.push(format!("  {}", preview));
                }
            }
        }

        lines.push("".to_string());
        lines.push("hint: type `member` or `lane-member`".to_string());
        self.open_modal("Members", lines);
    }

    pub(super) fn cmd_member(&mut self, args: &[String]) {
        if args.is_empty() {
            self.start_member_wizard(None);
            return;
        }

        // Prompt-first UX:
        // - `member` -> wizard
        // - `member add` / `member remove` -> wizard
        // - `member add <handle> [read|publish]`
        // - `member remove <handle>`
        let sub = args[0].as_str();
        if matches!(sub, "add" | "remove" | "rm") {
            let action = if sub == "add" {
                Some(MemberAction::Add)
            } else {
                Some(MemberAction::Remove)
            };
            if args.len() == 1 {
                self.start_member_wizard(action);
                return;
            }
            let handle = args[1].trim().to_string();
            if handle.is_empty() {
                self.start_member_wizard(action);
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
                Some(MemberAction::Add) => {
                    let role = args.get(2).cloned().unwrap_or_else(|| "read".to_string());
                    let role_lc = role.to_lowercase();
                    if role_lc != "read" && role_lc != "publish" {
                        self.push_error("role must be read or publish".to_string());
                        return;
                    }
                    match client.add_repo_member(&handle, &role_lc) {
                        Ok(()) => {
                            self.push_output(vec![format!("added {} ({})", handle, role_lc)]);
                            self.refresh_root_view();
                        }
                        Err(err) => self.push_error(format!("member add: {:#}", err)),
                    }
                }
                Some(MemberAction::Remove) => match client.remove_repo_member(&handle) {
                    Ok(()) => {
                        self.push_output(vec![format!("removed {}", handle)]);
                        self.refresh_root_view();
                    }
                    Err(err) => self.push_error(format!("member remove: {:#}", err)),
                },
                None => {
                    self.start_member_wizard(None);
                }
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
        let mut handle: Option<String> = None;
        let mut role: String = "read".to_string();

        let mut i = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--handle" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --handle".to_string());
                        return;
                    }
                    handle = Some(args[i].clone());
                }
                "--role" => {
                    i += 1;
                    if i >= args.len() {
                        self.push_error("missing value for --role".to_string());
                        return;
                    }
                    role = args[i].clone();
                }
                a => {
                    self.push_error(format!("unknown arg: {}", a));
                    return;
                }
            }
            i += 1;
        }

        let Some(handle) = handle else {
            self.push_error("missing --handle".to_string());
            return;
        };

        match sub.as_str() {
            "add" => match client.add_repo_member(&handle, &role) {
                Ok(()) => {
                    self.push_output(vec![format!("added {} ({})", handle, role)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("member add: {:#}", err)),
            },
            "remove" | "rm" => match client.remove_repo_member(&handle) {
                Ok(()) => {
                    self.push_output(vec![format!("removed {}", handle)]);
                    self.refresh_root_view();
                }
                Err(err) => self.push_error(format!("member remove: {:#}", err)),
            },
            _ => self.start_member_wizard(None),
        }
    }

    pub(super) fn cmd_lane_member(&mut self, args: &[String]) {
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
