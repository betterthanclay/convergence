use super::*;

impl App {
    pub(in crate::tui_shell) fn cmd_remote_unset(&mut self, args: &[String]) {
        let _ = args;
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let mut cfg = match ws.store.read_config() {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("read config: {:#}", err));
                return;
            }
        };

        if let Some(remote) = cfg.remote.take()
            && let Err(err) = ws.store.clear_remote_token(&remote)
        {
            self.push_error(format!("clear remote token: {:#}", err));
            return;
        }

        cfg.remote = None;
        if let Err(err) = ws.store.write_config(&cfg) {
            self.push_error(format!("write config: {:#}", err));
            return;
        }
        self.push_output(vec!["remote unset".to_string()]);
        self.refresh_root_view();
    }
}
