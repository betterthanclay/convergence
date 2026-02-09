use super::prompts;

pub(super) fn on_bootstrap_url(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_string();
    if v.is_empty() {
        abort_bootstrap(app, "bootstrap: missing url");
        return;
    }
    if let Some(w) = app.bootstrap_wizard.as_mut() {
        w.url = Some(v);
    }
    let default = app
        .bootstrap_wizard
        .as_ref()
        .and_then(|w| w.repo.clone())
        .unwrap_or_else(|| "test".to_string());
    prompts::open_repo_prompt(app, default);
}

pub(super) fn on_bootstrap_repo(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_string();
    if v.is_empty() {
        abort_bootstrap(app, "bootstrap: missing repo");
        return;
    }
    if let Some(w) = app.bootstrap_wizard.as_mut() {
        w.repo = Some(v);
    }
    let default = app
        .bootstrap_wizard
        .as_ref()
        .map(|w| w.scope.clone())
        .unwrap_or_else(|| "main".to_string());
    prompts::open_scope_prompt(app, default);
}

pub(super) fn on_bootstrap_scope(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_string();
    if v.is_empty() {
        abort_bootstrap(app, "bootstrap: missing scope");
        return;
    }
    if let Some(w) = app.bootstrap_wizard.as_mut() {
        w.scope = v;
    }
    let default = app
        .bootstrap_wizard
        .as_ref()
        .map(|w| w.gate.clone())
        .unwrap_or_else(|| "dev-intake".to_string());
    prompts::open_gate_prompt(app, default);
}

pub(super) fn on_bootstrap_gate(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_string();
    if v.is_empty() {
        abort_bootstrap(app, "bootstrap: missing gate");
        return;
    }
    if let Some(w) = app.bootstrap_wizard.as_mut() {
        w.gate = v;
    }
    prompts::open_token_prompt(app);
}

pub(super) fn on_bootstrap_token(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_string();
    if v.is_empty() {
        abort_bootstrap(app, "bootstrap: missing token");
        return;
    }
    if let Some(w) = app.bootstrap_wizard.as_mut() {
        w.bootstrap_token = Some(v);
    }
    prompts::open_handle_prompt(app);
}

pub(super) fn on_bootstrap_handle(app: &mut crate::tui_shell::App, value: String) {
    let v = value.trim().to_string();
    if v.is_empty() {
        abort_bootstrap(app, "bootstrap: missing handle");
        return;
    }
    if let Some(w) = app.bootstrap_wizard.as_mut() {
        w.handle = v;
    }
    prompts::open_display_name_prompt(app);
}

pub(super) fn on_bootstrap_display_name(app: &mut crate::tui_shell::App, value: String) {
    if let Some(w) = app.bootstrap_wizard.as_mut() {
        let v = value.trim().to_string();
        w.display_name = if v.is_empty() { None } else { Some(v) };
    }
    app.finish_bootstrap_wizard();
}

fn abort_bootstrap(app: &mut crate::tui_shell::App, message: &str) {
    app.push_error(message.to_string());
    app.bootstrap_wizard = None;
}
