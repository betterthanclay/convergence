use super::*;

impl App {
    pub(in crate::tui_shell::app) fn cmd_member(&mut self, args: &[String]) {
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
}
