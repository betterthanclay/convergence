use super::remote_scope_query_parse::parse_scope_query_args;
use super::*;

impl App {
    pub(super) fn cmd_fetch(&mut self, args: &[String]) {
        if args.is_empty() {
            self.start_fetch_wizard();
            return;
        }
        self.cmd_fetch_impl(args);
    }

    pub(super) fn cmd_inbox(&mut self, args: &[String]) {
        if args.len() == 1 && args[0] == "edit" {
            self.start_browse_wizard(BrowseTarget::Inbox);
            return;
        }

        let cfg = match self.remote_config() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let parsed = match parse_scope_query_args(args) {
            Ok(v) => v,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };

        let scope = parsed.scope.unwrap_or(cfg.scope);
        let gate = parsed.gate.unwrap_or(cfg.gate);
        self.open_inbox_view(scope, gate, parsed.filter, parsed.limit);
    }

    pub(super) fn cmd_bundles(&mut self, args: &[String]) {
        if args.len() == 1 && args[0] == "edit" {
            self.start_browse_wizard(BrowseTarget::Bundles);
            return;
        }

        let cfg = match self.remote_config() {
            Some(c) => c,
            None => {
                self.start_login_wizard();
                return;
            }
        };

        let parsed = match parse_scope_query_args(args) {
            Ok(v) => v,
            Err(msg) => {
                self.push_error(msg);
                return;
            }
        };

        let scope = parsed.scope.unwrap_or(cfg.scope);
        let gate = parsed.gate.unwrap_or(cfg.gate);
        self.open_bundles_view(scope, gate, parsed.filter, parsed.limit);
    }
}
