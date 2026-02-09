use super::select::{selected_inbox_publication_id, selected_inbox_snap_id};
use super::*;

impl App {
    pub(in crate::tui_shell) fn cmd_inbox_bundle_mode(&mut self, args: &[String]) {
        if args.len() > 1 {
            self.push_error("usage: bundle [<publication_id>]".to_string());
            return;
        }

        let pub_id = if let Some(id) = args.first() {
            id.clone()
        } else {
            let Some(id) = selected_inbox_publication_id(self) else {
                return;
            };
            id
        };

        self.cmd_bundle(&["--publication".to_string(), pub_id]);
    }

    pub(in crate::tui_shell) fn cmd_inbox_fetch_mode(&mut self, args: &[String]) {
        if args.len() > 1 {
            self.push_error("usage: fetch [<snap_id>]".to_string());
            return;
        }

        let snap_id = if let Some(id) = args.first() {
            id.clone()
        } else {
            let Some(id) = selected_inbox_snap_id(self) else {
                return;
            };
            id
        };

        self.cmd_fetch(&["--snap-id".to_string(), snap_id]);
    }
}
