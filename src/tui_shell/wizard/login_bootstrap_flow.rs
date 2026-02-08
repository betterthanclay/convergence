use super::super::TextInputAction;
use super::types::{BootstrapWizard, LoginWizard};

impl super::super::App {
    pub(in crate::tui_shell) fn start_bootstrap_wizard(&mut self) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let remote = ws.store.read_config().ok().and_then(|c| c.remote);

        let default_url = remote
            .as_ref()
            .map(|r| r.base_url.clone())
            .unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
        let default_repo = remote
            .as_ref()
            .map(|r| r.repo_id.clone())
            .unwrap_or_else(|| "test".to_string());
        let default_scope = remote
            .as_ref()
            .map(|r| r.scope.clone())
            .unwrap_or_else(|| "main".to_string());
        let default_gate = remote
            .as_ref()
            .map(|r| r.gate.clone())
            .unwrap_or_else(|| "dev-intake".to_string());

        self.bootstrap_wizard = Some(BootstrapWizard {
            url: Some(default_url.clone()),
            bootstrap_token: None,
            handle: "admin".to_string(),
            display_name: None,
            repo: Some(default_repo),
            scope: default_scope,
            gate: default_gate,
        });

        self.open_text_input_modal(
            "Bootstrap",
            "url> ",
            TextInputAction::BootstrapUrl,
            Some(default_url),
            vec![
                "Server base URL (example: http://127.0.0.1:8080)".to_string(),
                "Start converge-server with --bootstrap-token first.".to_string(),
            ],
        );
    }

    pub(in crate::tui_shell) fn start_login_wizard(&mut self) {
        let Some(ws) = self.require_workspace() else {
            return;
        };

        let remote = ws.store.read_config().ok().and_then(|c| c.remote);

        let default_url = remote.as_ref().map(|r| r.base_url.clone());
        let default_repo = remote.as_ref().map(|r| r.repo_id.clone());
        let default_scope = remote
            .as_ref()
            .map(|r| r.scope.clone())
            .unwrap_or_else(|| "main".to_string());
        let default_gate = remote
            .as_ref()
            .map(|r| r.gate.clone())
            .unwrap_or_else(|| "dev-intake".to_string());

        self.login_wizard = Some(LoginWizard {
            url: default_url.clone(),
            token: None,
            repo: default_repo,
            scope: default_scope,
            gate: default_gate,
        });

        self.open_text_input_modal(
            "Login",
            "url> ",
            TextInputAction::LoginUrl,
            default_url,
            vec![
                "Remote base URL (example: https://example.com)".to_string(),
                "Esc cancels; Enter continues.".to_string(),
            ],
        );
    }
}
