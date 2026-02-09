use super::select::selected_bundle_id;
use super::*;

impl App {
    pub(in crate::tui_shell) fn cmd_bundles_approve_mode(&mut self, args: &[String]) {
        if args.len() > 1 {
            self.push_error("usage: approve [<bundle_id>]".to_string());
            return;
        }

        let bundle_id = if let Some(id) = args.first() {
            id.clone()
        } else {
            let Some(id) = selected_bundle_id(self) else {
                return;
            };
            id
        };

        self.cmd_approve(&["--bundle-id".to_string(), bundle_id]);
    }

    pub(in crate::tui_shell) fn cmd_bundles_pin_mode(&mut self, args: &[String]) {
        if args.len() > 1 {
            self.push_error("usage: pin [unpin]".to_string());
            return;
        }

        let Some(bundle_id) = selected_bundle_id(self) else {
            return;
        };
        let mut argv = vec!["--bundle-id".to_string(), bundle_id];
        if args.first().is_some_and(|s| s == "unpin") {
            argv.push("--unpin".to_string());
        }
        self.cmd_pin(&argv);
    }

    pub(in crate::tui_shell) fn cmd_bundles_promote_mode(&mut self, args: &[String]) {
        let Some(bundle_id) = selected_bundle_id(self) else {
            return;
        };

        let mut argv = vec!["--bundle-id".to_string(), bundle_id];
        argv.extend(args.iter().cloned());
        self.cmd_promote(&argv);
    }

    pub(in crate::tui_shell) fn cmd_bundles_release_mode(&mut self, args: &[String]) {
        let Some(bundle_id) = selected_bundle_id(self) else {
            return;
        };

        if args.is_empty() {
            self.start_release_wizard(bundle_id);
            return;
        }
        if args.len() != 1 {
            self.push_error("usage: release [<channel>]".to_string());
            return;
        }

        self.cmd_release(&[
            "--channel".to_string(),
            args[0].clone(),
            "--bundle-id".to_string(),
            bundle_id,
        ]);
    }

    pub(in crate::tui_shell) fn cmd_bundles_superpositions_mode(&mut self, args: &[String]) {
        if !args.is_empty() {
            self.push_error("usage: superpositions".to_string());
            return;
        }

        let Some(bundle_id) = selected_bundle_id(self) else {
            return;
        };

        self.cmd_superpositions(&["--bundle-id".to_string(), bundle_id]);
    }
}
