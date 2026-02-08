use super::super::TextInputAction;
use super::login_bootstrap_validate::validate_login_inputs;

impl super::super::App {
    pub(in crate::tui_shell) fn continue_bootstrap_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.bootstrap_wizard.is_none() {
            self.push_error("bootstrap wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::BootstrapUrl => self.bootstrap_transition_url(value),
            TextInputAction::BootstrapRepo => self.bootstrap_transition_repo(value),
            TextInputAction::BootstrapScope => self.bootstrap_transition_scope(value),
            TextInputAction::BootstrapGate => self.bootstrap_transition_gate(value),
            TextInputAction::BootstrapToken => self.bootstrap_transition_token(value),
            TextInputAction::BootstrapHandle => self.bootstrap_transition_handle(value),
            TextInputAction::BootstrapDisplayName => self.bootstrap_transition_display_name(value),
            _ => self.push_error("unexpected bootstrap wizard input".to_string()),
        }
    }

    pub(in crate::tui_shell) fn continue_login_wizard(
        &mut self,
        action: TextInputAction,
        value: String,
    ) {
        if self.login_wizard.is_none() {
            self.push_error("login wizard not active".to_string());
            return;
        }

        match action {
            TextInputAction::LoginUrl => {
                if let Some(w) = self.login_wizard.as_mut() {
                    w.url = Some(value);
                }
                self.open_text_input_modal(
                    "Login",
                    "token> ",
                    TextInputAction::LoginToken,
                    None,
                    vec![
                        "Access token (will be stored locally).".to_string(),
                        "Tip: paste it, then Enter.".to_string(),
                    ],
                );
            }
            TextInputAction::LoginToken => {
                if let Some(w) = self.login_wizard.as_mut() {
                    w.token = Some(value);
                }
                let repo_initial = self.login_wizard.as_ref().and_then(|w| w.repo.clone());
                self.open_text_input_modal(
                    "Login",
                    "repo> ",
                    TextInputAction::LoginRepo,
                    repo_initial,
                    vec!["Repo id".to_string()],
                );
            }
            TextInputAction::LoginRepo => {
                if let Some(w) = self.login_wizard.as_mut() {
                    w.repo = Some(value);
                }
                let scope_initial = self.login_wizard.as_ref().map(|w| w.scope.clone());
                self.open_text_input_modal(
                    "Login",
                    "scope> ",
                    TextInputAction::LoginScope,
                    scope_initial,
                    vec!["Scope id".to_string()],
                );
            }
            TextInputAction::LoginScope => {
                if let Some(w) = self.login_wizard.as_mut()
                    && !value.is_empty()
                {
                    w.scope = value;
                }
                let gate_initial = self.login_wizard.as_ref().map(|w| w.gate.clone());
                self.open_text_input_modal(
                    "Login",
                    "gate> ",
                    TextInputAction::LoginGate,
                    gate_initial,
                    vec!["Gate id".to_string()],
                );
            }
            TextInputAction::LoginGate => {
                if let Some(w) = self.login_wizard.as_mut()
                    && !value.is_empty()
                {
                    w.gate = value;
                }

                let (base_url, token, repo_id, scope, gate) = match self.login_wizard.as_ref() {
                    Some(w) => {
                        let base_url = w.url.clone().unwrap_or_default();
                        let token = w.token.clone().unwrap_or_default();
                        let repo_id = w.repo.clone().unwrap_or_default();
                        let scope = w.scope.clone();
                        let gate = w.gate.clone();
                        (base_url, token, repo_id, scope, gate)
                    }
                    None => {
                        self.push_error("login wizard not active".to_string());
                        return;
                    }
                };

                if let Err(err) = validate_login_inputs(&base_url, &token, &repo_id) {
                    self.push_error(err);
                    self.login_wizard = None;
                    return;
                }

                self.login_wizard = None;
                self.apply_login_config(base_url, token, repo_id, scope, gate);
            }

            _ => self.push_error("unexpected login wizard input".to_string()),
        }
    }

    fn bootstrap_transition_url(&mut self, value: String) {
        let v = value.trim().to_string();
        if v.is_empty() {
            self.push_error("bootstrap: missing url".to_string());
            self.bootstrap_wizard = None;
            return;
        }
        if let Some(w) = self.bootstrap_wizard.as_mut() {
            w.url = Some(v);
        }
        let default = self
            .bootstrap_wizard
            .as_ref()
            .and_then(|w| w.repo.clone())
            .unwrap_or_else(|| "test".to_string());
        self.open_text_input_modal(
            "Bootstrap",
            "repo> ",
            TextInputAction::BootstrapRepo,
            Some(default),
            vec![
                "Repo id to use for the client config.".to_string(),
                "If it doesn't exist, the wizard will create it.".to_string(),
            ],
        );
    }

    fn bootstrap_transition_repo(&mut self, value: String) {
        let v = value.trim().to_string();
        if v.is_empty() {
            self.push_error("bootstrap: missing repo".to_string());
            self.bootstrap_wizard = None;
            return;
        }
        if let Some(w) = self.bootstrap_wizard.as_mut() {
            w.repo = Some(v);
        }
        let default = self
            .bootstrap_wizard
            .as_ref()
            .map(|w| w.scope.clone())
            .unwrap_or_else(|| "main".to_string());
        self.open_text_input_modal(
            "Bootstrap",
            "scope> ",
            TextInputAction::BootstrapScope,
            Some(default),
            vec!["Default scope for remote operations.".to_string()],
        );
    }

    fn bootstrap_transition_scope(&mut self, value: String) {
        let v = value.trim().to_string();
        if v.is_empty() {
            self.push_error("bootstrap: missing scope".to_string());
            self.bootstrap_wizard = None;
            return;
        }
        if let Some(w) = self.bootstrap_wizard.as_mut() {
            w.scope = v;
        }
        let default = self
            .bootstrap_wizard
            .as_ref()
            .map(|w| w.gate.clone())
            .unwrap_or_else(|| "dev-intake".to_string());
        self.open_text_input_modal(
            "Bootstrap",
            "gate> ",
            TextInputAction::BootstrapGate,
            Some(default),
            vec!["Default gate for remote operations.".to_string()],
        );
    }

    fn bootstrap_transition_gate(&mut self, value: String) {
        let v = value.trim().to_string();
        if v.is_empty() {
            self.push_error("bootstrap: missing gate".to_string());
            self.bootstrap_wizard = None;
            return;
        }
        if let Some(w) = self.bootstrap_wizard.as_mut() {
            w.gate = v;
        }

        self.open_text_input_modal(
            "Bootstrap",
            "bootstrap token> ",
            TextInputAction::BootstrapToken,
            None,
            vec![
                "One-time bootstrap token (the same value passed to converge-server --bootstrap-token)."
                    .to_string(),
                "Generate one: openssl rand -hex 32".to_string(),
            ],
        );
    }

    fn bootstrap_transition_token(&mut self, value: String) {
        let v = value.trim().to_string();
        if v.is_empty() {
            self.push_error("bootstrap: missing token".to_string());
            self.bootstrap_wizard = None;
            return;
        }
        if let Some(w) = self.bootstrap_wizard.as_mut() {
            w.bootstrap_token = Some(v);
        }
        self.open_text_input_modal(
            "Bootstrap",
            "admin handle> ",
            TextInputAction::BootstrapHandle,
            Some("admin".to_string()),
            vec![
                "Admin handle to create (one-time).".to_string(),
                "Response includes a plaintext admin token; it will be stored in .converge/state.json"
                    .to_string(),
            ],
        );
    }

    fn bootstrap_transition_handle(&mut self, value: String) {
        let v = value.trim().to_string();
        if v.is_empty() {
            self.push_error("bootstrap: missing handle".to_string());
            self.bootstrap_wizard = None;
            return;
        }
        if let Some(w) = self.bootstrap_wizard.as_mut() {
            w.handle = v;
        }
        self.open_text_input_modal(
            "Bootstrap",
            "display name (optional)> ",
            TextInputAction::BootstrapDisplayName,
            None,
            vec!["Optional display name (leave blank to skip).".to_string()],
        );
    }

    fn bootstrap_transition_display_name(&mut self, value: String) {
        if let Some(w) = self.bootstrap_wizard.as_mut() {
            let v = value.trim().to_string();
            w.display_name = if v.is_empty() { None } else { Some(v) };
        }
        self.finish_bootstrap_wizard();
    }
}
