use super::remote_action_parse::{
    parse_approve_args, parse_pin_args, parse_promote_args, parse_release_args,
};
use super::*;

impl App {
    pub(super) fn cmd_pin(&mut self, args: &[String]) {
        if args.is_empty() {
            self.start_pin_wizard();
            return;
        }

        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let parsed = match parse_pin_args(args) {
            Ok(p) => p,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };
        let Some(bundle_id) = parsed.bundle_id else {
            self.push_error("usage: pin <bundle_id> [unpin]".to_string());
            return;
        };

        let res = if parsed.unpin {
            client.unpin_bundle(&bundle_id)
        } else {
            client.pin_bundle(&bundle_id)
        };
        match res {
            Ok(()) => {
                if parsed.unpin {
                    self.push_output(vec![format!("unpinned {}", bundle_id)]);
                } else {
                    self.push_output(vec![format!("pinned {}", bundle_id)]);
                }
                self.refresh_root_view();
            }
            Err(err) => {
                self.push_error(format!("pin: {:#}", err));
            }
        }
    }

    pub(super) fn cmd_approve(&mut self, args: &[String]) {
        if args.is_empty() {
            self.open_text_input_modal(
                "Approve",
                "bundle id> ",
                TextInputAction::ApproveBundleId,
                None,
                vec!["Bundle id".to_string()],
            );
            return;
        }

        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };
        let parsed = match parse_approve_args(args) {
            Ok(p) => p,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };
        let Some(bundle_id) = parsed.bundle_id else {
            self.push_error("usage: approve <bundle_id>".to_string());
            return;
        };

        match client.approve_bundle(&bundle_id) {
            Ok(_) => self.push_output(vec![format!("approved {}", bundle_id)]),
            Err(err) => self.push_error(format!("approve: {:#}", err)),
        }
    }

    pub(super) fn cmd_promote(&mut self, args: &[String]) {
        if args.is_empty() {
            self.open_text_input_modal(
                "Promote",
                "bundle id> ",
                TextInputAction::PromoteBundleId,
                None,
                vec!["Bundle id".to_string()],
            );
            return;
        }

        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let parsed = match parse_promote_args(args) {
            Ok(p) => p,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };
        let Some(bundle_id) = parsed.bundle_id else {
            self.open_text_input_modal(
                "Promote",
                "bundle id> ",
                TextInputAction::PromoteBundleId,
                None,
                vec!["Bundle id".to_string()],
            );
            return;
        };

        let to_gate = match parsed.to_gate {
            Some(g) => g,
            None => {
                // Convenience: if exactly one downstream gate, use it.
                let graph = match client.get_gate_graph() {
                    Ok(g) => g,
                    Err(err) => {
                        self.push_error(format!("get gate graph: {:#}", err));
                        return;
                    }
                };

                let bundle = match client.get_bundle(&bundle_id) {
                    Ok(b) => b,
                    Err(err) => {
                        self.push_error(format!("get bundle: {:#}", err));
                        return;
                    }
                };
                let mut next = graph
                    .gates
                    .iter()
                    .filter(|g| g.upstream.iter().any(|u| u == &bundle.gate))
                    .map(|g| g.id.clone())
                    .collect::<Vec<_>>();
                next.sort();
                if next.len() == 1 {
                    next[0].clone()
                } else {
                    if next.is_empty() {
                        self.push_error("no downstream gates for bundle gate".to_string());
                        return;
                    }
                    self.start_promote_wizard(bundle_id.clone(), next, None);
                    return;
                }
            }
        };

        match client.promote_bundle(&bundle_id, &to_gate) {
            Ok(_) => self.push_output(vec![format!("promoted {} -> {}", bundle_id, to_gate)]),
            Err(err) => self.push_error(format!("promote: {:#}", err)),
        }
    }

    pub(in crate::tui_shell) fn cmd_release(&mut self, args: &[String]) {
        if args.is_empty() {
            self.open_text_input_modal(
                "Release",
                "bundle id> ",
                TextInputAction::ReleaseBundleId,
                None,
                vec!["Bundle id".to_string()],
            );
            return;
        }

        let client = match self.remote_client() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let parsed = match parse_release_args(args) {
            Ok(p) => p,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };
        let (Some(channel), Some(bundle_id)) = (parsed.channel, parsed.bundle_id) else {
            self.push_error("usage: release <channel> <bundle_id> [notes...]".to_string());
            return;
        };

        match client.create_release(&channel, &bundle_id, parsed.notes) {
            Ok(r) => {
                self.push_output(vec![format!("released {} -> {}", r.channel, r.bundle_id)]);
                self.refresh_root_view();
            }
            Err(err) => {
                self.push_error(format!("release: {:#}", err));
            }
        }
    }
}
