use super::remote_fetch_parse::{FetchSpec, parse_fetch_spec};
use super::*;

mod bundle_release;
mod restore;
mod snaps;

impl App {
    pub(in crate::tui_shell) fn cmd_fetch_impl(&mut self, args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };
        let client = match self.remote_client() {
            Some(c) => c,
            None => return,
        };

        let parsed = match parse_fetch_spec(args) {
            Ok(p) => p,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };

        if let Some(bundle_id) = parsed.bundle_id.as_deref() {
            bundle_release::fetch_bundle(self, &client, &ws, &parsed, bundle_id);
            return;
        }

        if let Some(channel) = parsed.release.as_deref() {
            bundle_release::fetch_release(self, &client, &ws, &parsed, channel);
            return;
        }

        snaps::fetch_snaps(self, &client, &ws, &parsed);
    }
}
