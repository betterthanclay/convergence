use crate::model::RemoteConfig;

use super::login_bootstrap_validate::parse_bootstrap_inputs;

impl super::super::App {
    pub(in crate::tui_shell) fn apply_login_config(
        &mut self,
        base_url: String,
        token: String,
        repo_id: String,
        scope: String,
        gate: String,
    ) {
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

        let remote = RemoteConfig {
            base_url: base_url.clone(),
            token: None,
            repo_id,
            scope,
            gate,
        };

        if let Err(err) = ws.store.set_remote_token(&remote, &token) {
            self.push_error(format!("store remote token: {:#}", err));
            return;
        }

        cfg.remote = Some(remote);
        if let Err(err) = ws.store.write_config(&cfg) {
            self.push_error(format!("write config: {:#}", err));
            return;
        }

        self.push_output(vec![format!("logged in to {}", base_url)]);
        self.refresh_root_view();
    }

    pub(in crate::tui_shell) fn finish_bootstrap_wizard(&mut self) {
        let Some(w) = self.bootstrap_wizard.clone() else {
            self.push_error("bootstrap wizard not active".to_string());
            return;
        };
        self.bootstrap_wizard = None;

        let (base_url, bootstrap_token, handle, repo_id) = match parse_bootstrap_inputs(&w) {
            Ok(inputs) => (
                inputs.base_url,
                inputs.bootstrap_token,
                inputs.handle,
                inputs.repo_id,
            ),
            Err(err) => {
                self.push_error(err);
                return;
            }
        };

        let remote = RemoteConfig {
            base_url: base_url.clone(),
            token: None,
            repo_id: repo_id.clone(),
            scope: w.scope.clone(),
            gate: w.gate.clone(),
        };

        let client = match crate::remote::RemoteClient::new(remote.clone(), bootstrap_token) {
            Ok(c) => c,
            Err(err) => {
                self.push_error(format!("bootstrap: {:#}", err));
                return;
            }
        };

        let bootstrap = match client.bootstrap_first_admin(&handle, w.display_name.clone()) {
            Ok(r) => r,
            Err(err) => {
                self.push_error(format!("bootstrap: {:#}", err));
                return;
            }
        };

        self.apply_login_config(
            base_url.clone(),
            bootstrap.token.token.clone(),
            repo_id.clone(),
            w.scope.clone(),
            w.gate.clone(),
        );

        // Ensure the repo exists for the configured remote (best-effort).
        if let Some(client) = self.remote_client() {
            match client.get_repo(&repo_id) {
                Ok(_) => {
                    self.push_output(vec![format!("repo {} exists", repo_id)]);
                }
                Err(err) if err.to_string().contains("remote repo not found") => {
                    match client.create_repo(&repo_id) {
                        Ok(_) => self.push_output(vec![format!("created repo {}", repo_id)]),
                        Err(err) => self.push_error(format!("create repo: {:#}", err)),
                    }
                }
                Err(err) => {
                    self.push_error(format!("get repo: {:#}", err));
                }
            }
        }

        self.push_output(vec![
            format!("bootstrapped admin {}", bootstrap.user.handle),
            "Restart the server without --bootstrap-token.".to_string(),
        ]);
        self.refresh_root_view();
    }
}
