use super::*;

impl App {
    pub(in crate::tui_shell) fn cmd_bootstrap(&mut self, args: &[String]) {
        let Some(_) = self.require_workspace() else {
            return;
        };
        if !args.is_empty() {
            self.push_error("usage: bootstrap".to_string());
            return;
        }
        self.start_bootstrap_wizard();
    }

    pub(in crate::tui_shell) fn cmd_logout(&mut self, _args: &[String]) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let cfg = match ws.store.read_config() {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("read config: {:#}", err));
                return;
            }
        };

        let Some(remote) = cfg.remote else {
            self.push_error("no remote configured".to_string());
            return;
        };

        if let Err(err) = ws.store.clear_remote_token(&remote) {
            self.push_error(format!("clear remote token: {:#}", err));
            return;
        }

        self.push_output(vec!["logged out".to_string()]);
        self.refresh_root_view();
    }
}
