use super::super::*;

impl App {
    pub(in crate::tui_shell::app) fn dispatch_root(&mut self, cmd: &str, args: &[String]) {
        match self.root_ctx {
            RootContext::Local => match cmd {
                "status" => self.cmd_status(args),
                "refresh" | "r" => {
                    let _ = args;
                    self.refresh_root_view();
                    self.push_output(vec!["refreshed".to_string()]);
                }
                "init" => self.cmd_init(args),
                "snap" => self.cmd_snap(args),
                "publish" => self.cmd_publish(args),
                "sync" => self.cmd_sync(args),
                "history" => self.cmd_snaps(args),
                "show" => self.cmd_show(args),
                "restore" => self.cmd_restore(args),
                "move" => self.cmd_move(args),
                "purge" => self.cmd_gc(args),

                "clear" => {
                    self.log.clear();
                    self.last_command = None;
                    self.last_result = None;
                }
                "quit" => {
                    self.quit = true;
                }

                "bootstrap" | "create-repo" | "gates" | "remote" | "ping" | "fetch" | "lanes"
                | "releases" | "members" | "member" | "lane-member" | "inbox" | "bundles"
                | "bundle" | "pins" | "pin" | "approve" | "promote" | "release"
                | "superpositions" | "supers" => {
                    self.switch_to_remote_root();
                    self.push_output(vec![format!("switched to remote context for `{}`", cmd)]);
                    self.dispatch_root(cmd, args);
                }

                _ => {
                    if !self.dispatch_global(cmd, args) {
                        self.push_error(format!("unknown command: {}", cmd));
                    }
                }
            },
            RootContext::Remote => match cmd {
                "status" => self.cmd_status(args),
                "bootstrap" => self.cmd_bootstrap(args),
                "create-repo" => self.cmd_create_repo(args),
                "gates" => self.cmd_gate_graph(args),
                "refresh" | "r" => {
                    let _ = args;
                    self.refresh_root_view();
                    self.push_output(vec!["refreshed".to_string()]);
                }
                "remote" => self.cmd_remote(args),
                "ping" => self.cmd_ping(args),
                "fetch" => self.cmd_fetch(args),
                "lanes" => self.cmd_lanes(args),
                "releases" => self.cmd_releases(args),
                "members" => self.cmd_members(args),
                "member" => self.cmd_member(args),
                "lane-member" => self.cmd_lane_member(args),
                "inbox" => self.cmd_inbox(args),
                "bundles" => self.cmd_bundles(args),
                "bundle" => self.cmd_bundle(args),
                "pins" => self.cmd_pins(args),
                "pin" => self.cmd_pin(args),
                "approve" => self.cmd_approve(args),
                "promote" => self.cmd_promote(args),
                "release" => self.cmd_release(args),
                "superpositions" => self.cmd_superpositions(args),
                "supers" => self.cmd_superpositions(args),

                "clear" => {
                    self.log.clear();
                    self.last_command = None;
                    self.last_result = None;
                }
                "quit" => {
                    self.quit = true;
                }

                "init" | "snap" | "publish" | "sync" | "history" | "show" | "restore" | "move"
                | "mv" | "purge" => {
                    self.switch_to_local_root();
                    self.push_output(vec![format!("switched to local context for `{}`", cmd)]);
                    self.dispatch_root(cmd, args);
                }

                _ => {
                    if !self.dispatch_global(cmd, args) {
                        self.push_error(format!("unknown command: {}", cmd));
                    }
                }
            },
        }
    }
}
