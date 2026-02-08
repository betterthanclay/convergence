use super::super::*;

impl App {
    pub(in crate::tui_shell::app) fn dispatch_mode(
        &mut self,
        mode: UiMode,
        cmd: &str,
        args: &[String],
    ) {
        match mode {
            UiMode::Snaps => match cmd {
                "back" => self.dispatch_mode_back(),
                "filter" => self.cmd_snaps_filter(args),
                "clear-filter" => self.cmd_snaps_clear_filter(args),
                "snap" => self.cmd_snaps_snap(args),
                "msg" => self.cmd_snaps_msg(args),
                "revert" => self.cmd_snaps_revert(args),
                "unsnap" => self.cmd_snaps_unsnap(args),
                "restore" => self.cmd_snaps_restore(args),
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::Inbox => match cmd {
                "back" => self.dispatch_mode_back(),
                "edit" => {
                    if !args.is_empty() {
                        self.push_error("usage: edit".to_string());
                        return;
                    }
                    self.start_browse_wizard(BrowseTarget::Inbox);
                }
                "bundle" => self.cmd_inbox_bundle_mode(args),
                "fetch" => self.cmd_inbox_fetch_mode(args),
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::Bundles => match cmd {
                "back" => self.dispatch_mode_back(),
                "edit" => {
                    if !args.is_empty() {
                        self.push_error("usage: edit".to_string());
                        return;
                    }
                    self.start_browse_wizard(BrowseTarget::Bundles);
                }
                "approve" => self.cmd_bundles_approve_mode(args),
                "pin" => self.cmd_bundles_pin_mode(args),
                "promote" => self.cmd_bundles_promote_mode(args),
                "release" => self.cmd_bundles_release_mode(args),
                "superpositions" | "supers" => self.cmd_bundles_superpositions_mode(args),
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::Releases => match cmd {
                "back" => self.dispatch_mode_back(),
                "fetch" => self.cmd_releases_fetch_mode(args),
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::Lanes => match cmd {
                "back" => self.dispatch_mode_back(),
                "fetch" => self.cmd_lanes_fetch_mode(args),
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::Superpositions => match cmd {
                "back" => self.dispatch_mode_back(),
                "pick" => self.cmd_superpositions_pick_mode(args),
                "clear" => self.cmd_superpositions_clear_mode(args),
                "next-missing" => self.cmd_superpositions_next_missing_mode(args),
                "next-invalid" => self.cmd_superpositions_next_invalid_mode(args),
                "validate" => self.cmd_superpositions_validate_mode(args),
                "apply" => self.cmd_superpositions_apply_mode(args),
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::GateGraph => match cmd {
                "back" => self.dispatch_mode_back(),
                "refresh" | "r" => {
                    let _ = args;
                    self.open_gate_graph_view();
                }
                "add-gate" => {
                    let _ = args;
                    self.cmd_gate_graph_add_gate();
                }
                "remove-gate" => {
                    let _ = args;
                    self.cmd_gate_graph_remove_gate();
                }
                "edit-upstream" => {
                    let _ = args;
                    self.cmd_gate_graph_edit_upstream();
                }
                "set-approvals" => {
                    let _ = args;
                    self.cmd_gate_graph_set_approvals();
                }
                "toggle-releases" => {
                    let _ = args;
                    self.cmd_gate_graph_toggle_releases();
                }
                "toggle-superpositions" => {
                    let _ = args;
                    self.cmd_gate_graph_toggle_superpositions();
                }
                "toggle-metadata-only" => {
                    let _ = args;
                    self.cmd_gate_graph_toggle_metadata_only();
                }
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::Settings => match cmd {
                "back" => self.dispatch_mode_back(),
                "do" => {
                    if !args.is_empty() {
                        self.push_error("usage: do".to_string());
                        return;
                    }
                    self.cmd_settings_do_mode();
                }
                _ => self.push_unknown_mode_command(mode, cmd, args),
            },
            UiMode::Root => {
                self.dispatch_root(cmd, args);
            }
        }
    }

    fn dispatch_mode_back(&mut self) {
        self.pop_mode();
        self.push_output(vec!["back".to_string()]);
    }

    fn push_unknown_mode_command(&mut self, mode: UiMode, cmd: &str, args: &[String]) {
        if !self.dispatch_global(cmd, args) {
            self.push_error(format!(
                "unknown command in {:?} mode: {} (try /help)",
                mode, cmd
            ));
        }
    }
}
